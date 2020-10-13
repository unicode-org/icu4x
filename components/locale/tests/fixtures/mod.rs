// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use icu_locale::extensions::private;
use icu_locale::extensions::transform;
use icu_locale::extensions::unicode;
use icu_locale::extensions::Extensions;
use icu_locale::{subtags, LanguageIdentifier, Locale, ParserError};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LocaleIdentifier {
    #[serde(rename = "type")]
    pub field_type: String,
    pub identifier: String,
}

#[derive(Deserialize, Clone)]
pub struct LocaleExtensionUnicode {
    #[serde(default)]
    keywords: HashMap<String, Option<String>>,
    #[serde(default)]
    attributes: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct LocaleExtensionTransform {
    tlang: Option<String>,
    #[serde(default)]
    tfields: HashMap<String, Option<String>>,
}

#[derive(Deserialize, Clone)]
pub struct LocaleExtensions {
    unicode: Option<LocaleExtensionUnicode>,
    transform: Option<LocaleExtensionTransform>,
    #[serde(default)]
    private: Vec<String>,
    other: Option<String>,
}

impl TryFrom<LocaleExtensions> for Extensions {
    type Error = ParserError;

    fn try_from(input: LocaleExtensions) -> Result<Self, Self::Error> {
        let mut ext = Extensions::default();
        if let Some(unicode) = input.unicode {
            let mut v: Vec<(unicode::Key, unicode::Value)> = unicode
                .keywords
                .iter()
                .map(|(k, v)| {
                    (
                        unicode::Key::from_bytes(k.as_bytes()).expect("Parsing key failed."),
                        v.as_ref().map_or(
                            unicode::Value::from_bytes(b"").expect("Failed to parse Value"),
                            |v| {
                                unicode::Value::from_bytes(v.as_bytes())
                                    .expect("Parsing type failed.")
                            },
                        ),
                    )
                })
                .collect();
            v.sort_by_key(|i| i.0);
            ext.unicode.keywords = unicode::Keywords::from_vec_unchecked(v);
            let v: Vec<unicode::Attribute> = unicode
                .attributes
                .iter()
                .map(|v| {
                    unicode::Attribute::from_bytes(v.as_bytes()).expect("Parsing attribute failed.")
                })
                .collect();
            ext.unicode.attributes = unicode::Attributes::from_vec_unchecked(v);
        }
        if let Some(transform) = input.transform {
            let mut v: Vec<(transform::Key, transform::Value)> = transform
                .tfields
                .iter()
                .map(|(k, v)| {
                    (
                        transform::Key::from_bytes(k.as_bytes()).expect("Parsing key failed."),
                        v.as_ref()
                            .map(|v| {
                                transform::Value::from_bytes(v.as_bytes())
                                    .expect("Parsing value failed.")
                            })
                            .expect("Value cannot be empty."),
                    )
                })
                .collect();
            v.sort_by_key(|i| i.0);
            ext.transform.fields = transform::Fields::from_vec_unchecked(v);

            if let Some(tlang) = transform.tlang {
                ext.transform.lang = Some(tlang.parse().expect("Failed to parse tlang."));
            }
        }
        let v: Vec<private::Key> = input
            .private
            .iter()
            .map(|v| private::Key::from_bytes(v.as_bytes()).expect("Failed to add field."))
            .collect();
        ext.private = private::Private::from_vec_unchecked(v);
        Ok(ext)
    }
}

#[derive(Deserialize, Clone)]
pub struct LocaleSubtags {
    #[serde(rename = "type")]
    pub field_type: String,
    pub language: Option<String>,
    pub script: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    pub variants: Vec<String>,
    pub extensions: Option<LocaleExtensions>,
}

#[derive(Deserialize, Clone)]
pub struct LocaleError {
    pub error: String,
    pub text: String,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)] // test code
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

impl TryFrom<LocaleInfo> for Locale {
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

impl TryFrom<LocaleIdentifier> for Locale {
    type Error = ParserError;

    fn try_from(input: LocaleIdentifier) -> Result<Self, Self::Error> {
        Locale::from_bytes(input.identifier.as_bytes())
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
        let variants = subtags
            .variants
            .iter()
            .map(|v| v.parse().expect("Failed to parse variant subtag."))
            .collect::<Vec<_>>();
        Ok(LanguageIdentifier {
            language,
            script,
            region,
            variants: subtags::Variants::from_vec_unchecked(variants),
        })
    }
}

impl TryFrom<LocaleSubtags> for Locale {
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
        let variants = subtags
            .variants
            .iter()
            .map(|v| v.parse().expect("Failed to parse variant subtag."))
            .collect::<Vec<_>>();
        let extensions = if let Some(e) = subtags.extensions {
            e.try_into().expect("Failed to parse extensions.")
        } else {
            Extensions::default()
        };
        Ok(Locale {
            language,
            script,
            region,
            variants: subtags::Variants::from_vec_unchecked(variants),
            extensions,
        })
    }
}

impl From<LocaleError> for ParserError {
    fn from(e: LocaleError) -> Self {
        match e.error.as_str() {
            "InvalidLanguage" => ParserError::InvalidLanguage,
            "InvalidSubtag" => ParserError::InvalidSubtag,
            "InvalidExtension" => ParserError::InvalidExtension,
            _ => unreachable!("Unknown error name"),
        }
    }
}

#[derive(Deserialize)]
pub struct LocaleTest {
    pub input: LocaleInfo,
    pub output: LocaleInfo,
}
