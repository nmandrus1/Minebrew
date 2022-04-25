use serde::Deserialize;

use super::shared::*;

/// Holds all the information received by a GET request for a project
#[derive(Deserialize)]
pub struct Project {
    slug: String,
    title: String,
    description: String,
    categories: Vec<String>,
    client_side: Support,
    server_side: Support,
    body: String,
    issues_url: Option<String>,
    source_url: Option<String>,
    wiki_url: Option<String>,
    discord_url: Option<String>,
    donation_urls: Option<Vec<DonationInfo>>,
    project_type: ProjectType,
    downloads: usize,
    icon_url: Option<String>,
    id: String,
    team: String,
    body_url: Option<String>,
    moderator_message: Option<ModeratorMessage>,
    published: String,
    updated: String,
    followers: usize,
    status: ProjectStatus,
    license: Option<License>,
    versions: Option<Vec<String>>,
    gallery: Option<Vec<Picture>>,
}
