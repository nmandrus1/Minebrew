// This allows our Project type to be deserialized from a JSON automatically
use serde::Deserialize;

// Lets the Project type use types from the shared.rs file
use super::shared::*;

// The derive statement here is called a macro, what this does is it 
// implements the Deserialize interface for our type automatically, 
// this interface lets our struct be deserialized from a json into a 
// rust struct, meaning that the json file we recieve from modrinth 
// can be seamlessly created into a rust struct and used just like any 
// other struct, really really powerful

/// Holds all the information received by a GET request for a project
#[derive(Deserialize)]
pub struct Project {
    slug: String,
    title: String,
    description: String,
    #[serde(skip)]
    categories: Vec<String>,
    client_side: Support,
    server_side: Support,
    #[serde(skip)]
    body: String,
    #[serde(skip)]
    issues_url: Option<String>,
    #[serde(skip)]
    source_url: Option<String>,
    #[serde(skip)]
    wiki_url: Option<String>,
    #[serde(skip)]
    discord_url: Option<String>,
    #[serde(skip)]
    donation_urls: Option<Vec<DonationInfo>>,
    project_type: ProjectType,
    #[serde(skip)]
    downloads: usize,
    #[serde(skip)]
    icon_url: Option<String>,
    id: String,
    #[serde(skip)]
    team: String,
    body_url: Option<String>,
    #[serde(skip)]
    moderator_message: Option<ModeratorMessage>,
    #[serde(skip)]
    published: String,
    #[serde(skip)]
    updated: String,
    #[serde(skip)]
    followers: usize,
    status: ProjectStatus,
    #[serde(skip)]
    license: Option<License>,
    versions: Option<Vec<String>>,
    #[serde(skip)]
    gallery: Option<Vec<Picture>>,
}
