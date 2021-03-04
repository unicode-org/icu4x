// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::{TimeZoneNames, ZoneFormat};
use icu_datetime::provider::timezones::{TimeZoneFormatsV1, TimeZoneNameVariantsLongV1, TimeZoneNameVariantsShortV1, TimeZoneNameVariantsV1, TimeZoneNamesLongV1, TimeZoneNamesShortV1};
use std::collections::BTreeMap;

impl<'d> From<TimeZoneNames> for TimeZoneFormatsV1 {
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

impl<'d> From<TimeZoneNames> for TimeZoneNamesLongV1 {
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
                            .and_then(|long| long.0.get("generic"))
                            .map(|generic| (key.clone().into(), generic.clone().into()))
                    })
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for TimeZoneNamesShortV1 {
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
                            .and_then(|short| short.0.get("generic"))
                            .map(|generic| (key.clone().into(), generic.clone().into()))
                    })
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for TimeZoneNameVariantsLongV1 {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(BTreeMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .into_iter()
                    .filter_map(|(key, metazone)| {
                        metazone.long.map(|value| (key, value))
                    })
                    .map(|(key, value)| (key.into(), value.into()))
                    .collect(),
            ),
        }
    }
}

impl<'d> From<TimeZoneNames> for TimeZoneNameVariantsShortV1 {
    fn from(other: TimeZoneNames) -> Self {
        match other.metazone {
            None => Self(BTreeMap::new()),
            Some(metazones) => Self(
                metazones
                    .0
                    .into_iter()
                    .filter_map(|(key, metazone)| {
                        metazone.short.map(|value| (key, value))
                    })
                    .map(|(key, value)| (key.into(), value.into()))
                    .collect(),
            ),
        }
    }
}

impl<'d> From<ZoneFormat> for TimeZoneNameVariantsV1 {
    fn from(other: ZoneFormat) -> Self {
        Self(other.0.into_iter()
            .filter(|(key, _)| !key.eq("generic"))
            .map(|(key, value)| (key.into(), value.into()))
            .collect())
    }
}


