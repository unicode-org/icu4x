// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/currencies.json>

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct CurrencyPatterns {
    #[serde(rename = "symbol")]
    pub(crate) short: Option<String>,

    #[serde(rename = "symbol-alt-narrow")]
    pub(crate) narrow: Option<String>,

    #[serde(rename = "displayName")]
    pub(crate) display_name: Option<String>,

    #[serde(rename = "displayName-count-0")]
    pub(crate) explicit_zero: Option<String>,

    #[serde(rename = "displayName-count-1")]
    pub(crate) explicit_one: Option<String>,

    #[serde(rename = "displayName-count-zero")]
    pub(crate) zero: Option<String>,

    #[serde(rename = "displayName-count-one")]
    pub(crate) one: Option<String>,

    #[serde(rename = "displayName-count-two")]
    pub(crate) two: Option<String>,

    #[serde(rename = "displayName-count-few")]
    pub(crate) few: Option<String>,

    #[serde(rename = "displayName-count-many")]
    pub(crate) many: Option<String>,

    #[serde(rename = "displayName-count-other")]
    pub(crate) other: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Numbers {
    pub(crate) currencies: BTreeMap<String, CurrencyPatterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangNumbers {
    pub(crate) numbers: Numbers,
}

pub(crate) type Resource = super::super::LocaleResource<LangNumbers>;
