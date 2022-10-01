use super::check_sha1_hash;
use crate::{
    request::API_URL_BASE, structures::version::*, url_join_ext::UrlJoinExt, Ferinth,
    Result,
};
use std::collections::HashMap;

impl Ferinth {
    /// Get the version of a version file with hash `file_hash`. Only supports SHA1 hashes for now
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// // A mod file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, so we can get the version it belongs to
    /// let sodium_version = modrinth.get_version_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf").await?;
    /// assert!(sodium_version.project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn get_version_from_hash(&self, file_hash: &str) -> Result<Version> {
        check_sha1_hash(file_hash)?;
        self.get(API_URL_BASE.join_all(vec!["version_file", file_hash]))
            .await
    }

    /// Get the versions of version files with hashes `file_hashes`. Only supports SHA1 hashes for now
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let sodium_hash = "795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf";
    /// let snwylvspls_hash = "994ee99d172a5950a51ec2d08c158d270722d871";
    /// let versions = modrinth.get_versions_from_hashes(vec![
    ///     sodium_hash.into(),
    ///     snwylvspls_hash.into()
    /// ]).await?;
    /// assert!(versions[sodium_hash].project_id == "AANobbMI");
    /// assert!(versions[snwylvspls_hash].project_id == "of7wIinq");
    /// # Ok(()) }
    /// ```
    pub async fn get_versions_from_hashes(
        &self,
        file_hashes: Vec<String>,
    ) -> Result<HashMap<String, Version>> {
        for file_hash in &file_hashes {
            check_sha1_hash(file_hash)?;
        }
        self.post(
            API_URL_BASE.join_all(vec!["version_files"]),
            &HashesBody {
                hashes: file_hashes,
                algorithm: HashAlgorithm::SHA1,
            },
        )
        .await
    }

    /// Get the latest version of the given `file_hash` based on some `filters`
    pub async fn latest_version_from_hash(
        &self,
        file_hash: &str,
        filters: &LatestVersionBody,
    ) -> Result<Version> {
        check_sha1_hash(file_hash)?;
        self.post_with_query(
            API_URL_BASE.join_all(vec!["version_file", file_hash, "update"]),
            filters,
            &[("algorithm", &serde_json::to_string(&HashAlgorithm::SHA1)?)],
        )
        .await
    }

    /// Get the latest versions of the given `file_hashes` based on some `filters`
    pub async fn latest_versions_from_hashes(
        &self,
        file_hashes: Vec<String>,
        filters: LatestVersionBody,
    ) -> Result<Vec<Version>> {
        for file_hash in &file_hashes {
            check_sha1_hash(file_hash)?;
        }
        self.post(
            API_URL_BASE.join_all(vec!["version_files", "update"]),
            &LatestVersionsBody {
                hashes: file_hashes,
                algorithm: HashAlgorithm::SHA1,
                loaders: filters.loaders,
                game_versions: filters.game_versions,
            },
        )
        .await
    }
}
