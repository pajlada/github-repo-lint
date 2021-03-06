use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::branch_protection_rules::BranchProtectionRules;
use crate::repository_settings::RepositorySettings;
use crate::topic_operation::TopicOperations;

fn default_github_api_root() -> String {
    "https://api.github.com".to_string()
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default = "default_github_api_root")]
    pub github_api_root: String,

    pub settings: RepositorySettings,
    pub branch_protection_rules: BranchProtectionRules,
    pub topics: TopicOperations,
}

impl Config {
    fn load<R>(rdr: R) -> anyhow::Result<Config>
    where
        R: Read,
    {
        let config: Config = serde_json::from_reader(rdr)?;

        for (pattern, rule) in &config.branch_protection_rules {
            rule.validate(pattern)?;
        }

        Ok(config)
    }
}

pub fn load_config(path: &Path) -> anyhow::Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Config::load(reader)
}
