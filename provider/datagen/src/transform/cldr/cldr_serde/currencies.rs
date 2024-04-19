// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/currencies.json>

use serde::Deserialize;
use std::collections::BTreeMap;
use tinystr::UnvalidatedTinyAsciiStr;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct CurrencyPatterns {
    #[serde(rename = "symbol")]
    pub(in crate::provider) short: Option<String>,

    #[serde(rename = "symbol-alt-narrow")]
    pub(in crate::provider) narrow: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Numbers {
    pub(in crate::provider) currencies: BTreeMap<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct LangNumbers {
    pub(in crate::provider) numbers: Numbers,
}

pub(in crate::provider) type Resource = super::LocaleResource<LangNumbers>;
