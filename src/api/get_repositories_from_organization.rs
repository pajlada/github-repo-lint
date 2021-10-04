use std::io::Write;

use console::Term;
use log::*;

use crate::api::pagination::{get_pagination_data, PaginationData};
use crate::api::Client;
use crate::models::{Repository, RepositoryListing};

impl Client {
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
