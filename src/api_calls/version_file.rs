use super::check_sha1_hash;
use crate::{
    request::RequestBuilderCustomSend,
    structures::version::*,
    url_ext::{UrlJoinAll, UrlWithQuery},
    Ferinth, Result, API_BASE_URL,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Ferinth {
    /// Get the version of a version file with hash `file_hash`.
    /// Only supports SHA1 hashes for now.
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// // A mod file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, so we can get the version it belongs to
    /// let sodium_version = modrinth.get_version_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf").await?;
    /// assert_eq!(sodium_version.project_id, "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn get_version_from_hash(&self, file_hash: &str) -> Result<Version> {
        check_sha1_hash(&[file_hash])?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["version_file", file_hash])
                    .with_query("algorithm", "sha1"),
            )
            .custom_send_json()
            .await
    }

    /// Get the versions of version files with hashes `file_hashes`.
    /// Only supports SHA1 hashes for now.
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let sodium_hash = "795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf";
    /// let snwylvspls_hash = "994ee99d172a5950a51ec2d08c158d270722d871";
    /// let versions = modrinth.get_versions_from_hashes(vec![
    ///     sodium_hash.into(),
    ///     snwylvspls_hash.into(),
    /// ]).await?;
    /// assert_eq!(versions[sodium_hash].project_id, "AANobbMI");
    /// assert_eq!(versions[snwylvspls_hash].project_id, "of7wIinq");
    /// # Ok(()) }
    /// ```
    pub async fn get_versions_from_hashes(
        &self,
        file_hashes: Vec<String>,
    ) -> Result<HashMap<String, Version>> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct HashesBody {
            pub hashes: Vec<String>,
            pub algorithm: HashAlgorithm,
        }

        check_sha1_hash(&file_hashes)?;
        self.client
            .post(API_BASE_URL.join_all(vec!["version_files"]))
            .json(&HashesBody {
                hashes: file_hashes,
                algorithm: HashAlgorithm::SHA1,
            })
            .custom_send_json()
            .await
    }

    /**
    Delete the version file with the hash `file_hash`.
    Only supports SHA1 hashes for now.
    Optionally specify the version ID to delete the version file from, if multiple files of the same hash exist.
    */
    pub async fn delete_from_hash(&self, file_hash: &str, version_id: Option<&str>) -> Result<()> {
        let mut url = API_BASE_URL.join_all(vec!["version_file", file_hash]);
        if let Some(version_id) = version_id {
            url = url.with_query("version_id", version_id);
        }
        self.client.delete(url).custom_send().await?;
        Ok(())
    }

    /// Get the latest version of the project with a version file with the hash `file_hash` based on some `filters`
    pub async fn latest_version_from_hash(
        &self,
        file_hash: &str,
        filters: &LatestVersionBody,
    ) -> Result<Version> {
        check_sha1_hash(&[file_hash])?;
        self.client
            .post(
                API_BASE_URL
                    .join_all(vec!["version_file", file_hash, "update"])
                    .with_query_json("algorithm", HashAlgorithm::SHA1)?,
            )
            .json(filters)
            .custom_send_json()
            .await
    }

    /// Get the latest versions of the projects with version files with the hashes `file_hashes` based on some `filters`
    pub async fn latest_versions_from_hashes(
        &self,
        file_hashes: Vec<String>,
        filters: LatestVersionBody,
    ) -> Result<HashMap<String, Version>> {
        check_sha1_hash(&file_hashes)?;
        self.client
            .post(API_BASE_URL.join_all(vec!["version_files", "update"]))
            .json(&LatestVersionsBody {
                hashes: file_hashes,
                algorithm: HashAlgorithm::SHA1,
                loaders: filters.loaders,
                game_versions: filters.game_versions,
            })
            .custom_send_json()
            .await
    }
}
