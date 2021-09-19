use std::collections::HashMap;

use crate::api::Client;
use ::reqwest::blocking::Response;

impl Client {
    #[allow(dead_code)]
    pub fn update_repository_branch_protection_rules(
        &self,
        repo_owner: &str,
        repo_name: &str,
        branch_protection_name: &str,
        patch: HashMap<&str, serde_json::Value>,
    ) -> Result<Response, anyhow::Error> {
        let url = self.api_root.join(
            format!(
                "/repos/{}/{}/branches/{}/protection",
                repo_owner, repo_name, branch_protection_name
            )
            .as_str(),
        )?;

        // info!("Updating branch protection at url '{}'", url);

        let rb = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .json(&patch);

        let request = rb.build()?;

        let response = self.client.execute(request)?;

        Ok(response)
    }
}
