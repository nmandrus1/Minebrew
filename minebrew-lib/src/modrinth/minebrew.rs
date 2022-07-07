use reqwest::Client;
use anyhow::Result;
use futures::stream::{self, StreamExt};

use super::search::{Search, SearchResponse, SearchResult};
use super::version::{Version, ModFile};

use std::io::Write;
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
    pub async fn search<'a>(&self, search: &'a Search<'a>) -> Vec<SearchResponse<'a>> {
        // start all requests asyncronously
        stream::iter(search.urls()).map(|(u, q)| async {
            let resp = tokio::spawn(self.client.get(u).send()).await.unwrap().unwrap();
            let mut s_resp = tokio::spawn(resp.json::<SearchResponse>()).await.unwrap().unwrap();
            s_resp.set_query(q);
            s_resp
        }).buffer_unordered(search.queries.len()).collect().await
    }

    pub async fn files_from_results(&self, results: &[SearchResult], version: &str) -> Vec<ModFile> {
        stream::iter(results.into_iter()).map(|res| async {
            let url = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]", res.slug, version);
            let resp = tokio::spawn(self.client.get(url).send()).await.unwrap().unwrap();

            let mut vers = tokio::spawn(resp.json::<Vec<Version>>()).await.unwrap().unwrap();
            // just removing the first item in the vector
            let mut ver = vers.remove(0);

            match ver.files.iter().enumerate().find(|(_, f)| f.primary) {
                Some((i, _)) => ver.files.swap_remove(i),
                None => ver.files.swap_remove(0),
            }
        }).buffer_unordered(results.len()).collect().await
    }

    pub async fn download_files(&self, files: &[ModFile], download_dir: &Path) -> Result<()> {
        let mut handles = Vec::with_capacity(files.len());

        let mut total_bytes = 0_usize;
        for file in files {
            let handle = tokio::spawn(self.client.get(&file.url).send());
            handles.push((&file.filename, handle));
            total_bytes += file.size;
        }

        let mut downloaded = 0_usize;
        for (filename, handle) in handles {
            let resp = handle.await??;
            let mut byte_stream = resp.bytes_stream();
            let mut file = std::fs::File::create(download_dir.join(filename))?;

            while let Some(item) = byte_stream.next().await {
                let chunk = item?;
                downloaded += chunk.len();
                file.write_all(&chunk)?;

                let percent = (downloaded as f32)/(total_bytes as f32);
                let progress = (percent * 30.0) as usize;
                let space = 30 - progress;
                let bar = format!("[{:#<p_width$}{: <s_width$}] {}%", "", "", (percent * 100.0) as usize, p_width=progress, s_width = space);

                print!("\x1B[2K\x1B[60DDownloading...\t{}", bar);
                std::io::stdout().flush();
            }
        }


        Ok(())
    }
}
