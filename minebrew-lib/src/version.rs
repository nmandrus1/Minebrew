use std::fmt::Display;
use std::path::Path;

use serde::Deserialize;

use super::shared::*;

#[derive(Deserialize)]
pub struct Dependency {
    pub version_id: Option<String>,

    pub project_id: Option<String>,

    pub dependecy_type: DependencyType,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
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

    pub size: usize,
}

impl Display for ModFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.filename)
    }
}

#[derive(Deserialize, Default)]
pub struct Hashes {
    _sha512: String,
    _sha1: String,
}

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

    pub dependencies: Option<Vec<Dependency>>,

    pub game_versions: Vec<String>,

    pub version_type: VersionType,

    #[serde(skip)]
    #[serde(rename = "loaders")]
    _loaders: Vec<String>,

    #[serde(rename = "featured")]
    pub featured: bool,

    #[serde(skip)]
    #[serde(rename = "id")]
    _id: String,

    pub project_id: String,

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
        write!(f, "{}", self.file().filename)
    }
}

impl Version {
    /// retrieves the file of interest
    #[inline]
    pub fn file(&self) -> &ModFile {
        self.files.iter().find(|f| f.primary).unwrap_or(&self.files[0])
    }
    
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

    /// Convience function to download a version to a specific path
    pub async fn download_to(&self, path: &Path, client: &reqwest::Client) -> anyhow::Result<()> {
        // TODO: Find a way to use a reqwest::Client here
        let file = &self.file();

        let (url, filename) = (&file.url, &file.filename);

        let res = client.get(url).send().await?;
        std::fs::write(path.join(filename), res.bytes().await?)?;
        Ok(())
    }
}
