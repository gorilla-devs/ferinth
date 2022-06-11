use crate::{
    api_calls::check_id_slug, request::API_URL_BASE, structures::project_structs::*, Ferinth,
    Result,
};

impl Ferinth {
    /// Get a project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
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
        Ok(self
            .get(API_URL_BASE.join("project/")?.join(project_id)?)
            .await?
            .json()
            .await?)
    }

    /// Query a list of project ids at once
    /// 
    /// Examples:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let mut project_ids = vec![];
    /// project_ids.push("gvQqBUqZ".to_string()); // Lithium
    /// // can also use slugs
    /// project_ids.push("sodium".to_string()); 
    /// let projects = modrinth.get_multiple_projects(project_ids.clone()).await?;
    /// for project in projects {
    ///     assert!(project_ids.contains(&project.id) || project_ids.contains(&project.slug));
    /// }
    /// # Ok(())}
    /// ```
    pub async fn get_multiple_projects(&self, project_ids: Vec<String>) -> Result<Vec<Project>> {
        for id in &project_ids {
            check_id_slug(&id)?;
        }
        Ok(self
            .get(
                API_URL_BASE
                    .join("projects")?
                    .join(&format!("?ids={}", serde_json::to_string(&project_ids)?))?,
            )
            .await?
            .json()
            .await?)
    }

    /// Get the dependencies of the project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let fabric_api = modrinth.get_project_dependencies("fabric-api").await?;
    /// // The Fabric API should not have any dependencies
    /// assert!(fabric_api.projects.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn get_project_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(project_id)?;
        Ok(self
            .get(
                API_URL_BASE
                    .join("project/")?
                    .join(&format!("{}/", project_id))?
                    .join("dependencies")?,
            )
            .await?
            .json()
            .await?)
    }
}
