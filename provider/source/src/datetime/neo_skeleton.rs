// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::{cldr_serde, IterableDataProviderCached, SourceDataProvider};
use calendar::patterns::GenericLengthPatternsV1;
use either::Either;
use icu::datetime::fieldsets::enums::*;
use icu::datetime::options::Length;
use icu::datetime::provider::calendar::{DateSkeletonPatternsV1, TimeLengthsV1};
use icu::datetime::provider::fields::components;
use icu::datetime::provider::pattern::{reference, runtime};
use icu::datetime::provider::skeleton::PatternPlurals;
use icu::datetime::provider::*;
use icu::locale::extensions::unicode::{value, Value};
use icu::plurals::PluralElements;
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;
use icu_provider::prelude::*;

use super::supported_cals;

impl SourceDataProvider {
    fn load_neo_skeletons_key<M>(
        &self,
        req: DataRequest,
        calendar: Either<&Value, &str>,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
    ) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = PackedPatternsV1<'static>>,
        Self: crate::IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;
        // let neo_components = from_id_str(req.id.marker_attributes)
        //     .expect("Skeleton data provider called with unknown skeleton");
        let packed_skeleton_data = self.make_packed_skeleton_data(
            req.id.locale,
            calendar,
            req.id.marker_attributes,
            to_components_bag,
        )?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(packed_skeleton_data),
        })
    }

    fn make_packed_skeleton_data(
        &self,
        locale: &DataLocale,
        calendar: Either<&Value, &str>,
        attributes: &DataMarkerAttributes,
        to_components_bag: impl Fn(
            Length,
            &DataMarkerAttributes,
            &cldr_serde::ca::Dates,
        ) -> components::Bag,
    ) -> Result<PackedPatternsV1<'static>, DataError> {
        let data = self.get_datetime_resources(locale, calendar)?;

        let length_combinations_v1 = GenericLengthPatternsV1::from(&data.datetime_formats);
        let time_lengths_v1 = TimeLengthsV1::from(&data);
        let skeleton_patterns =
            DateSkeletonPatternsV1::from(&data.datetime_formats.available_formats);

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

        let [long, medium, short] = [Length::Long, Length::Medium, Length::Short]
            .map(|length| to_components_bag(length, attributes, &data))
            .map(|components| {
                // TODO: Use a Skeleton here in order to retain 'E' vs 'c'
                let pattern = expand_pp_to_pe(components.select_pattern(
                    &skeleton_patterns,
                    time_lengths_v1.preferred_hour_cycle,
                    &length_combinations_v1,
                ));
                match components {
                    components::Bag {
                        era: None,
                        year: Some(_),
                        ..
                    } => {
                        // TODO(#4478): Use CLDR data when it becomes available
                        // TODO: Set the length to NeoSkeletonLength? Or not, because
                        // the era should normally be displayed as short?
                        let mut components_with_full_year = components;
                        components_with_full_year.year = Some(components::Year::Numeric);
                        let mut components_with_era = components_with_full_year;
                        components_with_era.era = Some(components::Text::Short);
                        (
                            pattern,
                            Some(expand_pp_to_pe(components_with_full_year.select_pattern(
                                &skeleton_patterns,
                                time_lengths_v1.preferred_hour_cycle,
                                &length_combinations_v1,
                            ))),
                            Some(expand_pp_to_pe(components_with_era.select_pattern(
                                &skeleton_patterns,
                                time_lengths_v1.preferred_hour_cycle,
                                &length_combinations_v1,
                            ))),
                        )
                    }
                    components::Bag { hour: Some(_), .. } => {
                        let mut components_with_minute = components;
                        components_with_minute.minute = Some(components::Numeric::Numeric);
                        let mut components_with_second = components;
                        components_with_second.minute = Some(components::Numeric::Numeric);
                        components_with_second.second = Some(components::Numeric::Numeric);
                        (
                            pattern,
                            Some(expand_pp_to_pe(components_with_minute.select_pattern(
                                &skeleton_patterns,
                                time_lengths_v1.preferred_hour_cycle,
                                &length_combinations_v1,
                            ))),
                            Some(expand_pp_to_pe(components_with_second.select_pattern(
                                &skeleton_patterns,
                                time_lengths_v1.preferred_hour_cycle,
                                &length_combinations_v1,
                            ))),
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
                TimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES
                    .iter()
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
                DateFieldSet::ALL_DATA_MARKER_ATTRIBUTES
                    .iter()
                    .chain(CalendarPeriodFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
                    .chain(DateAndTimeFieldSet::ALL_DATA_MARKER_ATTRIBUTES.iter())
                    .map(move |attrs| {
                        DataIdentifierCow::from_borrowed_and_owned(attrs, locale.clone())
                    })
            })
            .collect())
    }
}

/// An internal function that checks if the attributes contain a field.
fn check_for_field(attributes: &DataMarkerAttributes, field: &str) -> bool {
    let f0 = field.as_bytes().first().unwrap();
    let f1 = field.as_bytes().get(1);
    let mut it = attributes.as_bytes().iter().peekable();
    while let Some(b) = it.next() {
        if b == f0 {
            let p = it.peek();
            if p == f1.as_ref() {
                return true;
            }
            if field.len() != 1 {
                return false;
            }
            let Some(q) = p else {
                // end of string
                return true;
            };
            if q.is_ascii_alphabetic() {
                return true;
            }
            // "m" != "m0"
            return false;
        }
    }
    false
}

#[test]
fn test_check_for_field() {
    assert!(check_for_field(
        DataMarkerAttributes::from_str_or_panic("ym0d"),
        "y"
    ));
    assert!(check_for_field(
        DataMarkerAttributes::from_str_or_panic("ym0d"),
        "m0"
    ));
    assert!(check_for_field(
        DataMarkerAttributes::from_str_or_panic("ym0d"),
        "d"
    ));
    assert!(!check_for_field(
        DataMarkerAttributes::from_str_or_panic("ym0d"),
        "y0"
    ));
    assert!(!check_for_field(
        DataMarkerAttributes::from_str_or_panic("ym0d"),
        "m"
    ));
    assert!(check_for_field(
        DataMarkerAttributes::from_str_or_panic("eh0"),
        "e"
    ));
    assert!(check_for_field(
        DataMarkerAttributes::from_str_or_panic("eh0"),
        "h0"
    ));
    assert!(!check_for_field(
        DataMarkerAttributes::from_str_or_panic("eh0"),
        "e0"
    ));
    assert!(!check_for_field(
        DataMarkerAttributes::from_str_or_panic("eh0"),
        "h"
    ));
}

/// Convert from a semantic time field set to classical component options for calculating the pattern.
fn gen_time_components(
    _: Length,
    attributes: &DataMarkerAttributes,
    _: &cldr_serde::ca::Dates,
) -> components::Bag {
    // TODO: Should this use timeSkeletons?
    // "full": "ahmmsszzzz",
    // "long": "ahmmssz",
    // "medium": "ahmmss",
    // "short": "ahmm"
    //
    // Probably depends on CLDR data being higher quality.
    // <https://unicode-org.atlassian.net/browse/CLDR-14993>
    let mut filtered_components = components::Bag::empty();
    filtered_components.hour = Some(components::Numeric::Numeric);
    // Select the correct hour cycle
    if check_for_field(attributes, "h") {
        filtered_components.hour_cycle = Some(HourCycle::H12);
    }
    if check_for_field(attributes, "h0") {
        filtered_components.hour_cycle = Some(HourCycle::H23);
    }
    filtered_components
}

/// Convert from a semantic date field set to classical component options for calculating the pattern.
fn gen_date_components(
    length: Length,
    attributes: &DataMarkerAttributes,
    data: &cldr_serde::ca::Dates,
) -> components::Bag {
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
    // TODO(#308): Utilize the numbering system pattern variations.
    let date_pattern: reference::Pattern = match length {
        Length::Long => data.date_formats.long.get_pattern().parse().unwrap(),
        Length::Medium => data.date_formats.medium.get_pattern().parse().unwrap(),
        Length::Short => data.date_formats.short.get_pattern().parse().unwrap(),
        _ => unreachable!(),
    };
    let date_bag = components::Bag::from(&date_pattern);
    let mut filtered_components = components::Bag::empty();
    if check_for_field(attributes, "y") {
        filtered_components.era = date_bag.era;
        filtered_components.year = date_bag.year;
    }
    if check_for_field(attributes, "m0") {
        filtered_components.month = date_bag.month;
    }
    if check_for_field(attributes, "m0")
        && !check_for_field(attributes, "y")
        && !check_for_field(attributes, "d")
    {
        // standalone month: use the skeleton length
        filtered_components.month = match length {
            Length::Long => Some(components::Month::Long),
            Length::Medium => Some(components::Month::Short),
            Length::Short => Some(components::Month::Numeric),
            _ => unreachable!(),
        };
    }
    if check_for_field(attributes, "d") {
        filtered_components.day = date_bag.day;
    }
    if check_for_field(attributes, "d") && !check_for_field(attributes, "m0") {
        // override the day field to use the skeleton day length
        filtered_components.day = Some(components::Day::NumericDayOfMonth);
    }
    if check_for_field(attributes, "e") {
        // Not all length patterns have the weekday
        filtered_components.weekday = match length {
            Length::Long => Some(components::Text::Long),
            Length::Medium => Some(components::Text::Short),
            Length::Short => Some(components::Text::Short),
            _ => unreachable!(),
        };
    }
    if check_for_field(attributes, "j") {
        filtered_components.hour = Some(components::Numeric::Numeric);
    }
    filtered_components
}

impl DataProvider<TimeNeoSkeletonPatternsV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimeNeoSkeletonPatternsV1Marker>, DataError> {
        self.load_neo_skeletons_key(req, Either::Right("generic"), gen_time_components)
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
                DataMarkerAttributes::from_str_or_panic("ym0d"),
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
                DataMarkerAttributes::from_str_or_panic("j"),
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

#[test]
fn test_en_overlap_patterns() {
    use icu::locale::locale;

    let provider = SourceDataProvider::new_testing();
    let payload: DataPayload<GregorianDateNeoSkeletonPatternsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("ej"),
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
  "variant_pattern_indices": [
    3,
    4,
    4,
    5,
    6,
    6
  ],
  "elements": [
    "cccc, h a",
    "ccc, h a",
    "EEEE h:m a",
    "E h:mm a",
    "EEEE h:m:s a",
    "E h:mm:ss a"
  ]
}"#
    );
}

/// This is a test that should eventually be moved to CLDR.
///
/// See: https://unicode-org.atlassian.net/browse/CLDR-14993
#[cfg(feature = "networking")]
#[cfg(test)]
mod date_skeleton_consistency_tests {
    use super::*;
    use crate::CoverageLevel;
    use icu::datetime::provider::fields;
    use pattern::CoarseHourCycle;

    /// When canonicalizing the pattern, normalize only (G=GGG) or be more aggressive
    /// (such as ignoring whitespace and certain punctuation characters)
    #[derive(Copy, Clone)]
    enum PatternCanonicalizationStrategy {
        NormalizeOnly,
        FlattenNumerics,
        Aggressive,
    }

    #[derive(Copy, Clone)]
    enum TestStrictness {
        Comprehensive,
        LowHangingFruit,
    }

    #[derive(Copy, Clone)]
    struct TestCaseFixedArgs<'a> {
        skeleton_patterns: &'a DateSkeletonPatternsV1<'a>,
        preferred_hour_cycle: CoarseHourCycle,
        length_combinations_v1: &'a GenericLengthPatternsV1<'a>,
        cldr_cal: &'a str,
        locale: &'a DataLocale,
        skeleton_pattern_set: &'a HashSet<String>,
        pattern_canonicalization_strategy: PatternCanonicalizationStrategy,
    }

    struct TestCaseInfo<'a> {
        pattern: &'a str,
        skeleton: &'a str,
        length: &'a str,
    }

    fn canonicalize_pattern(
        pattern: &mut reference::Pattern,
        strategy: PatternCanonicalizationStrategy,
    ) {
        use icu::datetime::provider::fields::{Field, FieldLength, FieldSymbol};
        use icu::datetime::provider::pattern::PatternItem;
        use PatternCanonicalizationStrategy::*;

        let mut items = core::mem::take(pattern).into_items();
        items.retain_mut(|item| {
            match (item, strategy) {
                (
                    PatternItem::Field(
                        ref mut field @ Field {
                            symbol: FieldSymbol::Era,
                            length: FieldLength::Three,
                        },
                    ),
                    NormalizeOnly | Aggressive | FlattenNumerics,
                ) => {
                    field.length = FieldLength::One;
                    true
                }
                // Ignore differences between 'y' and 'yy'?
                (
                    PatternItem::Field(
                        ref mut field @ Field {
                            length: FieldLength::Two,
                            ..
                        },
                    ),
                    Aggressive | FlattenNumerics,
                ) => {
                    field.length = FieldLength::One;
                    true
                }
                // TODO(#5892): For now, ignore differences between 'ccc', 'cccc', and 'EEE'
                (
                    PatternItem::Field(
                        ref mut field @ Field {
                            symbol: FieldSymbol::Weekday(fields::Weekday::StandAlone),
                            length: FieldLength::Four,
                        },
                    ),
                    Aggressive,
                ) => {
                    field.symbol = FieldSymbol::Weekday(fields::Weekday::Format);
                    field.length = FieldLength::Three;
                    true
                }
                // Ignore differences between 'MMM' and 'MMMM'?
                (
                    PatternItem::Field(
                        ref mut field @ Field {
                            length: FieldLength::Four,
                            ..
                        },
                    ),
                    Aggressive | FlattenNumerics,
                ) => {
                    field.length = FieldLength::Three;
                    true
                }
                // Ignore whitespace and ASCII punctuation?
                (PatternItem::Literal(' ' | '.' | ',' | '/' | '-'), Aggressive) => false,
                _ => true,
            }
        });
        *pattern = items.into();
    }

    /// Returns whether the check was successful.
    fn check_single_pattern(data: TestCaseFixedArgs, info: TestCaseInfo) -> bool {
        // TODO: Use a Skeleton here in order to retain 'E' vs 'c'
        let parsed_skeleton: reference::Pattern = info.skeleton.parse().unwrap();
        let components = components::Bag::from(&parsed_skeleton);
        let selected_pattern = match components.select_pattern(
            data.skeleton_patterns,
            data.preferred_hour_cycle,
            data.length_combinations_v1,
        ) {
            PatternPlurals::SinglePattern(x) => x,
            PatternPlurals::MultipleVariants(_) => unreachable!(),
        };

        // Canonicalize the two patterns to make comparison more precise
        let mut selected_pattern = reference::Pattern::from(&selected_pattern);
        canonicalize_pattern(
            &mut selected_pattern,
            data.pattern_canonicalization_strategy,
        );
        let selected_pattern = runtime::Pattern::from(&selected_pattern).to_string();
        let mut expected_pattern: reference::Pattern = info.pattern.parse().unwrap();
        let mut pattern_for_lookup = expected_pattern.clone();
        canonicalize_pattern(
            &mut expected_pattern,
            data.pattern_canonicalization_strategy,
        );
        let expected_pattern = runtime::Pattern::from(&expected_pattern).to_string();
        canonicalize_pattern(
            &mut pattern_for_lookup,
            PatternCanonicalizationStrategy::FlattenNumerics,
        );
        let pattern_for_lookup = runtime::Pattern::from(&pattern_for_lookup).to_string();

        // Check if there is a match
        if expected_pattern != selected_pattern {
            let locale = data.locale;
            let cal = data.cldr_cal;
            let length = info.length;
            let in_available_formats = data.skeleton_pattern_set.contains(&pattern_for_lookup);
            println!(
                "{}\t{expected_pattern}\t{selected_pattern}\t{locale}\t{cal}\t{length}",
                if in_available_formats {
                    "MATCH"
                } else {
                    "MISSING"
                }
            );
            // Don't return an error if there is no match in available formats!
            !in_available_formats
        } else {
            true
        }
    }

    fn check_all_patterns_for_calendar_and_locale(
        provider: &SourceDataProvider,
        cldr_cal: &str,
        locale: &DataLocale,
        strictness: TestStrictness,
    ) -> usize {
        let mut num_problems = 0;
        let data = provider
            .get_datetime_resources(locale, Either::Right(cldr_cal))
            .unwrap();
        let length_combinations_v1 = GenericLengthPatternsV1::from(&data.datetime_formats);
        let time_lengths_v1 = TimeLengthsV1::from(&data);
        let skeleton_patterns =
            DateSkeletonPatternsV1::from(&data.datetime_formats.available_formats);
        let skeleton_pattern_set = data
            .datetime_formats
            .available_formats
            .0
            .values()
            .map(|pattern_str| {
                let mut pattern: reference::Pattern = pattern_str.parse().unwrap();
                // always use FlattenNumerics mode for availableFormats lookup
                canonicalize_pattern(
                    &mut pattern,
                    PatternCanonicalizationStrategy::FlattenNumerics,
                );
                runtime::Pattern::from(&pattern).to_string()
            })
            .collect::<HashSet<_>>();
        let test_case_data = TestCaseFixedArgs {
            skeleton_patterns: &skeleton_patterns,
            preferred_hour_cycle: time_lengths_v1.preferred_hour_cycle,
            length_combinations_v1: &length_combinations_v1,
            cldr_cal,
            locale,
            skeleton_pattern_set: &skeleton_pattern_set,
            pattern_canonicalization_strategy: match strictness {
                TestStrictness::Comprehensive => PatternCanonicalizationStrategy::NormalizeOnly,
                TestStrictness::LowHangingFruit => PatternCanonicalizationStrategy::Aggressive,
            },
        };
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_formats.short.get_pattern(),
                skeleton: data.date_skeletons.short.get_pattern(),
                length: "date-short",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_formats.medium.get_pattern(),
                skeleton: data.date_skeletons.medium.get_pattern(),
                length: "date-medum",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_formats.long.get_pattern(),
                skeleton: data.date_skeletons.long.get_pattern(),
                length: "date-long",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_formats.full.get_pattern(),
                skeleton: data.date_skeletons.full.get_pattern(),
                length: "date-full",
            },
        ) as usize;
        // TODO: Also check time? Date seems more impactful in the short term
        num_problems
    }

    #[test]
    fn gregorian_only() {
        // NOTE: This test is intended to run over all modern locales
        let provider = SourceDataProvider::new_latest_tested();

        let mut num_problems = 0;
        for locale in provider
            .locales_for_coverage_levels([CoverageLevel::Modern])
            .unwrap()
        {
            num_problems += check_all_patterns_for_calendar_and_locale(
                &provider,
                "gregorian",
                &locale,
                TestStrictness::LowHangingFruit,
            );
        }
        if num_problems != 0 {
            panic!("{num_problems} problems");
        }
    }

    #[test]
    #[ignore]
    fn all_calendars() {
        // NOTE: This test is intended to run over all modern locales
        let provider = SourceDataProvider::new_latest_tested();

        let mut num_problems = 0;
        for (_calendar, cldr_cal) in supported_cals().iter() {
            for locale in provider
                .locales_for_coverage_levels([CoverageLevel::Modern])
                .unwrap()
            {
                num_problems += check_all_patterns_for_calendar_and_locale(
                    &provider,
                    cldr_cal,
                    &locale,
                    TestStrictness::Comprehensive,
                );
            }
        }
        if num_problems != 0 {
            panic!("{num_problems} problems");
        }
    }
}
