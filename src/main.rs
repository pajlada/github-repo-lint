use ::reqwest::blocking::Client;
use log::*;
use std::collections::HashMap;
use std::path::Path;

mod api;

mod branch_protection_rules;
mod repository_settings;

mod get_repositories_from_user;

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
    api_client: &api::Client,
    mut repository: api::graphql::Repository,
    expected_repository_settings: &RepositorySettings,
    expected_branch_protection_rules: &BranchProtectionRules,
) -> Result<(), anyhow::Error> {
    let repo_name = repository.name.as_str();
    let repo_owner = repository.owner.login.as_str();
    let repo_with_owner = repository.name_with_owner.as_str();

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
                let branch_protection_name = actual_branch_protection_rule.pattern.as_str();
                debug!("Repository {} Diff: {:?}", repo_with_owner, diff);
                let patch = diff.dump_patch(&actual_branch_protection_rule);
                debug!("Repository {} Patch: {:?}", repo_with_owner, patch);
                if repo_with_owner == "pajlada/TempestNotifier" {
                    let response = api_client.update_branch_protection(
                        repo_owner,
                        repo_name,
                        branch_protection_name,
                        patch,
                    )?;

                    info!("Response: {:?}", response);
                    info!("Response body: {:?}", response.text()?);
                }
                // Repo has a branch protection rule with this key, see if the values match!
            } else {
                debug!(
                    "Repository {} has a 100% matching branch protection rule:\n{:?}\n{:?}",
                    repo_with_owner, actual_branch_protection_rule, expected_branch_protection_rule
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
        if !patch.is_empty() {
            let response = api_client.update_repository_settings(repo_owner, repo_name, patch)?;
            info!("Update repository settings response: {:?}", response);
            info!(
                "Update repository settings body:     {:?}",
                response.text()?
            );
        }
    }

    Ok(())
}

fn check_repositories(
    api_client: &api::Client,
    owner: String,
    expected_repository_settings: RepositorySettings,
    expected_branch_protection_rules: BranchProtectionRules,
) -> Result<(), anyhow::Error> {
    let repositories = api_client.get_repositories_from_user(&owner)?;

    for repository in repositories {
        if repository.is_archived || repository.is_disabled {
            info!(
                "Skipping {} because it's archived or disabled",
                repository.name_with_owner
            );
            continue;
        }
        let name_with_owner = repository.name_with_owner.clone();
        if let Err(e) = check_repository(
            api_client,
            repository,
            &expected_repository_settings,
            &expected_branch_protection_rules,
        ) {
            error!("Error checking repository {}: {}", name_with_owner, e);
        }
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

    let api_root = std::env::var("GITHUB_API_ROOT")
        .or_else::<std::env::VarError, _>(|_| Ok("https://api.github.com".to_string()))?;

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

    let api_client = api::new(client, api_root.as_str())?;

    check_repositories(
        &api_client,
        "pajlada".to_string(),
        expected_repository_settings,
        expected_branch_protection_rules,
    )?;

    Ok(())
}
