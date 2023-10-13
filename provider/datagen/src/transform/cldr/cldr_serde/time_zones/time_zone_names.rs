// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timeZoneNames.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/timeZoneNames.json>

use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ZoneFormat(pub HashMap<String, String>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Metazone {
    pub long: Option<ZoneFormat>,
    pub short: Option<ZoneFormat>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Metazones(pub HashMap<String, Metazone>);

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
pub struct Location {
    pub long: Option<ZoneFormat>,
    pub short: Option<ZoneFormat>,
    #[serde(rename = "exemplarCity")]
    pub exemplar_city: Option<String>,
    #[serde(rename = "exemplarCity-alt-secondary")]
    pub exemplar_city_alt_secondary: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LocationOrSubRegion {
    Location(Location),
    SubRegion(BTreeMap<String, Location>),
}

#[derive(PartialEq, Debug, Clone, Default, Deserialize)]
pub struct Region(pub BTreeMap<String, LocationOrSubRegion>);

#[derive(PartialEq, Debug, Clone, Default, Deserialize)]
pub struct Zones(pub HashMap<String, Region>);

#[derive(PartialEq, Debug, Default, Clone)]
pub struct TimeZoneNames {
    pub hour_format: String,
    pub gmt_format: String,
    pub gmt_zero_format: String,
    pub region_format: String,
    pub region_format_variants: HashMap<String, String>,
    pub fallback_format: String,
    pub zone: Zones,
    pub metazone: Option<Metazones>,
}

pub struct TimeZoneNamesVisitor;

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
                let value = map.next_value::<String>()?;
                time_zone_names.gmt_format = value;
            } else if key.eq("gmtZeroFormat") {
                let value = map.next_value::<String>()?;
                time_zone_names.gmt_zero_format = value;
            } else if key.eq("fallbackFormat") {
                let value = map.next_value::<String>()?;
                time_zone_names.fallback_format = value;
            } else if key.starts_with("regionFormat") {
                let value = map.next_value::<String>()?;
                if key.contains('-') {
                    // key is of the form: "regionFormat-type-variant"
                    let variant = key.split('-').last().unwrap();
                    time_zone_names
                        .region_format_variants
                        .insert(variant.into(), value);
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
pub struct Dates {
    #[serde(rename = "timeZoneNames")]
    pub time_zone_names: TimeZoneNames,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangTimeZones {
    pub dates: Dates,
}

pub type Resource = super::super::LocaleResource<LangTimeZones>;
