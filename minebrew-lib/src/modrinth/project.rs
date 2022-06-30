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
    _slug: String,
    _title: String,
    _description: String,
    #[serde(skip)]
    _categories: Vec<String>,
    _client_side: Support,
    _server_side: Support,
    #[serde(skip)]
    _body: String,
    #[serde(skip)]
    _issues_url: Option<String>,
    #[serde(skip)]
    _source_url: Option<String>,
    #[serde(skip)]
    _wiki_url: Option<String>,
    #[serde(skip)]
    _discord_url: Option<String>,
    #[serde(skip)]
    _donation_urls: Option<Vec<DonationInfo>>,
    _project_type: ProjectType,
    #[serde(skip)]
    _downloads: usize,
    #[serde(skip)]
    _icon_url: Option<String>,
    _id: String,
    #[serde(skip)]
    _team: String,
    _body_url: Option<String>,
    #[serde(skip)]
    _moderator_message: Option<ModeratorMessage>,
    #[serde(skip)]
    _published: String,
    #[serde(skip)]
    _updated: String,
    #[serde(skip)]
    _followers: usize,
    _status: ProjectStatus,
    #[serde(skip)]
    _license: Option<License>,
    _versions: Option<Vec<String>>,
    #[serde(skip)]
    _gallery: Option<Vec<Picture>>,
}
