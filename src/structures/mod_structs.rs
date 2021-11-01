use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mod {
    /// The ID of the mod
    pub id: ID,
    /// The slug of a mod. This can change at any time
    pub slug: String,
    /// The ID of the team that has ownership of this mod
    pub team: ID,
    /// The title or name of the mod
    pub title: String,
    /// A short description of the mod
    pub description: String,
    /// A long form of the description
    pub body: String,
    #[deprecated(note = "Read from `Mod.body` instead")]
    /// The link to the long description of the mod
    pub body_url: Option<String>,
    /// The date at which the mod was first published
    pub published: Datetime,
    /// The date at which the mod was last updated
    pub updated: Datetime,
    /// The status of the mod
    pub status: ModStatus,
    /// The license of the mod
    pub license: License,
    /// The client side support range
    pub client_side: ModSupportRange,
    /// The server side support range
    pub server_side: ModSupportRange,
    /// The total number of downloads the mod has
    pub downloads: usize,
    /// A list of categories the mod is in
    pub categories: Vec<String>,
    /// A list of IDs for versions of the mod
    pub versions: Vec<ID>,
    /// The link to the mod's icon
    pub icon_url: Option<String>,
    /// A link to submit bugs or issues about the mod to
    pub issues_url: Option<String>,
    /// A link to the source code for the mod
    pub source_url: Option<String>,
    /// A link to the mod's wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// A link to the mod's discord
    pub discord_url: Option<String>,
    /// A list of all donation links the mod has
    pub donation_urls: Option<Vec<DonationLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct License {
    /// The license ID of a mod, retrieved from the license's get route
    pub id: String,
    /// The long name of the license
    pub name: String,
    /// The URL to this license
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
/// A donation link of a mod, representing the platform's id, platform name, and a URL
pub struct DonationLink {
    /// The platform ID of a mod, retrieved from the donation platform's get route
    pub id: String,
    /// The long name of the platform
    pub platform: String,
    /// The URL to this donation link
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ModStatus {
    Approved,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ModSupportRange {
    Required,
    Optional,
    Unsupported,
    Unknown,
}
