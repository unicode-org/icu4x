// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/territories.json>

use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Regions {
    #[serde(rename = "territories")]
    pub(crate) regions: HashMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub(crate) localedisplaynames: Regions,
}

pub(crate) type Resource = super::super::LocaleResource<LangDisplayNames>;
