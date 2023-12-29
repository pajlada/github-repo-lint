use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::branch_protection_rules::BranchProtectionRule;
use crate::repository_settings::RepositorySettings;
use crate::topic_operation::TopicOperations;

fn default_github_api_root() -> String {
    "https://api.github.com".to_string()
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_github_api_root")]
    pub github_api_root: String,

    pub settings: Option<RepositorySettings>,
    pub branch_protections: Option<Vec<BranchProtectionRule>>,
    pub topics: Option<TopicOperations>,
}

pub fn load_from_reader<R: std::io::Read>(reader: R) -> Result<Config> {
    Ok(serde_json::from_reader(reader)?)
}

pub fn load(path: &Path) -> Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    load_from_reader(reader)
}
