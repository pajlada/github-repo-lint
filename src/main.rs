use ::reqwest::blocking::Client;
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use log::*;
use std::collections::HashMap;
use std::path::Path;

mod branch_protection_rules;
mod repository_settings;

mod get_repositories_from_user;

use crate::get_repositories_from_user::GetRepositoriesFromUser;

use crate::get_repositories_from_user::get_repositories_from_user::GetRepositoriesFromUserUserRepositoriesNodes as GQLRepository;
use crate::get_repositories_from_user::get_repositories_from_user::GetRepositoriesFromUserUserRepositoriesNodesBranchProtectionRulesNodes as GQLBranchProtectionRules;

use crate::branch_protection_rules::BranchProtectionRules;
use crate::repository_settings::RepositorySettings;

fn convert_branch_protection_rules_to_hashmap(
    branch_protection_rules: Option<Vec<Option<GQLBranchProtectionRules>>>,
) -> Result<HashMap<String, GQLBranchProtectionRules>, anyhow::Error> {
    let mut rule_map = HashMap::new();

    if let Some(actual_branch_protection_rules) = branch_protection_rules {
        for rule in actual_branch_protection_rules.into_iter().flatten() {
            rule_map.insert(rule.pattern.clone(), rule);
        }
    }

    Ok(rule_map)
}

fn check_repository(
    client: &reqwest::blocking::Client,
    mut repository: GQLRepository,
    expected_repository_settings: &RepositorySettings,
    expected_branch_protection_rules: &BranchProtectionRules,
) -> Result<(), anyhow::Error> {
    let repo_name = repository.name_with_owner.clone();

    if repository.branch_protection_rules.page_info.has_next_page {
        return Err(anyhow::anyhow!(
            "Repository {} has more than 100 branch protection rules. We cannot do anything",
            repo_name
        ));
    }

    let mut actual_branch_protection_rules = convert_branch_protection_rules_to_hashmap(
        repository.branch_protection_rules.nodes.take(),
    )?;

    for (pattern, expected_branch_protection_rule) in expected_branch_protection_rules {
        // info!(
        //     "For pattern {} we expect {:?}",
        //     pattern, expected_branch_protection_rule
        // );

        if let Some(actual_branch_protection_rule) = actual_branch_protection_rules.remove(pattern)
        {
            if let Some(diff) = expected_branch_protection_rule.diff(&actual_branch_protection_rule)
            {
                debug!("Repository {} Diff: {:?}", repo_name, diff);
                let patch = diff.dump_patch(&actual_branch_protection_rule);
                debug!("Repository {} Patch: {:?}", repo_name, patch);
                if repo_name == "pajlada/TempestNotifier" {
                    let url = format!(
                        "https://api.github.com/repos/{}/branches/{}/protection",
                        repo_name, pattern
                    );
                    let rb = client
                        .put(url)
                        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
                        .json(&patch);
                    let request = rb.build().expect("Request must be built");
                    info!(
                        "[{}] Changing: {:?}",
                        repo_name,
                        request.body().expect("asd")
                    );
                    let response = client.execute(request)?;
                    info!("Response: {:?}", response);
                    info!("Response body: {:?}", response.text()?);
                }
                // Repo has a branch protection rule with this key, see if the values match!
            } else {
                debug!(
                    "Repository {} has a 100% matching branch protection rule:\n{:?}\n{:?}",
                    repo_name, actual_branch_protection_rule, expected_branch_protection_rule
                );
            }
        } else {
            // Repo does *not* have a branch protection rule with this key, should we create it?
            // Maybe ask the user?
        }
    }

    let result = expected_repository_settings.diff(&repository);

    if !result.empty() {
        // Update repository settings
        let patch = result.dump_patch();
        let url = format!("https://api.github.com/repos/{}", repo_name);
        let rb = client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .json(&patch);
        let request = rb.build().expect("Request must be built");
        info!(
            "[{}] Changing: {:?}",
            repo_name,
            request.body().expect("asd")
        );
        client.execute(request)?;
    }

    Ok(())
}

fn check_repositories(
    client: &reqwest::blocking::Client,
    owner: String,
    expected_repository_settings: RepositorySettings,
    expected_branch_protection_rules: BranchProtectionRules,
) -> Result<(), anyhow::Error> {
    let mut has_next_page = true;
    let mut cursor: Option<String> = None;

    while has_next_page {
        let variables = get_repositories_from_user::get_repositories_from_user::Variables {
            owner: owner.clone(),
            cursor: cursor.clone(),
        };

        let response_body = post_graphql::<GetRepositoriesFromUser, _>(
            client,
            "https://api.github.com/graphql",
            variables,
        )?;

        let response_data: get_repositories_from_user::get_repositories_from_user::ResponseData =
            response_body.data.expect("missing response data");

        let repositories = response_data.user.expect("No user found").repositories;

        for repository in repositories.nodes.expect("No repositories found") {
            let rep = repository.expect("xD");
            if rep.is_archived || rep.is_disabled {
                info!(
                    "Skipping {} because it's archived or disabled",
                    rep.name_with_owner
                );
                continue;
            }
            let name_with_owner = rep.name_with_owner.clone();
            if let Err(e) = check_repository(
                client,
                rep,
                &expected_repository_settings,
                &expected_branch_protection_rules,
            ) {
                error!("Error checking repository {}: {}", name_with_owner, e);
            }
        }

        has_next_page = repositories.page_info.has_next_page;
        cursor = repositories.page_info.end_cursor;
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let expected_repository_settings =
        repository_settings::load_from_file(Path::new("expected_repository_settings.json"))?;
    let expected_branch_protection_rules = branch_protection_rules::load_from_file(Path::new(
        "expected_branch_protection_rules.json",
    ))?;

    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    let client = Client::builder()
        .user_agent("test")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))?,
            ))
            .collect(),
        )
        .build()?;

    check_repositories(
        &client,
        "pajlada".to_string(),
        expected_repository_settings,
        expected_branch_protection_rules,
    )?;

    Ok(())
}
