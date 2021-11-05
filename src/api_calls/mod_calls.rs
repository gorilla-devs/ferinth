use crate::{request::request_rel, structures::mod_structs::*, Ferinth, Result};

impl Ferinth {
    /// Get mod with ID `mod_id`
    pub async fn get_mod(&self, mod_id: &str) -> Result<Mod> {
        request_rel(self, format!("/mod/{}", mod_id))
            .await?
            .json()
            .await
    }
}
