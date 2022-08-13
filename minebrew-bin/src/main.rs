use futures::stream::{self, StreamExt};

use anyhow::Error;

use minebrew_lib::{ Modrinth, Empty, VersionList, SearchResponse, SearchResult };
use minebrew_cfg::{ Options, Subcommands };

use std::io::Write;
use std::path::Path;

/// Modrinth contains all the info 
/// on currently installed packages
pub struct Minebrew {
    modrinth: Modrinth<Empty>,
    download_queue: VersionList,
    opts: Options,
    //db goes here
}

/// Default init for Minebrew
impl Default for Minebrew {
    fn default() -> Self {
        Self {
            modrinth: Modrinth::new(),
            download_queue: Vec::with_capacity(5).into(),
            opts: Options::parse(),
        }
    }
}

impl Minebrew {
    // - Parse args and load config
    /// Starts Minebrew
    pub async fn run() -> anyhow::Result<()> {
        let mut mbrew = Minebrew::default();
        match mbrew.opts.command {
            Subcommands::Install(_) => mbrew.install().await,
            _ => Ok(()) 
        }
    }
    
    /// Take a reference to a search struct and return a 
    /// Vec of Search Responses wrapped in a Result
    async fn search<'a>(&self, query: &'a str, target: &'a str) -> SearchResponse<'a> {
        let mut resp = self.modrinth.search(query).version(target);
        let sresp = resp.get().await;
        if let Err(e) = sresp {
            eprintln!("Error searching Modrinth: {}", e);
            std::process::exit(1);
        } else {
            sresp.unwrap()
        }
    }

    /// given a project id and a version number add a compatible version to the download queue
    async fn add_to_queue<'a>(&mut self, results: impl Iterator<Item=&SearchResult>, version: &'a str) -> anyhow::Result<()> {
        let size = results.size_hint().1.unwrap();
        // iterate over the search results and generate an API request for a VersionList,
        // removing the first element and then pushing that to the download queue
        let mut versions = stream::iter(results).map(|r| async { 
            let mut vlist = self.modrinth
                .project(&r.project_id)
                .list_versions()
                .game_version(version)
                .get().await.unwrap();
            vlist.remove(0)
        }).buffer_unordered(size);

        while let Some(v) = versions.next().await {
            self.download_queue.push(v);
        }

        Ok(())
    }

    /// List all the files in the download queue and confrim with the user
    fn confirm_queue(&self) {
        // print file names left to right until the next file name would 
        // mean more than 80 characters printed, then return and start over
        let mut chars_left: usize = 0;
        println!("\nMods ({})", self.download_queue.len());
        self.download_queue.iter().for_each(|v| {
            match chars_left.checked_sub(&v.file().filename.len() + 2) {
                Some(left) => {
                    print!("{}  ", v);
                    chars_left = left;
                },
                None => {
                    print!("\n\t{}  ", v);
                    chars_left = 80 - &v.file().filename.len();
                },
            }
        });

        if !Minebrew::user_in("Begin Installation? [y/n]", &["Y", "y", ""]) {
            std::process::exit(1);
        }
    }

    /// takes a prompt, returns twice, prints it then awaits user input,
    /// then it matches user input to make sure it is one of the expected inputs
    fn user_in(prompt: &str, expected: &[&str]) -> bool {
        print!("\n\n{}", prompt);
        std::io::stdout().flush().unwrap(); // flush buffer to print everything

        // get user input
        let mut input = String::with_capacity(3);
        std::io::stdin().read_line(&mut input).unwrap();

        expected.iter().any(|e| e == &input.trim())
    }

    /// Downloads all the files in the download queue
    async fn download_files(&self, download_dir: &Path) -> anyhow::Result<()> {
        let (mut finished, total) = (0, self.download_queue.len());

        let mut downloads = stream::iter(self.download_queue.iter())
            .map(|v| v.download_to(download_dir, &self.modrinth.client))
            .buffer_unordered(self.download_queue.len());

        // while let Some(_) = downloads.next().await {
        while (downloads.next().await).is_some() {
            finished += 1;
            print!("\x1B[2K\x1B[60DDownloaded\t[{}/{}]", finished, total);
            std::io::stdout().flush().unwrap();
        }

        Ok(())
    }

    // search for mods 
    //   - Make API calls
    //
    // filter results
    //   - Filter out results that have
    //     too high a levenshtein dist.
    //   - If more than 1 option remains
    //     get user input
    //
    // find the correct verion to install
    //   - Get a list of all the 
    //   versions of the mod
    //   - Pick the most recent version that 
    //     is acceptable to user
    //
    // download file
    pub async fn install(&mut self) -> anyhow::Result<()> {
        // unwraping is okay here because we should never not 
        // pass any other Subcommand variant other than Install
        let opts = self.opts.command.install_opts().unwrap();
        let num_queries = opts.queries.len();
        let target = opts.target;

        // Loop through every query made 
        // Turns quries into ModFile structs which have a download link
        // let searches = Search::new(&i_opts.queries, &i_opts.target);

        println!("Searching modrinth for {} mods", &target);

        // Make API calls
        let mut resps: Vec<SearchResponse> = stream::iter(opts.queries.iter())
            .map(|q| self.search(q, &target))
            .buffer_unordered(num_queries)
            .collect().await;

        // filter results
        let results = resps.iter_mut()
            .map(|sr| { 
                // filter out search results from each response
                sr.filter(2);
                sr.pick_result()
            });

        // queue versions to download
        // TEMPORARY until we can figure out how to do this better
        self.add_to_queue(results, &target).await?;

        // path to mods folder
        let mods_folder = opts.mc_dir.join("mods");

        println!("\nSearching for mods folder...");
        // if mods folder doesn't exist then make one
        if !mods_folder.exists() {
        println!("Not found, creating mods folder...");
            std::fs::create_dir_all(&mods_folder)?;
        } else {
            println!("Mods folder found...");
        }

        self.confirm_queue();

        // download all the files we've gathered
        self.download_files(&mods_folder).await?;

        println!("\nSuccess!");
        Ok(())
    }
}



#[tokio::main]
async fn main() -> Result<(), Error> { 
    Minebrew::run().await
}
