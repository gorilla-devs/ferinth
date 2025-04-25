use super::*;
use crate::structures::version::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Ferinth<Authenticated> {
    /**
    Delete the version file with the `hash`.
    Only supports SHA1 hashes for now.
    Optionally specify the version ID to delete the version file from, if multiple files of the same hash exist.

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.version_file_delete_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf", None).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_file_delete_from_hash(
        &self,
        hash: &str,
        version_id: Option<&str>,
    ) -> Result<()> {
        check_sha1_hash(&[hash])?;
        let mut url = API_BASE_URL.join_all(vec!["version_file", hash]);
        if let Some(version_id) = version_id {
            check_id_slug(&[version_id])?;
            url = url.with_query("version_id", version_id);
        }
        self.client.delete(url).custom_send().await?;
        Ok(())
    }
}

impl<T> Ferinth<T> {
    /**
    Get the version of the version file with `hash`.
    Only supports SHA1 hashes for now.

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    // If a mod file has the hash `795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf`, we can get the version it belongs to
    let sodium_version = modrinth.version_get_from_hash("795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf").await?;
    assert_eq!(sodium_version.project_id, "AANobbMI");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_get_from_hash(&self, hash: &str) -> Result<Version> {
        check_sha1_hash(&[hash])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["version_file", hash]))
            .custom_send_json()
            .await
    }

    /**
    Get the versions of the version files with `hashes`, only supports SHA1 hashes for now

    Returns a map where the keys are the hashes given.

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let sodium_hash = "795d4c12bffdb1b21eed5ff87c07ce5ca3c0dcbf";
    let snwylvspls_hash = "994ee99d172a5950a51ec2d08c158d270722d871";
    let versions = modrinth.version_get_from_multiple_hashes(vec![
        sodium_hash.into(),
        snwylvspls_hash.into(),
    ]).await?;
    assert_eq!(versions[sodium_hash].project_id, "AANobbMI");
    assert_eq!(versions[snwylvspls_hash].project_id, "of7wIinq");
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn version_get_from_multiple_hashes(
        &self,
        hashes: Vec<String>,
    ) -> Result<HashMap<String, Version>> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct HashesBody {
            pub hashes: Vec<String>,
            pub algorithm: HashAlgorithm,
        }

        check_sha1_hash(&hashes)?;
        self.client
            .post(API_BASE_URL.join_all(vec!["version_files"]))
            .json(&HashesBody {
                hashes,
                algorithm: HashAlgorithm::SHA1,
            })
            .custom_send_json()
            .await
    }

    /// Get the latest version for the project of the version file with `hash` based on some `filters`.
    /// Only supports SHA1 hashes for now.
    pub async fn version_get_latest_from_hash(
        &self,
        hash: &str,
        filters: &LatestVersionBody,
    ) -> Result<Version> {
        check_sha1_hash(&[hash])?;
        self.client
            .post(
                API_BASE_URL
                    .join_all(vec!["version_file", hash, "update"])
                    .with_query_json("algorithm", HashAlgorithm::SHA1)?,
            )
            .json(filters)
            .custom_send_json()
            .await
    }

    /// Get the latest versions of the projects of the version files with hashes based on some `filters`.
    /// Only supports SHA1 hashes for now.
    pub async fn version_get_latest_from_multiple_hashes(
        &self,
        hashes: Vec<String>,
        filters: LatestVersionBody,
    ) -> Result<HashMap<String, Version>> {
        check_sha1_hash(&hashes)?;
        self.client
            .post(API_BASE_URL.join_all(vec!["version_files", "update"]))
            .json(&LatestVersionsBody {
                hashes,
                algorithm: HashAlgorithm::SHA1,
                loaders: filters.loaders,
                game_versions: filters.game_versions,
            })
            .custom_send_json()
            .await
    }
}
