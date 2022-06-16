use async_trait::async_trait;
use dotium::{
    project::{Author, Project},
    request::Asset,
    DotiumTrait, Platform,
};

use crate::{api_calls, Ferinth, TraitResult};

use super::{box_err, converters};

#[async_trait]
impl DotiumTrait for Ferinth {
    fn new() -> Self {
        Self::new()
    }

    fn get_platform() -> Platform {
        Platform {
            name: "Modrinth".to_string(),
            short_name: "Mr".to_string(),
            logo: Asset::by_description(
                "https://github.com/modrinth/knossos/blob/master/assets/images/logo.svg"
                    .to_string(),
                "logo.svg".to_string(),
                "Full Modrinth platform logo".to_string(),
            ),
        }
    }

    async fn get_project(&self, id: String) -> TraitResult<Project> {
        Ok(converters::project(box_err(
            api_calls::project_calls::get_project(self, id.as_str()).await,
        )?))
    }

    /* TODO Default implementation loops get_project. Bulk route present, just needs implementation
    async fn get_projects(&self, ids: Vec<String>) -> TraitResult<Vec<dotium::project::Project>> {
        todo!()
    }
    */

    async fn get_project_body(&self, project_id: String) -> TraitResult<String> {
        Ok(box_err(api_calls::project_calls::get_project(self, project_id.as_str()).await)?.body)
    }

    async fn get_project_authors(&self, team_id: String) -> TraitResult<Vec<Author>> {
        Ok(
            box_err(api_calls::user_calls::list_team_members(self, team_id.as_str()).await)?
                .into_iter()
                .map(|a| converters::author(a.user))
                .collect(),
        )
    }

    async fn get_project_dependencies(
        &self,
        project_id: String,
    ) -> TraitResult<Vec<dotium::version::VersionDependency>> {
        let mut deps = Vec::new();
        for mut i in self.get_project_versions(project_id).await?.into_iter() {
            deps.append(&mut i.dependencies);
        }
        Ok(deps)
    }

    async fn get_project_version(
        &self,
        _: String,
        id: String,
    ) -> TraitResult<dotium::version::Version> {
        Ok(converters::version(box_err(
            api_calls::version_calls::get_version(self, id.as_str()).await,
        )?))
    }

    async fn get_project_versions(
        &self,
        project_id: String,
    ) -> TraitResult<Vec<dotium::version::Version>> {
        Ok(
            box_err(api_calls::version_calls::list_versions(self, &project_id).await)?
                .into_iter()
                .map(|v| converters::version(v))
                .collect(),
        )
    }

    async fn search(
        query: String,
        project_type: Option<String>,
        mc_version: Vec<String>,
        modloader: String,
        category: String,
    ) -> TraitResult<Vec<dotium::project::Project>> {
        todo!();
    }
}
