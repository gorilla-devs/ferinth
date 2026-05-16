//! Models related to tags

use super::*;

/// A category that projects of `project_type` specify
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    /// An SVG icon for the category
    pub icon: String,
    pub name: String,
    /// The project type this category is applicable to
    pub project_type: project::ProjectType,
    /// The header under which the category should go
    pub header: String,
}

/// A loader that can load projects of `project_type`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Loader {
    /// An SVG icon for the loader
    pub icon: String,
    pub name: String,
    /// The project types that this loader can load
    pub supported_project_types: Vec<project::ProjectType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameVersion {
    pub version: String,
    /// The type of the game version
    pub version_type: GameVersionType,
    /// When the game version released
    pub date: UtcTime,
    /// Whether this game version was considered a major version
    /// 
    /// This is used for featured versions.
    pub major: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The full display name of the license
    pub title: String,
    /// The full body text of the license
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationPlatform {
    /// A short identifier for the donation platform
    pub short: String,
    /// The display name of the platform
    pub name: String,
}

/// The type of a game version
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Release,
    Snapshot,
    Beta,
    Alpha,
}
