use core::fmt;

use serde::de;
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::Deserializer;

pub(crate) fn optionally_enabled<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct FieldVisitor;

    impl<'de> Visitor<'de> for FieldVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct with optional enabled bool key")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Option<bool>, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut v = None;
            while let Some(key) = access.next_key::<&str>()? {
                if key == "enabled" {
                    // return access.next_value();
                    v = access.next_value()?;
                } else {
                    access.next_value::<IgnoredAny>()?;
                }
            }
            Ok(v)
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Assume this is serde_json's null
            Ok(None)
        }
    }

    deserializer.deserialize_any(FieldVisitor)
}

#[cfg(test)]
mod tests {
    use super::optionally_enabled;
    use rstest::rstest;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Eq, PartialEq)]
    pub struct OptionallyEnabledTest {
        #[serde(deserialize_with = "optionally_enabled")]
        v: Option<bool>,
    }

    #[rstest]
    #[case(r#"{"v":null}"#, None)]
    #[case(r#"{"v":false}"#, Some(false))]
    #[case(r#"{"v":true}"#, Some(true))]
    #[case(r#"{"v":{"enabled": null}}"#, None)]
    #[case(r#"{"v":{"enabled": false}}"#, Some(false))]
    #[case(r#"{"v":{"enabled": true}}"#, Some(true))]
    #[case(r#"{"v":{"enabled": true, "foo": "bar"}}"#, Some(true))]
    #[case(r#"{"v":{"foo": "bar", "enabled": true}}"#, Some(true))]
    #[case(r#"{"v":{"enabled": true, "foo": ["bar"]}}"#, Some(true))]
    #[case(r#"{"v":{"foo": ["bar"], "enabled": true}}"#, Some(true))]
    fn test_optionally_enabled_ok(
        #[case] input: &'static str,
        #[case] expected: Option<bool>,
    ) -> anyhow::Result<()> {
        let actual: OptionallyEnabledTest = serde_json::from_str(input)?;
        assert_eq!(actual.v, expected);
        Ok(())
    }

    #[rstest]
    #[case(r#"{"v":"asd"}"#)]
    #[case(r#"{"v":-1}"#)]
    #[case(r#"{"v":1}"#)]
    #[case(r#"{"v":-1.0}"#)]
    #[case(r#"{"v":1.0}"#)]
    #[case(r#"{"v":[]}"#)]
    #[case(r#"{"v":[true]}"#)]
    #[case(r#"{"v":[null]}"#)]
    #[case(r#"{"v":["asd"]}"#)]
    #[case(r#"{"v":[5]}"#)]
    fn test_optionally_enabled_err(#[case] input: &'static str) {
        let r: serde_json::Result<OptionallyEnabledTest> = serde_json::from_str(input);
        assert!(r.is_err());
    }
}
