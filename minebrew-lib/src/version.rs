use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;

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

    url: String,

    filename: String,

    primary: bool,

    size: usize,
}

impl ModFile {
    pub fn hashes(&self) -> &Hashes {
        &self.hashes
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn is_primary(&self) -> bool {
        self.primary
    }

    pub fn file_size(&self) -> usize {
        self.size
    }
    
    pub fn file_name(&self) -> &str {
        &self.filename
    }
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
    pub fn len(&self) -> usize { self.0.len() }

    /// check if the list is empty
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// append a version to the end of the List
    pub fn push(&mut self, v: Version) { self.0.push(v) }

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

    pub id: String,

    project_id: String,

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
        self.id == other.id
    }
}

// Printing version struct now just means printing the name
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.file().filename)
    }
}

impl Version {
    /// retrieves the file of interest
    #[inline]
    pub fn file(&self) -> &ModFile {
        self.files.iter().find(|f| f.primary).unwrap_or(&self.files[0])
    }

    /// the Project ID
    pub fn pid(&self) -> &str { &self.project_id }

    /// retrieves the sha1 hash for this version
    #[inline]
    pub fn sha1(&self) -> &str {
        &self.file().hashes.sha1
    }

    /// Convience function to download a version to a specific path
    pub async fn download_to(self, path: Arc<PathBuf>, client: reqwest::Client) -> anyhow::Result<Self> {
        // TODO: Find a way to use a reqwest::Client here
        let file = &self.file();

        let (url, filename) = (&file.url, &file.filename);

        let res = tokio::spawn(client.get(url).send()).await??;
        let bytes = tokio::spawn(res.bytes()).await??;
        std::fs::write(path.join(filename), bytes)?;
        Ok(self)
    }
}
