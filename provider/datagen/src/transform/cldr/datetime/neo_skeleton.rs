// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{provider::IterableDataProviderInternal, DatagenProvider};
use icu_datetime::neo_skeleton::{DateComponents, DateSkeleton};
use icu_datetime::pattern::runtime::PatternPlurals;
use icu_datetime::{
    neo_skeleton::{DayComponents, NeoComponents, NeoSkeleton, TimeComponents},
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
        for skeleton_components in DateComponents::VALUES {
            if matches!(skeleton_components, DateComponents::Quarter) || matches!(skeleton_components, DateComponents::YearQuarter) {
                patterns.push("'unimplemented'".parse().unwrap());
                continue;
            }
            let skeleton = DateSkeleton {
                length: icu_datetime::neo_skeleton::Length::Long,
                components: *skeleton_components,
            };
            let components_bag = skeleton
            .to_components_bag();
            let pattern_plurals = 
            components_bag.select_pattern(
                    skeletons_data.get(),
                    &length_patterns_data.get().length_combinations,
                )
                .unwrap();
            let pattern = match pattern_plurals {
                PatternPlurals::SinglePattern(pattern) => pattern,
                PatternPlurals::MultipleVariants(variants) => {
                    // TODO: Handle all of the variants
                    variants.other
                }
            };
            patterns.push(pattern);
        }
        let indices = vec![SkeletonDataIndex(0)];
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
