use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    /// The SVG icon of the category
    pub icon: String,
    /// The name of the category
    pub name: String,
    /// The project type this category is applicable to
    pub project_type: project::ProjectType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Loader {
    /// The SVG icon of the loader
    pub icon: String,
    /// The name of the loader
    pub name: String,
    /// The project types that this loader is applicable to
    pub supported_project_types: Vec<project::ProjectType>,
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
