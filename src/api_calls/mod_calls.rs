use super::super::structures::mod_structs::*;
use crate::{request::request_rel, ModrinthAPI, Result};

impl ModrinthAPI {
    /// Get the mod corresponding to `mod_id`
    pub async fn get_mod(&self, mod_id: String) -> Result<Mod> {
        Ok(request_rel(self, format!("/mod/{}", mod_id))
            .await?
            .json()
            .await?)
    }
}
