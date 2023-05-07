//! API calls related to versions
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/versions)

use super::*;
use crate::structures::{version::*, UtcTime};

impl Ferinth {
    /**
    Get the versions of the project of `project_id`

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    sodium_versions.iter().for_each(|v| assert_eq!(v.project_id, "AANobbMI"));
    # Ok(()) }
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
    Get the versions of the project of `project_id`, filtered using the following:

    |||
    |-|-|
    | `loaders` | The loaders to filter for |
    | `game_versions` | The game versions to filter for |
    | `featured` | Filter for featured or non-featured versions only |

    ```rust
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_forge_versions = modrinth.list_versions_filtered(
        "AANobbMI",
        Some(&["forge"]),
        None,
        None
    ).await?;
    // Sodium is not made for Forge
    assert!(sodium_forge_versions.is_empty());
    # Ok(()) }
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
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_version = modrinth.get_version("xuWxRZPd").await?;
    assert_eq!(sodium_version.project_id, "AANobbMI");
    # Ok(()) }
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
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    modrinth.delete_version("XXXXXXXX").await?;
    # Ok(()) }
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
    Schedule a change in the `status` of the version of `version_id` at `time`

    REQUIRES AUTHENTICATION and appropriate permissions!

    ```no_run
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    // Release the version of ID `xuWxRZPd` in three hours to the public
    modrinth.schedule_version(
        "xuWxRZPd",
        &(chrono::offset::Utc::now() + chrono::Duration::hours(3)),
        &ferinth::structures::version::RequestedStatus::Listed
    ).await
    # }
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
    # #[tokio::main]
    # async fn main() -> ferinth::Result<()> {
    # let modrinth = ferinth::Ferinth::default();
    let versions = modrinth.get_multiple_versions(&[
        "sxWTUZpD",
        "mgPpe4NY",
    ]).await?;
    versions.iter().for_each(|v| assert_eq!(v.project_id, "of7wIinq"));
    # Ok(()) }
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
