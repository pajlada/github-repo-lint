use std::collections::HashSet;

use reqwest::StatusCode;
use serde_json::json;
use tracing::info;

use crate::api::Client;

impl Client {
    pub fn update_repository_topics(
        &self,
        repo_full_name: &str,
        topics: &HashSet<String>,
    ) -> anyhow::Result<()> {
        let url = self
            .api_root
            .join(format!("repos/{repo_full_name}/topics").as_str())?;

        let patch = json!({
            "names": topics,
        });

        info!("Send patch: {:?} to {}", patch, url);

        let response = self.client.put(url).json(&patch).send()?;

        match response.status() {
            StatusCode::OK => Ok(()),
            e => Err(anyhow::anyhow!("Error updating topics: {}", e)),
        }
    }
}
