use super::check_id_slug;
use crate::{
    request::API_URL_BASE, structures::project_structs::*, url_join_ext::UrlJoinExt, Ferinth,
    Result,
};

impl Ferinth {
    /// Get a project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
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
        self.get(API_URL_BASE.join_all(vec!["project", project_id]))
            .await
    }

    /// Get multiple projects with IDs `project_ids`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let mods = modrinth.get_multiple_projects(&[
    ///     "AANobbMI",
    ///     "P7dR8mSH",
    ///     "gvQqBUqZ",
    ///     "YL57xq9U",
    /// ]).await?;
    /// assert!(mods.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_projects(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
        for project_id in project_ids {
            check_id_slug(project_id)?;
        }
        self.get_with_query(
            API_URL_BASE.join_all(vec!["projects"]),
            &[("ids", &serde_json::to_string(project_ids)?)],
        )
        .await
    }

    /// Get the dependencies of the project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let fabric_api = modrinth.get_project_dependencies("fabric-api").await?;
    /// // The Fabric API should not have any dependencies
    /// assert!(fabric_api.projects.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn get_project_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(project_id)?;
        self.get(API_URL_BASE.join_all(vec!["project", project_id, "dependencies"]))
            .await
    }
}
