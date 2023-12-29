use std::collections::HashSet;

use serde::Deserialize;

use crate::models::RepositoryOwner;

#[derive(Debug, Deserialize)]
pub struct Listing {
    pub name: String,      // e.g. lidl-normalize
    pub full_name: String, // e.g. pajlada/lidl-normalize
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub full_name: String,
    pub owner: RepositoryOwner,
    pub default_branch: String,
    pub archived: bool,
    pub disabled: bool,

    // Exists in all tested GH environment, so should always be Some
    pub visibility: Option<String>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,

    pub allow_auto_merge: Option<bool>, // Doesn't exist in GHE3.0 or GHE3.1, but available in GHE3.2
}

#[derive(Debug, Deserialize)]
pub struct Topics {
    pub names: HashSet<String>,
}

pub struct Repository {
    pub info: Info,
    pub topics: Topics,
}
