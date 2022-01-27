use crate::{
    api_calls::{check_id_slug, check_sha1_hash},
    request::{request, request_rel},
    structures::version_structs::*,
    Ferinth, Result,
};
use bytes::Bytes;

impl Ferinth {
    /// Get the versions of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    /// assert_eq!(
    ///     sodium_versions[0].mod_id,
    ///     "AANobbMI",
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn list_versions(&self, mod_id: &str) -> Result<Vec<Version>> {
        check_id_slug(mod_id)?;
        Ok(request_rel(self, format!("/mod/{}/version", mod_id))
            .await?
            .json()
            .await?)
    }

    /// Get version with ID `version_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let sodium_version = modrinth.get_version("xuWxRZPd").await?;
    /// assert_eq!(
    ///     sodium_version.mod_id,
    ///     "AANobbMI",
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn get_version(&self, version_id: &str) -> Result<Version> {
        check_id_slug(version_id)?;
        Ok(request_rel(self, format!("/version/{}", version_id))
            .await?
            .json()
            .await?)
    }

    /// Get the version of a version file with hash `file_hash`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// // A version file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, so we can the version it belongs to
    /// let sodium_version = modrinth.get_version_from_file_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf").await?;
    /// assert_eq!(
    ///     sodium_version.mod_id,
    ///     "AANobbMI",
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn get_version_from_file_hash(&self, file_hash: &str) -> Result<Version> {
        check_sha1_hash(file_hash)?;
        Ok(request_rel(self, format!("/version_file/{}", file_hash))
            .await?
            .json()
            .await?)
    }

    /// Download `version_file`'s contents
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let sodium_versions = modrinth.list_versions("AANobbMI").await?;
    /// let version_file = &sodium_versions[0].files[0];
    /// // You can write this to a file and load it using a mod loader (Fabric in this case)
    /// let file_contents = modrinth.download_version_file(version_file).await?;
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn download_version_file(&self, version_file: &VersionFile) -> Result<Bytes> {
        Ok(request(self, &version_file.url).await?.bytes().await?)
    }
}
