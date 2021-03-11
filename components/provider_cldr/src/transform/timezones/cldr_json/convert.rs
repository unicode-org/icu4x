// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::{Location, TimeZoneNames, ZoneFormat};
use icu_datetime::provider::timezones::{
    ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
    MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, MetaZoneSpecificNamesV1,
    TimeZoneFormatsV1,
};
use std::{borrow::Cow, collections::BTreeMap};

impl<'d> From<TimeZoneNames> for TimeZoneFormatsV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        Self {
            hour_format: other.hour_format.into(),
            gmt_format: other.gmt_format.into(),
            gmt_zero_format: other.gmt_zero_format.into(),
            region_format: other.region_format.into(),
            region_format_variants: other
                .region_format_variants
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
            fallback_format: other.fallback_format.into(),
        }
    }
}

impl Location {
    fn exemplar_city(&self) -> Option<Cow<'static, str>> {
        match self {
            Location::LocationE(place) => Some(Cow::Owned(place.exemplar_city.clone())),
            Location::LocationL(place) => place
                .exemplar_city
                .as_ref()
                .map(|city| Cow::Owned(city.clone())),
            Location::LocationS(place) => place
                .exemplar_city
                .as_ref()
                .map(|city| Cow::Owned(city.clone())),
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
                .flat_map(|(_, region)| region.0)
                .flat_map(|(key, place_or_region)| match place_or_region {
                    super::LocationOrSubRegion::Location(place) => place
                        .exemplar_city()
                        .map(|city| vec![(key.into(), city)])
                        .unwrap_or_default(),
                    super::LocationOrSubRegion::SubRegion(region) => region
                        .into_iter()
                        .filter_map(|(key, place)| {
                            place
                                .exemplar_city()
                                .map(|value| (key.into(), value.into()))
                        })
                        .collect::<Vec<_>>(),
                })
                .collect(),
        )
    }
}

impl<'d> From<TimeZoneNames> for MetaZoneGenericNamesLongV1<'d> {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(BTreeMap::new()),
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
            None => Self(BTreeMap::new()),
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
            None => Self(BTreeMap::new()),
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
            None => Self(BTreeMap::new()),
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
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}
