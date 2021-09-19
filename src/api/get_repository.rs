use crate::api::Client;
use crate::models::Repository;

impl Client {
    pub fn get_repository(&self, repo_full_name: &str) -> anyhow::Result<Repository> {
        Ok(Repository {
            info: self.get_repository_info(repo_full_name)?,
            topics: self.get_repository_topics(repo_full_name)?,
        })
    }
}
