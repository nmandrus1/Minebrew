use futures::stream::{self, StreamExt};
use anyhow::Error;

use minebrew_lib::{ Modrinth, Empty, VersionList, Version, SearchResponse, version::VersionType};
use minebrew_cfg::{Command, Options};
use minebrew_db::{ ModDB, DBError };

use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use std::sync::Arc;

/// Modrinth contains all the info 
/// on currently installed packages
pub struct Minebrew {
    modrinth: Modrinth<Empty>,
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
            Command::Update => mbrew.update().await,
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

    /// An iterator over the Versions of a Mod given the project ID
    async fn versions(&self, pid: &str) -> VersionList {
        self.modrinth.project(pid)
             .list_versions()
             .game_version(&self.opts.target)
             .get().await.unwrap()
    }

    /// List all the files in the download queue and confrim with the user
    fn confirm_queue(&self, queue: &[Version]) {
        // print file names left to right until the next file name would 
        // mean more than 80 characters printed, then return and start over
        let mut chars_left: usize = 0;
        println!("\nMods ({})", queue.len());
        queue.iter().for_each(|v| {
            let name_len = v.file().file_name().len();
            match chars_left.checked_sub(name_len + 2) {
                Some(left) => {
                    print!("{}  ", v);
                    chars_left = left;
                },
                None => {
                    print!("\n\t{}  ", v);
                    chars_left = 80 - name_len;
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
    // async fn download_files(&mut self, download_dir: &Path, queue: Vec<Version>) -> anyhow::Result<()> {
    //     let num_downloads = queue.len();
    //     let (mut finished, total) = (0, num_downloads);
    //
    //     print!("Downloaded\t[{}/{}]", finished, total);
    //     std::io::stdout().flush().unwrap();
    //     
    //     let mut downloads = stream::iter(queue.into_iter())
    //         .map(|v| v.download_to(download_dir, &self.modrinth.client))
    //         .buffer_unordered(num_downloads);
    //         
    //     while let Some(download) = downloads.next().await {
    //
    //         // update mod database or insert the new mod
    //         self.db.replace_or_insert(download?);
    //         
    //         finished += 1;
    //         print!("\x1B[2K\x1B[60DDownloaded\t[{}/{}]", finished, total);
    //         std::io::stdout().flush().unwrap();
    //     }
    //
    //     Ok(())
    // }
    
    /// Downloads all the files in the download queue
    async fn download_files_exp(&mut self, download_dir: Arc<PathBuf>, queue: Vec<Version>) -> anyhow::Result<()> {
        let num_downloads = queue.len();
        let (mut finished, total) = (0, num_downloads);
        let mut futs = Vec::with_capacity(num_downloads);

        print!("Downloaded\t[{}/{}]", finished, total);
        std::io::stdout().flush().unwrap();
        
        for v in queue {
            let client = self.modrinth.client.clone();
            futs.push(tokio::spawn(v.download_to(download_dir.clone(), client)))
        }

        for fut in futs {
            let download = fut.await??;
            // update mod database or insert the new mod
            self.db.replace_or_insert(download);
            
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
    //  Compare with the currently installed mods
    //      - Dont install if its the same version
    //      - if its a more up-to-date version then 
    //        add it to the download queue
    //
    //  Download file
    // 
    //  Update Database
    //  - if the mod is a newly installed mod then 
    //    add it to the database
    //  - if the mod has been updated then
    //    update the database
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
                // select the right mod
                match sr.pick_result() {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("Error: {} not found...", q);
                        std::process::exit(1)
                    }
                }
        });
        
        // create a vector of Version structs that represents possible 
        // downloads of its corresponding mod
        let mut download_queue = stream::iter(results)
            .map(|sr| self.versions(&sr.project_id))
            .buffer_unordered(num_queries)
            // map to a single Version via filtering
            .map(|vlist| async { 
                let version = vlist.into_iter()
                    .find(|v| {
                        v.file().is_primary() 
                            && matches!(v.version_type, VersionType::Release | VersionType::Beta)
                    });

                // There may not have been a version that met our requirements
                match version {
                    Some(v) => v,
                    None => {
                        eprintln!("Version was found but does not meet requirements...");
                        std::process::exit(1);
                    }
                }
            }).buffer_unordered(num_queries).collect::<Vec<Version>>().await;

        // keep only those that need to be installed, 
        // ignoring mods that haven't been updated
        download_queue.retain(|v| {
            let ret = match self.db.get(v.pid()) {
                Some(old_v) => old_v != v,
                None => true
            }; 

            if !ret {println!("{} is up to date...", &v.name)}
            ret
        });

        // don't ask for user input if there are no mods to install!
        if download_queue.is_empty() {
            println!("All mods up to date!");
            return Ok(())
        }

        // path to mods folder
        let mods_folder = self.opts.directory.join("mods");

        println!("\nSearching for mods folder...");
        // if mods folder doesn't exist then make one
        if !mods_folder.exists() {
        println!("Not found, creating mods folder...");
            std::fs::create_dir_all(&mods_folder)?;
        } else {
            println!("Mods folder found...");
        }

        self.confirm_queue(&download_queue);

        // download all the files we've gathered
        self.download_files_exp(Arc::new(mods_folder), download_queue).await?;

        match self.db.save_to_file() {
            Ok(_) => { println!("Success!"); Ok(()) },
            Err(e) => match e {
                DBError::IOError(e) =>  anyhow::bail!("{e}"),
                DBError::SerializationErr(e) => anyhow::bail!("Error: {e} \n\nWhoops! Minebrew encountered an error that is \
                              100% the developers fault, please report this to the Minebrew \
                              development team along with the exact options and queries used, \
                              thank you!")
            }
        }
    }
    
    pub async fn update(&mut self) -> anyhow::Result<()> {
        let num_queries = self.db.len();
        let mut download_queue = stream::iter(self.db.values())
            .map(|v| self.versions(v.pid()))
            .buffer_unordered(num_queries)
            // map to a single Version via filtering
            .map(|vlist| async { 
                let version = vlist.into_iter()
                    .find(|v| {
                        v.file().is_primary() 
                            && matches!(v.version_type, VersionType::Release | VersionType::Beta)
                    });

                // There may not have been a version that met our requirements
                match version {
                    Some(v) => v,
                    None => {
                        eprintln!("Version was found but does not meet requirements...");
                        std::process::exit(1);
                    }
                }
            }).buffer_unordered(num_queries).collect::<Vec<Version>>().await;

        println!("Searching for updates...");
        download_queue.retain(|v| {
            let ret = match self.db.get(v.pid()) {
                Some(old_v) => old_v != v,
                None => true
            }; 

            if !ret {println!("{} is up to date...", &v.name)}
            ret
        });

        // don't ask for user input if there are no mods to install!
        if download_queue.is_empty() {
            println!("All mods up to date!");
            return Ok(())
        }
        
        self.confirm_queue(&download_queue);

        // download all the files we've gathered
        let mods_folder = &self.opts.directory.join("mods");
        // self.download_files(mods_folder, download_queue).await?;

        match self.db.save_to_file() {
            Ok(_) => { println!("Success!"); Ok(()) },
            Err(e) => match e {
                DBError::IOError(e) =>  anyhow::bail!("{e}"),
                DBError::SerializationErr(e) => anyhow::bail!("Error: {e} \n\nWhoops! Minebrew encountered an error that is \
                              100% the developers fault, please report this to the Minebrew \
                              development team along with the exact options and queries used, \
                              thank you!")
            }
        }

    }
}

#[tokio::main]
async fn main() -> Result<(), Error> { 
    Minebrew::run().await
}
