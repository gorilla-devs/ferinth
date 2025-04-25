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
    pub email_verified: Option<bool>,
    pub has_password: Option<bool>,
    pub has_totp: Option<bool>,
    pub auth_providers: Option<Vec<AuthProvider>>,
    /// A description of the user
    pub bio: Option<String>,
    /// Various data relating to the user's payouts status,
    /// only visible to the user itself when authenticated
    pub payout_data: Option<PayoutData>,
    pub id: ID,
    pub avatar_url: Option<Url>,
    pub created: UtcTime,
    pub role: UserRole,
    /// Bitflags of badges applicable to this user
    ///
    /// [code](https://github.com/modrinth/code/blob/6c16688ca93fc1ab878d9e915246d78fa723dca8/apps/labrinth/src/models/v3/users.rs#L9-L23)
    pub badges: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayoutData {
    pub balance: f64,
    pub paypal_address: Option<String>,
    pub paypal_country: Option<String>,
    pub venmo_handle: Option<String>,
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
    /// [code](https://github.com/modrinth/code/blob/6c16688ca93fc1ab878d9e915246d78fa723dca8/apps/labrinth/src/models/v3/teams.rs#L24-L38)
    pub permissions: Option<Int>,
    /// Whether the user has accepted membership of the team
    /// (requires authorisation to view)
    pub accepted: bool,
    /// The split of payouts going to this user.
    /// The proportion of payouts they get is their split divided by the sum of the splits of all members.
    pub payouts_split: Option<f64>,
    pub ordering: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: ID,
    /// The ID of the user who received the notification
    pub user_id: ID,
    pub title: String,
    pub text: String,
    /// A _relative_ link to the related project/version
    pub link: String,
    pub read: bool,
    pub created: UtcTime,
    /// A list of actions that can be performed
    pub actions: Vec<NotificationAction>,
    pub body: NotificationBody,
}

// Undocumented struct pulled from the labrinth source code
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NotificationAction {
    pub title: String,
    /// The route to call when this notification action is called.
    /// Contains the HTTP method and route respectively.
    pub action_route: (String, String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationBody {
    ProjectUpdate {
        project_id: ID,
        version_id: ID,
    },
    TeamInvite {
        project_id: ID,
        team_id: ID,
        invited_by: ID,
        role: String,
    },
    OrganizationInvite {
        organization_id: ID,
        invited_by: ID,
        team_id: ID,
        role: String,
    },
    StatusChange {
        project_id: ID,
        old_status: project::ProjectStatus,
        new_status: project::ProjectStatus,
    },
    ModeratorMessage {
        thread_id: ID,
        message_id: ID,

        project_id: Option<ID>,
        report_id: Option<ID>,
    },
    LegacyMarkdown {
        notification_type: Option<String>,
        title: String,
        text: String,
        link: String,
        actions: Vec<NotificationAction>,
    },
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    GitHub,
    Discord,
    Microsoft,
    GitLab,
    Google,
    Steam,
    PayPal,
}
