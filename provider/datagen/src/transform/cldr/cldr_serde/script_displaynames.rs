// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files for scripts.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/scripts.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Scripts {
    pub scripts: HashMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub localedisplaynames: Scripts,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, LangDisplayNames>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
