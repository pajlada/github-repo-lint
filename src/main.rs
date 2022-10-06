use clap::{Arg, ArgAction, Command};
use console::Term;
use reqwest::blocking::Client;

use const_format::formatcp;
use log::*;
use std::path::Path;

mod api;
mod app;
mod branch_protection_rules;
mod config;
mod context;
mod models;
mod options;
mod repository_settings;
mod topic_operation;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const USER_AGENT: &str = formatcp!("{}/{}", PKG_NAME, PKG_VERSION);

fn main() -> Result<(), anyhow::Error> {
    let terminal = Term::stdout();

    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to config file to use")
                .default_value("config.json"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("fix")
                .long("fix")
                .action(ArgAction::SetTrue)
                .help("Try to fix the issues found"),
        )
        .arg(
            Arg::new("repo")
                .long("repo")
                .action(ArgAction::Append)
                .help("Target GitHub repository"),
        )
        .arg(
            Arg::new("user")
                .long("user")
                .action(ArgAction::Append)
                .help("Target GitHub user"),
        )
        .arg(
            Arg::new("organization")
                .long("organization")
                .alias("org")
                .action(ArgAction::Append)
                .help("Target GitHub organization"),
        )
        .get_matches();

    let log_level = match matches.get_count("verbose") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .format_timestamp(None)
        .format_target(false)
        .filter_module(module_path!(), log_level)
        .init();

    let repos: Vec<&str> = matches
        .get_many::<String>("repo")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();
    let users: Vec<&str> = matches
        .get_many::<String>("user")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();
    let organizations: Vec<&str> = matches
        .get_many::<String>("organization")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let config_path = Path::new(matches.get_one::<String>("config").unwrap());

    let config = config::load_config(config_path)?;

    let github_api_token = std::env::var("GITHUB_API_TOKEN").map_err(|_| {
        anyhow::anyhow!(
            "Missing GitHub token, must be defined with the GITHUB_API_TOKEN environment variable."
        )
    })?;

    info!("Github API root: {:?}", config.github_api_root);

    let client = Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))?,
            ))
            .collect(),
        )
        .build()?;

    let api_client = api::new(client, config.github_api_root.as_str())?;

    let options = options::Options {
        dry_run: !matches.get_flag("fix"),
        dry_run_bpr: !matches.get_flag("fix"),
    };

    let ctx = context::Context {
        config,
        terminal,
        api_client,
        options,
    };

    // info!("Config: {:?}", ctx.config.topics);

    app::run(ctx, repos, users, organizations)?;

    // terminal.write_line("")?;

    Ok(())
}
