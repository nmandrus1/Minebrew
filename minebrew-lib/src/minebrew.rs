use futures::stream::{self, StreamExt};

use super::search::{ SearchResult, SearchResponse };
use super::version::*;

use super::{ Modrinth, EmptyReq };

use std::io::Write;
use std::path::Path;

/// Modrinth contains all the info 
/// on currently installed packages
pub struct Minebrew {
    modrinth: Modrinth<EmptyReq>,
    download_queue: Vec<Version>,
    //db goes here
}

/// Default init for Minebrew
impl Default for Minebrew {
    fn default() -> Self {
        Self {
            modrinth: Modrinth::new(),
            download_queue: Vec::with_capacity(5),
        }
    }
}

impl Minebrew {
    /// Take a reference to a search struct and return a 
    /// Vec of Search Responses wrapped in a Result
    pub async fn search<'a>(&self, query: String, target: &'a str) -> SearchResponse {
        // let resp = tokio::spawn(self.modrinth.search(query)).await.unwrap().unwrap();
        let resp = self.modrinth.search(&query).version(target).get().await.unwrap();
        let mut s_resp = resp.json::<SearchResponse>().await.unwrap();
        s_resp.set_query(query);
        s_resp
    }

    // given a project id and a version number add a compatible version to the download queue
    pub async fn add_to_queue<'a>(&mut self, results: impl Iterator<Item=&SearchResult>, version: &'a str) -> anyhow::Result<()> {
        let size = results.size_hint().1.unwrap();
        let futs = results.map(|r| self.modrinth.project(&r.project_id).list_versions().game_version(version).get());

        let mut versions = stream::iter(futs).map(|f| async {
            let resp = f.await.unwrap();
            let mut versions = resp.json::<Vec<Version>>().await.unwrap();
            versions.remove(0)
        }).buffer_unordered(size);

        while let Some(v) = versions.next().await {
            self.download_queue.push(v);
        }

        Ok(())
    }

    pub fn confirm_queue(&self) {
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
    pub fn user_in(prompt: &str, expected: &[&str]) -> bool {
        print!("\n\n{}", prompt);
        std::io::stdout().flush().unwrap(); // flush buffer to print everything

        // get user input
        let mut input = String::with_capacity(3);
        std::io::stdin().read_line(&mut input).unwrap();

        expected.iter().any(|e| e == &input.trim())
    }

    // pub async fn files_from_results(&self, results: &[SearchResult], version: &str) -> Vec<ModFile> {
    //     stream::iter(results.iter()).map(|res| async {
    //         let url = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]", res.slug, version);
    //         let resp = tokio::spawn(self.client.get(url).send()).await.unwrap().unwrap();
    //
    //         let mut vers = tokio::spawn(resp.json::<Vec<Version>>()).await.unwrap().unwrap();
    //         // just removing the first item in the vector
    //         let mut ver = vers.remove(0);
    //
    //         match ver.files.iter().enumerate().find(|(_, f)| f.primary) {
    //             Some((i, _)) => ver.files.swap_remove(i),
    //             None => ver.files.swap_remove(0),
    //         }
    //     }).buffer_unordered(results.len()).collect().await
    // }

    pub async fn download_files(&self, download_dir: &Path) -> anyhow::Result<()> {
        let (mut finished, total) = (0, self.download_queue.len());

        stream::iter(self.download_queue.iter())
            .for_each_concurrent(self.download_queue.len(), |v| async move {
                v.download_to(download_dir, &self.modrinth.client).await.unwrap();
                finished += 1;
                print!("\x1B[2K\x1B[60DDownloaded\t[{}/{}]", finished, total);
                std::io::stdout().flush().unwrap();
            }).await;
        //
        // let mut total_bytes = 0_usize;
        // for file in files {
        //     let handle = tokio::spawn(self.client.get(&file.url).send());
        //     handles.push((&file.filename, handle));
        //     total_bytes += file.size;
        // }
        //
        // let mut downloaded = 0_usize;
        // for (filename, handle) in handles {
        //     let resp = handle.await??;
        //     let mut byte_stream = resp.bytes_stream();
        //     let mut file = std::fs::File::create(download_dir.join(filename))?;
        //
        //     while let Some(item) = byte_stream.next().await {
        //         let chunk = item?;
        //         downloaded += chunk.len();
        //         file.write_all(&chunk)?;
        //
        //         let percent = (downloaded as f32)/(total_bytes as f32);
        //         let progress = (percent * 30.0) as usize;
        //         let space = 30 - progress;
        //         let bar = format!("[{:#<p_width$}{: <s_width$}] {}%", "", "", (percent * 100.0) as usize, p_width=progress, s_width = space);
        //
        //         print!("\x1B[2K\x1B[60DDownloading...\t{}", bar);
        //         std::io::stdout().flush().unwrap();
        //     }
        // }

        Ok(())
    }
}
