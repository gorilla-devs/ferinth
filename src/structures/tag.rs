//! Models related to tags
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/tags)

use super::{project::ProjectType, *};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    /// The category's SVG icon
    pub icon: String,
    pub name: String,
    /// The project type this category is applicable to
    pub project_type: ProjectType,
    /// The header under which the category should go
    pub header: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Loader {
    /// The loader's SVG icon
    pub icon: String,
    pub name: String,
    /// The project types that this loader is applicable to
    pub supported_project_types: Vec<ProjectType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameVersion {
    /// The name/number of the game version
    pub version: String,
    /// The type of the game version
    pub version_type: GameVersionType,
    /// The date of the game version release
    pub date: UtcTime,
    /// Whether this is a major version
    pub major: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The short identifier of the license
    pub short: String,
    /// The full name of the license
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationPlatform {
    /// The short identifier of the donation platform
    pub short: String,
    /// The full name of the donation platform
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Snapshot,
    Release,
    Beta,
    Alpha,
}
