use futures::stream::{self, StreamExt};
use anyhow::Error;

use minebrew_lib::{ Modrinth, Empty, VersionList, SearchResponse, SearchResult };
use minebrew_cfg::{Command, Options};
use minebrew_db::{ ModDB, DBError };

use std::io::Write;
use std::path::Path;

/// Modrinth contains all the info 
/// on currently installed packages
pub struct Minebrew {
    modrinth: Modrinth<Empty>,
    download_queue: VersionList,
    opts: Options,
    db: ModDB,
}

/// Default init for Minebrew
impl Default for Minebrew {
    fn default() -> Self {
        let opts = Options::load();
        
        println!("Loading local database...");
        let db = match ModDB::load(&opts.directory.join("minebrew.json")) {
            Ok(db) => db,
            Err(e) => match e {
                DBError::IOError(e) => { eprintln!("Error: {e}"); std::process::exit(1) },
                DBError::SerializationErr(e) => {
                    eprintln!("Error: {e} \n\nWhoops! Minebrew encountered an error that is \
                              100% the developers fault, please report this to the Minebrew \
                              development team along with the exact options and queries used, \
                              thank you!");
                    std::process::exit(1);
                }
            }
        };
        
        Self {
            modrinth: Modrinth::new(),
            download_queue: Vec::with_capacity(5).into(),
            opts,
            db
        }
    }
}

impl Minebrew {
    // - Parse args and load config
    /// Starts Minebrew
    pub async fn run() -> anyhow::Result<()> {
        let mut mbrew = Minebrew::default();
        match mbrew.opts.cmd {
            Command::Install => mbrew.install().await,
            _ => Ok(()) 
        }
    }
    
    /// takes a query and returns the Response and the corresponding query
    async fn search<'a>(&self, query: &'a str) -> (SearchResponse, &'a str) {
        let mut resp = self.modrinth.search(query).version(&self.opts.target);
        let sresp = resp.get().await;
        if let Err(e) = sresp {
            eprintln!("Error searching Modrinth: {}", e);
            std::process::exit(1);
        } else {
            (sresp.unwrap(), query)
        }
    }

    /// given a project id and a version number add a compatible version to the download queue
    async fn add_to_queue<'a>(&mut self, results: impl Iterator<Item=&SearchResult>) -> anyhow::Result<()> {
        let size = results.size_hint().1.unwrap();
        // iterate over the search results and generate an API request for a VersionList,
        // removing the first element and then pushing that to the download queue
        let mut versions = stream::iter(results).map(|r| async { 
            let mut vlist = self.modrinth
                .project(&r.project_id)
                .list_versions()
                .game_version(&self.opts.target)
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
    async fn download_files(&mut self, download_dir: &Path) -> anyhow::Result<()> {
        let num_downloads = self.download_queue.len();
        let (mut finished, total) = (0, num_downloads);

        // DOWNLOAD QUEUE MOVED OUT OF SELF
        let download_queue = std::mem::take(&mut self.download_queue);
        
        let mut downloads = stream::iter(download_queue.into_iter())
            .map(|v| async {
                v.download_to(download_dir, &self.modrinth.client).await.unwrap();
                v
            })
            .buffer_unordered(num_downloads);
            
        while let Some(download) = downloads.next().await {

            // update mod database or insert the new mod
            self.db.entry(download.project_id.clone())
                .or_insert_with_key(|_| download);
            
            finished += 1;
            print!("\x1B[2K\x1B[60DDownloaded\t[{}/{}]", finished, total);
            std::io::stdout().flush().unwrap();
        }

        // let dir = get_mc_dir().join("minebrew.json");
        let mut dir = download_dir.to_path_buf();
        dir.pop();
        dir.push("minebrew.json");
        
        match self.db.save_to_file(&dir) {
            Ok(_) => Ok(()),
            Err(e) => match e {
                DBError::IOError(e) => { eprintln!("Error: {e}"); std::process::exit(1) },
                DBError::SerializationErr(e) => {
                    eprintln!("Error: {e} \n\nWhoops! Minebrew encountered an error that is \
                              100% the developers fault, please report this to the Minebrew \
                              development team along with the exact options and queries used, \
                              thank you!");
                    std::process::exit(1);
                }
            }
        }
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
        let num_queries = self.opts.queries.len();
        let queries = std::mem::take(&mut self.opts.queries);

        // Loop through every query made 
        // Turns quries into ModFile structs which have a download link
        // let searches = Search::new(&i_opts.queries, &i_opts.target);

        println!("Searching modrinth for {} mods", &self.opts.target);

        // Make API calls
        let mut resps: Vec<(SearchResponse, &str)> = stream::iter(queries.iter())
            .map(|q| self.search(q))
            .buffer_unordered(num_queries)
            .collect().await;

        // filter results
        let results = resps.iter_mut()
            .map(|(sr, q)| { 
                // filter out search results from each response
                sr.filter(q, 2);
                match sr.pick_result() {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("Error: {} not found...", q);
                        std::process::exit(1)
                    }
                }
        });

        // queue versions to download
        // TEMPORARY until we can figure out how to do this better
        self.add_to_queue(results).await?;

        // path to mods folder
        let mods_folder = &self.opts.directory.join("mods");

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
        self.download_files(mods_folder).await?;

        println!("\nSuccess!");
        Ok(())
    }
}



#[tokio::main]
async fn main() -> Result<(), Error> { 
    Minebrew::run().await
}
