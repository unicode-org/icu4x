// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::common::CommonDateProvider;
use crate::error::Error;
use litemap::LiteMap;

use crate::cldr_serde;
use crate::CldrPaths;
use icu_datetime::provider::calendar::*;

use crate::support::KeyedDataProvider;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use tinystr::{tinystr16, TinyStr16};

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DateSymbolsProvider(CommonDateProvider);

impl TryFrom<&dyn CldrPaths> for DateSymbolsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        CommonDateProvider::try_from(cldr_paths).map(DateSymbolsProvider)
    }
}

impl KeyedDataProvider for DateSymbolsProvider {
    fn supported_keys() -> Vec<ResourceKey> {
        vec![DateSymbolsV1Marker::KEY]
    }
}

impl ResourceProvider<DateSymbolsV1Marker> for DateSymbolsProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<DateSymbolsV1Marker>, DataError> {
        let dates = self.0.dates_for::<DateSymbolsV1Marker>(req)?;
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        let calendar =
            req.options.variant.as_ref().ok_or_else(|| {
                DataErrorKind::NeedsVariant.with_req(DateSymbolsV1Marker::KEY, req)
            })?;
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(convert_dates(dates, calendar))),
        })
    }
}

icu_provider::impl_dyn_provider!(DateSymbolsProvider, [DateSymbolsV1Marker,], SERDE_SE);

impl IterableProvider for DateSymbolsProvider {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        self.0.supported_options_for_key(resc_key)
    }
}

fn convert_dates(other: &cldr_serde::ca::Dates, calendar: &str) -> DateSymbolsV1<'static> {
    DateSymbolsV1 {
        months: (&other.months).into(),
        weekdays: (&other.days).into(),
        day_periods: (&other.day_periods).into(),
        eras: convert_eras(&other.eras, calendar),
    }
}

fn convert_eras(eras: &cldr_serde::ca::Eras, calendar: &str) -> Eras<'static> {
    let map = get_era_code_map(calendar);
    let mut out_eras = Eras::default();

    for (cldr, code) in map.into_tuple_vec().into_iter() {
        if let Some(name) = eras.names.get(&cldr) {
            out_eras.names.insert(&code, name);
        }
        if let Some(abbr) = eras.abbr.get(&cldr) {
            out_eras.abbr.insert(&code, abbr);
        }
        if let Some(narrow) = eras.narrow.get(&cldr) {
            out_eras.narrow.insert(&code, narrow);
        }
    }
    out_eras
}

fn get_era_code_map(calendar: &str) -> LiteMap<String, TinyStr16> {
    match calendar {
        "gregory" => vec![
            ("0".to_string(), tinystr16!("bc")),
            ("1".to_string(), tinystr16!("ad")),
        ]
        .into_iter()
        .collect(),
        "buddhist" => vec![("0".to_string(), tinystr16!("be"))]
            .into_iter()
            .collect(),
        "japanese" => crate::transform::calendar::japanese::get_era_code_map(),
        _ => panic!("Era map unknown for {}", calendar),
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident $(,)?], [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl From<&cldr_serde::ca::$name::Symbols> for $name2::SymbolsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Symbols) -> Self {
                Self([
                    $(
                        Cow::Owned(other.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$name, $name2]);
    };
    ([$name: ident, $name2: ident $(,)?], { $($element: ident),+ $(,)? } $(,)?) => {
        impl From<&cldr_serde::ca::$name::Symbols> for $name2::SymbolsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Symbols) -> Self {
                Self {
                    $(
                        $element: other.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$name, $name]);
    };
    ([$name: ident, $name2: ident]) => {
        impl cldr_serde::ca::$name::Symbols {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl From<&cldr_serde::ca::$name::Contexts> for $name2::ContextsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Contexts) -> Self {
                Self {
                    format: (&other.format).into(),
                    stand_alone: other.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&other.format)
                    }).map(|ref stand_alone| stand_alone.into())
                }
            }
        }

        impl cldr_serde::ca::$name::StandAloneWidths {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &cldr_serde::ca::$name::FormatWidths) -> Option<Self> {
                let abbreviated = self.abbreviated.as_ref().and_then(|v| v.get_unaliased(&other.abbreviated));
                let narrow = self.narrow.as_ref().and_then(|v| v.get_unaliased(&other.narrow));
                let short = if self.short == other.short {
                    None
                } else {
                    self.short.clone()
                };
                let wide = self.wide.as_ref().and_then(|v| v.get_unaliased(&other.wide));

                if abbreviated.is_none() && narrow.is_none() && wide.is_none() && short.is_none() {
                    None
                } else {
                    Some(Self {
                        abbreviated,
                        narrow,
                        short,
                        wide,
                    })
                }
            }
        }

        impl From<&cldr_serde::ca::$name::FormatWidths> for $name2::FormatWidthsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::FormatWidths) -> Self {
                Self {
                    abbreviated: (&other.abbreviated).into(),
                    narrow: (&other.narrow).into(),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: (&other.wide).into(),
                }
            }
        }

        impl From<&cldr_serde::ca::$name::StandAloneWidths> for $name2::StandAloneWidthsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::StandAloneWidths) -> Self {
                Self {
                    abbreviated: other.abbreviated.as_ref().map(|width| width.into()),
                    narrow: other.narrow.as_ref().map(|width| width.into()),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: other.wide.as_ref().map(|width| width.into()),
                }
            }
        }
    };
}

symbols_from!(
    [months, months],
    [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12],
);

symbols_from!([days, weekdays], [sun, mon, tue, wed, thu, fri, sat]);

symbols_from!(
    [
        day_periods,
        day_periods,
    ],
    {
        am,
        pm,
        noon,
        midnight,
    },
);

#[test]
fn test_basic() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DateSymbolsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("gregory".into()),
                langid: Some(langid!("cs")),
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!("srpna", cs_dates.get().months.format.wide.0[7]);

    assert_eq!(
        "po",
        cs_dates.get().weekdays.format.short.as_ref().unwrap().0[1]
    );
}

#[test]
fn unalias_contexts() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DateSymbolsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("gregory".into()),
                langid: Some(langid!("cs")),
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates.get().months.stand_alone.is_some());

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates
        .get()
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .abbreviated
        .is_none());
    assert!(cs_dates
        .get()
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .short
        .is_none());
    assert!(cs_dates
        .get()
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .narrow
        .is_none());
    assert!(cs_dates
        .get()
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .wide
        .is_some());

    // Czech weekdays are unaliased because they completely overlap.
    assert!(cs_dates.get().weekdays.stand_alone.is_none());
}
