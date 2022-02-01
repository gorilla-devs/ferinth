use crate::{
    api_calls::check_id_slug,
    request::request_rel,
    structures::{user_structs::*, ID},
    Ferinth, Result,
};

impl Ferinth {
    /// Get user with ID `user_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let jellysquid = modrinth.get_user("TEZXhE2U").await?;
    /// assert_eq!(
    ///     jellysquid.role,
    ///     ferinth::structures::user_structs::UserRole::Developer,
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        check_id_slug(user_id)?;
        Ok(request_rel(self, format!("/user/{}", user_id))
            .await?
            .json()
            .await?)
    }

    /// Get a list of mod IDs of mods that the user owns
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let jellysquid_mods = modrinth.list_mods("TEZXhE2U").await?;
    /// assert!(vec![
    ///         "hEOCdOgW".to_string(),
    ///         "AANobbMI".to_string(),
    ///         "gvQqBUqZ".to_string(),
    ///         "AZomiSrC".to_string()
    ///     ]
    ///     .iter()
    ///     // So that the order of the mods isn't checked
    ///     .all(
    ///         |mod_id| jellysquid_mods.contains(mod_id)
    ///     )
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn list_mods(&self, user_id: &str) -> Result<Vec<ID>> {
        check_id_slug(user_id)?;
        Ok(request_rel(self, format!("/user/{}/projects", user_id))
            .await?
            .json()
            .await?)
    }

    /// List the members of team with ID `team_id`
    ///
    /// Example:
    /// ```rust
    /// # let modrinth = ferinth::Ferinth::new("ferinth-example");
    /// # tokio_test::block_on( async {
    /// let mod_menu_team = modrinth.list_team_members("VMz4FpgB").await?;
    /// assert_eq!(
    ///     mod_menu_team.len(),
    ///     4,
    /// );
    /// # Ok::<(), ferinth::Error>(())
    /// # } );
    /// ```
    pub async fn list_team_members(&self, team_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(team_id)?;
        Ok(request_rel(self, format!("/team/{}/members", team_id))
            .await?
            .json()
            .await?)
    }
}
