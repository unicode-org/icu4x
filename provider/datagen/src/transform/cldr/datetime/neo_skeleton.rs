// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{provider::IterableDataProviderInternal, DatagenProvider};
use icu_datetime::neo_skeleton::{DateComponents, DateSkeleton, Length};
use icu_datetime::options::components;
use icu_datetime::pattern::runtime::PatternPlurals;
use icu_datetime::{
    neo_skeleton::{DayComponents, NeoComponents, NeoSkeleton, TimeComponents},
    pattern::runtime::Pattern,
    provider::{
        calendar::{DateSkeletonPatternsV1Marker, GregorianDateLengthsV1Marker},
        neo::{GregorianDateSkeletonPatternsV1Marker, PackedSkeletonDataV1, SkeletonDataIndex},
    },
};
use icu_locid::extensions::unicode::{key, value};
use icu_provider::prelude::*;

use super::supported_cals;

impl DataProvider<GregorianDateSkeletonPatternsV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<GregorianDateSkeletonPatternsV1Marker>, DataError> {
        let mut skeletons_data_locale = req.locale.clone();
        skeletons_data_locale.set_unicode_ext(key!("ca"), value!("gregory"));
        let skeletons_data: DataPayload<DateSkeletonPatternsV1Marker> = self
            .load(DataRequest {
                locale: &skeletons_data_locale,
                metadata: req.metadata,
            })?
            .take_payload()?;
        let length_patterns_data: DataPayload<GregorianDateLengthsV1Marker> =
            self.load(req)?.take_payload()?;
        let mut patterns = vec![];
        let mut indices = vec![];
        for skeleton_components in DateComponents::VALUES {
            if matches!(skeleton_components, DateComponents::Quarter)
                || matches!(skeleton_components, DateComponents::YearQuarter)
            {
                indices.push(SkeletonDataIndex {
                    has_long: false,
                    has_medium: false,
                    has_plurals: false,
                    index: patterns.len().try_into().unwrap(),
                });
                patterns.push("'unimplemented'".parse().unwrap());
                continue;
            }
            let mut skeleton_data_index = SkeletonDataIndex {
                has_long: true,
                has_medium: true,
                has_plurals: false,
                index: patterns.len().try_into().unwrap(),
            };
            let long_medium_short = [Length::Long, Length::Medium, Length::Short]
                .map(|length| DateSkeleton {
                    length,
                    components: *skeleton_components,
                })
                .map(DateSkeleton::to_components_bag)
                .map(|bag| {
                    bag.select_pattern(
                        skeletons_data.get(),
                        &length_patterns_data.get().length_combinations,
                    )
                    .unwrap()
                });
            let [long, medium, short] = if long_medium_short
                .iter()
                .any(|pp| matches!(pp, PatternPlurals::MultipleVariants(_)))
            {
                // Expand all variants to vector of length 6
                skeleton_data_index.has_plurals = true;
                long_medium_short.map(|pp| match pp {
                    PatternPlurals::MultipleVariants(variants) => vec![
                        variants.zero.unwrap_or_else(|| variants.other.clone()),
                        variants.one.unwrap_or_else(|| variants.other.clone()),
                        variants.two.unwrap_or_else(|| variants.other.clone()),
                        variants.few.unwrap_or_else(|| variants.other.clone()),
                        variants.many.unwrap_or_else(|| variants.other.clone()),
                        variants.other,
                    ],
                    PatternPlurals::SinglePattern(pattern) => vec![
                        pattern.clone(),
                        pattern.clone(),
                        pattern.clone(),
                        pattern.clone(),
                        pattern.clone(),
                        pattern,
                    ],
                })
            } else {
                // Take a single variant of each pattern
                long_medium_short.map(|pp| match pp {
                    PatternPlurals::MultipleVariants(_) => unreachable!(),
                    PatternPlurals::SinglePattern(pattern) => vec![pattern],
                })
            };
            if long == medium {
                skeleton_data_index.has_long = false;
            } else {
                patterns.extend(long);
            }
            if medium == short {
                skeleton_data_index.has_medium = false;
            } else {
                patterns.extend(medium);
            }
            patterns.extend(short);
            indices.push(skeleton_data_index);
        }
        let packed_skeleton_data = PackedSkeletonDataV1 {
            indices: indices.into_iter().collect(),
            patterns: (&patterns).into(),
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(packed_skeleton_data)),
        })
    }
}

impl IterableDataProviderInternal<GregorianDateSkeletonPatternsV1Marker> for DatagenProvider {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
        let calendar = value!("gregory");
        let mut r = HashSet::new();

        let cldr_cal = supported_cals()
            .get(&calendar)
            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;
        r.extend(self.cldr()?.dates(cldr_cal).list_langs()?.map(Into::into));

        Ok(r)
    }
}
