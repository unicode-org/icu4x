// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON likelySubtags.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/likelySubtags.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(with = "tuple_vec_map", rename = "likelySubtags")]
    pub likely_subtags: Vec<(LanguageIdentifier, LanguageIdentifier)>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
