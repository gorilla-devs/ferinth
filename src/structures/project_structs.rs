use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// The project's ID
    pub id: ID,
    /// The project's slug.
    /// This can change at any time, so use the `id` for long term storage
    pub slug: String,
    /// The project type of the project
    pub project_type: ProjectType,
    /// The ID of the team that has ownership of this project
    pub team: ID,
    /// The project's title or display name
    pub title: String,
    /// A short description of the project
    pub description: String,
    /// A long form of the description
    pub body: String,
    #[deprecated = "Read from `Project.body` instead"]
    /// A link to the long description of the project (only present for old projects)
    pub body_url: Option<String>,
    /// When the project was first published
    pub published: Datetime,
    /// When the project was last updated
    pub updated: Datetime,
    /// The project's status
    pub status: ProjectStatus,
    /// A message that a moderator sent regarding the project
    pub moderator_message: Option<ModeratorMessage>,
    /// The project's license
    pub license: License,
    /// The project's client side support range
    pub client_side: ProjectSupportRange,
    /// The project's server side support range
    pub server_side: ProjectSupportRange,
    /// The total number of downloads the project has
    pub downloads: Number,
    /// The total number of user following this project
    pub followers: Number,
    /// A list of categories the project is in
    pub categories: Vec<String>,
    /// A list of the version IDs of the project
    pub versions: Vec<ID>,
    /// The link to the project's icon
    pub icon_url: Option<URL>,
    /// A link to submit bugs or issues about the project
    pub issues_url: Option<URL>,
    /// A link to the project's source code
    pub source_url: Option<URL>,
    /// A link to the project's wiki page or other relevant information
    pub wiki_url: Option<URL>,
    /// The project's discord invite
    pub discord_url: Option<URL>,
    /// A list of donation links the project has
    pub donation_urls: Option<Vec<DonationLink>>,
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
    /// The license's ID
    pub id: String,
    /// The license's long name
    pub name: String,
    /// A URL to this license
    pub url: Option<URL>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The donation platform's ID
    pub id: String,
    /// The donation platform's long name
    pub platform: String,
    /// A link to this donation
    pub url: URL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryItem {
    /// The URL of the gallery image
    pub url: URL,
    /// Whether the image is featured in the gallery
    pub featured: bool,
    /// The title of the gallery image
    pub title: Option<String>,
    /// The description of the gallery image
    pub description: Option<String>,
    /// The date and time the gallery image was created
    pub created: Datetime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectDependencies {
    pub projects: Vec<Project>,
    pub versions: Vec<version_structs::Version>,
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
pub enum ProjectType {
    Mod,
    Modpack,
}
