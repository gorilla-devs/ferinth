use crate::{
    api_calls::check_id_slug, request::request_rel, structures::mod_structs::*, Ferinth, Result,
};

impl Ferinth {
    /// Get mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let sodium_mod = modrinth.get_mod("AANobbMI").await?;
    /// assert_eq!(
    ///     sodium_mod.title,
    ///     "Sodium",
    /// );
    /// 
    /// // You can also use the mod slug
    /// let ok_zoomer_mod = modrinth.get_mod("ok-zoomer").await?;
    /// assert_eq!(
    ///     ok_zoomer_mod.title,
    ///     "Ok Zoomer",
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn get_mod(&self, mod_id: &str) -> Result<Mod> {
        check_id_slug(mod_id)?;
        Ok(request_rel(self, format!("/mod/{}", mod_id))
            .await?
            .json()
            .await?)
    }
}
