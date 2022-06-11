use crate::{
    api_calls::check_id_slug,
    request::API_URL_BASE,
    structures::{project_structs::Project, user_structs::*},
    Ferinth, Result,
};

impl Ferinth {
    /// Get user with ID `user_id`
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::user_structs::UserRole;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let jellysquid = modrinth.get_user("TEZXhE2U").await?;
    /// assert!(jellysquid.role == UserRole::Developer);
    /// # Ok(()) }
    /// ```
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        check_id_slug(user_id)?;
        Ok(self
            .get(API_URL_BASE.join("user/")?.join(user_id)?)
            .await?
            .json()
            .await?)
    }

    /// Get a list of projects that the user owns
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let jellysquid_projects = modrinth.list_projects("TEZXhE2U").await?;
    /// assert!(jellysquid_projects.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(user_id)?;
        Ok(self
            .get(
                API_URL_BASE
                    .join("user/")?
                    .join(&format!("{}/", user_id))?
                    .join("projects")?,
            )
            .await?
            .json()
            .await?)
    }

    /// List the members of team with ID `team_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new();
    /// let mod_menu_team = modrinth.list_team_members("VMz4FpgB").await?;
    /// assert!(mod_menu_team.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_team_members(&self, team_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(team_id)?;
        Ok(self
            .get(
                API_URL_BASE
                    .join("team/")?
                    .join(&format!("{}/", team_id))?
                    .join("members")?,
            )
            .await?
            .json()
            .await?)
    }
}
