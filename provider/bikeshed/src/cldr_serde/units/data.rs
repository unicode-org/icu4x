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
pub(crate) struct Patterns {
    #[serde(rename = "unitPattern-count-zero")]
    pub(crate) zero: Option<String>,

    #[serde(rename = "unitPattern-count-one")]
    pub(crate) one: Option<String>,

    #[serde(rename = "unitPattern-count-two")]
    pub(crate) two: Option<String>,

    #[serde(rename = "unitPattern-count-few")]
    pub(crate) few: Option<String>,

    #[serde(rename = "unitPattern-count-many")]
    pub(crate) many: Option<String>,

    #[serde(rename = "unitPattern-count-other")]
    pub(crate) other: Option<String>,
}

// TODO: replace Value with specific structs
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct UnitsData {
    pub(crate) long: BTreeMap<String, Patterns>,

    pub(crate) short: BTreeMap<String, Patterns>,

    pub(crate) narrow: BTreeMap<String, Patterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangUnits {
    pub(crate) units: UnitsData,
}

pub(crate) type Resource = super::super::LocaleResource<LangUnits>;
