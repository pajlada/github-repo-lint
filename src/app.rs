use anyhow::Result;
use std::io::Write;

use crate::context::Context;
use crate::models::Repository;
use crate::topic_operation::TopicOperation;

use console::{style, Style, Term};
use tracing::{debug, error, info};

fn update_line<S: Into<String>>(terminal: &mut Term, msg: S) {
    terminal.clear_line().unwrap();
    terminal.write_all(msg.into().as_bytes()).unwrap();
}

pub fn run(
    mut ctx: Context,
    repos: Vec<&str>,
    users: Vec<&str>,
    organizations: Vec<&str>,
) -> Result<()> {
    let mut repositories: Vec<Repository> = Vec::new();

    info!("Expected repository settings: {:?}", ctx.config.settings);

    for user in users {
        repositories.append(
            &mut ctx
                .api_client
                .get_repositories_from_user(&mut ctx.terminal, user)?,
        );
    }

    for organization in organizations {
        repositories.append(
            &mut ctx
                .api_client
                .get_repositories_from_organization(&mut ctx.terminal, organization)?,
        );
    }

    for repo_owner_and_name in repos {
        // TODO: Push into repositories
        repositories.push(ctx.api_client.get_repository(repo_owner_and_name)?);
    }

    check_repositories(&mut ctx, repositories)?;

    Ok(())
}

fn check_repositories(
    ctx: &mut Context,
    repositories: Vec<Repository>,
) -> Result<(), anyhow::Error> {
    for repository in repositories {
        // terminal.write_all("\n".as_bytes());
        let name_with_owner = repository.info.full_name.clone();

        // TODO: Move to the impl check_repository thing (or maybe filter out in list of repos?)
        if repository.info.archived || repository.info.disabled {
            let gray = Style::new().color256(242);
            ctx.terminal.clear_line()?;
            ctx.terminal.write_all(
                gray.apply_to(format!(
                    "Checking repository {name_with_owner} - Skipping, it's archived or disabled.\n"
                ))
                .to_string()
                .as_bytes(),
            )?;
            // info!(
            //     "Skipping {} because it's archived or disabled",
            //     repository.name_with_owner
            // );
            continue;
        }
        if let Err(e) = repository.check_repository(ctx) {
            error!("Error checking repository {}: {}", name_with_owner, e);
        }
    }

    Ok(())
}

impl Repository {
    #[allow(dead_code)]
    fn check_branch_protection_rules(&self, ctx: &mut Context) -> Result<()> {
        if ctx.config.branch_protections.is_none() {
            return Ok(());
        }
        let desired_branch_protections = ctx.config.branch_protections.as_ref().unwrap();
        info!(
            "Check {} branch protections",
            desired_branch_protections.len()
        );

        for desired_branch_protection in desired_branch_protections {
            info!("Desired branch protection: {desired_branch_protection:?}");

            let branch_name_pattern = desired_branch_protection.parsed_branch_name_pattern(self);

            // TODO: Support special $branch_name_pattern
            let branch_protection = ctx.api_client.get_branch_protection(
                &self.info.owner.login,
                &self.info.name,
                branch_name_pattern,
            )?;

            info!("Actual branch protection: {branch_protection:#?}");

            let diff = desired_branch_protection.diff(&branch_protection)?;

            info!("Diff required: {diff:?}");

            if let Some(diff) = diff {
                ctx.api_client.update_branch_protection(
                    &self.info.owner.login,
                    &self.info.name,
                    branch_name_pattern,
                    &diff,
                )?;
            }
        }

        Ok(())
        /*
        if self.branch_protection_rules.page_info.has_next_page {
            return Err(anyhow::anyhow!(
                "Repository {} has more than 100 branch protection rules. We cannot do anything",
                repo_name
            ));
        }

        let mut actual_branch_protection_rules =
            convert_branch_protection_rules_to_hashmap(self.branch_protection_rules.nodes.take())?;

        for (pattern, expected_branch_protection_rule) in &ctx.config.branch_protection_rules {
            // info!(
            //     "For pattern {} we expect {:?}",
            //     pattern, expected_branch_protection_rule
            // );

            if let Some(actual_branch_protection_rule) =
                actual_branch_protection_rules.remove(pattern)
            {
                if let Some(diff) =
                    expected_branch_protection_rule.diff(&actual_branch_protection_rule)
                {
                    let branch_protection_name = actual_branch_protection_rule.pattern.as_str();
                    debug!("Repository {} Diff: {:?}", repo_with_owner, diff);
                    let patch = diff.dump_patch(&actual_branch_protection_rule);
                    debug!("Repository {} Patch: {:?}", repo_with_owner, patch);
                    if !ctx.options.dry_run_bpr {
                        let response = ctx.api_client.update_branch_protection(
                            repo_owner,
                            repo_name,
                            branch_protection_name,
                            patch,
                        )?;

                        debug!("Response: {:?}", response);
                        debug!("Response body: {:?}", response.text()?);
                    } else {
                        debug!(
                            "DRY RUN: Update branch protection rule for {}",
                            repo_with_owner
                        );
                    }
                    // Repo has a branch protection rule with this key, see if the values match!
                } else {
                    debug!(
                        "Repository {} has a 100% matching branch protection rule:\n{:?}\n{:?}",
                        repo_with_owner,
                        actual_branch_protection_rule,
                        expected_branch_protection_rule
                    );
                }
            } else {
                // Repo does *not* have a branch protection rule with this key, should we create it?
                // Maybe ask the user?
            }
        }
        */
    }

    fn check_topics(&self, ctx: &mut Context) -> Result<()> {
        if ctx.config.topics.is_none() {
            return Ok(());
        }
        let topics = ctx.config.topics.as_ref().unwrap();

        let mut final_topics = self.topics.names.clone();

        for operation in topics {
            match operation {
                TopicOperation::MustExist { name } => {
                    final_topics.insert(name.clone());
                }
                TopicOperation::MustNotExist { name } => {
                    final_topics.remove(name);
                }
                TopicOperation::Rename { old_name, name } => {
                    if final_topics.remove(old_name) {
                        final_topics.insert(name.clone());
                    }
                }
            }
        }

        if self.topics.names == final_topics {
            let gray = Style::new().color256(242);
            println!(
                "{}",
                gray.apply_to(format!(
                    "Checking repository {} topics - no changes needed",
                    self.info.full_name
                ))
            );
        } else if ctx.options.dry_run {
            println!(
                "Checking repository {} topics - add({:?}), del({:?}) (DRY RUN)",
                self.info.full_name,
                final_topics.difference(&self.topics.names),
                self.topics.names.difference(&final_topics),
            );
        } else {
            println!(
                "Checking repository {} topics - add({:?}), del({:?})",
                self.info.full_name,
                final_topics.difference(&self.topics.names),
                self.topics.names.difference(&final_topics),
            );
            ctx.api_client
                .update_repository_topics(self.info.full_name.as_str(), final_topics)?;
        }

        Ok(())
    }

    fn check_settings(&self, ctx: &mut Context) -> Result<()> {
        if ctx.config.settings.is_none() {
            return Ok(());
        }
        let settings = ctx.config.settings.as_ref().unwrap();

        let gray = Style::new().color256(242);
        let repo_name = self.info.name.as_str();
        let repo_owner = self.info.owner.login.as_str();
        let repo_with_owner = self.info.full_name.as_str();

        let result = settings.diff(&self.info);

        if result.empty() {
            ctx.terminal.clear_line()?;
            ctx.terminal.write_all(
                gray.apply_to(format!(
                    "Checking repository {repo_with_owner} settings - nothing to change\n"
                ))
                .to_string()
                .as_bytes(),
            )?;
        } else {
            // Update repository settings
            let patch = result.dump_patch();
            if !patch.is_empty() {
                let patch_size = patch.len();
                if ctx.options.dry_run {
                    debug!(
                        "DRY RUN: Update repository {} settings with patch {:?}",
                        repo_with_owner, patch
                    );
                    update_line(
                        &mut ctx.terminal,
                        format!(
                            "Checking repository {} settings - found {} differing settings (DRY RUN)\n",
                            repo_with_owner,
                            style(patch_size).cyan()
                        ),
                    );

                    for (k, v) in &patch {
                        println!("    Set {k} to {v}");
                    }
                } else {
                    update_line(
                        &mut ctx.terminal,
                        format!(
                            "Checking repository {} settings - found {} differing settings",
                            repo_with_owner,
                            style(patch_size).cyan()
                        ),
                    );
                    let response = ctx
                        .api_client
                        .update_repository_settings(repo_owner, repo_name, patch)?;
                    debug!("Response: {:?}", response);
                    update_line(
                        &mut ctx.terminal,
                        format!(
                            "Checking repository {} settings - updated {} differing settings\n",
                            repo_with_owner,
                            style(patch_size).cyan()
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    fn check_repository(&self, ctx: &mut Context) -> Result<()> {
        // self.check_branch_protection_rules(ctx)?;

        self.check_topics(ctx)?;

        self.check_settings(ctx)?;

        Ok(())
    }
}

// fn convert_branch_protection_rules_to_hashmap(
//     branch_protection_rules: Option<Vec<Option<GQLBranchProtectionRules>>>,
// ) -> Result<HashMap<String, GQLBranchProtectionRules>, anyhow::Error> {
//     let mut rule_map = HashMap::new();
//
//     if let Some(actual_branch_protection_rules) = branch_protection_rules {
//         for rule in actual_branch_protection_rules.into_iter().flatten() {
//             rule_map.insert(rule.pattern.clone(), rule);
//         }
//     }
//
//     Ok(rule_map)
// }
