pub mod graphql;
pub mod rest;

use ::reqwest::blocking::Client as r_client;
use ::reqwest::Url;

pub struct Client {
    client: r_client,
    api_root: Url,
}

pub fn new<'a>(client: r_client, api_root: &str) -> Result<Client, anyhow::Error> {
    Ok(Client {
        client,
        api_root: Url::parse(api_root)?,
    })
}
