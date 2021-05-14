// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitinos for mapping CLDR-JSON Time Zones.
//! These provide in-memory stuctures into which `timeZoneNames.json` will be serialized.
//! Here is the `en` [timeZoneNames.json](https://raw.githubusercontent.com/unicode-org/cldr-json/master/cldr-json/cldr-dates-full/main/en/timeZoneNames.json) for context.

mod convert;

use crate::cldr_langid::CldrLangID;
use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ZoneFormat(pub BTreeMap<String, String>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZone {
    pub long: Option<ZoneFormat>,
    pub short: Option<ZoneFormat>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZones(pub BTreeMap<String, MetaZone>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct LocationWithExemplarCity {
    pub long: Option<ZoneFormat>,
    pub short: Option<ZoneFormat>,
    #[serde(rename = "exemplarCity")]
    pub exemplar_city: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct LocationWithShort {
    pub long: Option<ZoneFormat>,
    pub short: ZoneFormat,
    #[serde(rename = "exemplarCity")]
    pub exemplar_city: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct LocationWithLong {
    pub long: ZoneFormat,
    pub short: Option<ZoneFormat>,
    #[serde(rename = "exemplarCity")]
    pub exemplar_city: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Location {
    LocationWithCity(LocationWithExemplarCity),
    LocationWithLong(LocationWithLong),
    LocationWithShort(LocationWithShort),
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
pub struct Zones(pub BTreeMap<String, Region>);

#[derive(PartialEq, Debug, Default, Clone)]
pub struct TimeZoneNames {
    pub hour_format: String,
    pub gmt_format: String,
    pub gmt_zero_format: String,
    pub region_format: String,
    pub region_format_variants: BTreeMap<String, String>,
    pub fallback_format: String,
    pub zone: Zones,
    pub metazone: Option<MetaZones>,
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
                let value = map.next_value::<MetaZones>()?;
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

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(#[serde(with = "tuple_vec_map")] pub(crate) Vec<(CldrLangID, LangTimeZones)>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
