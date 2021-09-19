use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::io::Read;

use log::*;

use crate::models::RepositoryInfo;

macro_rules! ensure_same {
    ($s:ident, $r:ident, $field_name:ident) => {
        if let Some(expected) = $s.$field_name {
            if let Some(actual) = $r.$field_name {
                // info!(
                //     "Comparing ({}) e:{} to a:{}",
                //     stringify!($field_name),
                //     expected,
                //     actual
                // );
                if expected != actual {
                    Some(expected)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
}

macro_rules! define_repository_settings {
    ( $( $field_name:ident : $field_type:ty, )* ) => {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct RepositorySettings {
            $(
                pub $field_name: $field_type,
            )*
        }

        impl RepositorySettings {
            // fn dump(&self) {
            //     $(
            //         println!("{} = {:?}", stringify!($field_name), self.$field_name);
            //     )*
            // }

            pub fn dump_patch(&self) -> HashMap<&str, serde_json::Value> {
                let mut map = HashMap::new();

                $(
                    if let Some(v) = self.$field_name {
                        map.insert(stringify!($field_name), json!(v));
                    }
                )*

                    return map;
            }

            pub fn empty(&self) -> bool {
                $(
                    if self.$field_name.is_some() {
                        return false;
                    }
                )*

                return true;
            }

            pub fn diff(&self, repository: &RepositoryInfo) -> RepositorySettings {
                RepositorySettings {
                $(
                    $field_name: ensure_same!(self, repository, $field_name),
                )*
                }
            }

            // diff is used instead
            // fn compare(&self, repository: get_repositories_from_user::GetRepositoriesFromUserUserRepositoriesNodes) {
            //     $(
            //         if let Some(expected) = self.$field_name {
            //             if expected != repository.$field_name {
            //                 println!(
            //                     "Difference for {}: {:?} vs {:?}",
            //                     stringify!($field_name),
            //                     expected,
            //                     repository.$field_name,
            //                 );
            //             }
            //         }
            //     )*
            // }
        }
    }
}

impl RepositorySettings {
    fn load<R>(rdr: R) -> Result<RepositorySettings, anyhow::Error>
    where
        R: Read,
    {
        let settings = serde_json::from_reader(rdr)?;

        Ok(settings)
    }
}

define_repository_settings! {
    allow_auto_merge : Option<bool>,
    has_issues : Option<bool>,
    has_projects : Option<bool>,
    has_wiki : Option<bool>,
    allow_merge_commit : Option<bool>,
    allow_squash_merge : Option<bool>,
    allow_rebase_merge : Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_repository_settings() -> Result<(), anyhow::Error> {
        let contents = r#"
{
  "auto_merge_allowed": true,
  "has_issues_enabled": null,
  "has_projects_enabled": false,
  "has_wiki_enabled": null,
  "merge_commit_allowed": false,
  "squash_merge_allowed": true,
  "rebase_merge_allowed": false
}"#;
        let reader = std::io::Cursor::new(contents);
        let settings = RepositorySettings::load(reader)?;
        assert_ne!(
            settings,
            RepositorySettings {
                // auto_merge_allowed: Some(false),
                has_issues_enabled: None,
                has_projects_enabled: Some(false),
                has_wiki_enabled: None,
                merge_commit_allowed: Some(false),
                squash_merge_allowed: Some(true),
                rebase_merge_allowed: Some(false),
            }
        );
        assert_eq!(
            settings,
            RepositorySettings {
                // auto_merge_allowed: Some(true),
                has_issues_enabled: None,
                has_projects_enabled: Some(false),
                has_wiki_enabled: None,
                merge_commit_allowed: Some(false),
                squash_merge_allowed: Some(true),
                rebase_merge_allowed: Some(false),
            }
        );

        Ok(())
    }
}
