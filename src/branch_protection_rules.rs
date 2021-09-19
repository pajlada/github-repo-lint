use serde::Deserialize;
// use serde_json::json;
use std::collections::HashMap;
use std::io::Read;

// use crate::repository;

pub type BranchProtectionRules = HashMap<String, BranchProtectionRule>;

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

macro_rules! define_branch_protection_rule {
    ( $( $json_field_name:ident : $field_name:ident : $field_type:ty, )* ) => {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct BranchProtectionRule {
            required_approving_review_count: Option<i64>,
            required_status_check_contexts: Option<Vec<String>>,
            requires_status_checks: Option<bool>,
            requires_strict_status_checks: Option<bool>,
            $(
                $field_name: $field_type,
            )*
        }

        impl BranchProtectionRule {
            // NOTE: this isn't actually a patch, we need to fill in with values from the GET shit
            // pub fn dump_patch(&self, actual_rule: &repository::GQLBranchProtectionRules) -> HashMap<&str, serde_json::Value> {
            //     let mut map = HashMap::new();

            //     $(
            //         if let Some(v) = self.$field_name {
            //             map.insert(stringify!($json_field_name), json!(v));
            //         } else {
            //             map.insert(stringify!($json_field_name), json!(actual_rule.$field_name));
            //         }
            //     )*

            //     map.insert("required_pull_request_reviews", json!(null));
            //     map.insert("restrictions", json!(null));

            //     // TODO: Validate that this makes sense for all 9 permutations of
            //     // required_status_checks/requires_strict_status_checks
            //     //
            //     // u = unset, t = true, f = false
            //     // required | strict
            //     // -----------------
            //     //        u |      u
            //     //        u |      t INVALID
            //     //        u |      f INVALID
            //     //        t |      u INVALID
            //     //        t |      t
            //     //        t |      f
            //     //        f |      u INVALID
            //     //        f |      t
            //     //        f |      f
            //     if let Some(v) = self.requires_status_checks {
            //         if v {
            //             let mut status_check_map = HashMap::new();
            //             let strict = self.requires_strict_status_checks.unwrap_or(false);
            //             status_check_map.insert("strict", json!(strict));
            //             let actual_contexts: Vec<String> = actual_rule.required_status_check_contexts.as_ref().expect("xD").iter().filter_map(|f| f.clone()).collect();
            //             let contexts = self.required_status_check_contexts.as_ref().unwrap_or(&actual_contexts);
            //             status_check_map.insert("contexts", json!(contexts)); // TODO
            //             // status_check_map.insert("contexts", json!(["foo", "bar"])); // TODO
            //             map.insert("required_status_checks", json!(status_check_map));
            //         } else {
            //             map.insert("required_status_checks", json!(null));
            //         }
            //     } else {
            //             map.insert("required_status_checks", json!(null));
            //     }

            //     return map;
            // }

            // pub fn is_different(&self, repository: &repository::GQLBranchProtectionRules) -> bool {
            //     $(
            //         if let Some(v) = self.$field_name {
            //             if v != repository.$field_name {
            //                 return true;
            //             }
            //         }
            //     )*

            //         if let Some(v) = self.requires_status_checks {
            //             if v != repository.requires_status_checks {
            //                 return true;
            //             }
            //         }

            //         if let Some(v) = self.requires_strict_status_checks {
            //             if v != repository.requires_strict_status_checks {
            //                 return true;
            //             }
            //         }

            //     return false;
            // }

            // pub fn diff(&self, repository: &repository::GQLBranchProtectionRules) -> Option<BranchProtectionRule> {
            //     let r = BranchProtectionRule {
            //         required_approving_review_count: Some(-1),
            //         required_status_check_contexts: None,
            //         requires_status_checks: ensure_same!(self, repository, requires_status_checks),
            //         requires_strict_status_checks: ensure_same!(self, repository, requires_strict_status_checks),
            //     $(
            //         $field_name: ensure_same!(self, repository, $field_name),
            //     )*
            //     };

            //     if !r.is_different(repository) {
            //         return None;
            //     }

            //     return Some(r);
            // }
        }
    }
}

define_branch_protection_rule! {
    allow_deletions : allows_deletions: Option<bool>,
    allow_force_pushes : allows_force_pushes: Option<bool>,
    enforce_admins : is_admin_enforced: Option<bool>,
    required_linear_history : requires_linear_history: Option<bool>,
    // required_conversation_resolution : requires_conversation_resolution: Option<bool>,
}

impl BranchProtectionRule {
    pub fn validate(&self, pattern: &str) -> Result<(), anyhow::Error> {
        // TODO: Sanity check the requires_status_checks and requires_strict_status_checks values
        let required = self.requires_status_checks;
        let strict = self.requires_strict_status_checks;

        match required {
            None => match strict {
                None => {}
                Some(v) => {
                    return Err(anyhow::anyhow!("For pattern {}, requires_status_checks is null while requires_strict_status_check is {}. These values are incompatible. Valid values: null", pattern, v));
                }
            },
            Some(v) => match strict {
                None => {
                    return Err(anyhow::anyhow!("For pattern {}, requires_status_checks is {} while requires_strict_status_check is null. These values are incompatible. Valid values: true, false", pattern, v));
                }
                Some(_) => {}
            },
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub fn load_from_reader<R>(rdr: R) -> Result<BranchProtectionRules, anyhow::Error>
where
    R: Read,
{
    let rules: BranchProtectionRules = serde_json::from_reader(rdr)?;

    for (pattern, rule) in &rules {
        rule.validate(pattern)?;
    }

    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_branch_protection_rules() -> Result<(), anyhow::Error> {
        let contents = r#"
{
  "master": {
    "__comment_is_admin_enforced": "Include administrators - Enforce all configured restrictions above for administrators.",
    "__tested_is_admin_enforced": true,
    "is_admin_enforced": true,

    "__comment_required_approving_review_count": "doesn't work",
    "required_approving_review_count": null,

    "__comment_required_status_check_contexts": "Not yet implemented, handles null fine",
    "__example_required_status_check_contexts": [
      "build-$REPONAME",
      "lint-$REPONAME"
    ],
    "required_status_check_contexts": null,
    "__comment_requires_commit_signatures": "doesn't work because REST api doesn't support it",
    "requires_commit_signatures": null,
    "requires_linear_history": true,

    "requires_conversation_resolution": null,

    "__comment_requires_status_checks": "these two values are a bit special as they are kind of codependant. figure out which values are invalid before trying to diff",
    "requires_status_checks": true,
    "requires_strict_status_checks": true,

    "__comment_allows_force_pushes": "Allow force pushes - Permit force pushes for all users with push access.",
    "allows_force_pushes": false,

    "__comment_allows_deletions": "Allow deletions - Allow users with push access to delete matching branches.",
    "allows_deletions": false
  }
}"#;
        let reader = std::io::Cursor::new(contents);
        let actual_rules = load_from_reader(reader)?;
        let mut expected_rules: BranchProtectionRules = HashMap::new();
        expected_rules.insert(
            "master".to_string(),
            BranchProtectionRule {
                required_approving_review_count: None,
                required_status_check_contexts: None,
                requires_status_checks: Some(true),
                requires_strict_status_checks: Some(true),
                allows_deletions: Some(false),
                allows_force_pushes: Some(false),
                is_admin_enforced: Some(true),
                requires_linear_history: Some(true),
                // requires_conversation_resolution: None,
            },
        );
        for (pattern, actual_rule) in actual_rules {
            let expected_rule = expected_rules.remove(&pattern).expect("Must exist");
            assert_eq!(actual_rule, expected_rule);
        }

        assert!(expected_rules.is_empty());

        Ok(())
    }
}
