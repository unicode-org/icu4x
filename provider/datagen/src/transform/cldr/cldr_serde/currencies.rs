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
pub struct CurrencyPatterns {
    #[serde(rename = "symbol")]
    pub short: Option<String>,

    #[serde(rename = "symbol-alt-narrow")]
    pub narrow: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Numbers {
    pub currencies: BTreeMap<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangNumbers {
    pub numbers: Numbers,
}

pub type Resource = super::LocaleResource<LangNumbers>;
