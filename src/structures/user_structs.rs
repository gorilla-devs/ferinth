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
    /// The user's display name
    pub name: String,
    /// The user's email. Only visible to the user themselves
    pub email: Option<String>,
    /// A URL to the user's avatar's. Uses Github's icon
    pub avatar_url: Option<String>,
    /// A description of the user
    pub bio: String,
    /// The time at which the user was created
    pub created: Datetime,
    /// The user's role
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
