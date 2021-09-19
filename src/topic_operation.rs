use serde::Deserialize;

pub type TopicOperations = Vec<TopicOperation>;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum TopicOperation {
    MustExist { name: String },
    MustNotExist { name: String },
    Rename { old_name: String, name: String },
}
