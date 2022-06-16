use crate::{
    api_calls::check_id_slug,
    request::{request, API_URL_BASE},
    structures::project_structs::*,
    Ferinth, Result,
};

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
pub async fn get_project(client: &Ferinth, project_id: &str) -> Result<Project> {
    check_id_slug(project_id)?;
    Ok(
        request(client, API_URL_BASE.join("project/")?.join(project_id)?)
            .await?
            .json()
            .await?,
    )
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
pub async fn get_project_dependencies(
    client: &Ferinth,
    project_id: &str,
) -> Result<ProjectDependencies> {
    check_id_slug(project_id)?;
    Ok(request(
        client,
        API_URL_BASE
            .join("project/")?
            .join(&format!("{}/", project_id))?
            .join("dependencies")?,
    )
    .await?
    .json()
    .await?)
}
