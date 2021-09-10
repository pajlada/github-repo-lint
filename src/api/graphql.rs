use graphql_client::reqwest::post_graphql_blocking as post_graphql;

use crate::api::Client;
pub use crate::get_repositories_from_user::get_repositories_from_user::GetRepositoriesFromUserUserRepositoriesNodes as Repository;
use crate::get_repositories_from_user::get_repositories_from_user::Variables as GetRepositoriesVariables;
use crate::get_repositories_from_user::GetRepositoriesFromUser;

impl Client {
    pub fn get_repositories_from_user(
        &self,
        repo_owner: &str,
    ) -> Result<Vec<Repository>, anyhow::Error> {
        let url = self.api_root.join("/graphql")?;
        let mut has_next_page = true;
        let mut cursor: Option<String> = None;
        let mut repos = Vec::new();

        while has_next_page {
            let variables = GetRepositoriesVariables {
                owner: repo_owner.to_string(),
                cursor: cursor.clone(),
            };
            let response_body = post_graphql::<GetRepositoriesFromUser, _>(
                &self.client,
                url.to_string(),
                variables,
            )?;

            let response_data = match response_body.data {
                Some(v) => v,
                None => return Err(anyhow::anyhow!("Missing response body data")),
            };

            let repositories = response_data.user.expect("No user found").repositories;

            for repo in repositories
                .nodes
                .expect("No repositories found")
                .into_iter()
                .flatten()
            {
                // super efficient pagman
                repos.push(repo);
            }

            has_next_page = repositories.page_info.has_next_page;
            cursor = repositories.page_info.end_cursor;
        }

        Ok(repos)
    }

    pub fn get_repositories_from_organization(&self) {
        unimplemented!();
    }
}
