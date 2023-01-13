// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/territories.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Regions {
    #[serde(rename = "territories")]
    pub regions: HashMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub localedisplaynames: Regions,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, LangDisplayNames>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
