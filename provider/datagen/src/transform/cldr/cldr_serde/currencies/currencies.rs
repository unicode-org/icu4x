// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/currencies.json>

use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use tinystr::UnvalidatedTinyAsciiStr;

// TODO: uncomment when we have a use case for this
// #[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Deserialize)]
// pub struct LocaleDisplayNames {
//     #[serde(rename = "DisplayName-count-zero")]
//     pub zero: Option<String>,
//     #[serde(rename = "DisplayName-count-one")]
//     pub one: Option<String>,
//     #[serde(rename = "DisplayName-count-two")]
//     pub two: Option<String>,
//     #[serde(rename = "DisplayName-count-few")]
//     pub few: Option<String>,
//     #[serde(rename = "DisplayName-count-many")]
//     pub many: Option<String>,
//     #[serde(rename = "DisplayName-count-other")]
//     pub other: Option<String>,
// }

// #[derive(PartialEq, Debug, Deserialize)]
// pub struct DisplayNames(pub HashMap<UnvalidatedTinyAsciiStr<3>, LocaleDisplayNames>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct CurrencyPatterns {
    #[serde(rename = "symbol")]
    pub short: Option<String>,

    #[serde(rename = "symbol-alt-narrow")]
    pub narrow: Option<String>,

    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    #[serde(rename = "DisplayName-count-zero")]
    pub zero: Option<String>,
    #[serde(rename = "DisplayName-count-one")]
    pub one: Option<String>,
    #[serde(rename = "DisplayName-count-two")]
    pub two: Option<String>,
    #[serde(rename = "DisplayName-count-few")]
    pub few: Option<String>,
    #[serde(rename = "DisplayName-count-many")]
    pub many: Option<String>,
    #[serde(rename = "DisplayName-count-other")]
    pub other: Option<String>,
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
