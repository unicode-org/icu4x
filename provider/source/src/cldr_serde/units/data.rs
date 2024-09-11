// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-units-full/main/en/units.json>

use icu_pattern::SinglePlaceholderPattern;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Represents various patterns for a unit according to plural rules.
/// The plural rule categories are: zero, one, two, few, many and other.
/// For more details, refer to the technical report:
///     https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct Patterns {
    #[serde(rename = "displayName-count-0")]
    pub(crate) explicit_zero: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "displayName-count-1")]
    pub(crate) explicit_one: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-zero")]
    pub(crate) zero: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-one")]
    pub(crate) one: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-two")]
    pub(crate) two: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-few")]
    pub(crate) few: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-many")]
    pub(crate) many: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "unitPattern-count-other")]
    pub(crate) other: Option<Box<SinglePlaceholderPattern>>,

    #[serde(rename = "compoundUnitPattern")]
    pub(crate) compound_unit_pattern: Option<String>,

    #[serde(rename = "unitPrefixPattern")]
    pub(crate) unit_prefix_pattern: Option<String>,

    #[serde(rename = "compoundUnitPattern1")]
    pub(crate) compound_unit_pattern1: Option<String>,

    #[serde(rename = "compountUnitPattern1-count-0")]
    pub(crate) explicit_zero_compound_unit_pattern1: Option<String>,

    #[serde(rename = "compountUnitPattern1-count-1")]
    pub(crate) explicit_one_compound_unit_pattern1: Option<String>,

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
    pub(crate) long: UnitsLengthData,

    pub(crate) short: UnitsLengthData,

    pub(crate) narrow: UnitsLengthData,

    pub(crate) duration: DurationUnits,
}

#[derive(PartialEq, Debug)]
pub(crate) struct UnitsLengthData {
    pub(crate) units: BTreeMap<String, Patterns>,
    pub(crate) per: Patterns,
    pub(crate) times: Patterns,
    pub(crate) powers: BTreeMap<usize, Patterns>,
    pub(crate) binary: BTreeMap<u8, Patterns>,
    pub(crate) decimal: BTreeMap<i8, Patterns>,
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

        let construct = |mut map: BTreeMap<String, Patterns>| UnitsLengthData {
            per: map.remove("per").unwrap(),
            times: map.remove("times").unwrap(),
            powers: map
                .iter()
                .filter_map(|(k, v)| Some((k.strip_prefix("power")?.parse().ok()?, v.clone())))
                .collect(),
            binary: map
                .iter()
                .filter_map(|(k, v)| Some((k.strip_prefix("1024p")?.parse().ok()?, v.clone())))
                .collect(),
            decimal: map
                .iter()
                .filter_map(|(k, v)| Some((k.strip_prefix("10p")?.parse().ok()?, v.clone())))
                .collect(),
            units: map
                .into_iter()
                .filter_map(|(k, v)| {
                    if k.starts_with("10p")
                        || k.starts_with("1024p")
                        || (k.starts_with("power") && !k.starts_with("power-"))
                    {
                        return None;
                    }
                    k.split_once('-').map(|(_, unit)| (unit.to_string(), v))
                })
                .collect(),
        };

        Ok(Self {
            long: construct(long),
            short: construct(short),
            narrow: construct(narrow),
            duration,
        })
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangUnits {
    pub(crate) units: UnitsData,
}

pub(crate) type Resource = super::super::LocaleResource<LangUnits>;
