use crate::{request::request_rel, Ferinth, Result};

impl Ferinth {
    /// List the categories of mods
    pub async fn list_categories(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/category".into())
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft mod loaders
    pub async fn list_loaders(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/loader".into())
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft versions
    pub async fn list_game_versions(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/game_version".into())
            .await?
            .json()
            .await?)
    }
}
