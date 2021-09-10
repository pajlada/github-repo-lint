use ::reqwest::blocking::Response;
use log::*;
use std::collections::HashMap;

use crate::api::Client;

impl Client {
    pub fn update_branch_protection(
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

        info!("Updating branch protection at url '{}'", url);

        let rb = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .json(&patch);

        let request = rb.build()?;

        let response = self.client.execute(request)?;

        Ok(response)
    }

    pub fn update_repository_settings(
        &self,
        repo_owner: &str,
        repo_name: &str,
        patch: HashMap<&str, serde_json::Value>,
    ) -> Result<Response, anyhow::Error> {
        let url = self
            .api_root
            .join(format!("/repos/{}/{}", repo_owner, repo_name).as_str())?;

        let rb = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .json(&patch);
        let request = rb.build()?;
        debug!(
            "[{}/{}] Changing: {:?}",
            repo_owner,
            repo_name,
            request.body().expect("body must be set")
        );
        let response = self.client.execute(request)?;

        Ok(response)
    }
}
