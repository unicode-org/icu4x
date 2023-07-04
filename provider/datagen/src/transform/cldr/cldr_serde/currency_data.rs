// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/currencyData.json>

use icu_locid::LanguageIdentifier;
use itertools::Itertools;
use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use tinystr::TinyAsciiStr;

type ISOCode = TinyAsciiStr<3>;

#[derive(PartialEq, Debug, Deserialize)]
pub struct RoundingModes {
    #[serde(rename = "_rounding")]
    // #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub rounding: Option<String>,

    #[serde(rename = "_digits")]
    // #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub digits: Option<String>,

    #[serde(rename = "_cashRounding")]
    // #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub cash_rounding: Option<String>,

    #[serde(rename = "_cashDigits")]
    // #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub cash_digits: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Fractions {
    #[serde(rename = "DEFAULT")]
    default: RoundingModes,

    #[serde(flatten)]
    pub currencies: HashMap<ISOCode, RoundingModes>, /*change it to RoundingModes later */
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct CurrencyData {
    pub fractions: Fractions,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "currencyData")]
    pub currency_data: CurrencyData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
