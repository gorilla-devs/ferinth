use crate::{structures::mod_structs::*, request::request_rel, Ferinth, Result};

impl Ferinth {
    /// Get mod with ID `mod_id`
    pub async fn get_mod(&self, mod_id: &str) -> Result<Mod> {
        Ok(request_rel(self, format!("/mod/{}", mod_id))
            .await?
            .json()
            .await?)
    }
}
