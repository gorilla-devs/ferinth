//! Models related to versions
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/version_model)

use super::*;

/// A version of a project
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    /// The name of this version
    pub name: String,
    /// The version number.
    /// Ideally, this will follow semantic versioning.
    pub version_number: String,
    /// The changelog for this version
    pub changelog: Option<String>,
    /// A list of specific versions of other projects that this version depends on
    pub dependencies: Vec<Dependency>,
    /// A list of Minecraft versions that this version supports
    pub game_versions: Vec<String>,
    /// The release channel for this version
    pub version_type: VersionType,
    /// The mod loaders that this version supports
    pub loaders: Vec<String>,
    /// Whether the version is featured or not
    pub featured: bool,
    /// The status of this version
    pub status: Option<Status>,
    /// The requested status of this version
    pub requested_status: Option<RequestedStatus>,
    /// The ID of this version
    pub id: ID,
    /// The ID of the project this version is for
    pub project_id: ID,
    /// The ID of the author who published this version
    pub author_id: ID,
    /// When this version was published
    pub date_published: UtcTime,
    /// The number of times this version has been downloaded
    pub downloads: Number,
    /// A link to the version's changelog
    #[deprecated = "Read from `changelog` instead"]
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub changelog_url: Option<Url>,
    /// A list of files available for download
    pub files: Vec<VersionFile>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionFile {
    /// A map of hashes of the file.
    pub hashes: Hashes,
    /// A direct link to the file
    pub url: Url,
    /// The name of the file
    pub filename: String,
    /// Whether the file is the primary file of its version.
    /// There can only be a maximum of one primary file per version.
    /// If there are no primary files specified, the first file can be taken as the primary file.
    pub primary: bool,
    /// The size of the file in bytes
    pub size: Number,
    /// The type of the additional file, used mainly for adding resource packs to datapacks
    pub file_type: Option<AdditionalFileType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct Hashes {
    /// The SHA512 hash of the version file
    pub sha512: String,
    /// The SHA1 hash of the version file
    pub sha1: String,
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

/// A dependency which describes what versions or projects are required, break the mod, are optional, or embedded.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    /// The ID of the version that this version depends on
    pub version_id: Option<ID>,
    /// The ID of the project that this version depends on
    pub project_id: Option<ID>,
    /// The file name of the dependency, mostly for showing external dependencies on modpacks
    pub file_name: Option<String>,
    /// The type of dependency that this version has
    pub dependency_type: DependencyType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    SHA512,
    SHA1,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Alpha,
    Beta,
    Release,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AdditionalFileType {
    RequiredResourcePack,
    OptionalResourcePack,
}
