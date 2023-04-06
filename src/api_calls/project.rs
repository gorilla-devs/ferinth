//! API calls related to projects
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/projects)

use super::check_id_slug;
use crate::{
    request::RequestBuilderCustomSend,
    structures::{project::*, Number},
    url_ext::{UrlJoinAll, UrlWithQuery},
    Ferinth, Result, API_BASE_URL,
};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Url,
};

impl Ferinth {
    /// Get the project of `project_id`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// // Get a mod using its project ID
    /// let sodium = modrinth.get_project("AANobbMI").await?;
    /// assert_eq!(
    ///     sodium.title,
    ///     "Sodium",
    /// );
    ///
    /// // You can also use the project's slug
    /// let ok_zoomer = modrinth.get_project("ok-zoomer").await?;
    /// assert_eq!(ok_zoomer.title, "Ok Zoomer");
    /// # Ok(()) }
    /// ```
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send_json()
            .await
    }

    /// Delete the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.delete_project(project_id).await?;
    /// ```
    pub async fn delete_project(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Get multiple projects with IDs `project_ids`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let mods = modrinth.get_multiple_projects(&[
    ///     "AANobbMI",
    ///     "P7dR8mSH",
    ///     "gvQqBUqZ",
    ///     "YL57xq9U",
    /// ]).await?;
    /// assert_eq!(mods.len(), 4);
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_projects(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
        check_id_slug(project_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["projects"])
                    .with_query(&[("ids", &serde_json::to_string(project_ids)?)]),
            )
            .custom_send_json()
            .await
    }

    /// Edit multiple projects with IDs `project_ids` with the given `edits`
    pub async fn edit_multiple_projects(
        &self,
        project_ids: &[&str],
        edits: EditMultipleProjectsRequestBody,
    ) -> Result<()> {
        check_id_slug(project_ids)?;
        self.client
            .patch(API_BASE_URL.join_all(vec!["projects"]))
            .json(&edits)
            .custom_send()
            .await?;
        Ok(())
    }

    /// Get `count` number of random projects
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let random_mods = modrinth.get_random_projects(5).await?;
    /// assert_eq!(random_mods.len(), 5);
    /// # Ok(()) }
    /// ```
    pub async fn get_random_projects(&self, count: Number) -> Result<Vec<Project>> {
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["projects_random"])
                    .with_query(&[("count", &count.to_string())]),
            )
            .custom_send_json()
            .await
    }

    /// Change the icon of the project of `project_id` to `image` with file `ext`ension
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// # let project_id = env!("TEST_PROJECT_ID");
    /// # let image = std::fs::read("test_image.png").expect("Failed to read test image");
    /// modrinth.change_project_icon(
    ///     project_id,
    ///     image,
    ///     ferinth::structures::project::FileExt::PNG
    /// ).await
    /// # }
    /// ```
    pub async fn change_project_icon<B: Into<Body>>(
        &self,
        project_id: &str,
        image: B,
        ext: FileExt,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .patch(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "icon"])
                    .with_query(&[("ext", ext.to_string())]),
            )
            .body(image)
            .header(CONTENT_TYPE, format!("image/{}", ext))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Check if the given ID or slug refers to an existing project.
    /// If so, the ID of the project will be returned.
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let project_id = modrinth.does_exist("sodium").await?;
    /// assert_eq!(project_id, "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn does_exist(&self, project_id: &str) -> Result<String> {
        #[derive(serde::Deserialize)]
        struct Response {
            id: String,
        }
        check_id_slug(&[project_id])?;
        let res: Response = self
            .client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "check"]))
            .custom_send_json()
            .await?;
        Ok(res.id)
    }

    /// Add the given gallery `image`, with a file `ext`ention and an optional `title`, to `project_id`.
    /// State whether the image should be `featured` or not, and optionally provide a `description`.
    ///
    /// The image data can have a maximum size of `5 MiB`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.add_gallery_image(
    ///     project_id,
    ///     image_data,
    ///     project::FileExt::PNG,
    ///     true,
    ///     Some("Test image".to_string()),
    ///     Some("This is a test image".to_string()),
    /// ).await?;
    /// ```
    pub async fn add_gallery_image<B: Into<Body>>(
        &self,
        project_id: &str,
        image: B,
        ext: FileExt,
        featured: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        let mut query = vec![("ext", ext.to_string()), ("featured", featured.to_string())];
        if let Some(title) = title {
            query.push(("title", title));
        }
        if let Some(description) = description {
            query.push(("description", description));
        }
        self.client
            .post(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "gallery"])
                    .with_query(query),
            )
            .body(image)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_str(&format!("image/{}", ext))?,
            )
            .custom_send()
            .await?;
        Ok(())
    }

    /// Delete the gallery image of `image_url` from the project of `project_id`
    ///
    /// ```ignore
    /// modrinth.delete_gallery_image(project_id, image_url).await?;
    /// ```
    pub async fn delete_gallery_image<U: Into<Url>>(
        &self,
        project_id: &str,
        image_url: U,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "gallery"])
                    .with_query(&[("url", image_url.into())]),
            )
            .custom_send()
            .await?;
        Ok(())
    }

    /// Get the dependencies of the project of `project_id`
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let fabric_api = modrinth.get_project_dependencies("fabric-api").await?;
    /// // Fabric API should not have any dependencies
    /// assert!(fabric_api.projects.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn get_project_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "dependencies"]))
            .custom_send_json()
            .await
    }

    /// Follow the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.follow(project_id).await?;
    /// ```
    pub async fn follow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .post(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Unfollow the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// modrinth.unfollow(project_id).await?;
    /// ```
    pub async fn unfollow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .custom_send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{structures::project, Ferinth, Result};

    #[tokio::test]
    async fn follow() -> Result<()> {
        let modrinth = Ferinth::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            Some(env!("MODRINTH_TOKEN")),
        )?;
        let project_id = env!("TEST_PROJECT_ID");

        modrinth.follow(project_id).await?;
        modrinth.unfollow(project_id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn gallery() -> Result<()> {
        let modrinth = Ferinth::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            Some(env!("MODRINTH_TOKEN")),
        )?;
        let project_id = env!("TEST_PROJECT_ID");

        let project = modrinth.get_project(project_id).await?;
        modrinth
            .delete_gallery_image(project_id, project.gallery[0].url.clone())
            .await?;

        let image_data = std::fs::read("test_image.png").expect("Failed to read test image");
        modrinth
            .add_gallery_image(
                project_id,
                image_data,
                project::FileExt::PNG,
                true,
                Some("Test image, do not delete".to_string()),
                Some(chrono::offset::Local::now().to_string()),
            )
            .await?;

        Ok(())
    }
}
