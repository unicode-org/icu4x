// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON weekData.json files.
//!
//! Sample file:
//! `<https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/weekData.json>`

use icu::locale::{subtags::region, subtags::Region};
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

#[derive(Debug, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl From<&Weekday> for icu::calendar::types::Weekday {
    fn from(day: &Weekday) -> Self {
        use icu::calendar::types::Weekday as CalWeekday;
        match day {
            Weekday::Mon => CalWeekday::Monday,
            Weekday::Tue => CalWeekday::Tuesday,
            Weekday::Wed => CalWeekday::Wednesday,
            Weekday::Thu => CalWeekday::Thursday,
            Weekday::Fri => CalWeekday::Friday,
            Weekday::Sat => CalWeekday::Saturday,
            Weekday::Sun => CalWeekday::Sunday,
        }
    }
}

impl From<Weekday> for icu::calendar::types::Weekday {
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
pub(crate) enum Territory {
    // A territory string, e.g. "AD" for Andorra.
    Region(Region),
    // An alternative variant for a given territory (e.g. the first day of the
    // week can be sunday rather than monday GB). The string is set to the region
    // with the "-alt-variant" suffix present in the json.
    AltVariantRegion(Region),
}

/// The string used to represent the default territory.
pub(crate) const DEFAULT_TERRITORY: Territory = Territory::Region(region!("001"));

/// Suffix used to denote alternative week data variants for a given territory (e.g. English BC/AD v English BCE/CE).
const ALT_VARIANT_SUFFIX: &str = "-alt-variant";

impl<'de> Deserialize<'de> for Territory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerritoryVisitor;

        impl serde::de::Visitor<'_> for TerritoryVisitor {
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

#[derive(Debug, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WeekData {
    pub(crate) min_days: BTreeMap<Territory, String>,
    pub(crate) first_day: BTreeMap<Territory, Weekday>,
    pub(crate) weekend_start: BTreeMap<Territory, Weekday>,
    pub(crate) weekend_end: BTreeMap<Territory, Weekday>,
}

#[derive(serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Supplemental {
    pub(crate) week_data: WeekData,
}

#[derive(serde_derive::Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
