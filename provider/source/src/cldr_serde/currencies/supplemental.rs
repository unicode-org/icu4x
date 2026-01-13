// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/currencyData.json>

use icu::experimental::dimension::currency::CurrencyCode;
use icu::locale::subtags::Region;
use serde::Deserialize;
use std::collections::BTreeMap;

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
    pub(crate) default: RoundingModes,

    #[serde(flatten)]
    pub(crate) currencies: BTreeMap<CurrencyCode, RoundingModes>,
}

/// Metadata for a currency in a region, including validity dates and tender status.
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct RegionCurrencyData {
    /// Start date when this currency became valid in the region (e.g., "1999-01-01")
    #[serde(rename = "_from")]
    pub(crate) from: Option<String>,

    /// End date when this currency stopped being valid in the region
    #[serde(rename = "_to")]
    pub(crate) to: Option<String>,

    /// Whether this is legal tender (defaults to true if not specified)
    /// "false" means it's not legal tender (e.g., special purpose currencies)
    #[serde(rename = "_tender")]
    pub(crate) tender: Option<String>,

    /// Timezone for the `_from` date (e.g., "Europe/Zagreb")
    #[serde(rename = "_tz")]
    pub(crate) from_tz: Option<String>,

    /// Timezone for the `_to` date
    #[serde(rename = "_to-tz")]
    pub(crate) to_tz: Option<String>,
}

/// A single currency entry in the region array.
/// The JSON structure is: { "USD": { "_from": "...", "_to": "..." } }
pub(crate) type RegionCurrencyEntry = BTreeMap<CurrencyCode, RegionCurrencyData>;

/// Map from region code to list of currencies used in that region.
/// Each region can have multiple currencies (current and historical).
pub(crate) type RegionToCurrency = BTreeMap<Region, Vec<RegionCurrencyEntry>>;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CurrencyData {
    pub(crate) fractions: Fractions,
    /// Map from region code (e.g., "US", "DE") to list of currencies used in that region
    pub(crate) region: RegionToCurrency,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "currencyData")]
    pub(crate) currency_data: CurrencyData,
}

#[derive(PartialEq, Debug, Deserialize)]
#[allow(
    dead_code,
    reason = "This is WIP, remove this annotation when this component is done"
)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
