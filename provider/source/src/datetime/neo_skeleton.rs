// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{IterableDataProviderCached, SourceDataProvider};
use either::Either;
use icu::datetime::neo_skeleton::{
    NeoCalendarPeriodComponents, NeoComponents, NeoDateComponents, NeoSkeletonLength,
    NeoTimeComponents,
};
use icu::datetime::options::DateTimeFormatterOptions;
use icu::datetime::options::{components, length, preferences};
use icu::datetime::provider::calendar::{DateLengthsV1, DateSkeletonPatternsV1, TimeLengthsV1};
use icu::datetime::provider::pattern::runtime;
use icu::datetime::provider::skeleton::PatternPlurals;
use icu::datetime::provider::*;
use icu::locale::extensions::unicode::{value, Value};
use icu::plurals::PluralElements;
use icu_provider::prelude::*;

use super::supported_cals;

impl SourceDataProvider {
    fn load_neo_skeletons_key<M, C>(
        &self,
        req: DataRequest,
        calendar: Either<&Value, &str>,
        from_id_str: impl Fn(&DataMarkerAttributes) -> Option<C>,
        to_components_bag: impl Fn(NeoSkeletonLength, &C, &DateLengthsV1) -> DateTimeFormatterOptions,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = PackedPatternsV1<'static>>,
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
    ) -> Result<PackedPatternsV1<'static>, DataError> {
        let data = self.get_datetime_resources(locale, calendar)?;

        let date_lengths_v1 = DateLengthsV1::from(&data);
        let time_lengths_v1 = TimeLengthsV1::from(&data);
        let skeleton_patterns = DateSkeletonPatternsV1::from(&data);

        fn expand_pp_to_pe(
            pp: PatternPlurals,
        ) -> PluralElements<icu::datetime::provider::pattern::runtime::Pattern> {
            match pp {
                PatternPlurals::MultipleVariants(variants) => PluralElements::new(variants.other)
                    .with_zero_value(variants.zero.clone())
                    .with_one_value(variants.one.clone())
                    .with_two_value(variants.two.clone())
                    .with_few_value(variants.few.clone())
                    .with_many_value(variants.many.clone()),
                PatternPlurals::SinglePattern(pattern) => PluralElements::new(pattern),
            }
        }

        let [long, medium, short] = [
            NeoSkeletonLength::Long,
            NeoSkeletonLength::Medium,
            NeoSkeletonLength::Short,
        ]
        .map(|length| to_components_bag(length, &neo_components, &date_lengths_v1))
        .map(|bag| {
            let pattern = expand_pp_to_pe(bag.select_pattern(
                &skeleton_patterns,
                &date_lengths_v1,
                &time_lengths_v1,
            ));
            match bag {
                DateTimeFormatterOptions::Components(
                    components @ components::Bag {
                        era: None,
                        year: Some(_),
                        ..
                    },
                ) => {
                    // TODO(#4478): Use CLDR data when it becomes available
                    // TODO: Set the length to NeoSkeletonLength? Or not, because
                    // the era should normally be displayed as short?
                    let mut components_with_full_year = components;
                    components_with_full_year.year = Some(components::Year::Numeric);
                    let mut components_with_era = components_with_full_year;
                    components_with_era.era = Some(components::Text::Short);
                    (
                        pattern,
                        Some(expand_pp_to_pe(
                            DateTimeFormatterOptions::Components(components_with_full_year)
                                .select_pattern(
                                    &skeleton_patterns,
                                    &date_lengths_v1,
                                    &time_lengths_v1,
                                ),
                        )),
                        Some(expand_pp_to_pe(
                            DateTimeFormatterOptions::Components(components_with_era)
                                .select_pattern(
                                    &skeleton_patterns,
                                    &date_lengths_v1,
                                    &time_lengths_v1,
                                ),
                        )),
                    )
                }
                DateTimeFormatterOptions::Components(
                    components @ components::Bag {
                        hour: Some(_),
                        ..
                    },
                ) => {
                    let mut components_with_minute = components;
                    components_with_minute.minute = Some(components::Numeric::Numeric);
                    let mut components_with_second = components;
                    components_with_second.minute = Some(components::Numeric::Numeric);
                    components_with_second.second = Some(components::Numeric::Numeric);
                    (
                        pattern,
                        Some(expand_pp_to_pe(
                            DateTimeFormatterOptions::Components(components_with_minute)
                                .select_pattern(
                                    &skeleton_patterns,
                                    &date_lengths_v1,
                                    &time_lengths_v1,
                                ),
                        )),
                        Some(expand_pp_to_pe(
                            DateTimeFormatterOptions::Components(components_with_second)
                                .select_pattern(
                                    &skeleton_patterns,
                                    &date_lengths_v1,
                                    &time_lengths_v1,
                                ),
                        )),
                    )
                }
                _ => (pattern, None, None),
            }
        });
        let builder = PackedPatternsBuilder {
            standard: LengthPluralElements {
                long: long.0.as_ref().map(runtime::Pattern::as_ref),
                medium: medium.0.as_ref().map(runtime::Pattern::as_ref),
                short: short.0.as_ref().map(runtime::Pattern::as_ref),
            },
            variant0: Some(LengthPluralElements {
                long: long
                    .1
                    .unwrap_or(long.0.as_ref().map(runtime::Pattern::as_ref)),
                medium: medium
                    .1
                    .unwrap_or(medium.0.as_ref().map(runtime::Pattern::as_ref)),
                short: short
                    .1
                    .unwrap_or(short.0.as_ref().map(runtime::Pattern::as_ref)),
            }),
            variant1: Some(LengthPluralElements {
                long: long
                    .2
                    .unwrap_or(long.0.as_ref().map(runtime::Pattern::as_ref)),
                medium: medium
                    .2
                    .unwrap_or(medium.0.as_ref().map(runtime::Pattern::as_ref)),
                short: short
                    .2
                    .unwrap_or(short.0.as_ref().map(runtime::Pattern::as_ref)),
            }),
        };
        Ok(builder.build())
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
                                | NeoTimeComponents::HourMinute
                                | NeoTimeComponents::HourMinuteSecond
                                | NeoTimeComponents::Hour12Minute
                                | NeoTimeComponents::Hour12MinuteSecond
                                | NeoTimeComponents::Hour24Minute
                                | NeoTimeComponents::Hour24MinuteSecond
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
                    .chain(
                        NeoCalendarPeriodComponents::VALUES
                            .iter()
                            .copied()
                            .map(NeoCalendarPeriodComponents::id_str),
                    )
                    .chain(NeoComponents::attributes_with_overrides().iter().copied())
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(attrs, locale.clone())
                    })
            })
            .collect())
    }
}

/// Convert from a semantic time field set to classical component options for calculating the pattern.
fn gen_time_components(
    length: NeoSkeletonLength,
    neo_components: &NeoTimeComponents,
    _: &DateLengthsV1<'_>,
) -> DateTimeFormatterOptions {
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
        | NeoTimeComponents::Hour12MinuteSecond => Some(preferences::Bag::from_hour_cycle(
            preferences::HourCycle::H12,
        )),
        NeoTimeComponents::Hour24
        | NeoTimeComponents::Hour24Minute
        | NeoTimeComponents::Hour24MinuteSecond => Some(preferences::Bag::from_hour_cycle(
            preferences::HourCycle::H23,
        )),
        _ => None,
    };
    DateTimeFormatterOptions::Components(filtered_components)
}

/// Convert from a semantic date field set to classical component options for calculating the pattern.
fn gen_date_components(
    length: NeoSkeletonLength,
    neo_components: &NeoComponents,
    date_lengths_v1: &DateLengthsV1<'_>,
) -> DateTimeFormatterOptions {
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
    if matches!(neo_components, NeoComponents::Date(NeoDateComponents::Auto)) {
        return DateTimeFormatterOptions::Length(length::Bag::from_date_style(
            length.to_date_style(),
        ));
    }
    if length == NeoSkeletonLength::Long
        && matches!(
            neo_components,
            NeoComponents::Date(NeoDateComponents::AutoWeekday)
        )
    {
        return DateTimeFormatterOptions::Length(length::Bag::from_date_style(length::Date::Full));
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
    if neo_components.has_month() {
        filtered_components.month = date_bag.month;
    }
    if neo_components.has_month() && !neo_components.has_year() && !neo_components.has_day() {
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
    if neo_components.has_hour() {
        filtered_components.hour = Some(components::Numeric::Numeric);
    }
    if neo_components.has_minute() {
        filtered_components.minute = Some(components::Numeric::Numeric);
    }
    if neo_components.has_second() {
        filtered_components.second = Some(components::Numeric::Numeric);
    }
    DateTimeFormatterOptions::Components(filtered_components)
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
            gen_time_components,
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
                    |id_str| {
                        NeoComponents::from_id_str(id_str)
                            .or_else(|| {
                                NeoDateComponents::from_id_str(id_str).map(NeoComponents::Date)
                            })
                            .or_else(|| {
                                NeoCalendarPeriodComponents::from_id_str(id_str)
                                    .map(NeoComponents::CalendarPeriod)
                            })
                    },
                    gen_date_components,
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

#[test]
fn test_en_year_patterns() {
    use icu::locale::locale;

    let provider = SourceDataProvider::new_testing();
    let payload: DataPayload<GregorianDateNeoSkeletonPatternsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                NeoDateComponents::YearMonthDay.id_str(),
                &locale!("en").into(),
            ),
            metadata: Default::default(),
        })
        .unwrap()
        .payload;

    let json_str = serde_json::to_string_pretty(payload.get()).unwrap();

    assert_eq!(
        json_str,
        r#"{
  "has_explicit_medium": true,
  "has_explicit_short": true,
  "variant_pattern_indices": [
    0,
    0,
    4,
    5,
    6,
    7
  ],
  "elements": [
    "MMMM d, y",
    "MMM d, y",
    "M/d/yy",
    "M/d/y",
    "MMMM d, y GGG",
    "MMM d, y GGG",
    "M/d/y GGG"
  ]
}"#
    );
}

#[test]
fn test_en_hour_patterns() {
    use icu::locale::locale;

    let provider = SourceDataProvider::new_testing();
    let payload: DataPayload<TimeNeoSkeletonPatternsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                NeoTimeComponents::Hour.id_str(),
                &locale!("en").into(),
            ),
            metadata: Default::default(),
        })
        .unwrap()
        .payload;

    let json_str = serde_json::to_string_pretty(payload.get()).unwrap();

    assert_eq!(
        json_str,
        r#"{
  "variant_pattern_indices": [
    2,
    2,
    2,
    3,
    3,
    3
  ],
  "elements": [
    "h a",
    "h:mm a",
    "h:mm:ss a"
  ]
}"#
    );
}
