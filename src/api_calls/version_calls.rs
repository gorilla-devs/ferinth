use std::collections::HashMap;

use crate::{
    api_calls::{check_id_slug, check_sha1_hash},
    request::API_URL_BASE,
    structures::version_structs::*,
    Ferinth, Result,
};
use bytes::Bytes;

impl Ferinth {
    /// Get the versions of project with ID `project_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    /// assert!(sodium_versions[0].project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn list_versions(&self, project_id: &str) -> Result<Vec<Version>> {
        check_id_slug(project_id)?;
        Ok(self
            .get(
                API_URL_BASE
                    .join("project/")?
                    .join(&format!("{}/", project_id))?
                    .join("version")?,
            )
            .await?
            .json()
            .await?)
    }

    /// Get version with ID `version_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let sodium_version = modrinth.get_version("xuWxRZPd").await?;
    /// assert!(sodium_version.project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn get_version(&self, version_id: &str) -> Result<Version> {
        check_id_slug(version_id)?;
        Ok(self
            .get(API_URL_BASE.join("version/")?.join(version_id)?)
            .await?
            .json()
            .await?)
    }

    /// Query a list of versions at once
    /// 
    /// Examples:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let mut version_ids = vec![];
    /// version_ids.push("74Y5Z8fo".to_string()); // Sodium 0.4.1
    /// version_ids.push("Yp8wLY1P".to_string()); // Sodium 0.4.2
    /// let versions = modrinth.get_multiple_versions(version_ids.clone()).await?;
    /// for version in versions {
    ///     assert!(version_ids.contains(&version.id));
    /// }
    /// # Ok(())}
    /// ```
    pub async fn get_multiple_versions(&self, version_ids: Vec<String>) -> Result<Vec<Version>> {
        for id in &version_ids {
            check_id_slug(&id)?;
        }
        Ok(self
            .get(
                API_URL_BASE
                    .join("versions")?
                    .join(&format!("?ids={}", serde_json::to_string(&version_ids)?))?,
            )
            .await?
            .json()
            .await?)
    }


    /// Get the version of a version file with hash `file_hash`. Only supports SHA1 hashes for now
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// // A version file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, so we can get the version it belongs to
    /// let sodium_version = modrinth.get_version_from_file_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf").await?;
    /// // That version file belongs to (surprise, surprise) the Sodium mod!
    /// assert!(sodium_version.project_id == "AANobbMI");
    /// # Ok(()) }
    /// ```
    pub async fn get_version_from_file_hash(&self, file_hash: &str) -> Result<Version> {
        check_sha1_hash(file_hash)?;
        Ok(self
            .get(API_URL_BASE.join("version_file/")?.join(file_hash)?)
            .await?
            .json()
            .await?)
    }

    /// Query a list of SHA1 file hashes at once
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let mut hashes = vec![];
    /// hashes.push("f839863a6be7014b8d80058ea1f361521148d049".to_string()); // Sodium 0.4.1
    /// hashes.push("d5c19c3d4edb4228652adcc8abb94f9bd80a634c".to_string()); // Lithium 0.7.10
    /// let result = modrinth.get_versions_from_hashes(hashes).await?;
    /// assert!(&result["f839863a6be7014b8d80058ea1f361521148d049"].project_id == "AANobbMI");
    /// assert!(&result["d5c19c3d4edb4228652adcc8abb94f9bd80a634c"].project_id == "gvQqBUqZ");
    /// # Ok(()) }
    /// ```
    pub async fn get_versions_from_hashes(
        &self,
        file_hashes: Vec<String>,
    ) -> Result<HashMap<String, Version>> {
        Ok(self
            .post(
                API_URL_BASE.join("version_files")?,
                GetFilesByHashesBody {
                    hashes: file_hashes,
                    algorithm: "sha1".to_owned(),
                },
            )
            .await?
            .json()
            .await?)
    }

    /// Download `version_file`'s contents
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    /// let version_file = &sodium_versions[0].files[0];
    /// // You can write this to a file and load it using a mod loader (Fabric in this case)
    /// let file_contents = modrinth.download_version_file(version_file).await?;
    /// # Ok::<(), ferinth::Error>(()) }
    /// ```
    pub async fn download_version_file(&self, version_file: &VersionFile) -> Result<Bytes> {
        Ok(self.get(&version_file.url).await?.bytes().await?)
    }
}
