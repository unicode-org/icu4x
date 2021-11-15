// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::provider::*;
use icu_locid::LanguageIdentifier;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::GREGORY_DATE_SYMBOLS_V1, //
];

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DateSymbolsProvider {
    data: Vec<(LanguageIdentifier, cldr_json::LangDates)>,
}

impl TryFrom<&dyn CldrPaths> for DateSymbolsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_dates()?.join("main");

        let locale_dirs = get_langid_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("ca-gregorian.json");

            let mut resource: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self { data })
    }
}

impl KeyedDataProvider for DateSymbolsProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::GREGORY_DATE_SYMBOLS_V1.match_key(*resc_key)
    }
}

impl DataProvider<gregory::DateSymbolsV1Marker> for DateSymbolsProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<gregory::DateSymbolsV1Marker>, DataError> {
        DateSymbolsProvider::supports_key(&req.resource_path.key)?;
        let langid = req.try_langid()?;
        let dates = match self.data.binary_search_by_key(&langid, |(lid, _)| lid) {
            Ok(idx) => &self.data[idx].1.dates,
            Err(_) => return Err(DataError::MissingResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(gregory::DateSymbolsV1::from(dates))),
        })
    }
}

icu_provider::impl_dyn_provider!(DateSymbolsProvider, {
    _ => gregory::DateSymbolsV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for DateSymbolsProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                // TODO: Avoid the clone
                langid: Some(l.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_json::Dates> for gregory::DateSymbolsV1 {
    fn from(other: &cldr_json::Dates) -> Self {
        Self {
            months: (&other.calendars.gregorian.months).into(),
            weekdays: (&other.calendars.gregorian.days).into(),
            day_periods: (&other.calendars.gregorian.day_periods).into(),
        }
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident $(,)?], [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl From<&cldr_json::$name::Symbols> for gregory::$name2::SymbolsV1 {
            fn from(other: &cldr_json::$name::Symbols) -> Self {
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
        impl From<&cldr_json::$name::Symbols> for gregory::$name2::SymbolsV1 {
            fn from(other: &cldr_json::$name::Symbols) -> Self {
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
        impl cldr_json::$name::Symbols {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl From<&cldr_json::$name::Contexts> for gregory::$name2::ContextsV1 {
            fn from(other: &cldr_json::$name::Contexts) -> Self {
                Self {
                    format: (&other.format).into(),
                    stand_alone: other.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&other.format)
                    }).map(|ref stand_alone| stand_alone.into())
                }
            }
        }

        impl cldr_json::$name::StandAloneWidths {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &cldr_json::$name::FormatWidths) -> Option<Self> {
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

        impl From<&cldr_json::$name::FormatWidths> for gregory::$name2::FormatWidthsV1 {
            fn from(other: &cldr_json::$name::FormatWidths) -> Self {
                Self {
                    abbreviated: (&other.abbreviated).into(),
                    narrow: (&other.narrow).into(),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: (&other.wide).into(),
                }
            }
        }

        impl From<&cldr_json::$name::StandAloneWidths> for gregory::$name2::StandAloneWidthsV1 {
            fn from(other: &cldr_json::$name::StandAloneWidths) -> Self {
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

    let cs_dates: DataPayload<gregory::DateSymbolsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_DATE_SYMBOLS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
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

    let cs_dates: DataPayload<gregory::DateSymbolsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_DATE_SYMBOLS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
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
