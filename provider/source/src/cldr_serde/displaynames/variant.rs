// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON display name files for variants.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/variants.json>

use std::collections::HashMap;

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Variants {
    pub(crate) variants: HashMap<String, String>,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct LangDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub(crate) localedisplaynames: Variants,
}

pub(crate) type Resource = super::super::LocaleResource<LangDisplayNames>;
