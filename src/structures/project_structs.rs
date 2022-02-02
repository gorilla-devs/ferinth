use super::{Datetime, ID};
use serde::{Deserialize, Serialize};
use std::{
    clone::Clone,
    cmp::PartialEq,
    fmt::{Display, Formatter},
};

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
    /// The project's title or name
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
    /// WHen the project was last updated
    pub updated: Datetime,

    /// The project's status
    pub status: ProjectStatus,
    /// The rejection data of the project
    pub moderator_message: Option<ModeratorMessage>,

    /// The project's license
    pub license: License,

    /// The project's client side support range
    pub client_side: ProjectSupportRange,
    /// The project's server side support range
    pub server_side: ProjectSupportRange,

    /// The total number of downloads the project has
    pub downloads: usize,
    /// The total number of followers this project has accumulated
    pub followers: u32,

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

    /// A string of URLs to visual content featuring the project
    pub gallery: Vec<GalleryItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    /// The license's ID
    pub id: String,
    /// The license's long name
    pub name: String,
    /// A link to this license
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The donation link's platform ID
    pub id: String,
    /// The platform's long name
    pub platform: String,
    /// A link to this donation
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "unlisted")]
    Unlisted,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProjectSupportRange {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Display for ProjectSupportRange {
    fn fmt(&self, fmt: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Self::Required => "required",
                Self::Optional => "optional",
                Self::Unsupported => "unsupported",
                Self::Unknown => "unknown",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: Datetime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectType {
    #[serde(rename = "mod")]
    Mod,
    #[serde(rename = "modpack")]
    Modpack,
}
