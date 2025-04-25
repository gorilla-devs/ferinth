//! Models related to miscellaneous API calls

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Report {
    pub id: ID,
    pub report_type: String,
    /// The ID of the item being reported
    pub item_id: ID,
    /// The type of item being reported
    pub item_type: ReportItemType,
    /// An extended explanation of the report
    pub body: String,
    /// The ID of the user who submitted the report
    pub reporter: ID,
    pub created: UtcTime,
    pub closed: bool,
    // The ID of the moderation thread associated with this report
    pub thread_id: ID,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReportSubmission {
    pub report_type: String,
    /// The ID of the item being reported
    pub item_id: ID,
    /// The type of item being reported
    pub item_type: ReportItemType,
    /// An extended explanation of the report
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportItemType {
    Project,
    User,
    Version,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Statistics {
    /// The number of project on Modrinth
    pub projects: Int,
    /// The number of versions on Modrinth
    pub versions: Int,
    /// The number of version files on Modrinth
    pub files: Int,
    /// The number of authors (users with projects) on Modrinth
    pub authors: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Welcome {
    pub about: String,
    pub documentation: Url,
    pub name: String,
    pub version: String,
}
