use serde::Deserialize;

// This module contains enums and structs used in multiple 
// other structs that are constructed through HTTP GET requests

/// Enum that represents a mod's dependency requirement
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatable,
}

/// Enum that represents a mods client/server side support
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Support {
    Required,
    Optional,
    Unsupported
}

/// Enum that represents whether a project is a Mod or Modpack
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack
}

// Enum that represents the project's status on modrinth
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct DonationInfo {
    _id: String,
    _platform: String,
    _url: String,
}

/// A struct containg a message from a moderator
#[derive(Deserialize)]
pub struct ModeratorMessage {
    _message: Option<String>,
    _body: Option<String>,
}

/// Struct containing software License information
#[derive(Deserialize)]
pub struct License {
    _id: Option<String>,
    _name: Option<String>,
    _url: Option<String>,
}

/// struct containing the information about showcase pictures on the mod's 
/// page on modrinth
#[derive(Deserialize)] 
pub struct Picture {
    _url: String,
    _featured: bool,
    _title: Option<String>,
    _description: Option<String>,
    _creating: String
}
