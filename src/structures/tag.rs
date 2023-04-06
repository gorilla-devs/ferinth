//! Models related to tags
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/tags)

use super::{project::ProjectType, *};

/// A category that projects of `project_type` can fall under
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    /// An SVG icon for the category
    pub icon: String,
    /// The name of the category
    pub name: String,
    /// The project type this category is applicable to
    pub project_type: ProjectType,
    /// The header under which the category should go
    pub header: String,
}

/// A loader that can load projects of `project_type`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Loader {
    /// An SVG icon for the loader
    pub icon: String,
    /// The name of the loader
    pub name: String,
    /// The project types that this loader can load
    pub supported_project_types: Vec<ProjectType>,
}

/// A version of Minecraft
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameVersion {
    /// The name of the game version
    pub version: String,
    /// The type of the game version
    pub version_type: GameVersionType,
    /// When the game version released
    pub date: UtcTime,
    /// Whether this game version was considered a major version
    ///
    /// True if this version introduced many breaking changes to internal APIs
    /// that causes most mods made for previous versions of the game to break on this version.
    pub major: bool,
}

/// The licenses that projects can be searched with
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The SPDX license ID of a project
    pub short: String,
    /// The license's long name
    pub name: String,
}

/// A donation platform that can be added to a project
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationPlatform {
    /// A short identifier for the donation platform
    pub short: String,
    /// The full name of the donation platform
    pub name: String,
}

/// The type of a game version
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum GameVersionType {
    Snapshot,
    Release,
    Beta,
    Alpha,
}
