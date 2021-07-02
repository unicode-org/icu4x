// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{Location, TimeZoneNames, ZoneFormat};
use icu_datetime::provider::time_zones::{
    ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
    MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, MetaZoneSpecificNamesV1,
    TimeZoneFormatsV1,
};
use litemap::LiteMap;
use std::borrow::Cow;
use tinystr::TinyStr8;

fn parse_hour_format<'d>(hour_format: &str) -> (Cow<'d, str>, Cow<'d, str>) {
    // e.g. "+HH:mm;-HH:mm" -> ("+HH:mm", "-HH:mm")
    let index = hour_format.rfind(';').unwrap();
    let positive = String::from(&hour_format[0..index]);
    let negative = String::from(&hour_format[index + 1..]);
    (Cow::Owned(positive), Cow::Owned(negative))
}

impl<'d> From<TimeZoneNames> for TimeZoneFormatsV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        Self {
            hour_format: parse_hour_format(&other.hour_format),
            gmt_format: other.gmt_format.into(),
            gmt_zero_format: other.gmt_zero_format.into(),
            region_format: other.region_format.into(),
            region_format_variants: other
                .region_format_variants
                .into_iter()
                .map(|(key, value)| {
                    (
                        Cow::Owned(
                            key.parse::<TinyStr8>()
                                .expect("Time-zone variant was not compatible with TinyStr8"),
                        ),
                        value.into(),
                    )
                })
                .collect(),
            fallback_format: other.fallback_format.into(),
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
}

impl<'d> From<TimeZoneNames> for ExemplarCitiesV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        Self(
            other
                .zone
                .0
                .into_iter()
                .flat_map(|(key, region)| {
                    region
                        .0
                        .into_iter()
                        .flat_map(move |(inner_key, place_or_region)| {
                            let mut key = key.clone();
                            key.push('/');
                            key.push_str(&inner_key);
                            match place_or_region {
                                super::LocationOrSubRegion::Location(place) => place
                                    .exemplar_city()
                                    .map(|city| vec![(key.into(), city.into())])
                                    .unwrap_or_default(),
                                super::LocationOrSubRegion::SubRegion(region) => region
                                    .into_iter()
                                    .filter_map(|(inner_key, place)| {
                                        let mut key = key.clone();
                                        key.push('/');
                                        key.push_str(&inner_key);
                                        place.exemplar_city().map(|city| (key.into(), city.into()))
                                    })
                                    .collect::<Vec<_>>(),
                            }
                        })
                })
                .collect(),
        )
    }
}

impl<'d> From<TimeZoneNames> for MetaZoneGenericNamesLongV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(LiteMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .iter()
                    .filter_map(|(key, metazone)| {
                        metazone
                            .long
                            .as_ref()
                            .and_then(|long| {
                                if long.0.len() == 1 {
                                    long.0.values().next()
                                } else {
                                    long.0.get("generic")
                                }
                            })
                            .map(|generic| (key.clone().into(), generic.clone().into()))
                    })
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for MetaZoneGenericNamesShortV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(LiteMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .iter()
                    .filter_map(|(key, metazone)| {
                        metazone
                            .short
                            .as_ref()
                            .and_then(|short| {
                                if short.0.len() == 1 {
                                    short.0.values().next()
                                } else {
                                    short.0.get("generic")
                                }
                            })
                            .map(|generic| (key.clone().into(), generic.clone().into()))
                    })
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for MetaZoneSpecificNamesLongV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(LiteMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .into_iter()
                    .filter_map(|(key, metazone)| metazone.long.map(|value| (key, value)))
                    .map(|(key, value)| (key.into(), value.into()))
                    .filter(
                        |(_, value): &(Cow<'static, str>, MetaZoneSpecificNamesV1)| {
                            !value.is_empty()
                        },
                    )
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for MetaZoneSpecificNamesShortV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(LiteMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .into_iter()
                    .filter_map(|(key, metazone)| metazone.short.map(|value| (key, value)))
                    .map(|(key, value)| (key.into(), value.into()))
                    .filter(
                        |(_, value): &(Cow<'static, str>, MetaZoneSpecificNamesV1)| {
                            !value.is_empty()
                        },
                    )
                    .collect(),
            ),
        }
    }
}

impl<'d> From<ZoneFormat> for MetaZoneSpecificNamesV1<'d> {
    fn from(other: ZoneFormat) -> Self {
        let len = other.0.len();
        Self(
            other
                .0
                .into_iter()
                .filter(|(key, _)| len > 1 && !key.eq("generic"))
                .map(|(key, value)| {
                    (
                        Cow::Owned(
                            key.parse::<TinyStr8>()
                                .expect("Time-zone variant was not compatible with TinyStr8"),
                        ),
                        value.into(),
                    )
                })
                .collect(),
        )
    }
}
