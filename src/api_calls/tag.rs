//! API calls related to tags
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/tags)

use super::*;
use crate::structures::tag::*;

impl Ferinth {
    /**
    List the categories, their icons, and applicable project types

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let categories = modrinth.list_categories().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "category"]))
            .custom_send_json()
            .await
    }

    /**
    List the loaders, their icons, and supported project types

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let loaders = modrinth.list_loaders().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_loaders(&self) -> Result<Vec<Loader>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "loader"]))
            .custom_send_json()
            .await
    }

    /**
    List the game versions and information about them

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let game_versions = modrinth.list_game_versions().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_game_versions(&self) -> Result<Vec<GameVersion>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "game_version"]))
            .custom_send_json()
            .await
    }

    /**
    List licenses and information about them

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let licenses = modrinth.list_licenses().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_licenses(&self) -> Result<Vec<License>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "license"]))
            .custom_send_json()
            .await
    }

    /**
    List donation platforms and information about them

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let donation_platforms = modrinth.list_donation_platforms().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_donation_platforms(&self) -> Result<Vec<DonationPlatform>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "donation_platform"]))
            .custom_send_json()
            .await
    }

    /**
    List valid report types

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let report_types = modrinth.list_report_types().await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_report_types(&self) -> Result<Vec<String>> {
        self.client
            .get(API_BASE_URL.join_all(vec!["tag", "report_type"]))
            .custom_send_json()
            .await
    }
}
