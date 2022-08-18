use serde::{ Serialize, Deserialize };

// This module contains enums and structs used in multiple 
// other structs that are constructed through HTTP GET requests

/// Enum that represents a mod's dependency requirement
#[derive(Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Default, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    #[default]
    #[serde(skip)]
    None,
    Mod,
    Modpack
}

impl ProjectType {
    /// method to get the string representation of a ProjectType
    pub fn to_str(&self) -> &str {
        match self {
            ProjectType::Mod => "mod",
            ProjectType::Modpack => "modpack",
            ProjectType::None => ""
        }
    }
}

impl From<&str> for ProjectType {
    fn from(s: &str) -> Self {
        match s {
            "mod" => ProjectType::Mod,
            "modpack" => ProjectType::Modpack,
            _ => ProjectType::None
        }
    }
}

/// Category Enum for the different Categories Modrinth 
/// will filter mods through
#[derive(Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    #[default]
    None,
    Adventure,
    Cursed,
    Decoration,
    Equipment,
    Food,
    Library,
    Magic,
    Misc,
    Optimization,
    Storage,
    Technology,
    Utility,
    Worldgen,
}

impl Category {
    /// method to get the string representation of a Category
    pub fn to_str(&self) -> &str {
        match self {
            Category::Adventure => "adventure",
            Category::Cursed => "cursed",
            Category::Decoration => "decoration",
            Category::Equipment => "equipment",
            Category::Food => "food",
            Category::Library => "library",
            Category::Magic => "magic",
            Category::Misc => "misc",
            Category::Optimization => "optimization",
            Category::Storage => "storage",
            Category::Technology => "technology",
            Category::Utility => "utility",
            Category::Worldgen => "worldgen",
            Category::None => ""
        }
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "adventure" => Category::Adventure,
            "cursed" => Category::Cursed,
            "decoration" => Category::Decoration, 
            "equipment" => Category::Equipment, 
            "food" => Category::Food,
            "library" => Category::Library,
            "magic" => Category::Magic,
            "misc" => Category::Misc,
            "optimization" => Category::Optimization,
            "storage" => Category::Storage,
            "technology" => Category::Technology,
            "utility" => Category::Utility,
            "worldgen" => Category::Worldgen,
            _ => Category::None
        }
    }
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
