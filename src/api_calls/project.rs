use super::check_id_slug;
use crate::{
    request::API_URL_BASE, structures::project::*, url_join_ext::UrlJoinExt, Ferinth,
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

    /// Check if the given ID or slug refers to an existing project.
    /// If so, the ID of the project will be returned.
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let project_id = modrinth.does_exist("sodium").await?;
    /// assert!(project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn does_exist(&self, project_id: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct Response {
            id: String,
        }
        check_id_slug(project_id)?;
        let res: Response = self
            .get(API_URL_BASE.join_all(vec!["project", project_id, "check"]))
            .await?;
        Ok(res.id)
    }

    /// Add the given gallery `image`, with the file `ext`ention and an optional `title`, to `project_id`.
    /// State whether the image should be `featured` or not, and optionally provide a `description`.
    ///
    /// The image data can have a maximum size of `5 MiB`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// modrinth.add_gallery_image(
    ///     env!("TEST_PROJECT_ID"),
    ///     &std::fs::read("test_image.png").expect("Failed to read test image"),
    ///     ferinth::structures::project::FileExt::PNG,
    ///     false,
    ///     Some("Test image".to_string()),
    ///     None,
    /// ).await?;
    /// # Ok(()) }
    /// ```

    /* TODO: Binary POST body required
    pub async fn add_gallery_image(
        &self,
        project_id: &str,
        image: &[u8],
        ext: FileExt,
        featured: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        check_id_slug(project_id)?;
        self.post_with_query(
            API_URL_BASE.join_all(vec!["project", project_id, "gallery"]),
            image,
            &{
                let mut query = vec![
                    ("ext", serde_json::to_string(&ext)?),
                    ("featured", serde_json::to_string(&featured)?),
                ];
                if let Some(title) = title {
                    query.push(("title", title));
                }
                if let Some(description) = description {
                    query.push(("description", description));
                }
                query
            },
        )
        .await
    } */

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

    /// Follow the given `project_id`.
    /// 
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// # if let Err(ferinth::Error::ReqwestError(error)) =
    /// modrinth.follow(env!("TEST_PROJECT_ID")).await
    /// # {   if let Some(status) = error.status()  {
    /// #         // The test project might have already been followed
    /// #         if status.as_u16() == 400 {
    /// #             return Ok(());
    /// #         }
    /// #     }
    /// #     return Err(error.into());
    /// # }
    /// # Ok(()) }
    /// ```
    pub async fn follow(&self, project_id: &str) -> Result<()> {
        check_id_slug(project_id)?;
        self.post(
            API_URL_BASE.join_all(vec!["project", project_id, "follow"]),
            "",
        )
        .await
    }
}
