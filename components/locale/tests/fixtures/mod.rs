use std::convert::{TryFrom, TryInto};

use icu_locale::{subtags, LanguageIdentifier, ParserError};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LocaleIdentifier {
    #[serde(rename = "type")]
    pub field_type: String,
    pub identifier: String,
}

#[derive(Deserialize, Clone)]
pub struct LocaleSubtags {
    #[serde(rename = "type")]
    pub field_type: String,
    pub language: Option<String>,
    pub script: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    pub variant: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct LocaleError {
    pub error: String,
    pub text: String,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum LocaleInfo {
    String(String),
    Error(LocaleError),
    Identifier(LocaleIdentifier),
    Object(LocaleSubtags),
}

impl TryFrom<LocaleInfo> for LanguageIdentifier {
    type Error = ParserError;

    fn try_from(input: LocaleInfo) -> Result<Self, Self::Error> {
        match input {
            LocaleInfo::String(s) => s.parse(),
            LocaleInfo::Error(e) => Err(e.into()),
            LocaleInfo::Identifier(ident) => ident.try_into(),
            LocaleInfo::Object(o) => o.try_into(),
        }
    }
}

impl TryFrom<LocaleIdentifier> for LanguageIdentifier {
    type Error = ParserError;

    fn try_from(input: LocaleIdentifier) -> Result<Self, Self::Error> {
        LanguageIdentifier::from_locale_bytes(input.identifier.as_bytes())
    }
}

impl TryFrom<LocaleSubtags> for LanguageIdentifier {
    type Error = ParserError;

    fn try_from(subtags: LocaleSubtags) -> Result<Self, Self::Error> {
        let language = if let Some(lang) = subtags.language {
            lang.parse().expect("Failed to parse language subtag")
        } else {
            subtags::Language::default()
        };
        let script = subtags
            .script
            .map(|s| s.parse().expect("Failed to parse script subtag."));
        let region = subtags
            .region
            .map(|s| s.parse().expect("Failed to parse region subtag."));
        let variant = subtags
            .variant
            .map(|v| v.parse().expect("Failed to parse variant subtag."));
        Ok(LanguageIdentifier {
            language,
            script,
            region,
            variant,
        })
    }
}

impl From<LocaleError> for ParserError {
    fn from(e: LocaleError) -> Self {
        match e.error.as_str() {
            "InvalidLanguage" => ParserError::InvalidLanguage,
            "InvalidSubtag" => ParserError::InvalidSubtag,
            _ => unreachable!("Unknown error name"),
        }
    }
}

#[derive(Deserialize)]
pub struct LocaleTest {
    pub input: LocaleInfo,
    pub output: LocaleInfo,
}
