use crate::{
    request::{request, API_URL_BASE},
    structures::tag_structs::*,
    Ferinth, Result,
};

impl Ferinth {
    /// List the categories a project can take
    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        Ok(request(self, API_URL_BASE.join("tag/")?.join("category")?)
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft mod loaders
    pub async fn list_loaders(&self) -> Result<Vec<Loader>> {
        Ok(request(self, API_URL_BASE.join("tag/")?.join("loader")?)
            .await?
            .json()
            .await?)
    }

    /// List the Minecraft versions
    pub async fn list_game_versions(&self) -> Result<Vec<GameVersion>> {
        Ok(
            request(self, API_URL_BASE.join("tag/")?.join("game_version")?)
                .await?
                .json()
                .await?,
        )
    }

    /// List licenses
    pub async fn list_licenses(&self) -> Result<Vec<License>> {
        Ok(request(self, API_URL_BASE.join("tag/")?.join("license")?)
            .await?
            .json()
            .await?)
    }

    /// List donation platforms
    pub async fn list_donation_platforms(&self) -> Result<Vec<DonationPlatform>> {
        Ok(
            request(self, API_URL_BASE.join("tag/")?.join("donation_platform")?)
                .await?
                .json()
                .await?,
        )
    }

    /// List report types
    pub async fn list_report_types(&self) -> Result<Vec<String>> {
        Ok(
            request(self, API_URL_BASE.join("tag/")?.join("report_type")?)
                .await?
                .json()
                .await?,
        )
    }
}
