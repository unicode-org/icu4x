// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::{provider::*, skeleton::SkeletonError};
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
    data: Vec<(CldrLangID, cldr_json::LangDates)>,
    _phantom: PhantomData<&'data ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for DateSkeletonPatternsProvider<'_> {
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
                langid: Some(l.langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_json::DateTimeFormats> for gregory::DateSkeletonPatternsV1 {
    fn from(other: &cldr_json::DateTimeFormats) -> Self {
        use gregory::{patterns::PatternPluralsV1, SkeletonV1};
        use icu_datetime::{
            pattern::reference::{Pattern, PatternPlurals, PluralPattern},
            skeleton::reference::Skeleton,
        };
        use litemap::LiteMap;
        use std::collections::HashMap;

        let mut patterns: HashMap<String, HashMap<String, String>> = HashMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in other.available_formats.0.iter() {
            let (skeleton_str, variant_str) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, v),
                None => (skeleton_str.as_ref(), "other"),
            };

            patterns
                .entry(skeleton_str.to_string())
                .and_modify(|map| {
                    map.insert(variant_str.to_string(), pattern_str.to_string());
                })
                .or_insert_with(|| {
                    let mut map = HashMap::new();
                    map.insert(variant_str.to_string(), pattern_str.to_string());
                    map
                });
        }

        let mut skeletons = LiteMap::new();

        for (skeleton_str, patterns) in patterns {
            let skeleton = match Skeleton::try_from(skeleton_str.as_str()) {
                Ok(s) => s,
                Err(SkeletonError::SymbolUnimplemented(_)) => continue,
                Err(err) => panic!(
                    "Unexpected skeleton error while parsing skeleton {:?} {}",
                    skeleton_str, err
                ),
            };

            let pattern_str = patterns.get("other").expect("Other variant must exist");
            let pattern = Pattern::from_bytes(pattern_str).expect("Unable to parse a pattern");

            let mut pattern_plurals = if patterns.len() == 1 {
                PatternPlurals::SinglePattern(pattern)
            } else {
                let mut plural_pattern =
                    PluralPattern::new(pattern).expect("Unable to construct PluralPattern");

                for (key, pattern_str) in patterns {
                    if key == "other" {
                        continue;
                    }
                    let cat = PluralCategory::from_tr35_string(&key)
                        .expect("Failed to retrieve plural category");
                    let pattern =
                        Pattern::from_bytes(&pattern_str).expect("Unable to parse a pattern");
                    plural_pattern.maybe_set_variant(cat, pattern);
                }
                PatternPlurals::MultipleVariants(plural_pattern)
            };
            // In some cases we may encounter duplicated patterns, which will
            // get deduplicated and result in a single-variant `MultiVariant` branch
            // here. The following `normalize` will turn those cases to `SingleVariant`.
            pattern_plurals.normalize();

            skeletons.insert(SkeletonV1(skeleton), PatternPluralsV1(pattern_plurals));
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
        Pattern::from_bytes("'linggo' w 'ng' Y").expect("Failed to create pattern"),
    )
    .expect("Failed to create PatternPlurals");
    expected.maybe_set_variant(
        PluralCategory::One,
        Pattern::from_bytes("'ika'-w 'linggo' 'ng' Y").expect("Failed to create pattern"),
    );
    assert_eq!(
        Some(&expected.into()),
        skeletons.get(&SkeletonV1::try_from("yw").expect("Failed to create Skeleton"))
    );
}
