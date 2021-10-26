// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::{provider::*, skeleton::SkeletonError};
use icu_locid::LanguageIdentifier;
use icu_plurals::PluralCategory;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::GREGORY_DATE_SKELETON_PATTERNS_V1, //
];

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DateSkeletonPatternsProvider<'data> {
    data: Vec<(LanguageIdentifier, cldr_json::LangDates)>,
    _phantom: PhantomData<&'data ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for DateSkeletonPatternsProvider<'_> {
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

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'data> KeyedDataProvider for DateSkeletonPatternsProvider<'data> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::GREGORY_DATE_SKELETON_PATTERNS_V1.match_key(*resc_key)
    }
}

impl<'data> DataProvider<'data, gregory::DateSkeletonPatternsV1Marker>
    for DateSkeletonPatternsProvider<'data>
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, gregory::DateSkeletonPatternsV1Marker>, DataError> {
        DateSkeletonPatternsProvider::supports_key(&req.resource_path.key)?;
        let langid = req.try_langid()?;
        let dates = match self.data.binary_search_by_key(&langid, |(lid, _)| lid) {
            Ok(idx) => &self.data[idx].1.dates,
            Err(_) => return Err(DataError::MissingResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(
                gregory::DateSkeletonPatternsV1::from(&dates.calendars.gregorian.datetime_formats),
            )),
        })
    }
}

icu_provider::impl_dyn_provider!(DateSkeletonPatternsProvider<'data>, {
    _ => gregory::DateSkeletonPatternsV1Marker,
}, SERDE_SE, 'data);

impl<'data> IterableDataProviderCore for DateSkeletonPatternsProvider<'data> {
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

impl From<&cldr_json::DateTimeFormats> for gregory::DateSkeletonPatternsV1 {
    fn from(other: &cldr_json::DateTimeFormats) -> Self {
        use gregory::{patterns::PatternPluralsV1, SkeletonV1};
        use icu_datetime::pattern::reference::{Pattern, PatternPlurals, PluralPattern};
        use litemap::LiteMap;

        let mut skeletons = LiteMap::new();

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

            let pattern =
                Pattern::from_bytes(pattern_str as &str).expect("Unable to parse a pattern");

            match variant_parts.as_slice() {
                // A single pattern for a skeleton.
                // i.e. "${skeleton_str}" : "${pattern}",
                [] => {
                    skeletons.insert(skeleton_fields_v1, pattern.into());
                }
                // One of several pattern plural variants for a given skeleton.
                // i.e. "${skeleton_str}-count-${plural_category_str}" : "${pattern}",
                ["count", plural_category_str] => {
                    let plural_category = PluralCategory::from_tr35_string(plural_category_str)
                        .expect("Unable to parse a plural category");

                    match skeletons.get_mut(&skeleton_fields_v1) {
                        Some(PatternPluralsV1(PatternPlurals::MultipleVariants(v))) => {
                            v.add_variant(plural_category, pattern)
                        }
                        Some(PatternPluralsV1(PatternPlurals::SinglePattern(_))) => panic!(
                            "Skeleton {:?} has a mix of plural & non-plural variants",
                            skeleton_str
                        ),
                        None => {
                            skeletons.insert(
                                skeleton_fields_v1,
                                PluralPattern::new(plural_category, pattern)
                                    .expect("Unable to create a PluralPattern")
                                    .into(),
                            );
                        }
                    };
                }
                _ => eprintln!(
                    "This skeleton string is not yet supported: {:?}",
                    skeleton_str
                ),
            };
        }

        for (_, patterns) in skeletons.iter_mut() {
            patterns.0.normalize()
        }

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self(skeletons)
    }
}

#[test]
fn test_datetime_skeletons() {
    use gregory::SkeletonV1;
    use icu_datetime::pattern::reference::{Pattern, PluralPattern};
    use icu_locid_macros::langid;
    use icu_plurals::PluralCategory;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DateSkeletonPatternsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let skeletons: DataPayload<gregory::DateSkeletonPatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::GREGORY_DATE_SKELETON_PATTERNS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("fil")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();
    let skeletons = &skeletons.get().0;

    assert_eq!(
        Some(
            &Pattern::from_bytes("L")
                .expect("Failed to create pattern")
                .into()
        ),
        skeletons.get(&SkeletonV1::try_from("M").expect("Failed to create Skeleton"))
    );

    let mut expected = PluralPattern::new(
        PluralCategory::One,
        Pattern::from_bytes("'ika'-w 'linggo' 'ng' Y").expect("Failed to create pattern"),
    )
    .expect("Failed to create PatternPlurals");
    expected.add_variant(
        PluralCategory::Other,
        Pattern::from_bytes("'linggo' w 'ng' Y").expect("Failed to create pattern"),
    );
    assert_eq!(
        Some(&expected.into()),
        skeletons.get(&SkeletonV1::try_from("yw").expect("Failed to create Skeleton"))
    );
}
