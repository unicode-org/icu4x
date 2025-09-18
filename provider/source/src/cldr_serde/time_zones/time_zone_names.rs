// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timeZoneNames.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/timeZoneNames.json>

use icu_pattern::{DoublePlaceholder, PatternString, SinglePlaceholder};
use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct ZoneFormat(pub(crate) BTreeMap<String, String>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Metazone {
    pub(crate) long: Option<ZoneFormat>,
    pub(crate) short: Option<ZoneFormat>,
}

impl Metazone {
    pub(crate) fn long_short(&self, long: bool) -> Option<&ZoneFormat> {
        if long {
            self.long.as_ref()
        } else {
            self.short.as_ref()
        }
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Metazones(pub(crate) BTreeMap<String, Metazone>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
// Since this value can be either a Location or a table of sub-regions, we use
// the presence of fields to distinguish the object types. If CLDR-JSON adds
// more fields to Location in the future, they must also be added here.
//
// Example errors that may be caused by this:
//
// - Cannot find bcp47 for "America/Argentina".
// - Cannot find bcp47 for "Etc/UTC/short".
#[serde(deny_unknown_fields)]
pub(crate) struct Location {
    pub(crate) long: Option<ZoneFormat>,
    pub(crate) short: Option<ZoneFormat>,
    #[serde(rename = "exemplarCity")]
    pub(crate) exemplar_city: Option<String>,
    #[serde(rename = "exemplarCity-alt-secondary")]
    pub(crate) _exemplar_city_alt_secondary: Option<String>,
    #[serde(rename = "_type")]
    pub(crate) _ty: Option<String>,
}

impl Location {
    pub(crate) fn long_short(&self, long: bool) -> Option<&ZoneFormat> {
        if long {
            self.long.as_ref()
        } else {
            self.short.as_ref()
        }
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum LocationOrSubRegion {
    Location(Location),
    SubRegion(BTreeMap<String, Location>),
}

#[derive(PartialEq, Debug, Clone, Default, Deserialize)]
pub(crate) struct Region(pub(crate) BTreeMap<String, LocationOrSubRegion>);

#[derive(PartialEq, Debug, Clone, Default, Deserialize)]
pub(crate) struct Zones(pub(crate) BTreeMap<String, Region>);

#[derive(PartialEq, Debug, Default, Clone)]
pub(crate) struct TimeZoneNames {
    pub(crate) hour_format: String,
    pub(crate) gmt_format: PatternString<SinglePlaceholder>,
    pub(crate) gmt_zero_format: String,
    pub(crate) gmt_unknown_format: String,
    pub(crate) region_format: PatternString<SinglePlaceholder>,
    pub(crate) region_format_dt: PatternString<SinglePlaceholder>,
    pub(crate) region_format_st: PatternString<SinglePlaceholder>,
    pub(crate) fallback_format: PatternString<DoublePlaceholder>,
    pub(crate) zone: Zones,
    pub(crate) metazone: Option<Metazones>,
}

pub(crate) struct TimeZoneNamesVisitor;

impl<'de> Visitor<'de> for TimeZoneNamesVisitor {
    type Value = TimeZoneNames;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("formatting data by numbering system")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut time_zone_names = TimeZoneNames::default();
        while let Some(key) = map.next_key::<String>()? {
            if key.eq("hourFormat") {
                let value = map.next_value::<String>()?;
                time_zone_names.hour_format = value;
            } else if key.eq("gmtFormat") {
                let value = map.next_value::<PatternString<SinglePlaceholder>>()?;
                time_zone_names.gmt_format = value;
            } else if key.eq("gmtZeroFormat") {
                let value = map.next_value::<String>()?;
                time_zone_names.gmt_zero_format = value;
            } else if key.eq("gmtUnknownFormat") {
                let value = map.next_value::<String>()?;
                time_zone_names.gmt_unknown_format = value;
            } else if key.eq("fallbackFormat") {
                let value = map.next_value::<PatternString<DoublePlaceholder>>()?;
                time_zone_names.fallback_format = value;
            } else if key.starts_with("regionFormat") {
                let value = map.next_value::<PatternString<SinglePlaceholder>>()?;
                // key is of the form: "regionFormat-type-variant"
                if key.ends_with("-standard") {
                    time_zone_names.region_format_st = value;
                } else if key.ends_with("-daylight") {
                    time_zone_names.region_format_dt = value;
                } else {
                    time_zone_names.region_format = value
                }
            } else if key.eq("metazone") {
                let value = map.next_value::<Metazones>()?;
                time_zone_names.metazone = Some(value);
            } else if key.eq("zone") {
                let value = map.next_value::<Zones>()?;
                time_zone_names.zone = value;
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        Ok(time_zone_names)
    }
}

impl<'de> Deserialize<'de> for TimeZoneNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TimeZoneNamesVisitor)
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Dates {
    #[serde(rename = "timeZoneNames")]
    pub(crate) time_zone_names: TimeZoneNames,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangTimeZones {
    pub(crate) dates: Dates,
}

pub(crate) type Resource = super::super::LocaleResource<LangTimeZones>;
