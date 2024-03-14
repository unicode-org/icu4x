// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{provider::IterableDataProviderInternal, DatagenProvider};
use icu_datetime::pattern::runtime::PatternPlurals;
use icu_datetime::{
    neo_skeleton::{DayComponents, NeoComponents, NeoSkeleton, TimeComponents},
    provider::{
        calendar::{DateSkeletonPatternsV1Marker, GregorianDateLengthsV1Marker},
        neo::{GregorianDateSkeletonPatternsV1Marker, PackedSkeletonDataV1, SkeletonDataIndex},
    },
};
use icu_locid::extensions::unicode::value;
use icu_provider::prelude::*;

use super::supported_cals;

impl DataProvider<GregorianDateSkeletonPatternsV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<GregorianDateSkeletonPatternsV1Marker>, DataError> {
        let skel = NeoSkeleton {
            length: icu_datetime::neo_skeleton::Length::Long,
            components: NeoComponents::DateTime(
                DayComponents::MonthDay,
                TimeComponents::HourMinute,
            ),
            time_zone: None,
        };
        let skeletons_data: DataPayload<DateSkeletonPatternsV1Marker> =
            self.load(req)?.take_payload()?;
        let length_patterns_data: DataPayload<GregorianDateLengthsV1Marker> =
            self.load(req)?.take_payload()?;
        let pattern_plurals = skel
            .to_components_bag()
            .select_pattern(
                skeletons_data.get(),
                &length_patterns_data.get().length_combinations,
            )
            .unwrap();
        let PatternPlurals::SinglePattern(pattern) = pattern_plurals else {
            panic!()
        };
        let indices = vec![SkeletonDataIndex(0)];
        let patterns = vec![pattern];
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
