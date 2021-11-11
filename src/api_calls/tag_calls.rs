use crate::{request::request_rel, Ferinth, Result};

impl Ferinth {
    /// List the categories of mods
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let categories = modrinth.list_categories().await?;
    /// assert_eq!(
    ///     categories.len(),
    ///     12,
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn list_categories(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/category".into())
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft mod loaders
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let mod_loaders = modrinth.list_loaders().await?;
    /// assert_eq!(
    ///     mod_loaders,
    ///     vec!["forge", "fabric"],
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn list_loaders(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/loader".into())
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft versions
    ///
    /// Example: NO
    /// 
    /// I don't know why this exists. 
    /// Just use [Mojang's version manifest](https://launchermeta.mojang.com/mc/game/version_manifest_v2.json) which is more informative
    pub async fn list_game_versions(&self) -> Result<Vec<String>> {
        Ok(request_rel(self, "/tag/game_version".into())
            .await?
            .json()
            .await?)
    }
}
