use crate::{
    request::{request, API_URL_BASE},
    structures::tag_structs::*,
    Ferinth, Result,
};

impl Ferinth {
    /// List the categories a project can take
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// let categories = modrinth.list_categories().await?;
    /// assert!(categories.len() == 12);
    /// # Ok(()) }
    /// ```
    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        Ok(request(self, API_URL_BASE.join("tag/")?.join("category")?)
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft mod loaders
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// let mod_loaders = modrinth.list_loaders().await?;
    /// assert!(mod_loaders.len() == 2);
    /// # Ok(()) }
    /// ```
    pub async fn list_loaders(&self) -> Result<Vec<Loader>> {
        Ok(request(self, API_URL_BASE.join("tag/")?.join("loader")?)
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft versions
    ///
    /// Example: no
    ///
    /// I don't know why this exists.
    /// Just use [Mojang's version manifest](https://launchermeta.mojang.com/mc/game/version_manifest_v2.json) which is much more informative
    pub async fn list_game_versions(&self) -> Result<Vec<String>> {
        Ok(
            request(self, API_URL_BASE.join("tag/")?.join("game_version")?)
                .await?
                .json()
                .await?,
        )
    }
}
