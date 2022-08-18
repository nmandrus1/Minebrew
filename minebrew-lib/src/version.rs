use std::fmt::Display;
use std::path::Path;

use serde::{ Serialize, Deserialize };

use super::shared::*;

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    pub version_id: Option<String>,

    pub project_id: Option<String>,

    pub dependency_type: DependencyType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Release,
    Beta,
    Alpha
}

#[derive(Serialize, Deserialize)]
pub struct ModFile {
    hashes: Hashes,

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

#[derive(Serialize, Deserialize, Default)]
pub struct Hashes {
    sha512: String,
    sha1: String,
}

/// struct that represents a list of Version structs, internally it is a just a wrapper
/// over a Vec<Version>
#[derive(Deserialize, Default)]
pub struct VersionList(Vec<Version>);

impl From<Vec<Version>> for VersionList {
    fn from(v: Vec<Version>) -> Self {
       Self(v) 
    }
}

impl VersionList {
    /// length of the version list
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// append a version to the end of the List
    pub fn push(&mut self, v: Version) {
        self.0.push(v)
    }

    /// Remove a Version from the list
    pub fn remove(&mut self, index: usize) -> Version {
        self.0.remove(index)
    }

    /// Return an iterator over the VersionList
    pub fn iter(&self) -> impl Iterator<Item=&Version> {
        self.0.iter()
    }
}

impl IntoIterator for VersionList {
    type Item = Version;
    type IntoIter = std::vec::IntoIter<Version>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// struct that represents a particular downloadable version of a mod
#[derive(Serialize, Deserialize)]
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

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.sha1() == other.sha1()
    }
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

    /// retrieves the sha1 hash for this version
    #[inline]
    pub fn sha1(&self) -> &str {
        &self.file().hashes.sha1
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
