// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale::LanguageIdentifier;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use tinystr::TinyStr8;

/// A struct similar to LanguageIdentifier that supports "root"
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct CldrLangID {
    /// CLDR LanguageIdentifier (root => "root")
    cldr_language: TinyStr8,

    /// Normalized LanguageIdentifier (root => "und")
    pub langid: LanguageIdentifier,
}

impl CldrLangID {
    /// Return the CldrLangID for "root"
    pub fn root() -> Self {
        // TODO: Use LanguageIdentifier::default()
        Self {
            cldr_language: "root".parse().unwrap(),
            langid: LanguageIdentifier::default(),
        }
    }
}

impl From<LanguageIdentifier> for CldrLangID {
    /// Return a CldrLangID for a generic LanguageIdentifier. "und" becomes "root".
    fn from(langid: LanguageIdentifier) -> Self {
        if langid == LanguageIdentifier::from_str("und").unwrap() {
            Self::root()
        } else {
            Self {
                cldr_language: langid.language.as_str().parse().unwrap(),
                langid,
            }
        }
    }
}

impl FromStr for CldrLangID {
    type Err = <LanguageIdentifier as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "root" {
            Ok(Self::root())
        } else if s == "und" {
            // Reject "und" since we want a 1-to-1 mapping from strings to structs
            Err(Self::Err::InvalidLanguage)
        } else {
            s.parse::<LanguageIdentifier>().map(|langid| langid.into())
        }
    }
}

impl<'de> Deserialize<'de> for CldrLangID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CldrLanguageVisitor;

        impl<'de> serde::de::Visitor<'de> for CldrLanguageVisitor {
            type Value = CldrLangID;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a valid Unicode Language Identifier or 'root'")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_string(CldrLanguageVisitor)
    }
}

#[test]
fn test_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    use icu_locale_macros::langid;

    let fr = serde_json::from_str::<CldrLangID>(r#""fr""#)?;
    let en = serde_json::from_str::<CldrLangID>(r#""en-US""#)?;
    let root = serde_json::from_str::<CldrLangID>(r#""root""#)?;

    assert_eq!(
        fr,
        CldrLangID {
            cldr_language: "fr".parse().unwrap(),
            langid: langid!("fr"),
        }
    );
    assert_eq!(
        en,
        CldrLangID {
            cldr_language: "en".parse().unwrap(),
            langid: langid!("en-US"),
        }
    );
    assert_eq!(
        root,
        CldrLangID {
            cldr_language: "root".parse().unwrap(),
            langid: langid!("und"),
        }
    );

    let failed = serde_json::from_str::<CldrLangID>(r#""2Xs""#);
    assert!(failed.is_err());
    let err = failed.unwrap_err();
    assert!(err.is_data());
    assert_eq!(
        err.to_string(),
        "The given language subtag is invalid at line 1 column 5".to_string()
    );

    Ok(())
}

/// Assert that CLDR order matches Ord on CldrLangID
#[test]
fn test_order() {
    let cldr_strings = [
        "ar",    //
        "ar-EG", //
        "ars",   //
        "ro",    //
        "ro-RO", //
        "rof",   //
        "root",  //
        "ru",    //
        "zh-CN", //
    ];
    let mut cldr_strings_sorted: Vec<&str> = cldr_strings.iter().copied().collect();
    cldr_strings_sorted.sort_unstable();
    assert_eq!(cldr_strings[..], cldr_strings_sorted[..]);

    let cldr_langids: Vec<CldrLangID> = cldr_strings.iter().map(|s| s.parse().unwrap()).collect();
    let cldr_langids_sorted: Vec<CldrLangID> = cldr_langids.iter().map(|s| (*s).clone()).collect();
    assert_eq!(cldr_langids, cldr_langids_sorted);
}

/// Assert that "root" and "und" are equivalent
#[test]
fn test_und_root() {
    CldrLangID::from_str("und").expect_err("und should not be allowed as a string");

    let und = CldrLangID::from(LanguageIdentifier::default());
    let root = CldrLangID::from_str("root").unwrap();
    assert_eq!(und, root);
}
