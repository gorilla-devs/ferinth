use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    /// The user's ID
    pub id: ID,
    /// The user's GitHub ID
    pub github_id: usize,
    /// The user's username
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email, only visible to the user
    pub email: Option<String>,
    /// A link to the user's avatar
    pub avatar_url: String,
    /// A description of the user
    pub bio: Option<String>,
    /// The time at which the user was created
    pub created: Datetime,
    /// The user's role
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: ID,
    /// The user associated with the member
    pub user: User,
    /// This team member's role
    pub role: String,
    /// The user's permissions in bitflag format
    /// (requires authorization to view)
    ///
    /// In order from first to eighth bit, the bits are:
    /// - UPLOAD_VERSION
    /// - DELETE_VERSION
    /// - EDIT_DETAILS
    /// - EDIT_BODY
    /// - MANAGE_INVITES
    /// - REMOVE_MEMBER
    /// - EDIT_MEMBER
    /// - DELETE_PROJECT
    pub permissions: Option<u8>,
    /// Whether the user has accepted membership on the team
    /// (requires authorization to view)
    pub accepted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
