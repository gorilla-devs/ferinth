//! API calls related to versions
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/versions)

use super::*;
use crate::structures::{version::*, UtcTime};

impl Ferinth {
    /**
    Get the versions of the project of `project_id`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    sodium_versions.iter().for_each(|v| assert_eq!(v.project_id, "AANobbMI"));
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_versions(&self, project_id: &str) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "version"]))
            .custom_send_json()
            .await
    }

    /**
    Get the versions of the project of `project_id`, filtered based on
    mod `loaders`, `game_versions`, and whether the version is `featured`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_forge_versions = modrinth.list_versions_filtered(
        "AANobbMI",
        Some(&["forge"]),
        None,
        None
    ).await?;
    // Sodium is not made for Forge
    assert!(sodium_forge_versions.is_empty());
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_versions_filtered(
        &self,
        project_id: &str,
        loaders: Option<&[&str]>,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;
        let mut url = API_BASE_URL.join_all(vec!["project", project_id, "version"]);
        if let Some(loaders) = loaders {
            url = url.with_query_json("loaders", loaders)?;
        }
        if let Some(game_versions) = game_versions {
            url = url.with_query_json("game_versions", game_versions)?;
        }
        if let Some(featured) = featured {
            url = url.with_query_json("featured", featured)?;
        }
        self.client.get(url).custom_send_json().await
    }

    /**
    Get the version of `version_id`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_version = modrinth.get_version("xuWxRZPd").await?;
    assert_eq!(sodium_version.project_id, "AANobbMI");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_version(&self, version_id: &str) -> Result<Version> {
        check_id_slug(&[version_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["version", version_id]))
            .custom_send_json()
            .await
    }

    /**
    Delete the version of `version_id`

    REQUIRES AUTHENTICATION and appropriate permissions!

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    modrinth.delete_version("XXXXXXXX").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn delete_version(&self, version_id: &str) -> Result<()> {
        check_id_slug(&[version_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["version", version_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the version of the version `number` from the project of `project_id`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let version = modrinth.get_version_from_number("sodium", "mc1.17.1-0.3.2").await?;
    assert_eq!(version.id, "xuWxRZPd");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_version_from_number(&self, project_id: &str, number: &str) -> Result<Version> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "version", number]))
            .custom_send_json()
            .await
    }

    /**
    Schedule changing the status of version of `version_id` to `requested_status` at `time`

    REQUIRES AUTHENTICATION and appropriate permissions!

    ```no_run
    # use ferinth::structures::version::RequestedStatus;
    # use chrono::{Duration, offset::Utc};
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    // Release the version of ID `xuWxRZPd` to the public in three hours
    modrinth.schedule_version(
        "xuWxRZPd",
        &(Utc::now() + Duration::hours(3)),
        &RequestedStatus::Listed
    ).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn schedule_version(
        &self,
        version_id: &str,
        time: &UtcTime,
        status: &RequestedStatus,
    ) -> Result<()> {
        check_id_slug(&[version_id])?;
        self.client
            .post(
                API_BASE_URL
                    .join_all(vec!["version", version_id, "schedule"])
                    .with_query_json("time", time)?
                    .with_query_json("requested_status", status)?,
            )
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the versions of `version_ids`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let versions = modrinth.get_multiple_versions(&[
        "sxWTUZpD",
        "mgPpe4NY",
    ]).await?;
    versions.iter().for_each(|v| assert_eq!(v.project_id, "of7wIinq"));
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_multiple_versions(&self, version_ids: &[&str]) -> Result<Vec<Version>> {
        check_id_slug(version_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["versions"])
                    .with_query_json("ids", version_ids)?,
            )
            .custom_send_json()
            .await
    }
}
