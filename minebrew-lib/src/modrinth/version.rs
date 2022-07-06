use std::fmt::Display;

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

impl Display for ModFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.filename)
    }
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
    pub name: String,

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

// Printing version struct now just means printing the name
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Version {
    pub async fn search(slug: &str, version: &str) -> Result<Vec<Version>, reqwest::Error> { 
        let json_str = match reqwest::get(format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]", slug, version)).await {
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
            Ok(response) => response.text().await.unwrap() // Parse json on success
        };
 
        let versions: Vec<Version> = serde_json::from_str(&json_str).unwrap();
        Ok(versions)
    }

    pub async fn download_file(url: &str) -> Vec<u8> {
        let res = reqwest::get(url).await.unwrap();
        res.bytes().await.unwrap().to_vec()
    }
}
