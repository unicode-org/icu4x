// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/currencyData.json>

use serde::Deserialize;
use std::collections::BTreeMap;
use tinystr::TinyAsciiStr;

type ISOCode = TinyAsciiStr<3>;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct RoundingModes {
    // TODO: Get `deserialize_option_number_from_string` to work
    //    HINT: #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")])
    #[serde(rename = "_rounding")]
    pub(in crate::provider) rounding: Option<String>,

    #[serde(rename = "_digits")]
    pub(in crate::provider) digits: Option<String>,

    #[serde(rename = "_cashRounding")]
    pub(in crate::provider) cash_rounding: Option<String>,

    #[serde(rename = "_cashDigits")]
    pub(in crate::provider) cash_digits: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Fractions {
    #[serde(rename = "DEFAULT")]
    default: RoundingModes,

    #[serde(flatten)]
    pub(in crate::provider) currencies: BTreeMap<ISOCode, RoundingModes>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct CurrencyData {
    pub(in crate::provider) fractions: Fractions,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "currencyData")]
    pub(in crate::provider) currency_data: CurrencyData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
