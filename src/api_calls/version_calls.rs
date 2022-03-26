use crate::{
    api_calls::{check_id_slug, check_sha1_hash},
    request::{request, API_URL_BASE},
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
        Ok(request(
            self,
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
        Ok(
            request(self, API_URL_BASE.join("version/")?.join(version_id)?)
                .await?
                .json()
                .await?,
        )
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
        Ok(
            request(self, API_URL_BASE.join("version_file/")?.join(file_hash)?)
                .await?
                .json()
                .await?,
        )
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
        Ok(request(self, &version_file.url).await?.bytes().await?)
    }
}
