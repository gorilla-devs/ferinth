use super::super::structures::user_structs::*;
use crate::{request::request_rel, ModrinthAPI, Result};

impl ModrinthAPI {
    /// Get user with ID `user_id`
    pub async fn get_user(&self, user_id: String) -> Result<User> {
        Ok(request_rel(self, format!("/user/{}", user_id))
            .await?
            .json()
            .await?)
    }
}
