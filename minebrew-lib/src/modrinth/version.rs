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
    Aplha
}

#[derive(Deserialize)]
struct ModFile {
    hashes: Vec<String>,
    url: String,
    filename: String,
    primary: bool,
}

/// struct that represents a particular downloadable version of a mod
#[derive(Deserialize)]
pub struct Version {
    name: String,
    version_number: String,
    #[serde(skip)]
    changelog: Option<String>,
    dependecies: Option<Vec<Dependency>>,
    game_versions: Vec<String>,
    version_type: VersionType,
    loaders: Vec<String>,
    featured: bool,
    id: String,
    project_id: String,
    author_id: String,
    date_published: String,
    #[serde(skip)]
    changelog_url: Option<String>,
    files: Vec<ModFile>,
}

impl Version {
}
