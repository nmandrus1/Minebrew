use serde::Deserialize;

use super::shared::*;

#[derive(Deserialize)]
struct Dependency {
    #[serde(skip)]
    #[serde(rename = "version_id")]
    _version_id: Option<String>,

    #[serde(skip)]
    #[serde(rename = "project_id")]
    _project_id: Option<String>,

    #[serde(rename = "dependecy_type")]
    _dependecy_type: DependencyType,
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
    #[serde(skip)]
    #[serde(rename = "hashes")]
    _hashes: Hashes,

    pub url: String,

    pub filename: String,

    pub primary: bool,

    #[serde(skip)]
    #[serde(rename = "size")]
    _size: usize,
}

#[derive(Deserialize, Default)]
pub struct Hashes {
    _sha512: Option<String>,
    _sha1: Option<String>,
}

#[derive(Deserialize)]
pub struct Versions (Vec<Version>);

/// struct that represents a particular downloadable version of a mod
#[derive(Deserialize)]
pub struct Version {
    #[serde(skip)]
    #[serde(rename = "name")]
    _name: String,

    #[serde(skip)]
    #[serde(rename = "version_number")]
    _version_number: String,

    #[serde(skip)]
    #[serde(rename = "changelog")]
    _changelog: Option<String>,

    #[serde(skip)]
    #[serde(rename = "dependencies")]
    _dependencies: Option<Vec<Dependency>>,

    pub game_versions: Vec<String>,

    #[serde(rename = "version_type")]
    _version_type: VersionType,

    #[serde(skip)]
    #[serde(rename = "loaders")]
    _loaders: Vec<String>,

    #[serde(skip)]
    #[serde(rename = "featured")]
    _featured: bool,

    #[serde(skip)]
    #[serde(rename = "id")]
    _id: String,

    #[serde(skip)]
    #[serde(rename = "project_id")]
    _project_id: String,

    #[serde(skip)]
    #[serde(rename = "author_id")]
    _author_id: String,

    #[serde(skip)]
    #[serde(rename = "date_published")]
    _date_published: String,

    #[serde(skip)]
    #[serde(rename = "changelog_url")]
    _changelog_url: Option<String>,

    pub files: Vec<ModFile>,
}

impl Version {
    pub fn search(slug: &str, version: &str) -> Result<Vec<Version>, reqwest::Error> { 
        let json_str = match reqwest::blocking::get(format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]", slug, version)) {
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
