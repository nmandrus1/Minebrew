use serde::Deserialize;

use super::shared::*;

#[derive(Deserialize)]
struct Dependency {
    version_id: Option<String>,
    project_id: Option<String>,
    dependecy_type: DependencyType,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum VersionType {
    Release,
    Beta,
    Alpha
}

#[derive(Deserialize)]
pub struct ModFile {
    hashes: Hashes,
    pub url: String,
    pub filename: String,
    primary: bool,
    size: usize,
}

#[derive(Deserialize)]
pub struct Hashes {
    sha512: Option<String>,
    sha1: Option<String>,
}

#[derive(Deserialize)]
pub struct Versions (Vec<Version>);

/// struct that represents a particular downloadable version of a mod
#[derive(Deserialize)]
pub struct Version {
    name: String,
    version_number: String,
    #[serde(skip)]
    changelog: Option<String>,
    #[serde(skip)]
    dependecies: Option<Vec<Dependency>>,
    pub game_versions: Vec<String>,
    version_type: VersionType,
    #[serde(skip)]
    loaders: Vec<String>,
    #[serde(skip)]
    featured: bool,
    #[serde(skip)]
    id: String,
    #[serde(skip)]
    project_id: String,
    #[serde(skip)]
    author_id: String,
    #[serde(skip)]
    date_published: String,
    #[serde(skip)]
    changelog_url: Option<String>,
    pub files: Vec<ModFile>,
}

impl Version {
    pub fn search(slug: &str) -> Result<Vec<Version>, reqwest::Error> { 
        let json_str = match reqwest::blocking::get(format!("https://api.modrinth.com/v2/project/{}/version", slug)) {
            Err(e) => { // Handle ERROR case for GET request
                // if it is a connection error then let the user know
                if e.is_connect() {
                    eprintln!("Error connecting to host...");
                    std::process::exit(1)
                // If it isnt then something else happened that shouldnt have
                } else {
                    panic!("Unexpected reqwest error...")
                }
            },
            Ok(response) => response.text().unwrap() // Parse json on success
        };
 
        let versions: Vec<Version> = serde_json::from_str(&json_str).unwrap();
        Ok(versions)
    }

    pub fn download_file(url: &str) -> Vec<u8> {
        let res = reqwest::blocking::get(url).unwrap();
        res.bytes().unwrap().to_vec()
    }

    pub fn json_works() -> Self {
        let path = std::path::Path::new("/home/nels/dev/rust/Minebrew/minebrew-lib/src/modrinth/example_json");
        let str = std::fs::read_to_string(&path).unwrap();
        serde_json::from_str::<Self>(&str).unwrap()
    }
}
