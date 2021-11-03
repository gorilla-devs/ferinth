use super::{ID, Datetime};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mod {
    /// The mod's ID
    pub id: ID,
    /// The mod's slug. This can change at any time
    pub slug: String,
    /// The ID of the team that has ownership of this mod
    pub team: ID,
    /// The mod's title or name
    pub title: String,
    /// A short description of the mod
    pub description: String,
    /// A long form of the description
    pub body: String,
    #[deprecated(note = "Read from `Mod.body` instead")]
    /// A link to the long description of the mod
    pub body_url: Option<String>,
    /// When the mod was first published
    pub published: Datetime,
    /// WHen the mod was last updated
    pub updated: Datetime,
    /// The mod's status
    pub status: ModStatus,
    /// The mod's license of the mod
    pub license: License,
    /// The mod's client side support range
    pub client_side: ModSupportRange,
    /// The mod's server side support range
    pub server_side: ModSupportRange,
    /// The total number of downloads the mod has
    pub downloads: usize,
    /// A list of categories the mod is in
    pub categories: Vec<String>,
    /// The mod's version listed as their IDs
    pub versions: Vec<ID>,
    /// The link to the mod's icon
    pub icon_url: Option<String>,
    /// A link to submit bugs or issues about the mod
    pub issues_url: Option<String>,
    /// A link to the mod's source code
    pub source_url: Option<String>,
    /// A link to the mod's wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// A link to the mod's discord
    pub discord_url: Option<String>,
    /// A list of donation links the mod has
    pub donation_urls: Option<Vec<DonationLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct License {
    /// The license's ID
    pub id: String,
    /// The license's long name
    pub name: String,
    /// A link to this license
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DonationLink {
    /// The donation link's platform ID
    pub id: String,
    /// The platform's long name
    pub platform: String,
    /// A link to this donation
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ModStatus {
    #[serde(rename = "approved")]
    Approved,
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

#[derive(Deserialize, Serialize, Debug)]
pub enum ModSupportRange {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Display for ModSupportRange {
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
