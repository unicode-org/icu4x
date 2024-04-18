// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON exemplar character files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/de/characters.json>

use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct LocaleExemplarCharacters {
    #[serde(rename = "exemplarCharacters")]
    pub(in crate::provider) main: Option<String>,
    pub(in crate::provider) auxiliary: Option<String>,
    pub(in crate::provider) index: Option<String>,
    pub(in crate::provider) numbers: Option<String>,
    pub(in crate::provider) punctuation: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Characters {
    pub(in crate::provider) characters: LocaleExemplarCharacters,
}

pub(in crate::provider) type Resource = super::LocaleResource<Characters>;
