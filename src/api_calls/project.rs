//! API calls related to projects
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/projects)

use super::*;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body,
};
use structures::{project::*, Number, UtcTime};

impl Ferinth {
    /**
    Get the project of `project_id`

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    // Get a mod using its project ID
    let sodium = modrinth.get_project("AANobbMI").await?;
    assert_eq!(sodium.title, "Sodium");

    // You can also use the project's slug, which is case-insensitive
    let ok_zoomer = modrinth.get_project("ok-zoomer").await?;
    assert_eq!(ok_zoomer.title, "Ok Zoomer");
    # Ok(()) }
    ```
    */
    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send_json()
            .await
    }

    /// Delete the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION and appropriate permissions!
    pub async fn delete_project(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the projects of `project_ids`

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let projects = modrinth.get_multiple_projects(&[
        "AANobbMI",
        "P7dR8mSH",
        "gvQqBUqZ",
        "YL57xq9U",
    ]).await?;
    assert_eq!(projects.len(), 4);
    # Ok(()) }
    ```
    */
    pub async fn get_multiple_projects(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
        check_id_slug(project_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["projects"])
                    .with_query_json("ids", project_ids)?,
            )
            .custom_send_json()
            .await
    }

    /// Edit the projects of `project_ids` with the given `edits`
    ///
    /// REQUIRES AUTHENTICATION and appropriate permissions!
    pub async fn edit_multiple_projects(
        &self,
        project_ids: &[&str],
        edits: EditMultipleProjectsBody,
    ) -> Result<()> {
        check_id_slug(project_ids)?;
        self.client
            .patch(API_BASE_URL.join_all(vec!["projects"]))
            .json(&edits)
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get `count` number of random projects

    Due to [an issue with labrinth](https://github.com/modrinth/labrinth/issues/548),
    the amount of mods returned will most likely be less than `count`.

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let random_mods = modrinth.get_random_projects(5).await?;
    // The proper check has been disabled due to the reason mentioned above
    // assert_eq!(random_mods.len(), 5);
    assert!(random_mods.len() <= 5);
    # Ok(()) }
    ```
    */
    pub async fn get_random_projects(&self, count: Number) -> Result<Vec<Project>> {
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["projects_random"])
                    .with_query("count", count),
            )
            .custom_send_json()
            .await
    }

    /// Change the icon of the project of `project_id` to `image` with file `ext`ension
    ///
    /// REQUIRES AUTHENTICATION and appropriate permissions!
    pub async fn change_project_icon<B: Into<Body>>(
        &self,
        project_id: &str,
        image: B,
        ext: &ImageFileExt,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .patch(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "icon"])
                    .with_query("ext", ext),
            )
            .body(image)
            .header(CONTENT_TYPE, format!("image/{}", ext))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Delete the icon of the project of `project_id`
    ///
    /// REQUIRES AUTHENTICATION and appropriate permissions!
    pub async fn delete_project_icon(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id, "icon"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Check if the given ID or slug refers to an existing project.
    If so, the ID of the project will be returned.

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let project_id = modrinth.check_validity("sodium").await?;
    assert_eq!(project_id, "AANobbMI");
    # Ok(()) }
    ```
    */
    pub async fn check_validity(&self, project_id: &str) -> Result<String> {
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

    /**
    Add the given gallery `image` of file `ext`ention, and an optional `title`, to `project_id`.
    State whether the image should be `featured` or not, and optionally provide a `description`.

    The image data can have a maximum size of `5 MiB`.

    REQUIRES AUTHENTICATION and appropriate permissions!
    */
    pub async fn add_gallery_image<B: Into<Body>>(
        &self,
        project_id: &str,
        image: B,
        ext: &ImageFileExt,
        featured: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        let mut url = API_BASE_URL.join_all(vec!["project", project_id, "gallery"]);
        url = url.with_query("ext", ext).with_query("featured", featured);
        if let Some(title) = title {
            url = url.with_query("title", title);
        }
        if let Some(description) = description {
            url = url.with_query("description", description);
        }
        self.client
            .post(url)
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
    /// REQUIRES AUTHENTICATION and appropriate permissions!
    pub async fn delete_gallery_image<S: ToString>(
        &self,
        project_id: &str,
        image_url: S,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "gallery"])
                    .with_query("url", image_url),
            )
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the dependencies of the project of `project_id`

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let fabric_api = modrinth.get_project_dependencies("fabric-api").await?;
    // Fabric API should not have any dependencies
    assert!(fabric_api.projects.is_empty());
    # Ok(()) }
    ```
    */
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
    pub async fn unfollow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Schedule a change of `status` at `time` to the project of `project_id`

    REQUIRES AUTHENTICATION and appropriate permissions!

    ```no_run
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    // Release the project of ID `eZ2NOONn` in three hours to the public
    modrinth.schedule_project(
        "eZ2NOONn",
        &(chrono::offset::Utc::now() + chrono::Duration::hours(3)),
        &ferinth::structures::project::RequestedStatus::Approved
    ).await
    # }
    ```
    */
    pub async fn schedule_project(
        &self,
        project_id: &str,
        time: &UtcTime,
        status: &RequestedStatus,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .post(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "schedule"])
                    .with_query_json("time", time)?
                    .with_query_json("requested_status", status)?,
            )
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
        modrinth.unfollow(project_id).await
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
                &project::ImageFileExt::PNG,
                true,
                Some("Test image, do not delete".to_string()),
                Some(chrono::offset::Local::now().to_string()),
            )
            .await
    }

    #[tokio::test]
    async fn project_icon() -> Result<()> {
        let modrinth = Ferinth::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            Some(env!("MODRINTH_TOKEN")),
        )?;
        let project_id = env!("TEST_PROJECT_ID");

        modrinth.delete_project_icon(project_id).await?;
        let image = std::fs::read("test_image.png").expect("Cannot read test image");
        modrinth
            .change_project_icon(project_id, image, &project::ImageFileExt::PNG)
            .await
    }
}
