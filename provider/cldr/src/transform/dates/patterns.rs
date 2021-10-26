// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::pattern::CoarseHourCycle;
use icu_datetime::{pattern, provider::*};
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::GREGORY_DATE_PATTERNS_V1, //
];

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DatePatternsProvider<'data> {
    data: Vec<(CldrLangID, cldr_json::LangDates)>,
    _phantom: PhantomData<&'data ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for DatePatternsProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_dates()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("ca-gregorian.json");

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

impl<'data> KeyedDataProvider for DatePatternsProvider<'data> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::GREGORY_DATE_PATTERNS_V1.match_key(*resc_key)
    }
}

impl<'data> DataProvider<'data, gregory::DatePatternsV1Marker> for DatePatternsProvider<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, gregory::DatePatternsV1Marker>, DataError> {
        DatePatternsProvider::supports_key(&req.resource_path.key)?;
        let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
        let dates = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.dates,
            Err(_) => return Err(DataError::MissingResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(gregory::DatePatternsV1::from(
                dates,
            ))),
        })
    }
}

icu_provider::impl_dyn_provider!(DatePatternsProvider<'data>, {
    _ => gregory::DatePatternsV1Marker,
}, SERDE_SE, 'data);

impl<'data> IterableDataProviderCore for DatePatternsProvider<'data> {
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

impl From<&cldr_json::LengthPatterns> for gregory::patterns::LengthPatternsV1 {
    fn from(other: &cldr_json::LengthPatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: Cow::Owned(other.full.get_pattern().clone()),
            long: Cow::Owned(other.long.get_pattern().clone()),
            medium: Cow::Owned(other.medium.get_pattern().clone()),
            short: Cow::Owned(other.short.get_pattern().clone()),
        }
    }
}

impl From<&cldr_json::DateTimeFormats> for gregory::patterns::LengthPatternsV1 {
    fn from(other: &cldr_json::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: Cow::Owned(other.full.get_pattern().clone()),
            long: Cow::Owned(other.long.get_pattern().clone()),
            medium: Cow::Owned(other.medium.get_pattern().clone()),
            short: Cow::Owned(other.short.get_pattern().clone()),
        }
    }
}

impl From<&cldr_json::Dates> for gregory::DatePatternsV1 {
    fn from(other: &cldr_json::Dates) -> Self {
        let length_combinations_v1 =
            gregory::patterns::LengthPatternsV1::from(&other.calendars.gregorian.datetime_formats);
        let skeletons_v1 =
            gregory::DateSkeletonPatternsV1::from(&other.calendars.gregorian.datetime_formats);

        let pattern_str_full = other.calendars.gregorian.time_formats.full.get_pattern();
        let pattern_str_long = other.calendars.gregorian.time_formats.long.get_pattern();
        let pattern_str_medium = other.calendars.gregorian.time_formats.medium.get_pattern();
        let pattern_str_short = other.calendars.gregorian.time_formats.short.get_pattern();

        use pattern::reference::Pattern;

        let pattern_full = Pattern::from_bytes(pattern_str_full)
            .expect("Failed to create a full Pattern from bytes.");
        let pattern_long = Pattern::from_bytes(pattern_str_long)
            .expect("Failed to create a long Pattern from bytes.");
        let pattern_medium = Pattern::from_bytes(pattern_str_medium)
            .expect("Failed to create a medium Pattern from bytes.");
        let pattern_short = Pattern::from_bytes(pattern_str_short)
            .expect("Failed to create a short Pattern from bytes.");

        let mut preferred_hour_cycle: Option<CoarseHourCycle> = None;
        let arr = [
            pattern::CoarseHourCycle::determine(&pattern_full),
            pattern::CoarseHourCycle::determine(&pattern_long),
            pattern::CoarseHourCycle::determine(&pattern_medium),
            pattern::CoarseHourCycle::determine(&pattern_short),
        ];
        let iter = arr.iter().flatten();
        for hour_cycle in iter {
            if let Some(preferred_hour_cycle) = preferred_hour_cycle {
                assert_eq!(
                    *hour_cycle, preferred_hour_cycle,
                    "A locale contained a mix of coarse hour cycle types"
                );
            } else {
                preferred_hour_cycle = Some(*hour_cycle);
            }
        }

        let preferred_hour_cycle =
            preferred_hour_cycle.expect("Could not find a preferred hour cycle.");
        let alt_hour_cycle = if preferred_hour_cycle == CoarseHourCycle::H11H12 {
            CoarseHourCycle::H23H24
        } else {
            CoarseHourCycle::H11H12
        };

        let (time_h11_h12, time_h23_h24) = {
            let time = (&other.calendars.gregorian.time_formats).into();
            let alt_time = gregory::patterns::LengthPatternsV1 {
                full: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_full,
                        pattern_full,
                    )
                    .expect("Failed to apply a coarse hour cycle to a full pattern.")
                    .into(),
                long: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_long,
                        pattern_long,
                    )
                    .expect("Failed to apply a coarse hour cycle to a long pattern.")
                    .into(),
                medium: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_medium,
                        pattern_medium,
                    )
                    .expect("Failed to apply a coarse hour cycle to a medium pattern.")
                    .into(),
                short: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_short,
                        pattern_short,
                    )
                    .expect("Failed to apply a coarse hour cycle to a short pattern.")
                    .into(),
            };

            match preferred_hour_cycle {
                CoarseHourCycle::H11H12 => (time, alt_time),
                CoarseHourCycle::H23H24 => (alt_time, time),
            }
        };

        Self {
            date: (&other.calendars.gregorian.date_formats).into(),
            time_h11_h12,
            time_h23_h24,
            preferred_hour_cycle,
            length_combinations: length_combinations_v1,
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatePatternsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: DataPayload<gregory::DatePatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!("d. M. y", cs_dates.get().date.medium);
}

#[test]
fn test_with_numbering_system() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatePatternsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let cs_dates: DataPayload<gregory::DatePatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("haw")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!("d MMM y", cs_dates.get().date.medium);
    // TODO(#308): Support numbering system variations. We currently throw them away.
    assert_eq!("d/M/yy", cs_dates.get().date.short);
}
