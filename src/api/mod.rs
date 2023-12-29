mod pagination;

mod get_repositories;

mod branch_protection;
mod update_repository_settings;
mod update_repository_topics;

use reqwest::blocking::Client as r_client;
use reqwest::Url;

pub struct Client {
    client: r_client,
    api_root: Url,
}

pub fn new(client: r_client, api_root: &str) -> Result<Client, anyhow::Error> {
    Ok(Client {
        client,
        api_root: Url::parse(api_root)?,
    })
}
