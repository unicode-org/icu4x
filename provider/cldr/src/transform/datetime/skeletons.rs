// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::common::CommonDateProvider;
use crate::error::Error;

use crate::cldr_serde;
use crate::CldrPaths;
use icu_datetime::provider::calendar::*;
use icu_datetime::skeleton::SkeletonError;

use icu_plurals::PluralCategory;
use icu_provider::iter::IterableResourceProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;

/// A data provider reading from CLDR JSON dates files.
#[derive(Debug)]
pub struct DateSkeletonPatternsProvider(CommonDateProvider);

impl TryFrom<&dyn CldrPaths> for DateSkeletonPatternsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        CommonDateProvider::try_from(cldr_paths).map(DateSkeletonPatternsProvider)
    }
}

impl ResourceProvider<DateSkeletonPatternsV1Marker> for DateSkeletonPatternsProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<DateSkeletonPatternsV1Marker>, DataError> {
        let dates = self.0.dates_for::<DateSkeletonPatternsV1Marker>(req)?;
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(DateSkeletonPatternsV1::from(
                &dates.datetime_formats,
            ))),
        })
    }
}

icu_provider::impl_dyn_provider!(
    DateSkeletonPatternsProvider,
    [DateSkeletonPatternsV1Marker,],
    SERDE_SE,
    impl DataConverter
);

impl IterableResourceProvider<DateSkeletonPatternsV1Marker> for DateSkeletonPatternsProvider {
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        self.0.supported_options()
    }
}

impl From<&cldr_serde::ca::DateTimeFormats> for DateSkeletonPatternsV1<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        use icu_datetime::{
            pattern::runtime::{PatternPlurals, PluralPattern},
            skeleton::reference::Skeleton,
        };
        use litemap::LiteMap;
        use std::collections::HashMap;

        let mut patterns: HashMap<String, HashMap<String, String>> = HashMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in other.available_formats.0.iter() {
            let (skeleton_str, plural_form_str) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, v),
                None => (skeleton_str.as_ref(), "other"),
            };

            patterns
                .entry(skeleton_str.to_string())
                .and_modify(|map| {
                    map.insert(plural_form_str.to_string(), pattern_str.to_string());
                })
                .or_insert_with(|| {
                    let mut map = HashMap::new();
                    map.insert(plural_form_str.to_string(), pattern_str.to_string());
                    map
                });
        }

        let mut skeletons = LiteMap::new();

        for (skeleton_str, patterns) in patterns {
            let skeleton = match Skeleton::try_from(skeleton_str.as_str()) {
                Ok(s) => s,
                Err(SkeletonError::SymbolUnimplemented(_)) => continue,
                Err(SkeletonError::SkeletonHasVariant) => continue,
                Err(err) => panic!(
                    "Unexpected skeleton error while parsing skeleton {:?} {}",
                    skeleton_str, err
                ),
            };

            let pattern_str = patterns.get("other").expect("Other variant must exist");
            let pattern = pattern_str.parse().expect("Unable to parse a pattern");

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
                    let pattern = pattern_str.parse().expect("Unable to parse a pattern");
                    plural_pattern.maybe_set_variant(cat, pattern);
                }
                PatternPlurals::MultipleVariants(plural_pattern)
            };
            // In some cases we may encounter duplicated patterns, which will
            // get deduplicated and result in a single-variant `MultiVariant` branch
            // here. The following `normalize` will turn those cases to `SingleVariant`.
            pattern_plurals.normalize();

            skeletons.insert(SkeletonV1(skeleton), pattern_plurals);
        }

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self(skeletons)
    }
}

#[test]
fn test_datetime_skeletons() {
    use icu_datetime::pattern::runtime::{Pattern, PluralPattern};
    use icu_locid::langid;
    use icu_plurals::PluralCategory;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DateSkeletonPatternsProvider::try_from(&cldr_paths as &dyn CldrPaths)
        .expect("Failed to retrieve provider");

    let skeletons: DataPayload<DateSkeletonPatternsV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("gregory".into()),
                langid: Some(langid!("fil")),
            },
            metadata: Default::default(),
        })
        .expect("Failed to load payload")
        .take_payload()
        .expect("Failed to retrieve payload");
    let skeletons = &skeletons.get().0;

    assert_eq!(
        Some(
            &"L".parse::<Pattern>()
                .expect("Failed to create pattern")
                .into()
        ),
        skeletons.get(&SkeletonV1::try_from("M").expect("Failed to create Skeleton"))
    );

    let mut expected = PluralPattern::new(
        "'linggo' w 'ng' Y"
            .parse()
            .expect("Failed to create pattern"),
    )
    .expect("Failed to create PatternPlurals");
    expected.maybe_set_variant(
        PluralCategory::One,
        "'ika'-w 'linggo' 'ng' Y"
            .parse()
            .expect("Failed to create pattern"),
    );
    assert_eq!(
        Some(&expected.into()),
        skeletons.get(&SkeletonV1::try_from("yw").expect("Failed to create Skeleton"))
    );
}
