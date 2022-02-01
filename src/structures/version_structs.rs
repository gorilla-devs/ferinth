use super::*;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    /// The version's ID
    pub id: ID,
    /// The ID of the mod this version is for
    pub project_id: ID,
    /// The ID of the author who published this version
    pub author_id: ID,
    /// Whether the version is 'featured' (?) or not
    pub featured: bool,

    /// The name of this version
    pub name: String,
    /// The version's number. Ideally, this will follow semantic versioning
    pub version_number: String,
    /// The version's changelog
    pub changelog: Option<String>,
    #[deprecated(note = "Read from `Version.changelog` instead")]
    /// A link to the version's changelog
    pub changelog_url: Option<String>,
    /// When this version was published
    pub date_published: Datetime,
    /// The number of downloads this version has
    pub downloads: usize,
    /// The version's type
    pub version_type: VersionType,

    /// A list of files available for download
    pub files: Vec<VersionFile>,
    /// This version's dependencies, as a list of dependencies' versions' IDs
    pub dependencies: Vec<Dependency>,
    /// A list of Minecraft versions that this version supports
    pub game_versions: Vec<String>,
    /// The mod loaders that this version supports
    pub loaders: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VersionType {
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "release")]
    Release,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionFile {
    /// The file's hashes
    pub hashes: Hashes,
    /// A direct link to the file
    pub url: String,
    /// The file's name
    pub filename: String,
    /// Whether the file is the primary file of a version
    pub primary: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hashes {
    /// The SHA512 hash of the version file
    pub sha512: Option<String>,
    /// The SHA1 hash of the version file
    pub sha1: Option<String>,
}

/// A dependency which describes what versions are required, break support, or are optional to the
/// version's functionality
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    /// The specific version id that the dependency uses
    pub version_id: Option<ID>,
    /// The project ID that the dependency is synced with and auto-updated
    pub project_id: Option<ID>,
    /// The type of the dependency
    pub dependency_type: DependencyType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DependencyType {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "incompatible")]
    Incompatible,
}
