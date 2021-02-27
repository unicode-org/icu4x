// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::provider::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;
use timezones::ZoneFormatV1;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [key::TIMEZONES_V1];

/// A data provider reading from CLDR JSON zones files.
#[derive(PartialEq, Debug)]
pub struct TimeZonesProvider<'d> {
    data: Vec<(CldrLangID, cldr_json::LangTimeZones)>,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for TimeZonesProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_dates()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("timeZoneNames.json");

            let mut resource: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl TryFrom<&str> for TimeZonesProvider<'_> {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let mut resource: cldr_json::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        data.append(&mut resource.main.0);

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for TimeZonesProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::TimeZones {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d, timezones::TimeZonesV1> for TimeZonesProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, timezones::TimeZonesV1>, DataError> {
        TimeZonesProvider::supports_key(&req.resource_path.key)?;
        let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
        let zones = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.dates.time_zone_names,
            Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(Cow::Owned(timezones::TimeZonesV1::from(zones.clone()))),
        })
    }
}

icu_provider::impl_erased!(TimeZonesProvider<'d>, 'd);

impl<'d> IterableDataProvider<'d> for TimeZonesProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                langid: Some(l.langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}
/// Serde structs for the CLDR JSON zones files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use std::borrow::Cow;
    //use litemap::LiteMap;
    type LiteMap<K, V> = std::collections::BTreeMap<K, V>;

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct MetaZone {
        pub long: Option<ZoneFormat>,
        pub short: Option<ZoneFormat>,
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct MetaZones(pub LiteMap<Cow<'static, str>, MetaZone>);

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct ZoneFormat {
        pub generic: Option<Cow<'static, str>>,
        pub standard: Option<Cow<'static, str>>,
        pub daylight: Option<Cow<'static, str>>,
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct LocationWithExemplarCity {
        pub long: Option<ZoneFormat>,
        pub short: Option<ZoneFormat>,
        #[serde(rename = "exemplarCity")]
        pub exemplar_city: Cow<'static, str>,
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct LocationWithShort {
        pub long: Option<ZoneFormat>,
        pub short: ZoneFormat,
        #[serde(rename = "exemplarCity")]
        pub exemplar_city: Option<Cow<'static, str>>,
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct LocationWithLong {
        pub long: ZoneFormat,
        pub short: Option<ZoneFormat>,
        #[serde(rename = "exemplarCity")]
        pub exemplar_city: Option<Cow<'static, str>>,
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    #[serde(untagged)]
    pub enum Location {
        LocationE(LocationWithExemplarCity),
        LocationL(LocationWithLong),
        LocationS(LocationWithShort),
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    #[serde(untagged)]
    pub enum LocationOrSubRegion {
        Location(Location),
        SubRegion(LiteMap<Cow<'static, str>, Location>),
    }

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct Region(pub LiteMap<Cow<'static, str>, LocationOrSubRegion>);

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct Zones(pub LiteMap<Cow<'static, str>, Region>);

    #[derive(PartialEq, Debug, Clone, Deserialize)]
    pub struct TimeZoneNames {
        #[serde(rename = "hourFormat")]
        pub hour_format: Cow<'static, str>,
        #[serde(rename = "gmtFormat")]
        pub gmt_format: Cow<'static, str>,
        #[serde(rename = "gmtZeroFormat")]
        pub gmt_zero_format: Cow<'static, str>,
        #[serde(rename = "regionFormat")]
        pub region_format: Cow<'static, str>,
        #[serde(rename = "regionFormat-type-daylight")]
        pub region_format_daylight: Cow<'static, str>,
        #[serde(rename = "regionFormat-type-standard")]
        pub region_format_standard: Cow<'static, str>,
        #[serde(rename = "fallbackFormat")]
        pub fallback_format: Cow<'static, str>,
        pub zone: Zones,
        pub metazone: Option<MetaZones>,
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
    pub struct LangData(
        #[serde(with = "tuple_vec_map")] pub(crate) Vec<(CldrLangID, LangTimeZones)>,
    );

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}

impl From<cldr_json::TimeZoneNames> for timezones::TimeZonesV1 {
    fn from(other: cldr_json::TimeZoneNames) -> Self {
        Self {
            hour_format: other.hour_format,
            gmt_format: other.gmt_format,
            gmt_zero_format: other.gmt_zero_format,
            region_format: other.region_format,
            region_format_daylight: other.region_format_daylight,
            region_format_standard: other.region_format_standard,
            fallback_format: other.fallback_format,
            zones: other.zone.into(),
            metazones: other.metazone.map(timezones::MetaZonesV1::from),
        }
    }
}

impl From<cldr_json::MetaZones> for timezones::MetaZonesV1 {
    fn from(other: cldr_json::MetaZones) -> Self {
        Self(
            other
                .0
                .iter()
                .map(|(key, value)| (key.clone(), value.clone().into()))
                .collect(),
        )
    }
}

impl From<cldr_json::MetaZone> for timezones::MetaZoneV1 {
    fn from(other: cldr_json::MetaZone) -> Self {
        Self {
            long: other.long.map(timezones::ZoneFormatV1::from),
            short: other.short.map(timezones::ZoneFormatV1::from),
        }
    }
}

impl From<cldr_json::Zones> for timezones::ZonesV1 {
    fn from(other: cldr_json::Zones) -> Self {
        Self(
            other
                .0
                .into_iter()
                .flat_map(|(_, region)| region.0)
                .flat_map(|(key, place_or_region)| match place_or_region {
                    cldr_json::LocationOrSubRegion::Location(place) => vec![(key, place.into())],
                    cldr_json::LocationOrSubRegion::SubRegion(region) => region
                        .into_iter()
                        .map(|(key, place)| (key, place.into()))
                        .collect::<Vec<_>>(),
                })
                .collect(),
        )
    }
}

impl From<cldr_json::Location> for timezones::LocationV1 {
    fn from(other: cldr_json::Location) -> Self {
        match other {
            cldr_json::Location::LocationE(place) => place.into(),
            cldr_json::Location::LocationL(place) => place.into(),
            cldr_json::Location::LocationS(place) => place.into(),
        }
    }
}

impl From<cldr_json::LocationWithExemplarCity> for timezones::LocationV1 {
    fn from(other: cldr_json::LocationWithExemplarCity) -> Self {
        Self {
            exemplar_city: Some(other.exemplar_city.clone()),
            long: other.long.map(timezones::ZoneFormatV1::from),
            short: other.short.map(timezones::ZoneFormatV1::from),
        }
    }
}

impl From<cldr_json::LocationWithShort> for timezones::LocationV1 {
    fn from(other: cldr_json::LocationWithShort) -> Self {
        Self {
            exemplar_city: other.exemplar_city.clone(),
            long: other.long.map(timezones::ZoneFormatV1::from),
            short: Some(other.short.into()),
        }
    }
}

impl From<cldr_json::LocationWithLong> for timezones::LocationV1 {
    fn from(other: cldr_json::LocationWithLong) -> Self {
        Self {
            exemplar_city: other.exemplar_city.clone(),
            long: Some(other.long.into()),
            short: other.short.map(timezones::ZoneFormatV1::from),
        }
    }
}

impl From<&cldr_json::ZoneFormat> for timezones::ZoneFormatV1 {
    fn from(other: &cldr_json::ZoneFormat) -> Self {
        Self {
            generic: other.generic.clone(),
            standard: other.standard.clone(),
            daylight: other.daylight.clone(),
        }
    }
}

impl From<cldr_json::ZoneFormat> for timezones::ZoneFormatV1 {
    fn from(other: cldr_json::ZoneFormat) -> Self {
        ZoneFormatV1::from(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cldr_timezones() {
        use icu_locid_macros::langid;

        let json_str = std::fs::read_to_string(
            "/Users/enordin/src/cldr-json/cldr-json/cldr-dates-full/main/en/timeZoneNames.json",
        )
        .unwrap();
        let provider = TimeZonesProvider::try_from(json_str.as_str()).unwrap();

        let time_zones = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONES_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert!(time_zones.zones["Tucuman"]
            .exemplar_city
            .as_ref()
            .unwrap()
            .eq("Tucuman"));

        assert!(time_zones.zones["Sydney"]
            .exemplar_city
            .as_ref()
            .unwrap()
            .eq("Sydney"));

        let pacific = time_zones.metazones.as_ref().unwrap().get("America_Pacific").unwrap();
        assert!(pacific
            .long
            .as_ref()
            .unwrap()
            .daylight
            .as_ref()
            .unwrap()
            .eq("Pacific Daylight Time"));

        assert!(pacific
            .short
            .as_ref()
            .unwrap()
            .standard
            .as_ref()
            .unwrap()
            .eq("PST"));
    }
}
