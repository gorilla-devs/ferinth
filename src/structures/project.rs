//! Models related to projects
//! 
//! [documentation](https://docs.modrinth.com/api-spec/#tag/project_model)

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// The project's slug, used for vanity URLs.
    /// This can change at any time, so use the `id` for long term storage.
    pub slug: String,
    /// The project's title or name
    pub title: String,
    /// A short description of the project
    pub description: String,
    /// A list of categories the project is in
    pub categories: Vec<String>,
    /// The project's client side support range
    pub client_side: ProjectSupportRange,
    /// The project's server side support range
    pub server_side: ProjectSupportRange,
    /// A long form description of the project
    pub body: String,
    /// A list of categories which are searchable but non-primary
    pub additional_categories: Vec<String>,
    /// A link to submit bugs or issues with the project
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub issues_url: Option<Url>,
    /// A link to the project's source code
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub source_url: Option<Url>,
    /// A link to the project's wiki page or other relevant information
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub wiki_url: Option<Url>,
    /// The project's Discord server invite
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub discord_url: Option<Url>,
    /// A list of donation links for the project
    pub donation_urls: Vec<DonationLink>,
    /// The project type of the project
    pub project_type: ProjectType,
    /// The total number of downloads the project has
    pub downloads: Number,
    /// The link to the project's icon
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
    /// The project's ID
    pub id: ID,
    /// The ID of the team that has ownership of this project
    pub team: ID,
    /// A link to the long description of the project (only present for old projects)
    #[deprecated = "Read from `body` instead"]
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub body_url: Option<Url>,
    /// A message that a moderator sent regarding the project
    pub moderator_message: Option<ModeratorMessage>,
    /// When the project was first published
    pub published: UtcTime,
    /// When the project was last updated
    pub updated: UtcTime,
    /// The date the project's status was set to approved or unlisted
    pub approved: Option<UtcTime>,
    /// The total number of users following the project
    pub followers: Number,
    /// The project's status
    pub status: ProjectStatus,
    /// The project's license
    pub license: License,
    /// A list of the version IDs of the project
    pub versions: Vec<ID>,
    /// A list of images that have been uploaded to the project's gallery
    pub gallery: Vec<GalleryItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModeratorMessage {
    /// The message that a moderator has left for the project
    pub message: String,
    /// The longer body of the message that a moderator has left for the project
    pub body: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The license ID of a project, retrieved from the license's get route
    pub id: String,
    /// The license's long name
    pub name: String,
    /// The URL to this license
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The donation platform's ID
    pub id: String,
    /// The donation platform this link is for
    pub platform: String,
    /// A link to the donation platform and user
    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryItem {
    /// The URL of the gallery image
    pub url: Url,
    /// Whether the image is featured in the gallery
    pub featured: bool,
    /// The title of the gallery image
    pub title: Option<String>,
    /// The description of the gallery image
    pub description: Option<String>,
    /// The date and time the gallery image was created
    pub created: UtcTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectDependencies {
    pub projects: Vec<Project>,
    pub versions: Vec<version::Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolveIDSlugResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Fields to edit on all projects specified
pub struct EditMultipleProjectsRequestBody {
    /// Set all of the categories to the categories specified here
    pub categories: Vec<String>,
    /// Add all of the categories specified here
    pub add_categories: Vec<String>,
    /// Remove all of the categories specified here
    pub remove_categories: Vec<String>,
    /// Set all of the additional categories to the categories specified here
    pub additional_categories: Vec<String>,
    /// Add all of the additional categories specified here
    pub add_additional_categories: Vec<String>,
    /// Remove all of the additional categories specified here
    pub remove_additional_categories: Vec<String>,
    /// Set all of the donation links to the donation links specified here
    pub donation_urls: Vec<DonationLink>,
    /// Add all of the donation links specified here
    pub add_donation_urls: Vec<DonationLink>,
    /// Remove all of the donation links specified here
    pub remove_donation_urls: Vec<DonationLink>,
    /// A link to where to submit bugs or issues with the projects
    pub issues_url: Option<String>,
    /// A link to the source code of the projects
    pub source_url: Option<String>,
    /// A link to the projects' wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the projects' discord
    pub discord_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Rejected,
    Draft,
    Unlisted,
    Archived,
    Processing,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSupportRange {
    Required,
    Optional,
    Unsupported,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive] // More project types may be added in the future
pub enum ProjectType {
    Mod,
    Plugin,
    Shader,
    Modpack,
    ResourcePack,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileExt {
    PNG,
    JPG,
    JPEG,
    BMP,
    GIF,
    WebP,
    SVG,
    SVGZ,
    RGB,
}

impl std::fmt::Display for FileExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
