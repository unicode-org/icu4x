use icu_locale::LanguageIdentifier;
use serde::{Deserialize, Deserializer};

// TODO: Make this non-pub
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CldrLanguage(pub LanguageIdentifier);

impl<'de> Deserialize<'de> for CldrLanguage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CldrLanguageVisitor;

        impl<'de> serde::de::Visitor<'de> for CldrLanguageVisitor {
            type Value = CldrLanguage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a valid Unicode Language Identifier or 'root'")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if s == "root" {
                    Ok(CldrLanguage("und".parse().unwrap()))
                } else {
                    s.parse::<LanguageIdentifier>()
                        .map(|v| CldrLanguage(v))
                        .map_err(serde::de::Error::custom)
                }
            }
        }

        deserializer.deserialize_string(CldrLanguageVisitor)
    }
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let fr = serde_json::from_str::<CldrLanguage>(r#""fr""#)?;
    let en = serde_json::from_str::<CldrLanguage>(r#""en-US""#)?;
    let root = serde_json::from_str::<CldrLanguage>(r#""root""#)?;

    assert_eq!(fr, CldrLanguage("fr".parse()?));
    assert_eq!(en, CldrLanguage("en-US".parse()?));
    assert_eq!(root, CldrLanguage("und".parse()?));

    let failed = serde_json::from_str::<CldrLanguage>(r#""2Xs""#);
    assert!(failed.is_err());
    let err = failed.unwrap_err();
    assert!(err.is_data());
    assert_eq!(
        err.to_string(),
        "The given language subtag is invalid at line 1 column 5".to_string()
    );

    Ok(())
}
