use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Category {
    /// An svg icon of the category
    icon: String,
    /// The name of the category
    name: String,
    /// What type of project this categorises
    project_type: ProjectType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Loader {
    /// An svg icon of the loader
    icon: String,
    /// The name of the loader
    name: String,
    /// What project types this loader supports
    supported_project_types: Vec<ProjectType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GameVersion {
    version: String,
    version_type: GameVersionType,
    date: Datetime,
    major: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionType {
    Snapshot,
    Release,
    Beta,
    Alpha,
}
