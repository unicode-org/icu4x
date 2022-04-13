// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::subtags::{Language, Region, Script, Variant};
use crate::LanguageIdentifier;
use alloc::string::ToString;
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

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LanguageVisitor;

        impl<'de> serde::de::Visitor<'de> for LanguageVisitor {
            type Value = Language;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "a valid BCP-47 language")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<Language>().map_err(serde::de::Error::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_string(LanguageVisitor)
        } else {
            Self::try_from_raw(Deserialize::deserialize(deserializer)?)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScriptVisitor;

        impl<'de> serde::de::Visitor<'de> for ScriptVisitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "a valid BCP-47 script")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<Script>().map_err(serde::de::Error::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_string(ScriptVisitor)
        } else {
            Self::try_from_raw(Deserialize::deserialize(deserializer)?)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegionVisitor;

        impl<'de> serde::de::Visitor<'de> for RegionVisitor {
            type Value = Region;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "a valid BCP-47 region")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<Region>().map_err(serde::de::Error::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_string(RegionVisitor)
        } else {
            Self::try_from_raw(Deserialize::deserialize(deserializer)?)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VariantVisitor;

        impl<'de> serde::de::Visitor<'de> for VariantVisitor {
            type Value = Variant;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "a valid BCP-47 variant")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<Variant>().map_err(serde::de::Error::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_string(VariantVisitor)
        } else {
            Self::try_from_raw(Deserialize::deserialize(deserializer)?)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[test]
fn json() {
    use crate::{langid, language, region, script};

    assert_eq!(
        serde_json::to_string(&langid!("en-US")).unwrap(),
        r#""en-US""#
    );
    assert_eq!(
        serde_json::from_str::<LanguageIdentifier>(r#""en-US""#).unwrap(),
        langid!("en-US")
    );
    assert!(serde_json::from_str::<LanguageIdentifier>(r#""2Xs""#).is_err());

    assert_eq!(serde_json::to_string(&language!("fr")).unwrap(), r#""fr""#);
    assert_eq!(
        serde_json::from_str::<Language>(r#""fr""#).unwrap(),
        language!("fr")
    );
    assert!(serde_json::from_str::<Language>(r#""2Xs""#).is_err());

    assert_eq!(
        serde_json::to_string(&script!("Latn")).unwrap(),
        r#""Latn""#
    );
    assert_eq!(
        serde_json::from_str::<Script>(r#""Latn""#).unwrap(),
        script!("Latn")
    );
    assert!(serde_json::from_str::<Script>(r#""2Xs""#).is_err());

    assert_eq!(serde_json::to_string(&region!("US")).unwrap(), r#""US""#);
    assert_eq!(
        serde_json::from_str::<Region>(r#""US""#).unwrap(),
        region!("US")
    );
    assert!(serde_json::from_str::<Region>(r#""2Xs""#).is_err());
}

#[test]
fn postcard() {
    use crate::{langid, language, region, script};

    assert_eq!(
        postcard::to_stdvec(&langid!("en-US")).unwrap(),
        &[5, b'e', b'n', b'-', b'U', b'S']
    );
    assert_eq!(
        postcard::from_bytes::<LanguageIdentifier>(&[5, b'e', b'n', b'-', b'U', b'S']).unwrap(),
        langid!("en-US")
    );
    assert!(postcard::from_bytes::<LanguageIdentifier>(&[3, b'2', b'X', b's']).is_err());

    assert_eq!(postcard::to_stdvec(&language!("fr")).unwrap(), b"fr\0");
    assert_eq!(
        postcard::from_bytes::<Language>(b"fr\0").unwrap(),
        language!("fr")
    );
    assert!(postcard::from_bytes::<Language>(b"2Xs").is_err());

    assert_eq!(postcard::to_stdvec(&script!("Latn")).unwrap(), b"Latn");
    assert_eq!(
        postcard::from_bytes::<Script>(b"Latn").unwrap(),
        script!("Latn")
    );
    assert!(postcard::from_bytes::<Script>(b"2Xss").is_err());

    assert_eq!(postcard::to_stdvec(&region!("US")).unwrap(), b"US\0");
    assert_eq!(
        postcard::from_bytes::<Region>(b"US\0").unwrap(),
        region!("US")
    );
    assert!(postcard::from_bytes::<Region>(b"2Xs").is_err());
}
