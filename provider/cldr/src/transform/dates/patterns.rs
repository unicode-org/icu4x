// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::{provider::*, skeleton::SkeletonError};
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
pub struct DatePatternsProvider<'d> {
    data: Vec<(CldrLangID, cldr_json::LangDates)>,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
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

impl<'d> KeyedDataProvider for DatePatternsProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::DatePatterns || resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d, 's> DataProvider<'d, 's, gregory::DatePatternsV1Marker> for DatePatternsProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, gregory::DatePatternsV1Marker>, DataError> {
        DatePatternsProvider::supports_key(&req.resource_path.key)?;
        let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
        let dates = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.dates,
            Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
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

icu_provider::impl_dyn_provider!(DatePatternsProvider<'d>, {
    _ => gregory::DatePatternsV1Marker,
}, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for DatePatternsProvider<'d> {
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
                langid: Some(l.langid.clone()),
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

impl From<&cldr_json::DateTimeFormats> for gregory::patterns::DateTimeFormatsV1 {
    fn from(other: &cldr_json::DateTimeFormats) -> Self {
        use gregory::patterns::{PatternV1, SkeletonV1, SkeletonsV1};
        use litemap::LiteMap;

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            length_patterns: gregory::patterns::LengthPatternsV1 {
                full: Cow::Owned(other.full.get_pattern().clone()),
                long: Cow::Owned(other.long.get_pattern().clone()),
                medium: Cow::Owned(other.medium.get_pattern().clone()),
                short: Cow::Owned(other.short.get_pattern().clone()),
            },
            skeletons: {
                let mut skeletons = SkeletonsV1(LiteMap::new());

                // The CLDR keys for available_formats can have duplicate skeletons with either
                // an additional variant, or with multiple variants for different plurals.
                for (skeleton_str, pattern_str) in other.available_formats.0.iter() {
                    let mut unique_skeleton = None;
                    let mut variant_parts = Vec::new();

                    for part in skeleton_str.split('-') {
                        match unique_skeleton {
                            None => {
                                unique_skeleton = Some(part);
                            }
                            Some(_) => variant_parts.push(part),
                        }
                    }

                    let unique_skeleton = unique_skeleton.expect("Expected to find a skeleton.");

                    let skeleton_fields_v1 = match SkeletonV1::try_from(unique_skeleton) {
                        Ok(s) => s,
                        Err(err) => match err {
                            // Ignore unimplemented fields for now.
                            SkeletonError::SymbolUnimplemented(_) => continue,
                            _ => panic!("{:?} {}", unique_skeleton, err),
                        },
                    };

                    if !variant_parts.is_empty() {
                        eprintln!(
                            "This skeleton string is not yet supported: {:?}",
                            skeleton_str
                        );
                        continue;
                    }

                    let pattern_v1 = PatternV1::try_from(pattern_str as &str)
                        .expect("Unable to parse a pattern");

                    skeletons.0.insert(skeleton_fields_v1, pattern_v1);
                }

                skeletons
            },
        }
    }
}

impl From<&cldr_json::Dates> for gregory::DatePatternsV1 {
    fn from(other: &cldr_json::Dates) -> Self {
        Self {
            date: (&other.calendars.gregorian.date_formats).into(),
            time: (&other.calendars.gregorian.time_formats).into(),
            datetime: (&other.calendars.gregorian.datetime_formats).into(),
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
