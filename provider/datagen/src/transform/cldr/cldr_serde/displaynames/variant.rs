// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files for variants.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/variants.json>

use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrEmptyMap {
    String(String),
    // CLDR-JSON 44 contains some empty maps in place of strings in the
    // variants table. This might be a bug. Track progress:
    // <https://unicode-org.atlassian.net/browse/CLDR-17171>
    Empty {},
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Variants {
    pub variants: HashMap<String, StringOrEmptyMap>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub localedisplaynames: Variants,
}

pub type Resource = super::super::LocaleResource<LangDisplayNames>;
