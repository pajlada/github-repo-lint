use serde::Serialize;

use super::{BranchProtection, ProtectedBranchRequiredStatusCheckChecksItem};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BranchProtectionUpdateRequiredStatusCheck {
    context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<i64>,
}

impl From<&ProtectedBranchRequiredStatusCheckChecksItem>
    for BranchProtectionUpdateRequiredStatusCheck
{
    fn from(check: &ProtectedBranchRequiredStatusCheckChecksItem) -> Self {
        Self {
            context: check.context.clone(),
            app_id: check.app_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BranchProtectionUpdateRequiredStatusChecks {
    strict: bool,
    checks: Vec<BranchProtectionUpdateRequiredStatusCheck>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ListOfUserTeamsOrApps {
    users: Vec<String>,
    teams: Vec<String>,
    apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BranchProtectionUpdateRequiredPullRequestReviews {
    dismissal_restrictions: ListOfUserTeamsOrApps,
    strict: bool,
    dismiss_stale_reviews: bool,
    require_codeowner_reviews: bool,
    // 0 = disable, 1-6 are valid
    required_approving_review_count: i64,
    require_last_push_approval: bool,
    bypass_pull_request_allowances: ListOfUserTeamsOrApps,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BranchProtectionUpdateRestrictions {
    // TODO: Fix
    strict: bool,
    checks: Vec<BranchProtectionUpdateRequiredStatusCheck>,
}

#[derive(Default, Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BranchProtectionUpdate {
    pub required_status_checks: Option<BranchProtectionUpdateRequiredStatusChecks>,
    pub enforce_admins: Option<bool>,
    pub required_pull_request_reviews: Option<BranchProtectionUpdateRequiredPullRequestReviews>,
    pub restrictions: Option<BranchProtectionUpdateRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_creations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_branch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fork_syncing: Option<bool>,
}

impl From<&BranchProtection> for BranchProtectionUpdate {
    fn from(branch_protection: &BranchProtection) -> Self {
        Self {
            required_status_checks: branch_protection.required_status_checks.as_ref().map(
                |required_status_checks| BranchProtectionUpdateRequiredStatusChecks {
                    strict: required_status_checks.strict.unwrap_or(false),
                    checks: required_status_checks
                        .checks
                        .iter()
                        .map(std::convert::Into::into)
                        .collect(),
                },
            ),
            enforce_admins: branch_protection.enforce_admins,
            required_pull_request_reviews: branch_protection
                .required_pull_request_reviews
                .as_ref()
                .map(|required_pull_request_reviews| {
                    BranchProtectionUpdateRequiredPullRequestReviews {
                        dismissal_restrictions: todo!(),
                        strict: todo!(),
                        dismiss_stale_reviews: todo!(),
                        require_codeowner_reviews: todo!(),
                        required_approving_review_count: required_pull_request_reviews
                            .required_approving_review_count
                            .unwrap_or(0),
                        require_last_push_approval: required_pull_request_reviews
                            .require_last_push_approval,
                        bypass_pull_request_allowances: todo!(),
                    }
                }),
            restrictions: None,

            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            required_linear_history: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            allow_force_pushes: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            allow_deletions: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            block_creations: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            required_conversation_resolution: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            lock_branch: None,
            // This is not fetched from branch_protection because if it shouldn't be changed,
            // it should be left as None
            allow_fork_syncing: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unnecessary_wraps)]

    use crate::models::{
        BranchProtectionBuilder, ProtectedBranchRequiredStatusCheck,
        ProtectedBranchRequiredStatusCheckChecksItem,
    };
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(BranchProtection::default(), BranchProtectionUpdate::default())]
    #[case(BranchProtectionBuilder::default().allow_deletions(Some(true)).build()?, BranchProtectionUpdate{
        allow_deletions: None,
        ..Default::default()
    })]
    fn test_branch_protection_update_from(
        #[case] input: BranchProtection,
        #[case] expected: BranchProtectionUpdate,
    ) -> anyhow::Result<()> {
        let actual = BranchProtectionUpdate::from(&input);

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_branch_protection_update_all_enabled() {
        let branch_protection_update = BranchProtectionUpdate::from(
            &(BranchProtection {
                allow_deletions: None,
                allow_force_pushes: None,
                allow_fork_syncing: None,
                block_creations: None,
                enabled: None,
                enforce_admins: Some(true),
                lock_branch: None,
                name: None,
                required_conversation_resolution: None,
                required_linear_history: None,
                required_pull_request_reviews: None,
                required_signatures: None,
                required_status_checks: Some(ProtectedBranchRequiredStatusCheck {
                    checks: vec![
                        ProtectedBranchRequiredStatusCheckChecksItem {
                            app_id: Some(15368),
                            context: "build (ubuntu)".to_string(),
                        },
                        ProtectedBranchRequiredStatusCheckChecksItem {
                            app_id: None,
                            context: "build (windows)".to_string(),
                        },
                    ],
                    contexts: vec![],
                    contexts_url: None,
                    enforcement_level: None,
                    strict: Some(true),
                    url: None,
                }),
                restrictions: None,
            }),
        );
        let expected_branch_protection_update = BranchProtectionUpdate {
            required_status_checks: Some(BranchProtectionUpdateRequiredStatusChecks {
                strict: true,
                checks: vec![
                    BranchProtectionUpdateRequiredStatusCheck {
                        app_id: Some(15368),
                        context: "build (ubuntu)".to_string(),
                    },
                    BranchProtectionUpdateRequiredStatusCheck {
                        app_id: None,
                        context: "build (windows)".to_string(),
                    },
                ],
            }),
            enforce_admins: Some(true),
            required_pull_request_reviews: None,
            restrictions: None,
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            block_creations: None,
            required_conversation_resolution: None,
            lock_branch: None,
            allow_fork_syncing: None,
        };

        assert_eq!(branch_protection_update, expected_branch_protection_update);
    }

    #[test]
    fn test_branch_protection_update_all_disabled() {
        let branch_protection_update = BranchProtectionUpdate::from(
            &(BranchProtection {
                allow_deletions: None,
                allow_force_pushes: None,
                allow_fork_syncing: None,
                block_creations: None,
                enabled: None,
                enforce_admins: Some(false),
                lock_branch: None,
                name: None,
                required_conversation_resolution: None,
                required_linear_history: None,
                required_pull_request_reviews: None,
                required_signatures: None,
                required_status_checks: None,
                restrictions: None,
            }),
        );
        let expected_branch_protection_update = BranchProtectionUpdate {
            required_status_checks: None,
            enforce_admins: Some(false),
            required_pull_request_reviews: None,
            restrictions: None,
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            block_creations: None,
            required_conversation_resolution: None,
            lock_branch: None,
            allow_fork_syncing: None,
        };

        assert_eq!(branch_protection_update, expected_branch_protection_update);
    }

    /*
    #[test]
    fn test_branch_protection_update_all_unset() {
        let branch_protection_update = BranchProtectionUpdate::from(
            &(BranchProtection {
                allow_deletions: None,
                allow_force_pushes: None,
                allow_fork_syncing: None,
                block_creations: None,
                enabled: None,
                enforce_admins: None,
                lock_branch: None,
                name: None,
                protection_url: None,
                required_conversation_resolution: None,
                required_linear_history: None,
                required_pull_request_reviews: None,
                required_signatures: None,
                required_status_checks: None,
                restrictions: None,
                url: None,
            }),
        );
        let expected_branch_protection_update = BranchProtectionUpdate {
            required_status_checks: None,
            enforce_admins: None,
            required_pull_request_reviews: None,
            restrictions: None,
            required_linear_history: None,
            allow_force_pushes: None,
            allow_deletions: None,
            block_creations: None,
            required_conversation_resolution: None,
            lock_branch: None,
            allow_fork_syncing: None,
        };

        assert_eq!(branch_protection_update, expected_branch_protection_update);
    }
    */
}
