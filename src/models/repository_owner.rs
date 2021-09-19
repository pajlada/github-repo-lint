use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RepositoryOwner {
    pub login: String,

    #[serde(rename = "type")]
    pub owner_type: String,
}
