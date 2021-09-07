pub mod graphql;
pub mod rest;

use ::reqwest::blocking::Client as r_client;
use ::reqwest::Url;

pub struct Client<'a> {
    client: &'a r_client,
    api_root: Url,
}

pub fn new<'a>(client: &'a r_client, api_root: &str) -> Result<Client<'a>, anyhow::Error> {
    Ok(Client {
        client,
        api_root: Url::parse(api_root)?,
    })
}
