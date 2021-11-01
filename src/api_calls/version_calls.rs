use super::super::structures::version_structs::*;
use crate::{
    request::{request, request_rel},
    ModrinthAPI, Result,
};
use bytes::Bytes;

impl ModrinthAPI {
    /// Get the versions of mod with `mod_id`
    pub async fn list_versions(&self, mod_id: String) -> Result<Vec<Version>> {
        Ok(request_rel(self, format!("/mod/{}/version", mod_id))
            .await?
            .json()
            .await?)
    }

    /// Get version with ID `version_id`
    pub async fn get_version(&self, version_id: String) -> Result<Version> {
        Ok(request_rel(self, format!("/version/{}", version_id))
            .await?
            .json()
            .await?)
    }

    /// Download `version_file`'s contents
    pub async fn download_version_file(&self, version_file: VersionFile) -> Result<Bytes> {
        Ok(request(self, version_file.url).await?.bytes().await?)
    }
}
