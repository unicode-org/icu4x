// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::LanguageIdentifier;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for LanguageIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for LanguageIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LanguageIdentifierVisitor;

        impl<'de> serde::de::Visitor<'de> for LanguageIdentifierVisitor {
            type Value = LanguageIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a valid Unicode Language Identifier")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<LanguageIdentifier>()
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_string(LanguageIdentifierVisitor)
    }
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let fr = serde_json::json!("fr".parse::<LanguageIdentifier>()?);
    let en = serde_json::json!("en-US".parse::<LanguageIdentifier>()?);

    assert_eq!(serde_json::to_string(&fr)?, r#""fr""#);
    assert_eq!(serde_json::to_string(&en)?, r#""en-US""#);

    Ok(())
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let fr = serde_json::from_str::<LanguageIdentifier>(r#""fr""#)?;
    let en = serde_json::from_str::<LanguageIdentifier>(r#""en-US""#)?;

    assert_eq!(fr, "fr".parse::<LanguageIdentifier>()?);
    assert_eq!(en, "en-US".parse::<LanguageIdentifier>()?);

    let failed = serde_json::from_str::<LanguageIdentifier>(r#""2Xs""#);
    assert!(failed.is_err());
    let err = failed.unwrap_err();
    assert!(err.is_data());
    assert_eq!(
        err.to_string(),
        "The given language subtag is invalid at line 1 column 5".to_string()
    );

    Ok(())
}
