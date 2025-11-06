// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON aliases.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/plurals.json>

use icu::locale::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Deserialize)]
pub(crate) struct LocalePluralRules {
    #[serde(rename = "pluralRule-count-zero")]
    pub(crate) zero: Option<String>,
    #[serde(rename = "pluralRule-count-one")]
    pub(crate) one: Option<String>,
    #[serde(rename = "pluralRule-count-two")]
    pub(crate) two: Option<String>,
    #[serde(rename = "pluralRule-count-few")]
    pub(crate) few: Option<String>,
    #[serde(rename = "pluralRule-count-many")]
    pub(crate) many: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Rules(pub(crate) HashMap<LanguageIdentifier, LocalePluralRules>);

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "plurals-type-cardinal")]
    pub(crate) plurals_type_cardinal: Option<Rules>,
    #[serde(rename = "plurals-type-ordinal")]
    pub(crate) plurals_type_ordinal: Option<Rules>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
