use log::*;

use crate::api::Client;
use crate::models::RepositoryInfo;

impl Client {
    pub fn get_repository_info(&self, repo_owner_and_name: &str) -> anyhow::Result<RepositoryInfo> {
        let url = self
            .api_root
            .join(format!("repos/{}", repo_owner_and_name).as_str())?;

        let response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .send()?;

        let result: RepositoryInfo = response.json()?;

        debug!("get_repository_info result: {:?}", result);

        Ok(result)
    }
}
