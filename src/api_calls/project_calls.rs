use crate::{
    api_calls::check_id_slug, request::request_rel, structures::project_structs::*, Ferinth, Result,
};

impl Ferinth {
    /// Get a project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let sodium_mod = modrinth.get_project("AANobbMI").await?;
    /// assert_eq!(
    ///     sodium_mod.title,
    ///     "Sodium",
    /// );
    ///
    /// // You can also use the project slug
    /// let ok_zoomer_mod = modrinth.get_project("ok-zoomer").await?;
    /// assert_eq!(
    ///     ok_zoomer_mod.title,
    ///     "Ok Zoomer",
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn get_project(&self, mod_id: &str) -> Result<Project> {
        check_id_slug(mod_id)?;
        Ok(request_rel(self, format!("/project/{}", mod_id))
            .await?
            .json()
            .await?)
    }
}
