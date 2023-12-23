use anyhow::Result;
use tracing::debug;

use crate::api::Client;
use crate::models::{Repository, RepositoryInfo, RepositoryListing, RepositoryTopics};
use std::io::Write;

use console::Term;

use super::pagination::{get_pagination_data, PaginationData};

impl Client {
    pub fn get_repository(&self, repo_full_name: &str) -> Result<Repository> {
        Ok(Repository {
            info: self.get_repository_info(repo_full_name)?,
            topics: self.get_repository_topics(repo_full_name)?,
        })
    }

    pub fn get_repository_info(&self, repo_owner_and_name: &str) -> Result<RepositoryInfo> {
        let url = self
            .api_root
            .join(format!("repos/{}", repo_owner_and_name).as_str())?;

        let response = self.client.get(url).send()?;

        Ok(response.json()?)
    }

    pub fn get_repository_topics(&self, repo_owner_and_name: &str) -> Result<RepositoryTopics> {
        let url = self
            .api_root
            .join(format!("repos/{}/topics", repo_owner_and_name).as_str())?;

        let response = self.client.get(url).send()?;

        Ok(response.json()?)
    }

    pub fn get_repositories_from_user(
        &self,
        terminal: &mut Term,
        repo_owner: &str,
    ) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        let mut pagination = PaginationData {
            next: Some(
                self.api_root
                    .join(format!("users/{}/repos", repo_owner).as_str())?,
            ),
        };

        terminal.write_all(format!("Loading repositories from user {}", repo_owner).as_bytes())?;

        while let Some(url) = pagination.next {
            let response = self
                .client
                .get(url)
                .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
                .send()?;

            // Read pagination from headers
            pagination = get_pagination_data(response.headers())?;

            let page_repos: Vec<RepositoryListing> = response.json()?;

            for repo in page_repos {
                terminal.clear_line()?;
                terminal.write_all(
                    format!(
                        "Loading repositories from user {} ({})",
                        repo_owner, repo.name
                    )
                    .as_bytes(),
                )?;

                repos.push(self.get_repository(&repo.full_name)?);
            }
        }

        terminal.write_line("").unwrap();

        Ok(repos)
    }

    pub fn get_repositories_from_organization(
        &self,
        terminal: &mut Term,
        repo_owner: &str,
    ) -> anyhow::Result<Vec<Repository>> {
        let mut repos = Vec::new();

        let mut pagination = PaginationData {
            next: Some(
                self.api_root
                    .join(format!("orgs/{}/repos", repo_owner).as_str())?,
            ),
        };

        terminal.write_all(
            format!("Loading repositories from organization {}", repo_owner).as_bytes(),
        )?;

        while let Some(url) = pagination.next {
            let response = self
                .client
                .get(url)
                .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
                .send()?;

            // Read pagination from headers
            pagination = get_pagination_data(response.headers())?;

            let page_repos: Vec<RepositoryListing> = response.json()?;

            for repo in page_repos {
                debug!("get_repositories_from_organization repo: {:?}", repo);
                terminal.clear_line()?;
                terminal.write_all(
                    format!(
                        "Loading repositories from organization {} ({})",
                        repo_owner, repo.name
                    )
                    .as_bytes(),
                )?;

                repos.push(self.get_repository(&repo.full_name)?);
            }
        }

        terminal.write_line("").unwrap();

        Ok(repos)
    }
}
