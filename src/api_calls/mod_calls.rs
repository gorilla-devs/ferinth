use super::super::structures::mod_structs::*;
use crate::{request::request_rel, Ferinth, Result};

impl Ferinth {
    /// Get the mod corresponding to `mod_id`
    pub async fn get_mod(&self, mod_id: &str) -> Result<Mod> {
        Ok(request_rel(self, format!("/mod/{}", mod_id))
            .await?
            .json()
            .await?)
    }
}
