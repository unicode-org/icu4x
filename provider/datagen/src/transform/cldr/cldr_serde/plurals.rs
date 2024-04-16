// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON aliases.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/plurals.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Deserialize)]
pub(in crate::provider) struct LocalePluralRules {
    #[serde(rename = "pluralRule-count-zero")]
    pub(in crate::provider) zero: Option<String>,
    #[serde(rename = "pluralRule-count-one")]
    pub(in crate::provider) one: Option<String>,
    #[serde(rename = "pluralRule-count-two")]
    pub(in crate::provider) two: Option<String>,
    #[serde(rename = "pluralRule-count-few")]
    pub(in crate::provider) few: Option<String>,
    #[serde(rename = "pluralRule-count-many")]
    pub(in crate::provider) many: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Rules(pub(in crate::provider) HashMap<LanguageIdentifier, LocalePluralRules>);

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "plurals-type-cardinal")]
    pub(in crate::provider) plurals_type_cardinal: Option<Rules>,
    #[serde(rename = "plurals-type-ordinal")]
    pub(in crate::provider) plurals_type_ordinal: Option<Rules>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
