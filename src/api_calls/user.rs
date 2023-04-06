use super::check_id_slug;
use crate::{
    request::RequestBuilderCustomSend,
    structures::{project::Project, user::*},
    url_ext::{UrlJoinAll, UrlWithQuery},
    Ferinth, Result, API_BASE_URL,
};

impl Ferinth {
    /// Get user with ID `user_id`
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::user::UserRole;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let jellysquid = modrinth.get_user("TEZXhE2U").await?;
    /// assert!(jellysquid.role == UserRole::Developer);
    /// # Ok(()) }
    /// ```
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id]))
            .custom_send_json()
            .await
    }

    /// Get the user of the current authorisation header
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// let current_user = modrinth.get_current_user().await?;
    /// // The email should be visible as we are authourised to view this user's email
    /// assert!(current_user.email.is_some());
    /// # Ok(()) }
    /// ```
    pub async fn get_current_user(&self) -> Result<User> {
        self.client
            .get(API_BASE_URL.join_all(vec!["user"]))
            .custom_send_json()
            .await
    }

    /// Get multiple users with IDs `user_ids`
    ///
    /// Example:
    /// ```rust
    /// # use ferinth::structures::user::UserRole;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// let users = modrinth.get_multiple_users(&["TEZXhE2U", "7Azq6eD8"]).await?;
    /// assert!(users.len() == 2);
    /// # Ok(()) }
    /// ```
    pub async fn get_multiple_users(&self, user_ids: &[&str]) -> Result<Vec<User>> {
        check_id_slug(user_ids)?;
        self.client
            .get(
                API_BASE_URL
                    .join_all(vec!["users"])
                    .with_query([("ids", serde_json::to_string(user_ids)?)]),
            )
            .custom_send_json()
            .await
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
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "projects"]))
            .custom_send_json()
            .await
    }

    /// Get a list of notifications the user has received
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// let current_user = modrinth.get_current_user().await?;
    /// modrinth.get_notifications(&current_user.id).await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_notifications(&self, user_id: &str) -> Result<Vec<Notification>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "notifications"]))
            .custom_send_json()
            .await
    }

    /// Get a list of the projects the user has followed
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// # let current_user = modrinth.get_current_user().await?;
    /// modrinth.followed_projects(&current_user.id).await
    /// # }
    /// ```
    pub async fn followed_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        check_id_slug(&[user_id])?;
        self.client
            .get(API_BASE_URL.join_all(vec!["user", user_id, "follows"]))
            .custom_send_json()
            .await
    }

    /// Submit a report to the moderators
    ///
    /// REQUIRES AUTHENTICATION!
    ///
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferinth::Error> {
    /// # let modrinth = ferinth::Ferinth::new(
    /// #     env!("CARGO_CRATE_NAME"),
    /// #     Some(env!("CARGO_PKG_VERSION")),
    /// #     None,
    /// #     Some(env!("MODRINTH_TOKEN")),
    /// # )?;
    /// let current_user = modrinth.submit_report(
    ///     ???,
    ///     "XXXXXXXX",
    ///     ferinth::structures::user::ReportItemType::User,
    ///     "This is an example report",
    /// ).await
    /// # }
    /// ```
    pub async fn submit_report(
        &self,
        report_type: String,
        item_id: String,
        item_type: ReportItemType,
        body: String,
    ) -> Result<Vec<Project>> {
        check_id_slug(&[&item_id])?;
        self.client
            .post(API_BASE_URL.join_all(vec!["report"]))
            .json(&ReportSubmission {
                report_type,
                item_id,
                item_type,
                body,
            })
            .custom_send_json()
            .await
    }
}
