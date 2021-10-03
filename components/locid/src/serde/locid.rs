// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Locale;
use alloc::string::ToString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocaleVisitor;

        impl<'de> serde::de::Visitor<'de> for LocaleVisitor {
            type Value = Locale;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "a valid Unicode Locale Identifier")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<Locale>().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_string(LocaleVisitor)
    }
}

#[test]
fn test_locid_serde_serialize() -> Result<(), Box<dyn std::error::Error>> {
    let fr = serde_json::json!("fr".parse::<Locale>()?);
    let en = serde_json::json!("en-US".parse::<Locale>()?);
    let de_ch = serde_json::json!("de-CH-1901".parse::<Locale>()?);
    let fi_x = serde_json::json!("fi-x-school".parse::<Locale>()?);

    assert_eq!(serde_json::to_string(&fr)?, r#""fr""#);
    assert_eq!(serde_json::to_string(&en)?, r#""en-US""#);
    assert_eq!(serde_json::to_string(&de_ch)?, r#""de-CH-1901""#);
    assert_eq!(serde_json::to_string(&fi_x)?, r#""fi-x-school""#);

    Ok(())
}

#[test]
fn test_locid_serde_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let fr = serde_json::from_str::<Locale>(r#""fr""#)?;
    let en = serde_json::from_str::<Locale>(r#""en-US""#)?;
    let de_ch = serde_json::from_str::<Locale>(r#""de-CH-1901""#)?;
    let fi_x = serde_json::from_str::<Locale>(r#""fi-x-school""#)?;

    assert_eq!(fr, "fr".parse::<Locale>()?);
    assert_eq!(en, "en-US".parse::<Locale>()?);
    assert_eq!(de_ch, "de-CH-1901".parse::<Locale>()?);
    assert_eq!(fi_x, "fi-x-school".parse::<Locale>()?);

    let non_alphanumerical_ext = serde_json::from_str::<Locale>(r#""fi-x-f@il""#);
    assert!(non_alphanumerical_ext.is_err());
    let err = non_alphanumerical_ext.unwrap_err();
    assert!(err.is_data());
    assert_eq!(
        err.to_string(),
        "Invalid extension at line 1 column 11".to_string()
    );

    Ok(())
}
