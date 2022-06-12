use super::check_id_slug;
use crate::{
    request::API_URL_BASE, structures::user_structs::*, url_join_ext::UrlJoinExt, Ferinth, Result,
};

impl Ferinth {
    /// List the members of team with ID `team_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let mod_menu_team = modrinth.list_team_members("VMz4FpgB").await?;
    /// assert!(mod_menu_team.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_team_members(&self, team_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(team_id)?;
        self.get(API_URL_BASE.join_all(vec!["team", team_id, "members"]))
            .await
    }

    /// List the members of team with ID `team_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let mod_menu_team = modrinth.list_project_team_members("mOgUt4GM").await?;
    /// assert!(mod_menu_team.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_project_team_members(&self, project_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(project_id)?;
        self.get(API_URL_BASE.join_all(vec!["project", project_id, "members"]))
            .await
    }
}
