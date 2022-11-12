use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    pub name: String,
    /// The version's number.
    /// Ideally, this will follow semantic versioning.
    pub version_number: String,
    pub changelog: Option<String>,
    /// A list of specific versions of projects that this version depends on
    pub dependencies: Vec<Dependency>,
    /// A list of Minecraft versions that this version supports
    pub game_versions: Vec<String>,
    /// The release channel for this version
    pub version_type: VersionType,
    /// The mod loaders that this version supports
    pub loaders: Vec<String>,
    /// Whether the version is featured or not
    pub featured: bool,
    pub id: ID,
    /// The ID of the project this version is for
    pub project_id: ID,
    /// The ID of the author who published this version
    pub author_id: ID,
    pub date_published: UtcTime,
    /// The number of times this version has been downloaded
    pub downloads: usize,
    /// A link to the version's changelog
    #[deprecated = "Read from `changelog` instead"]
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub changelog_url: Option<Url>,
    /// A list of files available for download
    pub files: Vec<VersionFile>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionFile {
    pub hashes: Hashes,
    /// A direct link to the file
    pub url: Url,
    /// The file's name
    pub filename: String,
    /// Whether the file is the primary file of its version
    pub primary: bool,
    /// The size of the file in bytes
    pub size: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hashes {
    /// The SHA512 hash of the version file
    pub sha512: String,
    /// The SHA1 hash of the version file
    pub sha1: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HashesBody {
    pub hashes: Vec<String>,
    pub algorithm: HashAlgorithm,
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

/// A dependency which describes what versions are required, break support, or are optional to the version's functionality
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency {
    pub version_id: Option<ID>,
    pub project_id: Option<ID>,
    /// The file name of the dependency, mostly for showing external dependencies on modpacks
    pub file_name: Option<String>,
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
#[non_exhaustive]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}
