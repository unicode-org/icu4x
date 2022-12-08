// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON exemplar character files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/de/characters.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct LocaleExemplarCharacters {
    #[serde(rename = "exemplarCharacters")]
    pub main: Option<String>,
    pub auxiliary: Option<String>,
    pub index: Option<String>,
    pub numbers: Option<String>,
    pub punctuation: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Characters {
    pub characters: LocaleExemplarCharacters,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, Characters>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
