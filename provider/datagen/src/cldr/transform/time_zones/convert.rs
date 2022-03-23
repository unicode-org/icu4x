// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::provider::time_zones::{
    ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
    MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, TimeZoneFormatsV1,
};
use std::borrow::Cow;
use tinystr::TinyStr8;
use zerovec::{ZeroMap, ZeroMap2d};

use crate::cldr::cldr_serde::{time_zones::time_zone_names::*, time_zones::CldrTimeZonesData};

/// Performs part 1 of type fallback as specified in the UTS-35 spec for TimeZone Goals:
/// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Goals
///
/// Part 2 of type fallback requires access to the IANA TimeZone Database
/// as well as a specific datetime context, so it is not relevant to DataProvier.
fn type_fallback(zone_format: &ZoneFormat) -> Option<&String> {
    zone_format
        .0
        .get("generic")
        .or_else(|| zone_format.0.get("standard"))
}

fn parse_hour_format(hour_format: &str) -> (Cow<'static, str>, Cow<'static, str>) {
    // e.g. "+HH:mm;-HH:mm" -> ("+HH:mm", "-HH:mm")
    #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let index = hour_format.rfind(';').unwrap();
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let positive = String::from(&hour_format[0..index]);
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let negative = String::from(&hour_format[index + 1..]);
    (Cow::Owned(positive), Cow::Owned(negative))
}

impl<'a> From<CldrTimeZonesData<'a>> for TimeZoneFormatsV1<'a> {
    fn from(other: CldrTimeZonesData) -> Self {
        let data = other.time_zone_names;
        Self {
            hour_format: parse_hour_format(&data.hour_format),
            gmt_format: data.gmt_format.clone().into(),
            gmt_zero_format: data.gmt_zero_format.clone().into(),
            region_format: data.region_format.clone().into(),
            region_format_variants: data
                .region_format_variants
                .iter()
                .map(|(key, value)| {
                    #[allow(clippy::expect_used)]
                    // TODO(#1668) Clippy exceptions need docs or fixing.
                    (
                        key.parse::<TinyStr8>()
                            .expect("Time-zone variant was not compatible with TinyStr8"),
                        value.clone(),
                    )
                })
                .collect(),
            fallback_format: data.fallback_format.clone().into(),
        }
    }
}

impl Location {
    fn exemplar_city(&self) -> Option<String> {
        match self {
            Self::LocationWithCity(place) => Some(place.exemplar_city.clone()),
            Self::LocationWithLong(place) => place.exemplar_city.clone(),
            Self::LocationWithShort(place) => place.exemplar_city.clone(),
        }
    }

    fn long_metazone_names(&self) -> Option<ZoneFormat> {
        match self {
            Self::LocationWithCity(place) => place.long.clone(),
            Self::LocationWithLong(place) => Some(place.long.clone()),
            Self::LocationWithShort(place) => place.long.clone(),
        }
    }

    fn short_metazone_names(&self) -> Option<ZoneFormat> {
        match self {
            Self::LocationWithCity(place) => place.short.clone(),
            Self::LocationWithLong(place) => place.short.clone(),
            Self::LocationWithShort(place) => Some(place.short.clone()),
        }
    }
}

impl<'a> From<CldrTimeZonesData<'a>> for ExemplarCitiesV1<'a> {
    fn from(other: CldrTimeZonesData) -> Self {
        let time_zone_names_data = other.time_zone_names;
        let bcp47_tzid_data = &other.bcp47_tzids;
        Self(
            time_zone_names_data
                .zone
                .0
                .iter()
                .flat_map(|(key, region)| {
                    region
                        .0
                        .iter()
                        .flat_map(move |(inner_key, place_or_region)| {
                            let mut key = key.clone();
                            key.push('/');
                            key.push_str(inner_key);
                            match place_or_region {
                                LocationOrSubRegion::Location(place) => {
                                    match bcp47_tzid_data.get(&key) {
                                        Some(bcp47) => place
                                            .exemplar_city()
                                            .map(|city| vec![(bcp47.clone(), city)])
                                            .unwrap_or_default(),
                                        None => panic!("Cannot find bcp47 for {:?}.", key),
                                    }
                                }
                                LocationOrSubRegion::SubRegion(region) => region
                                    .iter()
                                    .filter_map(|(inner_key, place)| {
                                        let mut key = key.clone();
                                        key.push('/');
                                        key.push_str(inner_key);
                                        match bcp47_tzid_data.get(&key) {
                                            Some(bcp47) => place
                                                .exemplar_city()
                                                .map(|city| (bcp47.clone(), city)),
                                            None => panic!("Cannot find bcp47 for {:?}.", key),
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                            }
                        })
                })
                .collect(),
        )
    }
}

macro_rules! long_short_impls {
    ($generic:ty, $specific:ty, $field:ident, $metazones_name:ident) => {
        impl<'a> From<CldrTimeZonesData<'a>> for $generic {
            fn from(other: CldrTimeZonesData) -> Self {
                let data = other.time_zone_names;
                Self {
                    defaults: match data.metazone {
                        None => ZeroMap::new(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| {
                                metazone
                                    .$field
                                    .as_ref()
                                    .and_then(type_fallback)
                                    .map(|format| (key.clone(), format.clone()))
                            })
                            .collect(),
                    },
                    overrides: data
                        .zone
                        .0
                        .iter()
                        .flat_map(|(key, region)| {
                            region
                                .0
                                .iter()
                                .flat_map(move |(inner_key, place_or_region)| {
                                    let mut key = key.clone();
                                    key.push('/');
                                    key.push_str(&inner_key);
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => place
                                            .$metazones_name()
                                            .and_then(|zf| type_fallback(&zf).cloned())
                                            .map(|format| vec![(key, format)])
                                            .unwrap_or_default(),
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let mut key = key.clone();
                                                key.push('/');
                                                key.push_str(&inner_key);
                                                place
                                                    .$metazones_name()
                                                    .and_then(|zf| type_fallback(&zf).cloned())
                                                    .map(|format| (key, format))
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .collect(),
                }
            }
        }

        impl<'a> From<CldrTimeZonesData<'a>> for $specific {
            fn from(other: CldrTimeZonesData) -> Self {
                let data = other.time_zone_names;
                Self {
                    defaults: match data.metazone {
                        None => ZeroMap2d::new(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| {
                                metazone
                                    .$field
                                    .as_ref()
                                    .map(|value| (key.clone(), value.clone()))
                            })
                            .flat_map(iterate_zone_format)
                            .collect(),
                    },
                    overrides: data
                        .zone
                        .0
                        .iter()
                        .flat_map(|(key, region)| {
                            region
                                .0
                                .iter()
                                .flat_map(move |(inner_key, place_or_region)| {
                                    let mut key = key.clone();
                                    key.push('/');
                                    key.push_str(&inner_key);
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => vec![place]
                                            .into_iter()
                                            .filter_map(|inner_place| {
                                                inner_place
                                                    .$metazones_name()
                                                    .map(|format| (key.clone(), format))
                                            })
                                            .collect::<Vec<_>>(),
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let mut key = key.clone();
                                                key.push('/');
                                                key.push_str(&inner_key);
                                                place.$metazones_name().map(|format| (key, format))
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .flat_map(iterate_zone_format)
                        .collect(),
                }
            }
        }
    };
}

long_short_impls!(
    MetaZoneGenericNamesLongV1<'_>,
    MetaZoneSpecificNamesLongV1<'_>,
    long,
    long_metazone_names
);

long_short_impls!(
    MetaZoneGenericNamesShortV1<'_>,
    MetaZoneSpecificNamesShortV1<'_>,
    short,
    short_metazone_names
);

fn iterate_zone_format(
    pair: (String, ZoneFormat),
) -> impl Iterator<Item = (String, TinyStr8, String)> {
    let (key1, zf) = pair;
    zf.0.into_tuple_vec()
        .into_iter()
        .filter(|(key, _)| !key.eq("generic"))
        .map(move |(key, value)| {
            (
                key1.clone(),
                #[allow(clippy::expect_used)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                key.parse::<TinyStr8>()
                    .expect("Time-zone variant was not compatible with TinyStr8"),
                value,
            )
        })
}
