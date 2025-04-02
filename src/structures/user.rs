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
    /// Various data relating to the user's payouts status,
    /// only visible to the user itself when authenticated
    pub payout_data: Option<PayoutData>,
    pub id: ID,
    /// The user's GitHub ID
    pub github_id: Option<Int>,
    pub avatar_url: Option<Url>,
    pub created: UtcTime,
    pub role: UserRole,
    /// Any badges applicable to this user.
    /// These are currently unused and not displayed, and as such are subject to change.
    ///
    /// [documentation](https://docs.modrinth.com/api-spec/#tag/user_model)
    pub badges: Int,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayoutData {
    balance: f64,
    payout_wallet: Option<PayoutWallet>,
    payout_wallet_type: Option<PayoutWalletType>,
    payout_address: Option<String>,
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
    /// In order from first to tenth bit, they indicate the ability to:
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
    pub permissions: Option<Int>,
    /// Whether the user has accepted membership of the team
    /// (requires authorisation to view)
    pub accepted: bool,
    /// The split of payouts going to this user.
    /// The proportion of payouts they get is their split divided by the sum of the splits of all members.
    pub payouts_split: Option<Int>,
    pub ordering: Option<Int>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: ID,
    /// The ID of the user who received the notification
    pub user_id: ID,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    pub title: String,
    pub text: String,
    /// A _relative_ link to the related project/version
    pub link: String,
    pub read: bool,
    pub created: UtcTime,
    /// A list of actions that can be performed
    pub actions: Vec<NotificationAction>,
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
pub struct Payout {
    pub created: UtcTime,
    pub amount: Int,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PayoutWallet {
    PayPal,
    Venmo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutWalletType {
    Email,
    Phone,
    UserHandle,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    ProjectUpdate,
    TeamInvite,
    StatusUpdate,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UserRole {
    Developer,
    Moderator,
    Admin,
}
