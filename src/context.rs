use console::Term;

use crate::api;
use crate::config::Config;
use crate::options::Options;

pub struct Context {
    pub config: Config,
    pub terminal: Term,
    pub api_client: api::Client,
    pub options: Options,
}
