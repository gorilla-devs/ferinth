//! Models related to miscellaneous API calls

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Report {
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ReportItemType {
    Project,
    User,
    Version,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Statistics {
    pub projects: Number,
    pub versions: Number,
    pub authors: Number,
    pub files: Number,
}
