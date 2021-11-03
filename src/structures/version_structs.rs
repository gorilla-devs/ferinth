use serde::{Deserialize, Serialize};
use super::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Version {
    /// The version's ID
    pub id: ID,
    /// The ID of the mod this version is for
    pub mod_id: ID,
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
    pub dependencies: Vec<ID>,
    /// A list of Minecraft versions that this version supports
    pub game_versions: Vec<String>,
    /// The mod loaders that this version supports
    pub loaders: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum VersionType {
    #[serde(rename = "alpha")]
	Alpha,
    #[serde(rename = "beta")]
	Beta,
    #[serde(rename = "release")]
	Release
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VersionFile {
    /// The file's hashes
    pub hashes: Hashes,
    /// A direct link to the file
    pub url: String,
    /// The file's name
    pub filename: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Hashes {
    /// The SHA512 hash of the version file
    pub sha512: Option<String>,
    /// The SHA1 hash of the version file
    pub sha1: Option<String>,
}
