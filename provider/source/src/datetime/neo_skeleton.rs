// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{IterableDataProviderCached, SourceDataProvider};
use either::Either;
use icu::datetime::neo_skeleton::{
    NeoDateComponents, NeoDayComponents, NeoSkeletonLength, NeoTimeComponents,
};
use icu::datetime::options::{components, length, preferences};
use icu::datetime::pattern::runtime::PatternPlurals;
use icu::datetime::provider::calendar::{DateLengthsV1, DateSkeletonPatternsV1, TimeLengthsV1};
use icu::datetime::provider::neo::TimeNeoSkeletonPatternsV1Marker;
use icu::datetime::provider::neo::*;
use icu::datetime::DateTimeFormatterOptions;
use icu::locale::extensions::unicode::{value, Value};
use icu_provider::prelude::*;

use super::supported_cals;

impl SourceDataProvider {
    fn load_neo_skeletons_key<M, C>(
        &self,
        req: DataRequest,
        calendar: Either<&Value, &str>,
        from_id_str: impl Fn(&DataMarkerAttributes) -> Option<C>,
        to_components_bag: impl Fn(NeoSkeletonLength, &C, &DateLengthsV1) -> DateTimeFormatterOptions,
        should_include_era: impl Fn(&C) -> bool,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = PackedSkeletonDataV1<'static>>,
        Self: crate::IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;
        let neo_components = from_id_str(req.id.marker_attributes)
            .expect("Skeleton data provider called with unknown skeleton");
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req.id.locale,
            calendar,
            neo_components,
            to_components_bag,
            should_include_era,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_skeleton_data),
        })
    }

    fn make_packed_skeleton_data<C>(
        &self,
        locale: &DataLocale,
        calendar: Either<&Value, &str>,
        neo_components: C,
        to_components_bag: impl Fn(NeoSkeletonLength, &C, &DateLengthsV1) -> DateTimeFormatterOptions,
        should_include_era: impl Fn(&C) -> bool,
    ) -> Result<PackedSkeletonDataV1<'static>, DataError> {
        let data = self.get_datetime_resources(locale, calendar)?;

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
        .map(|length| to_components_bag(length, &neo_components, &date_lengths_v1))
        .map(|bag| {
            let pattern =
                bag.select_pattern(&skeleton_patterns, &date_lengths_v1, &time_lengths_v1);
            // TODO: use should_include_era
            let pattern_with_era = if should_include_era(&neo_components) {
                match bag {
                    DateTimeFormatterOptions::Components(
                        components @ components::Bag { era: None, .. },
                    ) => {
                        // TODO(#4478): Use CLDR data when it becomes available
                        // TODO: Set the length to NeoSkeletonLength? Or not, because
                        // the era should normally be displayed as short?
                        let mut components_with_era = components;
                        components_with_era.era = Some(components::Text::Short);
                        Some(
                            DateTimeFormatterOptions::Components(components_with_era)
                                .select_pattern(
                                    &skeleton_patterns,
                                    &date_lengths_v1,
                                    &time_lengths_v1,
                                ),
                        )
                    }
                    _ => None,
                }
            } else {
                None
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
            .list_locales()?
            .flat_map(|locale| {
                NeoTimeComponents::VALUES
                    .iter()
                    .filter(|neo_components| {
                        !matches!(
                            neo_components,
                            NeoTimeComponents::DayPeriodHour12
                                | NeoTimeComponents::DayPeriodHour12Minute
                                | NeoTimeComponents::DayPeriodHour12MinuteSecond
                        )
                    })
                    .copied()
                    .map(NeoTimeComponents::id_str)
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(attrs, locale.clone())
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
            .list_locales()?
            .flat_map(|locale| {
                NeoDateComponents::VALUES
                    .iter()
                    .copied()
                    .map(NeoDateComponents::id_str)
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(attrs, locale.clone())
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
            |length, neo_components, _date_lengths_v1| {
                // TODO: Should this use timeSkeletons?
                // "full": "ahmmsszzzz",
                // "long": "ahmmssz",
                // "medium": "ahmmss",
                // "short": "ahmm"
                //
                // Probably depends on CLDR data being higher quality.
                // <https://unicode-org.atlassian.net/browse/CLDR-14993>
                if matches!(neo_components, NeoTimeComponents::Auto) {
                    return DateTimeFormatterOptions::Length(length::Bag::from_time_style(
                        length.to_time_style(),
                    ));
                }
                let mut filtered_components = components::Bag::empty();
                if neo_components.has_hour() {
                    filtered_components.hour = Some(components::Numeric::Numeric);
                }
                if neo_components.has_minute() {
                    filtered_components.minute = Some(components::Numeric::Numeric);
                }
                if neo_components.has_second() {
                    filtered_components.second = Some(components::Numeric::Numeric);
                }
                // Select the correct hour cycle
                filtered_components.preferences = match neo_components {
                    NeoTimeComponents::Hour12
                    | NeoTimeComponents::Hour12Minute
                    | NeoTimeComponents::Hour12MinuteSecond => Some(
                        preferences::Bag::from_hour_cycle(preferences::HourCycle::H12),
                    ),
                    NeoTimeComponents::Hour24
                    | NeoTimeComponents::Hour24Minute
                    | NeoTimeComponents::Hour24MinuteSecond => Some(
                        preferences::Bag::from_hour_cycle(preferences::HourCycle::H23),
                    ),
                    _ => None,
                };
                DateTimeFormatterOptions::Components(filtered_components)
            },
            |_| false,
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
                    |length, neo_components, date_lengths_v1| {
                        // Pull the field lengths from the date length patterns, and then use
                        // those lengths for classical skeleton datetime pattern generation.
                        //
                        // TODO: Should this use dateSkeletons?
                        // "full": "yMMMMEEEEd",
                        // "long": "yMMMMd",
                        // "medium": "yMMMd",
                        // "short": "yMMdd"
                        //
                        // Probably depends on CLDR data being higher quality.
                        // <https://unicode-org.atlassian.net/browse/CLDR-14993>
                        if matches!(
                            neo_components,
                            NeoDateComponents::Day(NeoDayComponents::Auto)
                        ) {
                            return DateTimeFormatterOptions::Length(length::Bag::from_date_style(
                                length.to_date_style(),
                            ));
                        }
                        if length == NeoSkeletonLength::Long
                            && matches!(
                                neo_components,
                                NeoDateComponents::Day(NeoDayComponents::AutoWeekday)
                            )
                        {
                            return DateTimeFormatterOptions::Length(length::Bag::from_date_style(
                                length::Date::Full,
                            ));
                        }
                        let date_pattern = match length {
                            NeoSkeletonLength::Long => &date_lengths_v1.date.long,
                            NeoSkeletonLength::Medium => &date_lengths_v1.date.medium,
                            NeoSkeletonLength::Short => &date_lengths_v1.date.short,
                            _ => unreachable!(),
                        };
                        let date_bag = components::Bag::from(date_pattern);
                        let mut filtered_components = components::Bag::empty();
                        if neo_components.has_year() {
                            filtered_components.era = date_bag.era;
                            filtered_components.year = date_bag.year;
                        }
                        if neo_components.has_full_year() {
                            // override the year field to be a full year
                            filtered_components.year = Some(components::Year::Numeric);
                        }
                        if neo_components.has_month() {
                            filtered_components.month = date_bag.month;
                        }
                        if neo_components.has_month()
                            && !neo_components.has_year()
                            && !neo_components.has_day()
                        {
                            // standalone month: use the skeleton length
                            filtered_components.month = match length {
                                NeoSkeletonLength::Long => Some(components::Month::Long),
                                NeoSkeletonLength::Medium => Some(components::Month::Short),
                                NeoSkeletonLength::Short => Some(components::Month::Numeric),
                                _ => unreachable!(),
                            };
                        }
                        if neo_components.has_day() {
                            filtered_components.day = date_bag.day;
                        }
                        if neo_components.has_day() && !neo_components.has_month() {
                            // override the day field to use the skeleton day length
                            filtered_components.day = Some(components::Day::NumericDayOfMonth);
                        }
                        if neo_components.has_weekday() {
                            // Not all length patterns have the weekday
                            filtered_components.weekday = match length {
                                NeoSkeletonLength::Long => Some(components::Text::Long),
                                NeoSkeletonLength::Medium => Some(components::Text::Short),
                                NeoSkeletonLength::Short => Some(components::Text::Short),
                                _ => unreachable!(),
                            };
                        }
                        DateTimeFormatterOptions::Components(filtered_components)
                    },
                    |neo_components| neo_components.has_full_year(),
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
