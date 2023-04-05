use crate::{structures::tag::*, url_ext::UrlJoinAll, Ferinth, Result, API_BASE_URL};

impl Ferinth {
    /// List the categories, their icons, and applicable project types
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let categories = modrinth.list_categories().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "category"]))
            .await
    }

    /// List the loaders, their icons, and supported project types
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let loaders = modrinth.list_loaders().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_loaders(&self) -> Result<Vec<Loader>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "loader"])).await
    }

    /// List the game versions and information about them
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let game_versions = modrinth.list_game_versions().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_game_versions(&self) -> Result<Vec<GameVersion>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "game_version"]))
            .await
    }

    /// List licenses and information about them
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let licenses = modrinth.list_licenses().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_licenses(&self) -> Result<Vec<License>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "license"]))
            .await
    }

    /// List donation platforms and information about them
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let donation_platforms = modrinth.list_donation_platforms().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_donation_platforms(&self) -> Result<Vec<DonationPlatform>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "donation_platform"]))
            .await
    }

    /// List valid report types
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let report_types = modrinth.list_report_types().await?;
    /// # Ok(()) }
    /// ```
    pub async fn list_report_types(&self) -> Result<Vec<String>> {
        self.get(API_BASE_URL.join_all(vec!["tag", "report_type"]))
            .await
    }
}
