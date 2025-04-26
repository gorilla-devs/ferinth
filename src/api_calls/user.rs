//! API calls related to users
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/users)

use super::*;
use crate::structures::{project::Project, user::*};

impl<T> Ferinth<T> {
    /**
    Get the user of `user_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let theRookieCoder = modrinth.user_get("7Azq6eD8").await?;
    assert_eq!(
        theRookieCoder.role,
        ferinth::structures::user::UserRole::Developer,
    );
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_get(&self, user_id: &str) -> Result<User> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id]))
            .custom_send_json()
            .await
    }

    /**
    Get the users of `user_ids`

    ## Example
    ```rust
    # use ferinth::structures::user::UserRole;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let users = modrinth.user_get_multiple(&["TEZXhE2U", "7Azq6eD8"]).await?;
    assert_eq!(users.len(), 2);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_get_multiple(&self, user_ids: &[&str]) -> Result<Vec<User>> {
        check_id_slug(user_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["users"])
                    .with_query_json("ids", user_ids)?,
            )
            .custom_send_json()
            .await
    }

    /**
    Get the projects of the user of `user_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let jellysquid_projects = modrinth.user_list_projects("TEZXhE2U").await?;
    assert_eq!(jellysquid_projects.len(), 4);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "projects"]))
            .custom_send_json()
            .await
    }
}

impl Ferinth<Authenticated> {
    /**
    Get the notifications of the user of `user_id`

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    # let user_id = modrinth.user_get_current().await?.id;
    let notifications = modrinth.user_list_notifications(&user_id).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_list_notifications(&self, user_id: &str) -> Result<Vec<Notification>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "notifications"]))
            .custom_send_json()
            .await
    }

    /**
    Get the projects that the user of `user_id` has followed

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    # let user_id = modrinth.user_get_current().await?.id;
    let projects = modrinth.user_list_followed_projects(&user_id).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_list_followed_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "follows"]))
            .custom_send_json()
            .await
    }

    /**
    Delete the user of `user_id`

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    modrinth.user_delete("XXXXXXXX").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_delete(&self, user_id: &str) -> Result<()> {
        check_id_slug(&[user_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["user", user_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the user from the current authorisation header

    ## Example
    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::<ferinth::Authenticated>::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     env!("MODRINTH_TOKEN"),
    # )?;
    let current_user = modrinth.user_get_current().await?;
    // The email should be visible as we are authorised
    assert!(current_user.email.is_some());
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn user_get_current(&self) -> Result<User> {
        self.client
            .get(API_BASE_URL.join_all(vec!["user"]))
            .custom_send_json()
            .await
    }
}
