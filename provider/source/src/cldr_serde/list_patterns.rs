// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/en/listPatterns.json>

use super::locale_resource::LocaleResource;

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct ListPattern {
    pub(crate) start: String,
    pub(crate) middle: String,
    pub(crate) end: String,
    #[serde(rename = "2")]
    pub(crate) pair: String,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct ListPatterns {
    #[serde(rename = "listPattern-type-standard")]
    pub(crate) standard: ListPattern,
    #[serde(rename = "listPattern-type-standard-narrow")]
    pub(crate) standard_narrow: ListPattern,
    #[serde(rename = "listPattern-type-standard-short")]
    pub(crate) standard_short: ListPattern,
    #[serde(rename = "listPattern-type-or")]
    pub(crate) or: ListPattern,
    #[serde(rename = "listPattern-type-or-narrow")]
    pub(crate) or_narrow: ListPattern,
    #[serde(rename = "listPattern-type-or-short")]
    pub(crate) or_short: ListPattern,
    #[serde(rename = "listPattern-type-unit")]
    pub(crate) unit: ListPattern,
    #[serde(rename = "listPattern-type-unit-narrow")]
    pub(crate) unit_narrow: ListPattern,
    #[serde(rename = "listPattern-type-unit-short")]
    pub(crate) unit_short: ListPattern,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct LangListPatterns {
    #[serde(rename = "listPatterns")]
    pub(crate) list_patterns: ListPatterns,
}

pub(crate) type Resource = LocaleResource<LangListPatterns>;
