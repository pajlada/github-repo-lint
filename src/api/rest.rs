use std::collections::{HashMap, HashSet};
use std::io::Write;

use ::reqwest::blocking::Response;
use ::reqwest::{StatusCode, Url};
use console::Term;
use log::*;
use serde_json::json;

use crate::api::Client;
use crate::models::{Repository, RepositoryInfo, RepositoryListing, RepositoryTopics};

struct PaginationData {
    next: Option<Url>,
}

fn get_pagination_data(
    headers: &reqwest::header::HeaderMap<reqwest::header::HeaderValue>,
) -> anyhow::Result<PaginationData> {
    let mut res = PaginationData { next: None };

    if let Some(links_header) = headers.get("link") {
        let links_header_str = links_header.to_str()?;

        for link in links_header_str.split(',') {
            let segments: Vec<&str> = link.split(';').collect();

            if segments.len() != 2 {
                // invalid segment
                continue;
            }

            let url_part = segments[0].trim();
            let rel_part = segments[1].trim();

            if !url_part.starts_with('<') || !url_part.ends_with('>') {
                // invalid href
                continue;
            }

            let len = url_part.len();

            let url = match Url::parse(&url_part[1..len - 1]) {
                Ok(u) => u,
                Err(_) => continue,
            };

            match rel_part {
                "rel=\"next\"" => {
                    if res.next.is_some() {
                        return Err(anyhow::anyhow!("next link found twice"));
                    }

                    res.next = Some(url);
                }
                "rel=\"prev\"" | "rel=\"first\"" | "rel=\"last\"" => {
                    // Valid values, but we don't care about them
                }
                e => return Err(anyhow::anyhow!("unknown rel: {}", e)),
            }
        }
    }

    Ok(res)
}

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

    pub fn update_repository_settings(
        &self,
        repo_owner: &str,
        repo_name: &str,
        patch: HashMap<&str, serde_json::Value>,
    ) -> Result<Response, anyhow::Error> {
        let url = self
            .api_root
            .join(format!("repos/{}/{}", repo_owner, repo_name).as_str())?;

        // info!("Updating repository settings at url '{}'", url);

        let rb = self
            .client
            .patch(url)
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

    pub fn update_repository_topics(
        &self,
        repo_full_name: &str,
        topics: HashSet<String>,
    ) -> anyhow::Result<()> {
        let url = self
            .api_root
            .join(format!("repos/{}/topics", repo_full_name).as_str())?;

        let patch = json!({
            "names": topics,
        });

        info!("Send patch: {:?} to {}", patch, url);

        let response = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.github.mercy-preview+json",
            )
            .json(&patch)
            .send()?;

        match response.status() {
            StatusCode::OK => Ok(()),
            e => Err(anyhow::anyhow!("Error updating topics: {}", e)),
        }
    }

    fn get_repository_info(&self, repo_owner_and_name: &str) -> anyhow::Result<RepositoryInfo> {
        let url = self
            .api_root
            .join(format!("repos/{}", repo_owner_and_name).as_str())?;

        let response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .send()?;

        Ok(response.json()?)
    }

    fn get_repository_topics(&self, repo_owner_and_name: &str) -> anyhow::Result<RepositoryTopics> {
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

    pub fn get_repository(&self, repo_full_name: &str) -> anyhow::Result<Repository> {
        Ok(Repository {
            info: self.get_repository_info(repo_full_name)?,
            topics: self.get_repository_topics(repo_full_name)?,
        })
    }

    pub fn get_repositories_from_user(
        &self,
        terminal: &mut Term,
        repo_owner: &str,
    ) -> anyhow::Result<Vec<Repository>> {
        let mut repos = Vec::new();

        let mut pagination = PaginationData {
            next: Some(
                self.api_root
                    .join(format!("users/{}/repos?per_page=5", repo_owner).as_str())?,
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

            pagination.next = None;
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
                    .join(format!("orgs/{}/repos?per_page=5", repo_owner).as_str())?,
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

            pagination.next = None;
        }

        terminal.write_line("").unwrap();

        Ok(repos)
    }
}
