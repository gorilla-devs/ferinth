//! API calls related to users
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/users)

use super::*;
use crate::structures::{project::Project, user::*};
use reqwest::Body;

impl Ferinth {
    /**
    Get the user of `user_id`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let theRookieCoder = modrinth.get_user("7Azq6eD8").await?;
    assert_eq!(
        theRookieCoder.role,
        ferinth::structures::user::UserRole::Developer,
    );
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id]))
            .custom_send_json()
            .await
    }

    /**
    Delete the user of `user_id`

    REQUIRES AUTHENTICATION and appropriate permissions!

    ```no_run
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    modrinth.delete_user("XXXXXXXX").await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        check_id_slug(&[user_id])?;
        self.client
            .delete(API_BASE_URL.join_all(vec!["user", user_id]))
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the user from the current authorisation header

    REQUIRES AUTHENTICATION!

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     Some(env!("MODRINTH_TOKEN")),
    # )?;
    let current_user = modrinth.get_current_user().await?;
    // The email should be visible as we are authorised
    assert!(current_user.email.is_some());
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_current_user(&self) -> Result<User> {
        self.client
            .get(API_BASE_URL.join_all(vec!["user"]))
            .custom_send_json()
            .await
    }

    /**
    Get the users of `user_ids`

    ```rust
    # use ferinth::structures::user::UserRole;
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let users = modrinth.get_multiple_users(&["TEZXhE2U", "7Azq6eD8"]).await?;
    assert_eq!(users.len(), 2);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_multiple_users(&self, user_ids: &[&str]) -> Result<Vec<User>> {
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
    Change the avatar of the user of `user_id` to `image`

    REQUIRES AUTHENTICATION!

    The image may be up to `2 Mib` large.
    By default, the user's GitHub avatar is used.
    */
    pub async fn change_avatar<B: Into<Body>>(&self, user_id: &str, image: B) -> Result<()> {
        check_id_slug(&[user_id])?;
        self.client
            .post(API_BASE_URL.join_all(vec!["user", user_id, "icon"]))
            .body(image)
            .custom_send()
            .await?;
        Ok(())
    }

    /**
    Get the projects of the user of `user_id`

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::default();
    let jellysquid_projects = modrinth.list_projects("TEZXhE2U").await?;
    assert_eq!(jellysquid_projects.len(), 4);
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "projects"]))
            .custom_send_json()
            .await
    }

    /**
    Get the notifications of the user of `user_id`

    REQUIRES AUTHENTICATION!

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     Some(env!("MODRINTH_TOKEN")),
    # )?;
    # let user_id = modrinth.get_current_user().await?.id;
    let notifications = modrinth.get_notifications(&user_id).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn get_notifications(&self, user_id: &str) -> Result<Vec<Notification>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "notifications"]))
            .custom_send_json()
            .await
    }

    /**
    Get the projects that the user of `user_id` has followed

    REQUIRES AUTHENTICATION!

    ```rust
    # tokio_test::block_on(async {
    # let modrinth = ferinth::Ferinth::new(
    #     env!("CARGO_CRATE_NAME"),
    #     Some(env!("CARGO_PKG_VERSION")),
    #     None,
    #     Some(env!("MODRINTH_TOKEN")),
    # )?;
    # let user_id = modrinth.get_current_user().await?.id;
    let projects = modrinth.followed_projects(&user_id).await?;
    # Ok::<_, ferinth::Error>(()) }).unwrap()
    ```
    */
    pub async fn followed_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "follows"]))
            .custom_send_json()
            .await
    }
}
