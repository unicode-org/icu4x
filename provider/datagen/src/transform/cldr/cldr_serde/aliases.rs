// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON aliases.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/aliases.json>

use serde::Deserialize;
use std::collections::HashMap;
use tinystr::TinyAsciiStr;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Replacement<T> {
    #[serde(rename = "_replacement")]
    pub replacement: T,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Alias {
    #[serde(rename = "languageAlias")]
    pub language_aliases: HashMap<String, Replacement<String>>,
    #[serde(rename = "scriptAlias")]
    pub script_aliases: HashMap<TinyAsciiStr<4>, Replacement<TinyAsciiStr<4>>>,
    #[serde(rename = "territoryAlias")]
    pub region_aliases: HashMap<TinyAsciiStr<3>, Replacement<String>>,
    #[serde(rename = "variantAlias")]
    pub variant_aliases: HashMap<TinyAsciiStr<8>, Replacement<TinyAsciiStr<8>>>,
    #[serde(rename = "subdivisionAlias")]
    pub subdivision_aliases: HashMap<TinyAsciiStr<7>, Replacement<String>>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Metadata {
    pub alias: Alias,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    pub metadata: Metadata,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
