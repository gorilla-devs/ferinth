use super::check_id_slug;
use crate::{
    request::API_URL_BASE,
    structures::{project_structs::Project, user_structs::*},
    url_join_ext::UrlJoinExt,
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
    /// # let modrinth = ferinth::Ferinth::default();
    /// let jellysquid = modrinth.get_user("TEZXhE2U").await?;
    /// assert!(jellysquid.role == UserRole::Developer);
    /// # Ok(()) }
    /// ```
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        check_id_slug(user_id)?;
        self.get(API_URL_BASE.join_all(vec!["user", user_id])).await
    }

    /// Get a list of projects that the user owns
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let jellysquid_projects = modrinth.list_projects("TEZXhE2U").await?;
    /// assert!(jellysquid_projects.len() == 4);
    /// # Ok(()) }
    /// ```
    pub async fn list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(user_id)?;
        self.get(API_URL_BASE.join_all(vec!["user", user_id, "projects"]))
            .await
    }

    /// Get multiple users with IDs `user_ids`
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::user_structs::UserRole;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let users = modrinth.get_multiple_users(&["TEZXhE2U", "7Azq6eD8"]).await?;
    /// assert!(&users[0].username == "jellysquid3");
    /// assert!(&users[1].username == "theRookieCoder");
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_users(&self, user_ids: &[&str]) -> Result<Vec<User>> {
        for user_id in user_ids {
            check_id_slug(user_id)?;
        }
        self.get_with_query(
            API_URL_BASE.join_all(vec!["users"]),
            &[("ids", &serde_json::to_string(user_ids)?)],
        )
        .await
    }
}
