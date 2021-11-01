use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Version {
    /// The ID of the version, encoded as a base62 string
    pub id: ID,
    /// The ID of the mod this version is for
    pub mod_id: ID,
    /// The ID of the author who published this version
    pub author_id: ID,
    /// Whether the version is featured or not
    pub featured: bool,
    /// The name of this version
    pub name: String,
    /// The version number. Ideally, this will follow semantic versioning
    pub version_number: String,
    /// The changelog for this version of the mod.
    pub changelog: Option<String>,
    #[deprecated(note = "Read from `Version.changelog` instead")]
    /// A link to the changelog for this version of the mod
    pub changelog_url: Option<String>,
    /// The date that this version was published
    pub date_published: Datetime,
    /// The number of downloads this version has
    pub downloads: usize,
    /// The type of the release
    pub version_type: VersionType,
    /// A list of files available for download
    pub files: Vec<VersionFile>,
    /// This version's dependencies, as a list of IDs of dependency's versions
    pub dependencies: Vec<ID>,
    /// A list of Minecraft versions that this version supports
    pub game_versions: Vec<String>,
    /// The mod loaders that this version supports
    pub loaders: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum VersionType {
	Alpha,
	Beta,
	Release
}

#[derive(Deserialize, Serialize, Debug)]
/// A single mod file, with a URL for the file and the file's hashes
pub struct VersionFile {
    /// The key is the hashing algorithm and the value is the hash
    pub hashes: HashMap<String, String>,
    /// A direct link to the file
    pub url: String,
    /// The name of the file
    pub filename: String,
}
