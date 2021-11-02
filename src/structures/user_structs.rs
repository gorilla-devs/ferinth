use serde::{Deserialize, Serialize};
use super::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    /// The user's ID
    pub id: ID,
    /// The user's Github ID. Only visible to the user themselves
    pub github_id: Option<usize>,
    /// The user's username
    pub username: String,
    /// The user's display name. Only visible to the user themselves
    pub name: Option<String>,
    /// The user's email. Only visible to the user themselves
    pub email: Option<String>,
    /// A URL to the user's avatar's. Uses Github's icon
    pub avatar_url: Option<String>,
    /// A description of the user
    pub bio: Option<String>,
    /// The time at which the user was created
    pub created: Datetime,
    /// The user's role
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: String,
    /// The user ID of this team member
    pub user_id: String,
    /// This team member's role
    /// 
    /// This field _should_ be an enum, but because it can be anything (for some reason) its a string
    // pub role: TeamRole,
    pub role: String,
    /// ? Unknown use
    pub permissions: Option<isize>,
    /// Whether the user has accepted membership
    pub accepted: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TeamRole {
    #[serde(alias = "OWNER")] // Bug in Labrinth. I WANT CASE INSENSITIVE DESERIALISATION
    Owner,
    Member,
    Maintainer,
    Contributor,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "admin")]
    Admin,
}
