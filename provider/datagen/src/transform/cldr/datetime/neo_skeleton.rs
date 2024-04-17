// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::provider::{DatagenProvider, IterableDataProviderInternal};
use icu_datetime::neo_skeleton::{
    NeoDateComponents, NeoDateSkeleton, NeoSkeletonLength, NeoTimeComponents, NeoTimeSkeleton,
};
use icu_datetime::options::{components, preferences};
use icu_datetime::pattern::runtime::PatternPlurals;
use icu_datetime::pattern::CoarseHourCycle;
use icu_datetime::provider::calendar::TimeLengthsV1Marker;
use icu_datetime::provider::neo::TimeNeoSkeletonPatternsV1Marker;
use icu_datetime::provider::{
    calendar::{DateSkeletonPatternsV1Marker, GregorianDateLengthsV1Marker},
    neo::{
        GregorianDateDayNeoSkeletonPatternsV1Marker, GregorianDateExtNeoSkeletonPatternsV1Marker,
        PackedSkeletonDataV1, SkeletonDataIndex,
    },
};
use icu_locid::extensions::unicode::{key, value};
use icu_provider::prelude::*;

use super::supported_cals;

impl DataProvider<GregorianDateDayNeoSkeletonPatternsV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<GregorianDateDayNeoSkeletonPatternsV1Marker>, DataError> {
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req,
            NeoDateComponents::VALUES
                .iter()
                .take(9)
                .map(|neo_components| (neo_components, true)),
            |length, neo_components| {
                NeoDateSkeleton::for_length_and_components(length, **neo_components)
                    .to_components_bag()
            },
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(packed_skeleton_data)),
        })
    }
}

impl DataProvider<GregorianDateExtNeoSkeletonPatternsV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<GregorianDateExtNeoSkeletonPatternsV1Marker>, DataError> {
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req,
            NeoDateComponents::VALUES
                .iter()
                .skip(9)
                .map(|neo_components| {
                    (
                        neo_components,
                        !matches!(neo_components, NeoDateComponents::Quarter)
                            && !matches!(neo_components, NeoDateComponents::YearQuarter),
                    )
                }),
            |length, neo_components| {
                NeoDateSkeleton::for_length_and_components(length, **neo_components)
                    .to_components_bag()
            },
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(packed_skeleton_data)),
        })
    }
}

impl DataProvider<TimeNeoSkeletonPatternsV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimeNeoSkeletonPatternsV1Marker>, DataError> {
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req,
            NeoTimeComponents::VALUES.iter().map(|neo_components| {
                (
                    neo_components,
                    matches!(neo_components, NeoTimeComponents::Hour)
                        || matches!(neo_components, NeoTimeComponents::HourMinute)
                        || matches!(neo_components, NeoTimeComponents::HourMinuteSecond),
                )
            }),
            |length, neo_components| {
                NeoTimeSkeleton::for_length_and_components(length, **neo_components)
                    .to_components_bag()
            },
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(packed_skeleton_data)),
        })
    }
}

impl DatagenProvider {
    fn make_packed_skeleton_data<C>(
        &self,
        req: DataRequest,
        values: impl Iterator<Item = (C, bool)>,
        to_components_bag: impl Fn(NeoSkeletonLength, &C) -> components::Bag,
    ) -> Result<PackedSkeletonDataV1<'static>, DataError> {
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
        let time_lengths_v1: DataPayload<TimeLengthsV1Marker> = self
            .load(DataRequest {
                locale: req.locale,
                metadata: req.metadata,
            })?
            .take_payload()?;
        let mut patterns = vec![];
        let mut indices = vec![];
        for (neo_components, is_supported) in values {
            if !is_supported {
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
            let long_medium_short = [
                NeoSkeletonLength::Long,
                NeoSkeletonLength::Medium,
                NeoSkeletonLength::Short,
            ]
            .map(|length| to_components_bag(length, &neo_components))
            .map(|bag| {
                bag.select_pattern(
                    skeletons_data.get(),
                    &length_patterns_data.get().length_combinations,
                    match time_lengths_v1.get().preferred_hour_cycle {
                        CoarseHourCycle::H11H12 => preferences::HourCycle::H12,
                        CoarseHourCycle::H23H24 => preferences::HourCycle::H23,
                    },
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
        Ok(PackedSkeletonDataV1 {
            indices: indices.into_iter().collect(),
            patterns: (&patterns).into(),
        })
    }
}

impl IterableDataProviderInternal<GregorianDateDayNeoSkeletonPatternsV1Marker> for DatagenProvider {
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

impl IterableDataProviderInternal<GregorianDateExtNeoSkeletonPatternsV1Marker> for DatagenProvider {
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

impl IterableDataProviderInternal<TimeNeoSkeletonPatternsV1Marker> for DatagenProvider {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
        let mut r = HashSet::new();
        r.extend(self.cldr()?.dates("generic").list_langs()?.map(Into::into));
        Ok(r)
    }
}
