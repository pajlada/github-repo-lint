use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::get_repositories_from_user::get_repositories_from_user;

macro_rules! ensure_same {
    ($s:ident, $r:ident, $field_name:ident) => {
        if let Some(expected) = $s.$field_name {
            let actual = $r.$field_name;
            if expected != actual {
                Some(expected)
            } else {
                None
            }
        } else {
            None
        }
    };
}

macro_rules! define_repository_settings {
    ( $( $xD:ident : $field_name:ident : $field_type:ty, )* ) => {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct RepositorySettings {
            $(
                $field_name: $field_type,
            )*
        }

        impl RepositorySettings {
            // fn dump(&self) {
            //     $(
            //         println!("{} = {:?}", stringify!($field_name), self.$field_name);
            //     )*
            // }

            pub fn dump_patch(&self) -> Option<HashMap<&str, serde_json::Value>> {
                let mut map = HashMap::new();

                $(
                    if let Some(v) = self.$field_name {
                        map.insert(stringify!($xD), json!(v));
                    }
                )*

                if !map.is_empty() {
                    return Some(map);
                }

                return None;
            }

            pub fn empty(&self) -> bool {
                $(
                    if self.$field_name.is_some() {
                        return false;
                    }
                )*

                return true;
            }

            pub fn diff(&self, repository: &get_repositories_from_user::GetRepositoriesFromUserUserRepositoriesNodes) -> RepositorySettings {
                return RepositorySettings {
                $(
                    $field_name: ensure_same!(self, repository, $field_name),
                )*
                };
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
    pub fn load<R>(rdr: R) -> Result<RepositorySettings, anyhow::Error>
    where
        R: Read,
    {
        let settings = serde_json::from_reader(rdr)?;

        Ok(settings)
    }
}

define_repository_settings! {
    allow_auto_merge : auto_merge_allowed: Option<bool>,
    has_issues : has_issues_enabled: Option<bool>,
    has_projects : has_projects_enabled: Option<bool>,
    has_wiki : has_wiki_enabled: Option<bool>,
    allow_merge_commit : merge_commit_allowed: Option<bool>,
    allow_squash_merge : squash_merge_allowed: Option<bool>,
    allow_rebase_merge : rebase_merge_allowed: Option<bool>,
}

pub fn load_from_file(path: &Path) -> Result<RepositorySettings, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    RepositorySettings::load(reader)
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
                auto_merge_allowed: Some(false),
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
                auto_merge_allowed: Some(true),
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
