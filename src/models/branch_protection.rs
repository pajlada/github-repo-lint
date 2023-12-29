use super::optionally_enabled;
use serde::{Deserialize, Serialize};

#[doc = "Branch Protection"]
#[derive(Default, Clone, Debug, Deserialize, Serialize, derive_builder::Builder)]
#[builder(default)]
pub struct BranchProtection {
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub allow_deletions: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub allow_force_pushes: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub allow_fork_syncing: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub block_creations: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub enforce_admins: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub lock_branch: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub required_conversation_resolution: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub required_linear_history: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "optionally_enabled"
    )]
    pub required_signatures: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<ProtectedBranchRequiredStatusCheck>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<IncomingUsersTeamsOrApps>,
}

#[doc = "GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitHubApp {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: Option<String>,
    #[doc = "The list of events for the GitHub app"]
    pub events: Vec<String>,
    #[doc = "Unique identifier of the GitHub app"]
    pub id: i64,
    #[doc = "The number of installations associated with the GitHub app"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installations_count: Option<i64>,
    #[doc = "The name of the GitHub app"]
    pub name: String,
    pub node_id: String,
    pub owner: Option<SimpleUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
    #[doc = "The slug name of the GitHub app"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

#[doc = "Protected Branch Pull Request Review"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtectedBranchPullRequestReview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_pull_request_allowances: Option<IncomingUsersTeamsOrApps>,

    pub dismiss_stale_reviews: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<IncomingUsersTeamsOrApps>,

    pub require_code_owner_reviews: bool,

    #[doc = "Whether the most recent push must be approved by someone other than the person who pushed it."]
    #[serde(default)]
    pub require_last_push_approval: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

#[doc = "Allow specific users, teams, or apps to bypass pull request requirements."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncomingUsersTeamsOrApps {
    #[serde(default)]
    pub apps: Vec<GitHubApp>,
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub users: Vec<SimpleUser>,
}

#[doc = "Protected Branch Required Status Check"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtectedBranchRequiredStatusCheck {
    pub checks: Vec<ProtectedBranchRequiredStatusCheckChecksItem>,
    pub contexts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtectedBranchRequiredStatusCheckChecksItem {
    pub app_id: Option<i64>,
    pub context: String,
}
#[doc = "A GitHub user."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleUser {
    pub avatar_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[doc = "Groups of organization members that gives permissions on specified repositories."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    pub description: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub members_url: String,
    pub name: String,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<String>,
    pub parent: Option<TeamSimple>,
    pub permission: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<TeamPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamPermissions {
    pub admin: bool,
    pub maintain: bool,
    pub pull: bool,
    pub push: bool,
    pub triage: bool,
}
#[doc = "Groups of organization members that gives permissions on specified repositories."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamSimple {
    #[doc = "Description of the team"]
    pub description: Option<String>,
    pub html_url: String,
    #[doc = "Unique identifier of the team"]
    pub id: i64,
    #[doc = "Distinguished Name (DN) that team maps to within LDAP environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap_dn: Option<String>,
    pub members_url: String,
    #[doc = "Name of the team"]
    pub name: String,
    pub node_id: String,
    #[doc = "The notification setting the team has set"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<String>,
    #[doc = "Permission that the team will have for its repositories"]
    pub permission: String,
    #[doc = "The level of privacy this team should have"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    pub repositories_url: String,
    pub slug: String,
    #[doc = "URL for the team"]
    pub url: String,
}
