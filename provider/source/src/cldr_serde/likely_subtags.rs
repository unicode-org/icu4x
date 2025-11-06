// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON likelySubtags.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/likelySubtags.json>

use icu::locale::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "likelySubtags")]
    pub(crate) likely_subtags: HashMap<LanguageIdentifier, LanguageIdentifier>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
