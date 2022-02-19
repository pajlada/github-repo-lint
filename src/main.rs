use ::reqwest::blocking::Client;
use clap::{Arg, Command};
use console::Term;

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
                .default_value("config.json")
                .takes_value(true),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("fix")
                .long("fix")
                .help("Try to fix the issues found"),
        )
        .arg(
            Arg::new("repo")
                .long("repo")
                .multiple_occurrences(true)
                .help("Target GitHub repository")
                .takes_value(true),
        )
        .arg(
            Arg::new("user")
                .long("user")
                .multiple_occurrences(true)
                .help("Target GitHub user")
                .takes_value(true),
        )
        .arg(
            Arg::new("organization")
                .long("organization")
                .alias("org")
                .multiple_occurrences(true)
                .help("Target GitHub organization")
                .takes_value(true),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .format_timestamp(None)
        .format_target(false)
        .filter_module(module_path!(), log_level)
        .init();

    let repos: Vec<&str> = matches.values_of("repo").unwrap_or_default().collect();
    let users: Vec<&str> = matches.values_of("user").unwrap_or_default().collect();
    let organizations: Vec<&str> = matches
        .values_of("organization")
        .unwrap_or_default()
        .collect();

    let config_path = Path::new(matches.value_of("config").expect("value MUST be set"));

    let config = config::load_config(config_path)?;

    let github_api_token: String = match std::env::var("GITHUB_API_TOKEN") {
        Ok(s) => s,
        Err(_) => return Err(anyhow::anyhow!("xD")),
    };

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
        dry_run: !matches.is_present("fix"),
        dry_run_bpr: !matches.is_present("fix"),
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
