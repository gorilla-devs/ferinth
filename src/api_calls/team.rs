use super::check_id_slug;
use crate::{
    request::API_URL_BASE, structures::user::*, url_join_ext::UrlJoinExt, Ferinth, Result,
};

impl Ferinth {
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
        check_id_slug(&[project_id])?;
        self.get(API_URL_BASE.join_all(vec!["project", project_id, "members"]))
            .await
    }

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
        check_id_slug(&[team_id])?;
        self.get(API_URL_BASE.join_all(vec!["team", team_id, "members"]))
            .await
    }

    /// Send an invite to `user_id` to join `team_id`.
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// modrinth.add_user("XXXXXXXX", "YYYYYYYY").await
    /// # }
    /// ```
    pub async fn add_user(&self, team_id: &str, user_id: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            user_id: &'a str,
        }

        self.post(
            API_URL_BASE.join_all(vec!["team", team_id, "members"]),
            &Body { user_id },
        )
        .await
    }

    /// List the members of teams with IDs `team_ids`.
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let teams = modrinth.list_multiple_teams_members(&[
    ///     "4reLOAKe",
    ///     "1HMZl6Mn",
    ///     "BZoBsPo6",
    ///     "peSx5UYg",
    /// ]).await?;
    /// assert!(teams.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_multiple_teams_members(
        &self,
        team_ids: &[&str],
    ) -> Result<Vec<Vec<TeamMember>>> {
        check_id_slug(team_ids)?;
        self.get_with_query(
            API_URL_BASE.join_all(vec!["teams"]),
            &[("ids", serde_json::to_string(&team_ids)?)],
        )
        .await
    }

    /// Accept an invite to join `team_id`.
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// modrinth.join_team("XXXXXXXX").await
    /// # }
    /// ```
    pub async fn join_team(&self, team_id: &str) -> Result<()> {
        self.post(API_URL_BASE.join_all(vec!["team", team_id, "join"]), "")
            .await
    }

    /// Transfer `team_id`'s ownership to `user_id`.
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// modrinth.add_user("XXXXXXXX", "YYYYYYYY").await
    /// # }
    /// ```
    pub async fn transfer_ownership(&self, team_id: &str, user_id: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            user_id: &'a str,
        }

        self.post(
            API_URL_BASE.join_all(vec!["team", team_id, "owner"]),
            &Body { user_id },
        )
        .await
    }
}
