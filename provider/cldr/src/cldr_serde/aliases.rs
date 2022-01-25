// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON aliases.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/aliases.json>

use serde::Deserialize;
use tinystr::{TinyStr4, TinyStr8};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Replacement<T> {
    #[serde(rename = "_replacement")]
    pub replacement: T,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Alias {
    #[serde(with = "tuple_vec_map", rename = "languageAlias")]
    pub language_aliases: Vec<(String, Replacement<String>)>,
    #[serde(with = "tuple_vec_map", rename = "scriptAlias")]
    pub script_aliases: Vec<(TinyStr4, Replacement<TinyStr4>)>,
    #[serde(with = "tuple_vec_map", rename = "territoryAlias")]
    pub region_aliases: Vec<(TinyStr4, Replacement<String>)>,
    #[serde(with = "tuple_vec_map", rename = "variantAlias")]
    pub variant_aliases: Vec<(TinyStr8, Replacement<TinyStr8>)>,
    #[serde(with = "tuple_vec_map", rename = "subdivisionAlias")]
    pub subdivision_aliases: Vec<(TinyStr8, Replacement<String>)>,
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
