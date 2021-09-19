use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::io::Read;

use crate::models::RepositoryInfo;

macro_rules! ensure_same {
    ($s:ident, $r:ident, $field_name:ident) => {
        if let Some(expected) = $s.$field_name {
            if let Some(actual) = $r.$field_name {
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
    #[allow(dead_code)]
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
    fn test_load_repository_settings() -> anyhow::Result<()> {
        let contents = r#"
{
  "allow_auto_merge": true,
  "has_issues": null,
  "has_projects": false,
  "has_wiki": null,
  "allow_merge_commit": false,
  "allow_squash_merge": true,
  "allow_rebase_merge": false
}"#;
        let reader = std::io::Cursor::new(contents);
        let settings = RepositorySettings::load(reader)?;
        assert_ne!(
            settings,
            RepositorySettings {
                allow_auto_merge: Some(false),
                has_issues: None,
                has_projects: Some(false),
                has_wiki: None,
                allow_merge_commit: Some(false),
                allow_squash_merge: Some(true),
                allow_rebase_merge: Some(false),
            }
        );
        assert_eq!(
            settings,
            RepositorySettings {
                allow_auto_merge: Some(true),
                has_issues: None,
                has_projects: Some(false),
                has_wiki: None,
                allow_merge_commit: Some(false),
                allow_squash_merge: Some(true),
                allow_rebase_merge: Some(false),
            }
        );

        Ok(())
    }
}
