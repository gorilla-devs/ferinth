//! Models related to versions
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/version_model)

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    pub name: String,
    /// Ideally will follow semantic versioning
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub game_versions: Vec<String>,
    /// The release channel for this version
    pub version_type: VersionType,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: Option<Status>,
    pub requested_status: Option<RequestedStatus>,
    pub id: ID,
    pub project_id: ID,
    /// The user ID of the author who published this version
    pub author_id: ID,
    pub date_published: UtcTime,
    pub downloads: Int,
    /// A list of files available for download
    pub files: Vec<VersionFile>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionFile {
    pub hashes: Hash,
    pub url: Url,
    pub filename: String,
    /// Whether the file is the primary file of its version
    ///
    /// There can only be a maximum of one primary file per version.
    /// If there are no primary files specified, the first file can be taken as the primary one.
    pub primary: bool,
    /// The size of the file in bytes
    pub size: Int,
    /// The type of the additional file, used mainly for adding resource packs to datapacks
    pub file_type: Option<AdditionalFileType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hash {
    pub sha512: String,
    pub sha1: String,
    /// Other hashes that may have been provided
    #[serde(flatten)]
    pub others: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LatestVersionBody {
    pub loaders: Vec<String>,
    pub game_versions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LatestVersionsBody {
    pub hashes: Vec<String>,
    pub algorithm: HashAlgorithm,
    pub loaders: Vec<String>,
    pub game_versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    pub version_id: Option<ID>,
    pub project_id: Option<ID>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    SHA512,
    SHA1,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AdditionalFileType {
    RequiredResourcePack,
    OptionalResourcePack,
}
