// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{IterableDataProviderCached, SourceDataProvider};
use either::Either;
use icu::datetime::neo_skeleton::{
    NeoDateComponents, NeoDateSkeleton, NeoSkeletonLength, NeoTimeComponents, NeoTimeSkeleton,
};
use icu::datetime::options::components;
use icu::datetime::pattern::runtime::PatternPlurals;
use icu::datetime::provider::calendar::{DateLengthsV1, DateSkeletonPatternsV1, TimeLengthsV1};
use icu::datetime::provider::neo::TimeNeoSkeletonPatternsV1Marker;
use icu::datetime::provider::neo::*;
use icu::datetime::DateTimeFormatterOptions;
use icu::locale::extensions::unicode::{value, Value};
use icu::locale::LanguageIdentifier;
use icu_provider::prelude::*;

use super::supported_cals;

impl SourceDataProvider {
    fn load_neo_skeletons_key<M, C>(
        &self,
        req: DataRequest,
        calendar: Either<&Value, &str>,
        from_id_str: impl Fn(&DataMarkerAttributes) -> Option<C>,
        to_components_bag: impl Fn(NeoSkeletonLength, &C) -> DateTimeFormatterOptions,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<Yokeable = PackedSkeletonDataV1<'static>>,
        Self: crate::IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;
        let neo_components = from_id_str(req.id.marker_attributes)
            .expect("Skeleton data provider called with unknown skeleton");
        let packed_skeleton_data = self.make_packed_skeleton_data(
            &req.id.locale.get_langid(),
            calendar,
            neo_components,
            to_components_bag,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_skeleton_data),
        })
    }

    fn make_packed_skeleton_data<C>(
        &self,
        langid: &LanguageIdentifier,
        calendar: Either<&Value, &str>,
        neo_components: C,
        to_components_bag: impl Fn(NeoSkeletonLength, &C) -> DateTimeFormatterOptions,
    ) -> Result<PackedSkeletonDataV1<'static>, DataError> {
        let data = self.get_datetime_resources(langid, calendar)?;

        let date_lengths_v1 = DateLengthsV1::from(&data);
        let time_lengths_v1 = TimeLengthsV1::from(&data);
        let skeleton_patterns = DateSkeletonPatternsV1::from(&data);

        let mut patterns = vec![];

        let mut skeleton_data_index = SkeletonDataIndex {
            has_long: true,
            has_medium: true,
            has_plurals: false,
            has_eras: false,
        };
        let long_medium_short = [
            NeoSkeletonLength::Long,
            NeoSkeletonLength::Medium,
            NeoSkeletonLength::Short,
        ]
        .map(|length| to_components_bag(length, &neo_components))
        .map(|bag| {
            let pattern =
                bag.select_pattern(&skeleton_patterns, &date_lengths_v1, &time_lengths_v1);
            let pattern_with_era = match bag {
                DateTimeFormatterOptions::Components(
                    components @ components::Bag {
                        year: Some(_),
                        era: None,
                        ..
                    },
                ) => {
                    // TODO(#4478): Use CLDR data when it becomes available
                    // TODO: Set the length to NeoSkeletonLength? Or not, because
                    // the era should normally be displayed as short?
                    let mut components_with_era = component;
                    components_with_era.era = Some(components::Text::Short);
                    Some(
                        DateTimeFormatterOptions::Components(components_with_era).select_pattern(
                            &skeleton_patterns,
                            &date_lengths_v1,
                            &time_lengths_v1,
                        ),
                    )
                }
                _ => None,
            };
            // Assert that if there are multiple variants in `pattern_with_era`, then
            // there are also multiple variants in `pattern`
            if matches!(pattern_with_era, Some(PatternPlurals::MultipleVariants(_))) {
                assert!(matches!(pattern, PatternPlurals::MultipleVariants(_)));
            }
            (pattern, pattern_with_era)
        });
        let [long, medium, short] = if long_medium_short
            .iter()
            .any(|pp| matches!(pp.0, PatternPlurals::MultipleVariants(_)))
        {
            // Expand all variants to vector of length 6
            fn expand_pp_to_vec(
                pp: PatternPlurals,
            ) -> Vec<icu::datetime::pattern::runtime::Pattern> {
                match pp {
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
                }
            }
            skeleton_data_index.has_plurals = true;
            long_medium_short.map(|(pp, pp_with_era)| {
                (
                    expand_pp_to_vec(pp),
                    match pp_with_era {
                        Some(pp_with_era) => expand_pp_to_vec(pp_with_era),
                        None => vec![],
                    },
                )
            })
        } else {
            // Take a single variant of each pattern
            long_medium_short.map(|(pp, pp_with_era)| {
                (
                    match pp {
                        PatternPlurals::MultipleVariants(_) => unreachable!(),
                        PatternPlurals::SinglePattern(pattern) => vec![pattern],
                    },
                    match pp_with_era {
                        Some(PatternPlurals::MultipleVariants(_)) => unreachable!(),
                        Some(PatternPlurals::SinglePattern(pattern)) => vec![pattern],
                        None => vec![],
                    },
                )
            })
        };
        skeleton_data_index.has_eras = !long.1.is_empty();
        // Assert that the presense of the era pattern is the same in all lengths
        // TODO: Enable this assertion
        // assert_eq!(skeleton_data_index.has_eras, !medium.1.is_empty());
        // assert_eq!(skeleton_data_index.has_eras, !short.1.is_empty());
        // TODO: Remove the era patterns if the are equivalent to the non-era patterns?
        if long == medium {
            skeleton_data_index.has_long = false;
        } else {
            patterns.extend(long.0);
            patterns.extend(long.1);
        }
        if medium == short {
            skeleton_data_index.has_medium = false;
        } else {
            patterns.extend(medium.0);
            patterns.extend(medium.1);
        }
        patterns.extend(short.0);
        patterns.extend(short.1);

        Ok(PackedSkeletonDataV1 {
            index_info: skeleton_data_index,
            patterns: (&patterns).into(),
        })
    }

    fn neo_time_skeleton_supported_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates("generic")
            .list_langs()?
            .flat_map(|langid| {
                NeoTimeComponents::VALUES
                    .iter()
                    .filter(|neo_components| {
                        matches!(neo_components, NeoTimeComponents::Hour)
                            || matches!(neo_components, NeoTimeComponents::HourMinute)
                            || matches!(neo_components, NeoTimeComponents::HourMinuteSecond)
                            || matches!(neo_components, NeoTimeComponents::Auto)
                    })
                    .copied()
                    .map(NeoTimeComponents::id_str)
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(
                            attrs,
                            DataLocale::from(langid.clone()),
                        )
                    })
            })
            .collect())
    }

    fn neo_date_skeleton_supported_locales(
        &self,
        calendar: &Value,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let cldr_cal = supported_cals()
            .get(calendar)
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;

        Ok(self
            .cldr()?
            .dates(cldr_cal)
            .list_langs()?
            .flat_map(|langid| {
                NeoDateComponents::VALUES
                    .iter()
                    .copied()
                    .map(NeoDateComponents::id_str)
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(
                            attrs,
                            DataLocale::from(langid.clone()),
                        )
                    })
            })
            .collect())
    }
}

impl DataProvider<TimeNeoSkeletonPatternsV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimeNeoSkeletonPatternsV1Marker>, DataError> {
        self.load_neo_skeletons_key(
            req,
            Either::Right("generic"),
            NeoTimeComponents::from_id_str,
            |length, neo_components| {
                NeoTimeSkeleton::for_length_and_components(length, *neo_components)
                    .to_components_bag()
            },
        )
    }
}

impl IterableDataProviderCached<TimeNeoSkeletonPatternsV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.neo_time_skeleton_supported_locales()
    }
}

macro_rules! impl_neo_skeleton_datagen {
    ($marker:ident, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_skeletons_key(
                    req,
                    Either::Left(&value!($calendar)),
                    |id_str| NeoDateComponents::from_id_str(id_str),
                    |length, neo_components| {
                        NeoDateSkeleton::for_length_and_components(length, *neo_components)
                            .to_components_bag()
                    },
                )
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.neo_date_skeleton_supported_locales(&value!($calendar))
            }
        }
    };
}

impl_neo_skeleton_datagen!(BuddhistDateNeoSkeletonPatternsV1Marker, "buddhist");
impl_neo_skeleton_datagen!(ChineseDateNeoSkeletonPatternsV1Marker, "chinese");
impl_neo_skeleton_datagen!(CopticDateNeoSkeletonPatternsV1Marker, "coptic");
impl_neo_skeleton_datagen!(DangiDateNeoSkeletonPatternsV1Marker, "dangi");
impl_neo_skeleton_datagen!(EthiopianDateNeoSkeletonPatternsV1Marker, "ethiopic");
impl_neo_skeleton_datagen!(GregorianDateNeoSkeletonPatternsV1Marker, "gregory");
impl_neo_skeleton_datagen!(HebrewDateNeoSkeletonPatternsV1Marker, "hebrew");
impl_neo_skeleton_datagen!(IndianDateNeoSkeletonPatternsV1Marker, "indian");
impl_neo_skeleton_datagen!(IslamicDateNeoSkeletonPatternsV1Marker, "islamic");
impl_neo_skeleton_datagen!(JapaneseDateNeoSkeletonPatternsV1Marker, "japanese");
impl_neo_skeleton_datagen!(JapaneseExtendedDateNeoSkeletonPatternsV1Marker, "japanext");
impl_neo_skeleton_datagen!(PersianDateNeoSkeletonPatternsV1Marker, "persian");
impl_neo_skeleton_datagen!(RocDateNeoSkeletonPatternsV1Marker, "roc");
