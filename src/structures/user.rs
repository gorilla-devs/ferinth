//! Models related to users
//!
//! [documentation](https://docs.modrinth.com/api-spec/#tag/user_model)

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email, only visible to the user itself when authenticated
    pub email: Option<String>,
    /// A description of the user
    pub bio: Option<String>,
    pub id: ID,
    /// The user's GitHub ID
    pub github_id: Option<Number>,
    pub avatar_url: Url,
    /// When the user was created
    pub created: UtcTime,
    pub role: UserRole,
    /// Any badges applicable to this user.
    /// These are currently unused and not displayed, and as such are subject to change.
    ///
    /// [documentation](https://docs.modrinth.com/api-spec/#tag/user_model)
    pub badges: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: ID,
    pub user: User,
    pub role: String,
    /// The user's permissions in bitflag format
    /// (requires authorisation to view)
    ///
    /// In order from first to tenth bit, they indicate:
    /// - UPLOAD_VERSION
    /// - DELETE_VERSION
    /// - EDIT_DETAILS
    /// - EDIT_BODY
    /// - MANAGE_INVITES
    /// - REMOVE_MEMBER
    /// - EDIT_MEMBER
    /// - DELETE_PROJECT
    /// - VIEW_ANALYTICS
    /// - VIEW_PAYOUTS
    pub permissions: Option<Number>,
    /// Whether the user has accepted membership of the team
    /// (requires authorisation to view)
    pub accepted: bool,
    /// The split of payouts going to this user.
    /// The proportion of payouts they get is their split divided by the sum of the splits of all members.
    pub payouts_split: Option<Number>,
    pub ordering: Option<Number>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: ID,
    /// The ID of the user who received the notification
    pub user_id: ID,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    pub title: String,
    /// The body text of the notification
    pub text: String,
    /// A _relative_ link to the related project/version
    pub link: String,
    /// Whether the notification has been read
    pub read: bool,
    /// When the notification was created
    pub created: UtcTime,
    /// A list of actions that can be performed
    pub actions: Vec<NotificationAction>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Report {
    pub report_type: String,
    /// The ID of the item being report
    pub item_id: ID,
    /// The type of item that is being reported
    pub item_type: ReportItemType,
    /// The extended explanation of the report
    pub body: String,
    /// The ID of the user who submitted the report
    pub reporter: ID,
    /// The time at which the report was created
    pub created: UtcTime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct ReportSubmission {
    pub report_type: String,
    /// The ID of the item being report
    pub item_id: ID,
    /// The type of item that is being reported
    pub item_type: ReportItemType,
    /// The extended explanation of the report
    pub body: String,
}

// Undocumented struct pulled from the API source code
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationAction {
    pub title: String,
    /// The route to call when this notification action is called.
    /// Contains the HTTP method and route respectively.
    pub action_route: (String, String),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ReportItemType {
    Project,
    Version,
    User,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    ProjectUpdate,
    TeamInvite,
    StatusUpdate,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
