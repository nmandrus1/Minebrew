use serde::Deserialize;

use super::shared::*;

/// Representation of a project in Modrinth
#[derive(Deserialize)]
pub struct Project {
    slug: String,
    title: String,
    description: String,
    categories: Vec<Category>,
    client_side: Support,
    server_side: Support,
    body: String,
    issues_url: Option<String>,
    source_url: Option<String>,
    wiki_url: Option<String>,
    discord_url: Option<String>,
    donation_url: Option<String>,
    project_type: ProjectType,
    downloads: usize,
    icon_url: Option<String>,
    id: String,
    team: String,
    moderator_message: ModeratorMessage,
    published: String,
    updated: String,
    followers: usize,
    status: ProjectStatus,
    license: Option<License>,
    version: Vec<String>,
    #[serde(skip)]
    gallery: Option<String>,
}
