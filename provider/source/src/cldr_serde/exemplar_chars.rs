// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON exemplar character files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/de/characters.json>

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct LocaleExemplarCharacters {
    #[serde(rename = "exemplarCharacters")]
    pub(crate) main: Option<String>,
    pub(crate) auxiliary: Option<String>,
    pub(crate) index: Option<String>,
    pub(crate) numbers: Option<String>,
    pub(crate) punctuation: Option<String>,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Characters {
    pub(crate) characters: LocaleExemplarCharacters,
}

pub(crate) type Resource = super::LocaleResource<Characters>;
