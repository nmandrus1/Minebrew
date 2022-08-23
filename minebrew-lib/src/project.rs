use serde::Deserialize;

use super::shared::*;

/// Representation of a project in Modrinth
#[derive(Deserialize)]
pub struct Project {
    #[serde(skip)]
    _slug: String,

    #[serde(skip)]
    _title: String,

    #[serde(skip)]
    _description: String,

    #[serde(skip)]
    _categories: Vec<Category>,

    #[serde(skip)]
    _client_side: Support,

    #[serde(skip)]
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
    _donation_url: Option<String>,

    #[serde(skip)]
    _project_type: ProjectType,

    #[serde(skip)]
    _downloads: usize,

    #[serde(skip)]
    _icon_url: Option<String>,

    #[serde(skip)]
    _id: String,

    #[serde(skip)]
    _team: String,

    #[serde(skip)]
    _moderator_message: ModeratorMessage,

    #[serde(skip)]
    _published: String,

    #[serde(skip)]
    _updated: String,

    #[serde(skip)]
    _followers: usize,

    #[serde(skip)]
    _status: ProjectStatus,

    #[serde(skip)]
    _license: Option<License>,

    #[serde(skip)]
    _version: Vec<String>,

    #[serde(skip)]
    _gallery: Option<String>,
}
