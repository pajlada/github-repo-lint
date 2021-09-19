use crate::api::Client;
use crate::models::RepositoryTopics;

impl Client {
    pub fn get_repository_topics(
        &self,
        repo_owner_and_name: &str,
    ) -> anyhow::Result<RepositoryTopics> {
        let url = self
            .api_root
            .join(format!("repos/{}/topics", repo_owner_and_name).as_str())?;

        let response = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.github.mercy-preview+json",
            )
            .send()?;

        Ok(response.json()?)
    }
}
