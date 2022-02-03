use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The project's ID
    pub id: ID,
    /// The project's slug. This can change at any time
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
    #[deprecated(note = "Read from `Project.body` instead")]
    /// A link to the long description of the project
    pub body_url: Option<String>,
    /// When the project was first published
    pub published: Datetime,
    /// When the project was last updated
    pub updated: Datetime,
    /// The project's status
    pub status: ProjectStatus,
    /// A message that a moderator sent regarding the project
    pub moderator_message: Option<String>,
    /// The project's license
    pub license: License,
    /// The project's client side support range
    pub client_side: ProjectSupportRange,
    /// The project's server side support range
    pub server_side: ProjectSupportRange,
    /// The total number of downloads the project has
    pub downloads: usize,
    /// The total number of user following this project
    pub followers: usize,
    /// A list of categories the project is in
    pub categories: Vec<String>,
    /// The project's versions listed as their IDs
    pub versions: Vec<ID>,
    /// The link to the project's icon
    pub icon_url: Option<String>,
    /// A link to submit bugs or issues about the project
    pub issues_url: Option<String>,
    /// A link to the project's source code
    pub source_url: Option<String>,
    /// A link to the project's wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// A link to the project's discord
    pub discord_url: Option<String>,
    /// A list of donation links the project has
    pub donation_urls: Option<Vec<DonationLink>>,
    /// A list of URLs to visual content featuring the project
    pub gallery: Vec<GalleryItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct License {
    /// The license's ID
    pub id: String,
    /// The license's long name
    pub name: String,
    /// A link to this license
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DonationLink {
    /// The donation link's platform ID
    pub id: String,
    /// The platform's long name
    pub platform: String,
    /// A link to this donation
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSupportRange {
    Required,
    Optional,
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: Datetime,
}
