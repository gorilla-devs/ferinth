use super::*;
use serde::{Deserialize, Serialize};
use std::{clone::Clone, cmp::PartialEq};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    /// The user's ID
    pub id: ID,
    /// The user's Github ID. Only visible to this user
    pub github_id: Option<usize>,
    /// The user's username
    pub username: String,
    /// The user's display name. Only visible to this user
    pub name: Option<String>,
    /// The user's email. Only visible to this user
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
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: String,
    /// The user associated with the member
    pub user: User,
    /// This team member's role
    pub role: String,
    /// ? Unknown use
    pub permissions: Option<isize>,
    /// Whether the user has accepted membership
    pub accepted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UserRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "admin")]
    Admin,
}
