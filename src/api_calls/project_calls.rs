use crate::{
    api_calls::check_id_slug,
    request::{request, API_URL_BASE},
    structures::project_structs::*,
    Ferinth, Result,
};

impl Ferinth {
    /// Get a project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
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
    /// # Ok(()) }
    /// ```
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        check_id_slug(project_id)?;
        Ok(request(self, API_URL_BASE.join("project/")?.join(project_id)?)
            .await?
            .json()
            .await?)
    }
}
