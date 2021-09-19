use std::collections::HashSet;

use serde::Deserialize;

use crate::models::RepositoryOwner;

#[derive(Debug, Deserialize)]
pub struct RepositoryListing {
    pub name: String,      // e.g. lidl-normalize
    pub full_name: String, // e.g. pajlada/lidl-normalize
}

#[derive(Debug, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub full_name: String,
    pub owner: RepositoryOwner,
    pub default_branch: String,
    pub archived: bool,
    pub disabled: bool,

    pub visibility: Option<String>, // optional because it doesn't exist in GHE3.0?
    pub allow_rebase_merge: Option<bool>, // Should always be Some
    pub allow_merge_commit: Option<bool>, // Should always be Some
    pub allow_squash_merge: Option<bool>, // Should always be Some
    pub allow_auto_merge: Option<bool>, // Doesn't exist in GHE3.0 or GHE3.1, but available in GHE3.2
    pub delete_branch_on_merge: Option<bool>, // Should always be Some
    pub has_issues: Option<bool>,       // Should always be Some
    pub has_projects: Option<bool>,     // Should always be Some
    pub has_downloads: Option<bool>,    // Should always be Some
    pub has_wiki: Option<bool>,         // Should always be Some
}

#[derive(Debug, Deserialize)]
pub struct RepositoryTopics {
    pub names: HashSet<String>,
}

pub struct Repository {
    pub info: RepositoryInfo,
    pub topics: RepositoryTopics,
}
