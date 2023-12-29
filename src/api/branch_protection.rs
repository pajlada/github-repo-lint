use anyhow::Result;
use tracing::info;

use crate::{
    api::Client,
    models::{BranchProtection, BranchProtectionUpdate},
};
use reqwest::{blocking::Response, StatusCode};

impl Client {
    pub fn get_branch_protection(
        &self,
        repo_owner: &str,
        repo_name: &str,
        branch_name: &str,
    ) -> Result<Option<BranchProtection>> {
        let url = self.api_root.join(
            format!("repos/{repo_owner}/{repo_name}/branches/{branch_name}/protection").as_str(),
        )?;

        let response = self.client.get(url).send()?.error_for_status()?;

        if response.status() == StatusCode::NOT_FOUND {
            info!(
                "{repo_owner}/{repo_name} {branch_name} did not have any branch protection rules"
            );
            return Ok(None);
        }

        Ok(Some(response.json()?))
    }

    pub fn update_branch_protection(
        &self,
        repo_owner: &str,
        repo_name: &str,
        branch: &str,
        patch: &BranchProtectionUpdate,
    ) -> Result<Response> {
        let url = self.api_root.join(
            format!("repos/{repo_owner}/{repo_name}/branches/{branch}/protection").as_str(),
        )?;

        info!("Updating branch protection at url '{}'", url);
        info!("patch: {patch:?}");

        let response = self.client.put(url).json(&patch).send()?;

        info!("Response: {response:?}");

        Ok(response)
    }
}
