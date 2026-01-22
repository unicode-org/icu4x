// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

use crate::SourceDataProvider;

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

#[test]
fn test_en_year_patterns() {
    use icu::locale::locale;

    let provider = SourceDataProvider::new_testing();
    let payload: DataPayload<DatetimePatternsDateGregorianV1> = provider
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
    let payload: DataPayload<DatetimePatternsTimeV1> = provider
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
    let payload: DataPayload<DatetimePatternsDateGregorianV1> = provider
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
    "EEEE h a",
    "E h a",
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
/// See: <https://unicode-org.atlassian.net/browse/CLDR-14993>
#[cfg(feature = "networking")]
mod date_skeleton_consistency_tests {
    use super::*;
    use crate::datetime::DatagenCalendar;
    use crate::CoverageLevel;
    use icu::datetime::provider::pattern::{reference, runtime, CoarseHourCycle};
    use icu::datetime::provider::skeleton::reference::Skeleton;
    use std::collections::BTreeMap;

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
        skeleton_patterns: &'a BTreeMap<Skeleton, PluralElements<runtime::Pattern<'static>>>,
        preferred_hour_cycle: CoarseHourCycle,
        length_combinations_v1: &'a GenericLengthPatterns<'a>,
        cal: DatagenCalendar,
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
        let selected_pattern = select_pattern(
            components,
            data.skeleton_patterns,
            data.preferred_hour_cycle,
            data.length_combinations_v1,
        )
        .into_inner()
        .try_into_other()
        .unwrap();

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
            let cal = data.cal;
            let length = info.length;
            let in_available_formats = data.skeleton_pattern_set.contains(&pattern_for_lookup);
            println!(
                "{}\t{expected_pattern}\t{selected_pattern}\t{locale}\t{cal:?}\t{length}",
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
        cal: DatagenCalendar,
        locale: &DataLocale,
        strictness: TestStrictness,
    ) -> usize {
        let mut num_problems = 0;
        let data = provider.get_dates_resource(locale, Some(cal)).unwrap();
        let length_combinations_v1 = GenericLengthPatterns::from(&data.datetime_formats_at_time);
        let skeleton_patterns = data.datetime_formats.available_formats.parse_skeletons();
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
            preferred_hour_cycle: preferred_hour_cycle(data, locale),
            length_combinations_v1: &length_combinations_v1,
            cal,
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
                pattern: data.date_skeletons.short.get_pattern(),
                skeleton: data.date_skeletons.short.get_pattern(),
                length: "date-short",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_skeletons.medium.get_pattern(),
                skeleton: data.date_skeletons.medium.get_pattern(),
                length: "date-medum",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_skeletons.long.get_pattern(),
                skeleton: data.date_skeletons.long.get_pattern(),
                length: "date-long",
            },
        ) as usize;
        num_problems += !check_single_pattern(
            test_case_data,
            TestCaseInfo {
                pattern: data.date_skeletons.full.get_pattern(),
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
        let provider = SourceDataProvider::new();

        let mut num_problems = 0;
        for locale in provider
            .locales_for_coverage_levels([CoverageLevel::Modern])
            .unwrap()
        {
            num_problems += check_all_patterns_for_calendar_and_locale(
                &provider,
                DatagenCalendar::Gregorian,
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
        let provider = SourceDataProvider::new();

        let mut num_problems = 0;
        use DatagenCalendar::*;
        for cal in [
            Buddhist, Chinese, Coptic, Dangi, Ethiopic, Gregorian, Hebrew, Indian, Hijri, Japanese,
            Persian, Roc,
        ] {
            for locale in provider
                .locales_for_coverage_levels([CoverageLevel::Modern])
                .unwrap()
            {
                num_problems += check_all_patterns_for_calendar_and_locale(
                    &provider,
                    cal,
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
