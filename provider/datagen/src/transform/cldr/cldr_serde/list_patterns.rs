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
pub struct ListPattern {
    pub start: String,
    pub middle: String,
    pub end: String,
    #[serde(rename = "2")]
    pub pair: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct ListPatterns {
    #[serde(rename = "listPattern-type-standard")]
    pub standard: ListPattern,
    #[serde(rename = "listPattern-type-standard-narrow")]
    pub standard_narrow: ListPattern,
    #[serde(rename = "listPattern-type-standard-short")]
    pub standard_short: ListPattern,
    #[serde(rename = "listPattern-type-or")]
    pub or: ListPattern,
    #[serde(rename = "listPattern-type-or-narrow")]
    pub or_narrow: ListPattern,
    #[serde(rename = "listPattern-type-or-short")]
    pub or_short: ListPattern,
    #[serde(rename = "listPattern-type-unit")]
    pub unit: ListPattern,
    #[serde(rename = "listPattern-type-unit-narrow")]
    pub unit_narrow: ListPattern,
    #[serde(rename = "listPattern-type-unit-short")]
    pub unit_short: ListPattern,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangListPatterns {
    #[serde(rename = "listPatterns")]
    pub list_patterns: ListPatterns,
}

pub type Resource = LocaleResource<LangListPatterns>;
