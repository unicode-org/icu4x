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
pub(crate) struct RoundingModes {
    // TODO: Get `deserialize_option_number_from_string` to work
    //    HINT: #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")])
    #[serde(rename = "_rounding")]
    pub(crate) rounding: Option<String>,

    #[serde(rename = "_digits")]
    pub(crate) digits: Option<String>,

    #[serde(rename = "_cashRounding")]
    pub(crate) cash_rounding: Option<String>,

    #[serde(rename = "_cashDigits")]
    pub(crate) cash_digits: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Fractions {
    #[serde(rename = "DEFAULT")]
    default: RoundingModes,

    #[serde(flatten)]
    pub(crate) currencies: BTreeMap<ISOCode, RoundingModes>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CurrencyData {
    pub(crate) fractions: Fractions,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "currencyData")]
    pub(crate) currency_data: CurrencyData,
}

#[derive(PartialEq, Debug, Deserialize)]
#[expect(
    dead_code,
    reason = "This is WIP, remove this annotation when this component is done"
)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
