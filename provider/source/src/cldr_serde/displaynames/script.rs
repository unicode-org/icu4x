// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files for scripts.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/scripts.json>

use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Scripts {
    pub(crate) scripts: HashMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub(crate) localedisplaynames: Scripts,
}

pub(crate) type Resource = super::super::LocaleResource<LangDisplayNames>;
