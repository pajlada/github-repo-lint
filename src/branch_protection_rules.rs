use anyhow::Result;
use serde::Deserialize;

use crate::models::{BranchProtection, BranchProtectionUpdate};

#[allow(unused_macros)]
macro_rules! ensure_same {
    ($s:ident, $r:ident, $field_name:ident) => {
        if let Some(expected) = $s.$field_name {
            let actual = $r.$field_name;
            if expected != actual {
                Some(expected)
            } else {
                Some(expected)
            }
        } else {
            None
        }
    };
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionOperation {
    MustExist,
    MayExist,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BranchProtectionRule {
    branch_name_pattern: String,
    operation: BranchProtectionOperation,
    is_admin_enforced: Option<bool>,
    allow_deletions: Option<bool>,
}

impl BranchProtectionRule {
    pub fn parsed_branch_name_pattern<'a>(
        &'a self,
        repo: &'a crate::models::Repository,
    ) -> &'a String {
        if self.branch_name_pattern == "$default_branch" {
            &repo.info.default_branch
        } else {
            &self.branch_name_pattern
        }
    }

    pub fn diff(
        &self,
        actual_branch_protection: &Option<BranchProtection>,
    ) -> Result<Option<BranchProtectionUpdate>> {
        let mut diff = if let Some(actual_branch_protection) = actual_branch_protection {
            BranchProtectionUpdate::from(actual_branch_protection)
        } else {
            match self.operation {
                BranchProtectionOperation::MayExist => {
                    // The repo does not contain a branch protection rule with this pattern
                    // We do not require a branch to be created
                    return Ok(None);
                }
                BranchProtectionOperation::MustExist => BranchProtectionUpdate::default(),
            }
        };

        if let Some(v) = self.is_admin_enforced {
            diff.enforce_admins = Some(v);
        }
        if let Some(v) = self.allow_deletions {
            diff.allow_deletions = Some(v);
        }

        Ok(Some(diff))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config_from_reader;

    use super::*;

    #[test]
    fn test_load_branch_protection_rules() -> Result<(), anyhow::Error> {
        let contents = r#"
{"branch_protections": [
{
    "branch_name_pattern": "master",
    "operation": "may_exist",
    "is_admin_enforced": true
}
]}"#;
        let reader = std::io::Cursor::new(contents);
        let actual_rules = load_config_from_reader(reader)?;
        let expected_rules: Vec<BranchProtectionRule> = vec![BranchProtectionRule {
            branch_name_pattern: "master".to_string(),
            operation: BranchProtectionOperation::MayExist,
            is_admin_enforced: Some(true),
            allow_deletions: None,
        }];

        assert_eq!(expected_rules, actual_rules.branch_protections.unwrap());

        Ok(())
    }
}
