use super::super::structures::user_structs::*;
use crate::{request::request_rel, Ferinth, Result};

impl Ferinth {
    /// Get user with ID `user_id`
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        Ok(request_rel(self, format!("/user/{}", user_id))
            .await?
            .json()
            .await?)
    }

    /// List the members of team with ID `team_id`
    pub async fn list_team_members(&self, team_id: &str) -> Result<Vec<TeamMember>> {
        Ok(request_rel(self, format!("/team/{}/members", team_id))
            .await?
            .json()
            .await?)
    }
}
