// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON weekData.json files.
//!
//! Sample file:
//! `<https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/weekData.json>`

use core::convert::TryFrom;
use icu::locale::{subtags::region, subtags::Region};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::provider) enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl From<&Weekday> for icu::calendar::types::IsoWeekday {
    fn from(day: &Weekday) -> Self {
        use icu::calendar::types::IsoWeekday;
        match day {
            Weekday::Mon => IsoWeekday::Monday,
            Weekday::Tue => IsoWeekday::Tuesday,
            Weekday::Wed => IsoWeekday::Wednesday,
            Weekday::Thu => IsoWeekday::Thursday,
            Weekday::Fri => IsoWeekday::Friday,
            Weekday::Sat => IsoWeekday::Saturday,
            Weekday::Sun => IsoWeekday::Sunday,
        }
    }
}

impl From<Weekday> for icu::calendar::types::IsoWeekday {
    fn from(day: Weekday) -> Self {
        (&day).into()
    }
}

/// The territory that data is keyed by.
///
/// For example the "AD" in "weekData": { "minDays": { "AD": 4, } }
///
/// The contained types are strings rather than [`icu::locale::subtags::Region`]
/// to avoid an extra parsing step of the variant in data providers.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::provider) enum Territory {
    // A territory string, e.g. "AD" for Andorra.
    Region(Region),
    // An alternative variant for a given territory (e.g. the first day of the
    // week can be sunday rather than monday GB). The string is set to the region
    // with the "-alt-variant" suffix present in the json.
    AltVariantRegion(Region),
}

/// The string used to represent the default territory.
pub(in crate::provider) const DEFAULT_TERRITORY: Territory = Territory::Region(region!("001"));

/// Suffix used to denote alternative week data variants for a given territory (e.g. English BC/AD v English BCE/CE).
const ALT_VARIANT_SUFFIX: &str = "-alt-variant";

impl<'de> Deserialize<'de> for Territory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerritoryVisitor;

        impl<'de> serde::de::Visitor<'de> for TerritoryVisitor {
            type Value = Territory;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    formatter,
                    "a valid Unicode Language Identifier or default territory literal"
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(prefix) = s.strip_suffix(ALT_VARIANT_SUFFIX) {
                    return Ok(Territory::AltVariantRegion(
                        prefix.parse::<Region>().map_err(serde::de::Error::custom)?,
                    ));
                }

                Ok(Territory::Region(
                    s.parse::<Region>().map_err(serde::de::Error::custom)?,
                ))
            }
        }

        deserializer.deserialize_string(TerritoryVisitor)
    }
}

/// Wrapper used to deserialize json string keys as u8s.
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub(in crate::provider) struct U8(pub(in crate::provider) u8);

impl TryFrom<String> for U8 {
    type Error = ParseIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(u8::from_str(&s)?))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::provider) struct WeekData {
    pub(in crate::provider) min_days: BTreeMap<Territory, U8>,
    pub(in crate::provider) first_day: BTreeMap<Territory, Weekday>,
    pub(in crate::provider) weekend_start: BTreeMap<Territory, Weekday>,
    pub(in crate::provider) weekend_end: BTreeMap<Territory, Weekday>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::provider) struct Supplemental {
    pub(in crate::provider) week_data: WeekData,
}

#[derive(Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
