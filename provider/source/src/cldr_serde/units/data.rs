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

    #[serde(rename = "compoundUnitPattern")]
    pub(crate) compound_unit_pattern: Option<String>,

    #[serde(rename = "unitPrefixPattern")]
    pub(crate) unit_prefix_pattern: Option<String>,

    #[serde(rename = "compoundUnitPattern1")]
    pub(crate) compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-zero")]
    pub(crate) zero_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-one")]
    pub(crate) one_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-two")]
    pub(crate) two_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-few")]
    pub(crate) few_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-many")]
    pub(crate) many_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compoundUnitPattern1-count-other")]
    pub(crate) other_compound_unit_pattern1: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct DurationUnit {
    #[serde(rename = "durationUnitPattern")]
    pub(crate) pat: String,
    #[serde(rename = "durationUnitPattern-alt-variant")]
    pub(crate) alt_pat: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct DurationUnits {
    #[serde(rename = "durationUnit-type-hm")]
    pub hm: DurationUnit,
    #[serde(rename = "durationUnit-type-hms")]
    pub hms: DurationUnit,
    #[serde(rename = "durationUnit-type-ms")]
    pub ms: DurationUnit,
}

// TODO: replace Value with specific structs
#[derive(PartialEq, Debug)]
pub(crate) struct UnitsData {
    pub(crate) long: BTreeMap<String, Patterns>,

    pub(crate) short: BTreeMap<String, Patterns>,

    pub(crate) narrow: BTreeMap<String, Patterns>,

    pub(crate) duration: DurationUnits,
}

impl<'de> Deserialize<'de> for UnitsData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            long: BTreeMap<String, Patterns>,
            short: BTreeMap<String, Patterns>,
            narrow: BTreeMap<String, Patterns>,
            #[serde(flatten)]
            duration: DurationUnits,
        }

        let Raw {
            long,
            short,
            narrow,
            duration,
        } = Raw::deserialize(deserializer)?;

        let strip_unit_prefixes = |(k, v): (String, Patterns)| {
            (
                if k == "per"
                    || k == "times"
                    || k.starts_with("power")
                    || k.starts_with("10p")
                    || k.starts_with("2p")
                {
                    k
                } else {
                    k.split_once('-')
                        .map(|(_prefix, suffix)| suffix.to_string())
                        .unwrap_or(k)
                },
                v,
            )
        };

        Ok(Self {
            long: long.into_iter().map(strip_unit_prefixes).collect(),
            short: short.into_iter().map(strip_unit_prefixes).collect(),
            narrow: narrow.into_iter().map(strip_unit_prefixes).collect(),
            duration,
        })
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangUnits {
    pub(crate) units: UnitsData,
}

pub(crate) type Resource = super::super::LocaleResource<LangUnits>;
