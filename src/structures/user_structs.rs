use super::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct User {
    /// The user's ID
    pub id: ID,
    /// The user's GitHub username. Only visible to the user
    pub github_id: Option<usize>,
    /// The user's username
    pub username: String,
    /// The user's display name. Only visible to the user
    pub name: Option<String>,
    /// The user's email. Only visible to the user
    pub email: Option<String>,
    /// A link to the user's avatar
    pub avatar_url: Option<String>,
    /// A description of the user
    pub bio: Option<String>,
    /// The time at which the user was created
    pub created: Datetime,
    /// The user's role
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: String,
    /// The user associated with the member
    pub user: User,
    /// This team member's role
    pub role: String,
    /// ? The user's permissions in bitflag format
    pub permissions: Option<isize>,
    /// Whether the user has accepted membership
    pub accepted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
