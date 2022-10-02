use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    /// The user's ID
    pub id: ID,
    /// The user's GitHub ID
    pub github_id: Number,
    /// The user's username
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email, only visible to the user
    pub email: Option<String>,
    /// A link to the user's avatar
    pub avatar_url: Url,
    /// A description of the user
    pub bio: Option<String>,
    /// The time at which the user was created
    pub created: UtcTime,
    /// The user's role
    pub role: UserRole,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamMember {
    /// The ID of the member's team
    pub team_id: ID,
    /// The user associated with the member
    pub user: User,
    /// This team member's role
    pub role: String,
    /// The user's permissions in bitflag format
    /// (requires authorization to view)
    ///
    /// In order from first to eighth bit, the bits are:
    /// - UPLOAD_VERSION
    /// - DELETE_VERSION
    /// - EDIT_DETAILS
    /// - EDIT_BODY
    /// - MANAGE_INVITES
    /// - REMOVE_MEMBER
    /// - EDIT_MEMBER
    /// - DELETE_PROJECT
    pub permissions: Option<u8>,
    /// Whether the user has accepted membership on the team
    /// (requires authorization to view)
    pub accepted: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: ID,
    /// The ID of the user who received the notification
    pub user_id: ID,
    #[serde(rename = "type")]
    pub type_field: Option<NotificationType>,
    pub title: String,
    /// The body of the notification
    pub text: String,
    /// A relative link to the related project/version
    pub link: String,
    /// Whether the notification has been read
    pub read: bool,
    /// The time at which the notification was created
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
    /// Contains the HTTP method and route respectively
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
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
