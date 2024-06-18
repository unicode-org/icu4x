// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-units-full/main/en/units.json>

use serde::Deserialize;
use std::collections::BTreeMap;

/// Represents various patterns for a unit according to plural rules.
/// The plural rule categories are: zero, one, two, few, many and other.
/// For more details, refer to the technical report:
///     https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Patterns {
    #[serde(rename = "unitPattern-count-zero")]
    pub(in crate::provider) zero: Option<String>,

    #[serde(rename = "unitPattern-count-one")]
    pub(in crate::provider) one: Option<String>,

    #[serde(rename = "unitPattern-count-two")]
    pub(in crate::provider) two: Option<String>,

    #[serde(rename = "unitPattern-count-few")]
    pub(in crate::provider) few: Option<String>,

    #[serde(rename = "unitPattern-count-many")]
    pub(in crate::provider) many: Option<String>,

    #[serde(rename = "unitPattern-count-other")]
    pub(in crate::provider) other: Option<String>,
}

// TODO: replace Value with specific structs
#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct UnitsData {
    pub(in crate::provider) long: BTreeMap<String, Patterns>,

    pub(in crate::provider) short: BTreeMap<String, Patterns>,

    pub(in crate::provider) narrow: BTreeMap<String, Patterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct LangUnits {
    pub(in crate::provider) units: UnitsData,
}

pub(in crate::provider) type Resource = super::super::LocaleResource<LangUnits>;
