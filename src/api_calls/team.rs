//! API calls related to teams
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/teams)

use super::*;
use crate::structures::user::*;

impl<T> Ferinth<T> {
    /**
    List the members of the team of the project of `project_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let saj_team = modrinth.team_list_project_members("stairautojump").await?;
    assert_eq!(saj_team.len(), 2);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_list_project_members(&self, project_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(&[project_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["project", project_id, "members"]))
            .custom_send_json()
            .await
    }

    /**
    List the members of the team of `team_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let saj_team = modrinth.team_list_members("zftNHDXi").await?;
    assert_eq!(saj_team.len(), 2);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_list_members(&self, team_id: &str) -> Result<Vec<TeamMember>> {
        check_id_slug(&[team_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["team", team_id, "members"]))
            .custom_send_json()
            .await
    }

    /**
    List the members of the teams of `team_ids`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let teams = modrinth.team_multiple_list_members(&[
        "4reLOAKe",
        "1HMZl6Mn",
        "BZoBsPo6",
        "peSx5UYg",
    ]).await?;
    assert_eq!(teams.len(), 4);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_multiple_list_members(
        &self,
        team_ids: &[&str],
    ) -> Result<Vec<Vec<TeamMember>>> {
        check_id_slug(team_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["teams"])
                    .with_query_json("ids", team_ids)?,
            )
            .custom_send_json()
            .await
    }
}

impl Ferinth<Authenticated> {
    /**
    Send an invite to the user of `user_id` to join the team of `team_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.team_add_user("XXXXXXXX", "YYYYYYYY").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_add_user(&self, team_id: &str, user_id: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            user_id: &'a str,
        }

        self.client
            .post(API_BASE_URL.join_all(vec!["team", team_id, "members"]))
            .json(&Body { user_id })
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Accept an invite to join the team of `team_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.team_join("XXXXXXXX").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_join(&self, team_id: &str) -> Result<()> {
        self.client
            .post(API_BASE_URL.join_all(vec!["team", team_id, "join"]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Remove the member of `user_id` from the team of `team_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.team_remove_member("XXXXXXXX", "YYYYYYYY").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_remove_member(&self, team_id: &str, user_id: &str) -> Result<()> {
        self.client
            .delete(API_BASE_URL.join_all(vec!["team", team_id, "members", user_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Transfer ownership of the team of `team_id` to the user of `user_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.team_transfer_ownership("XXXXXXXX", "YYYYYYYY").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn team_transfer_ownership(&self, team_id: &str, user_id: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            user_id: &'a str,
        }

        self.client
            .post(API_BASE_URL.join_all(vec!["team", team_id, "owner"]))
            .json(&Body { user_id })
            .custom_send()
            .await?;
        Ok(())
    }
}
