//! API calls related to projects
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/projects)

use super::*;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, IntoUrl,
};
use structures::{project::*, Int, UtcTime};

impl<T> Ferinth<T> {
    /**
    Get the project of `project_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    // Get a mod using its project ID
    let sodium = modrinth.project_get("AANobbMI").await?;
    assert_eq!(sodium.title, "Sodium");

    // You can also use the project's slug, which is case-insensitive
    let ok_zoomer = modrinth.project_get("ok-zoomer").await?;
    assert_eq!(ok_zoomer.title, "Ok Zoomer");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_get(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send_json()
            .await
    }

    /**
    Get the projects of `project_ids`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    // You can use both IDs and slugs
    let projects = modrinth.project_get_multiple(&[
        "sodium",
        "P7dR8mSH",
        "iris",
        "gvQqBUqZ",
    ]).await?;
    assert_eq!(projects.len(), 4);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_get_multiple(&self, project_ids: &[&str]) -> Result<Vec<Project>> {
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

    /**
    Get `count` number of random projects

    Due to [an issue with labrinth](https://github.com/modrinth/labrinth/issues/548),
    the amount of projects returned will most likely be less than `count`.

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let random_projects = modrinth.project_get_random(5).await?;
    // The proper check has been disabled due to the reason mentioned above
    // assert_eq!(random_projects.len(), 5);
    assert!(random_projects.len() <= 5);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_get_random(&self, count: Int) -> Result<Vec<Project>> {
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["projects_random"])
                    .with_query("count", count),
            )
            .custom_send_json()
            .await
    }

    /**
    Check if the given ID or slug refers to an existing project,
    if so the ID of the project will be returned

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let project_id = modrinth.project_check_validity("sodium").await?;
    assert_eq!(project_id, "AANobbMI");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_check_validity(&self, project_id: &str) -> Result<String> {
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
    Get the dependencies of the project of `project_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let fabric_api = modrinth.project_get_dependencies("fabric-api").await?;
    // Fabric API should not have any dependencies
    assert!(fabric_api.projects.is_empty());
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_get_dependencies(&self, project_id: &str) -> Result<ProjectDependencies> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "dependencies"]))
            .custom_send_json()
            .await
    }
}

impl Ferinth<Authenticated> {
    /// Delete the project of `project_id`
    pub async fn project_delete(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Bulk edit the projects of `project_ids` with the given `edits`
    pub async fn project_edit_multiple(
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

    /// Change the icon of the project of `project_id` to `image` with file `ext`ension
    pub async fn project_edit_icon(
        &self,
        project_id: &str,
        image: impl Into<Body>,
        ext: ImageFileExt,
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
    pub async fn project_delete_icon(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id, "icon"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Add `image` of file `ext`ention and optional `title` to the gallery of the project of `project_id`.
    State whether the image should be `featured` or not, and optionally provide a `description`.

    The image data can have a maximum size of `5 MiB`.
    */
    pub async fn project_add_gallery_image<B: Into<Body>>(
        &self,
        project_id: &str,
        image: B,
        ext: &ImageFileExt,
        featured: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        let mut url = API_BASE_URL
            .join_all(vec!["project", project_id, "gallery"])
            .with_query("ext", ext)
            .with_query("featured", featured);
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

    /// Modify the gallery image of `url` of the project of `project_id`
    pub async fn project_edit_gallery_image<U: IntoUrl>(
        &self,
        project_id: &str,
        url: U,
        featured: Option<bool>,
        title: Option<&str>,
        description: Option<&str>,
        ordering: Option<Int>,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        let mut url = API_BASE_URL
            .join_all(vec!["project", project_id, "gallery"])
            .with_query("url", url.into_url()?);
        if let Some(featured) = featured {
            url = url.with_query("featured", featured);
        }
        if let Some(title) = title {
            url = url.with_query("title", title);
        }
        if let Some(description) = description {
            url = url.with_query("description", description);
        }
        if let Some(ordering) = ordering {
            url = url.with_query("ordering", ordering);
        }
        self.client.patch(url).custom_send().await?;
        Ok(())
    }

    /// Delete the gallery image of `image_url` from the project of `project_id`
    pub async fn project_delete_gallery_image<U: IntoUrl>(
        &self,
        project_id: &str,
        image_url: U,
    ) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(
                API_BASE_URL
                    .join_all(vec!["project", project_id, "gallery"])
                    .with_query("url", image_url.into_url()?),
            )
            .custom_send()
            .await?;
        Ok(())
    }

    /// Follow the project of `project_id`
    pub async fn project_follow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .post(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /// Unfollow the project of `project_id`
    pub async fn project_unfollow(&self, project_id: &str) -> Result<()> {
        check_id_slug(&[project_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["project", project_id, "follow"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Schedule a change of `status` at `time` to the project of `project_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    // Release the project of ID `XXXXXXXX` in three hours to the public
    modrinth.project_schedule(
        "XXXXXXXX",
        &(chrono::offset::Utc::now() + chrono::Duration::hours(3)),
        &ferinth::structures::project::RequestedStatus::Approved
    ).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn project_schedule(
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
    use super::*;

    #[tokio::test]
    async fn follow() -> Result<()> {
        let modrinth = Ferinth::<Authenticated>::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            env!("MODRINTH_TOKEN"),
        )?;
        let project_id = env!("TEST_PROJECT_ID");
        let user_id = modrinth.user_get_current().await?.id;

        match modrinth.project_follow(project_id).await {
            Ok(_) => {}
            Err(Error::ReqwestError(e)) => {
                if !(e.is_status() && e.status().unwrap() == 400) {
                    return Err(Error::ReqwestError(e));
                }
            }
            Err(e) => return Err(e),
        }
        let followed_projects = modrinth.user_list_followed_projects(&user_id).await?;
        assert!(followed_projects
            .iter()
            .map(|p| &p.id)
            .any(|id| id == project_id));

        modrinth.project_unfollow(project_id).await?;
        let followed_projects = modrinth.user_list_followed_projects(&user_id).await?;
        assert!(followed_projects
            .iter()
            .map(|p| &p.id)
            .all(|id| id != project_id));

        Ok(())
    }

    #[tokio::test]
    async fn gallery() -> Result<()> {
        let modrinth = Ferinth::<Authenticated>::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            env!("MODRINTH_TOKEN"),
        )?;
        let project_id = env!("TEST_PROJECT_ID");

        let project = modrinth.project_get(project_id).await?;
        if !project.gallery.is_empty() {
            assert_ne!(project.gallery[0].title, Some("Modified test image".into()));
            modrinth
                .project_edit_gallery_image(
                    project_id,
                    project.gallery[0].url.clone(),
                    Some(false),
                    Some("Modified test image"),
                    None,
                    None,
                )
                .await?;
            let project = modrinth.project_get(project_id).await?;
            assert_eq!(project.gallery[0].title, Some("Modified test image".into()));
            assert!(!project.gallery[0].featured);

            modrinth
                .project_delete_gallery_image(project_id, project.gallery[0].url.clone())
                .await?;
            let project = modrinth.project_get(project_id).await?;
            assert!(project.gallery.is_empty());
        }

        let image_data = std::fs::read("test_image.png").expect("Failed to read test image");
        modrinth
            .project_add_gallery_image(
                project_id,
                image_data,
                &project::ImageFileExt::PNG,
                true,
                Some("Test image, do not delete".to_string()),
                Some(chrono::offset::Local::now().to_string()),
            )
            .await?;
        let project = modrinth.project_get(project_id).await?;
        assert_eq!(
            project.gallery[0].title,
            Some("Test image, do not delete".into())
        );

        Ok(())
    }

    #[tokio::test]
    async fn project_icon() -> Result<()> {
        let modrinth = Ferinth::<Authenticated>::new(
            env!("CARGO_CRATE_NAME"),
            Some(env!("CARGO_PKG_VERSION")),
            None,
            env!("MODRINTH_TOKEN"),
        )?;
        let project_id = env!("TEST_PROJECT_ID");

        modrinth.project_delete_icon(project_id).await?;
        let project = modrinth.project_get(project_id).await?;
        assert!(project.icon_url.is_none());

        let image = std::fs::read("test_image.png").expect("Cannot read test image");
        modrinth
            .project_edit_icon(project_id, image, project::ImageFileExt::PNG)
            .await?;
        let project = modrinth.project_get(project_id).await?;
        assert!(project.icon_url.is_some());

        Ok(())
    }
}
