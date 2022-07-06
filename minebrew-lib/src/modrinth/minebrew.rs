use reqwest::Client;
use anyhow::Result;

use super::search::{Search, SearchResponse, SearchResult};
use super::version::{Version, ModFile};

use std::path::Path;

/// Minebrew struct is going to do all the interfacing between 
/// modrinth.com and the user as well as contain all the info 
/// on currently installed packages
pub struct Minebrew {
    client: Client
    //db goes here
}

/// Default init for Minebrew
impl Default for Minebrew {
    fn default() -> Self {
        Self { client: Client::new() }
    }
}

impl Minebrew {
    /// Take a reference to a search struct and return a 
    /// Vec of Search Responses wrapped in a Result
    pub async fn search<'a>(&self, search: &'a Search<'a>) -> Result<Vec<SearchResponse>> {
        // Vec to store JoinHandles to Futures
        let mut handles = Vec::with_capacity(search.queries.len());
        let mut ret = Vec::with_capacity(search.queries.len());

        // start all requests asyncronously
        for url in search.urls() {
            let handle = tokio::spawn(self.client.get(url).send());
            handles.push(handle);
        }

        for handle in handles {
            let resp = handle.await??;
            let search_resp = resp.json::<SearchResponse>().await?;
            ret.push(search_resp);
        }

        Ok(ret)
    }

    pub async fn files_from_results(&self, results: &[SearchResult], version: &str) -> Result<Vec<ModFile>> {
        let mut handles = Vec::with_capacity(results.len());
        let mut ret = Vec::with_capacity(results.len());

        for res in results {
            let url = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]", res.slug, version);
            let handle = tokio::spawn(self.client.get(url).send());
            handles.push(handle);
        }

        for handle in handles {
            let resp = handle.await??;
            let mut vers: Vec<Version> = resp.json().await?;
            // just removing the first item in the vector
            let mut ver = vers.remove(0);

            let file = match ver.files.iter().enumerate().find(|(_, f)| f.primary) {
                Some((i, _)) => ver.files.remove(i),
                None => ver.files.remove(0),
            };

            ret.push(file);
        }

        Ok(ret)
    }

    pub async fn download_files(&self, files: &[ModFile], download_dir: &Path) -> Result<()> {
        let mut handles = Vec::with_capacity(files.len());

        for file in files {
            let handle = tokio::spawn(self.client.get(&file.url).send());
            handles.push((&file.filename, handle));
            println!("Downloading {}", file.filename);
        }

        for (filename, handle) in handles {
            let resp = handle.await??;
            let bytes = resp.bytes().await?;
            println!("Writing {} to disk...", filename);
            std::fs::write(download_dir.join(filename), &bytes)?;
        }

        Ok(())
    }
}
