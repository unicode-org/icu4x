// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-misc-full/main/en/listPatterns.json>

use serde::Deserialize;

use super::locale_resource::LocaleResource;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ListPattern {
    pub(in crate::provider) start: String,
    pub(in crate::provider) middle: String,
    pub(in crate::provider) end: String,
    #[serde(rename = "2")]
    pub(in crate::provider) pair: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ListPatterns {
    #[serde(rename = "listPattern-type-standard")]
    pub(in crate::provider) standard: ListPattern,
    #[serde(rename = "listPattern-type-standard-narrow")]
    pub(in crate::provider) standard_narrow: ListPattern,
    #[serde(rename = "listPattern-type-standard-short")]
    pub(in crate::provider) standard_short: ListPattern,
    #[serde(rename = "listPattern-type-or")]
    pub(in crate::provider) or: ListPattern,
    #[serde(rename = "listPattern-type-or-narrow")]
    pub(in crate::provider) or_narrow: ListPattern,
    #[serde(rename = "listPattern-type-or-short")]
    pub(in crate::provider) or_short: ListPattern,
    #[serde(rename = "listPattern-type-unit")]
    pub(in crate::provider) unit: ListPattern,
    #[serde(rename = "listPattern-type-unit-narrow")]
    pub(in crate::provider) unit_narrow: ListPattern,
    #[serde(rename = "listPattern-type-unit-short")]
    pub(in crate::provider) unit_short: ListPattern,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct LangListPatterns {
    #[serde(rename = "listPatterns")]
    pub(in crate::provider) list_patterns: ListPatterns,
}

pub(in crate::provider) type Resource = LocaleResource<LangListPatterns>;
