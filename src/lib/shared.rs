use serde::Deserialize;

// This module contains enums and structs used in multiple 
// other structs that are constructed through HTTP GET requests

/// Enum that represents a mods client/server side support
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Support {
    Required,
    Optional,
    Unsupported
}

/// Enum that represents whether a project is a Mod or Modpack
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack
}

// Enum that represents the project's status on modrinth
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Rejected,
    Draft,
    Unlisted,
    Archived,
    Processing,
    Unknown
}

/// Struct containing an id, platform, and url for donations
#[derive(Deserialize, Debug)]
pub struct DonationInfo {
    id: String,
    platform: String,
    url: String,
}

/// A struct containg a message from a moderator
#[derive(Deserialize, Debug)]
pub struct ModeratorMessage {
    message: Option<String>,
    body: Option<String>,
}

/// Struct containing software License information
#[derive(Deserialize, Debug)]
pub struct License {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

/// struct containing the information about showcase pictures on the mod's 
/// page on modrinth
#[derive(Deserialize, Debug)] 
pub struct Picture {
    url: String,
    featured: bool,
    title: Option<String>,
    description: Option<String>,
    creating: String
}
