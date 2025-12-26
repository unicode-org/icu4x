// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod patterns;

use fixtures::TestOutputItem;
use icu_calendar::cal::*;
use icu_calendar::AnyCalendarKind;
use icu_datetime::fieldsets::enums::*;
use icu_datetime::scaffold::CldrCalendar;
use icu_datetime::{
    pattern::DateTimePattern, pattern::FixedCalendarDateTimeNames, DateTimeFormatter,
    FixedCalendarDateTimeFormatter,
};
use icu_datetime::{
    preferences::{CalendarAlgorithm, HijriCalendarAlgorithm},
    DateTimeFormatterPreferences,
};
use icu_locale_core::{locale, Locale};
use icu_provider::prelude::*;
use icu_time::{
    zone::{IanaParser, UtcOffset},
    DateTime, TimeZoneInfo, ZonedDateTime,
};
use patterns::{
    dayperiods::{DayPeriodExpectation, DayPeriodTests},
    time_zones::TimeZoneTests,
};
use writeable::{assert_try_writeable_eq, assert_writeable_eq};

fn test_fixture(fixture_name: &str, file: &str) {
    for fx in serde_json::from_str::<fixtures::Fixture>(file)
        .expect("Unable to get fixture.")
        .0
    {
        let field_set = match fx.input.options.semantic {
            Some(semantic) => semantic.build_composite_datetime().unwrap(),
            None => {
                eprintln!("Warning: Skipping test with no semantic skeleton: {fx:?}");
                continue;
            }
        };
        let input = &fx.input.value;

        let description = fx
            .description
            .map(|d| format!("\n  test: {d:?}\n  file: {fixture_name}.json\n"))
            .unwrap_or_else(|| format!("\n  file: {fixture_name}.json\n"));

        for (locale, expected) in fx.output.values {
            let locale =
                Locale::try_from_str(&locale).expect("Expected parseable locale in fixture");
            let prefs = DateTimeFormatterPreferences::from(&locale);

            match prefs
                .calendar_algorithm
                .unwrap_or(CalendarAlgorithm::Gregory)
            {
                CalendarAlgorithm::Buddhist => assert_fixture_element(
                    prefs,
                    Buddhist,
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Chinese => assert_fixture_element(
                    prefs,
                    ChineseTraditional::new(),
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Coptic => {
                    assert_fixture_element(prefs, Coptic, input, &expected, field_set, &description)
                }
                CalendarAlgorithm::Dangi => assert_fixture_element(
                    prefs,
                    KoreanTraditional::new(),
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Ethiopic => assert_fixture_element(
                    prefs,
                    Ethiopian::new(),
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Ethioaa => assert_fixture_element(
                    prefs,
                    Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem),
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Gregory => assert_fixture_element(
                    prefs,
                    Gregorian,
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Hebrew => {
                    assert_fixture_element(prefs, Hebrew, input, &expected, field_set, &description)
                }
                CalendarAlgorithm::Indian => {
                    assert_fixture_element(prefs, Indian, input, &expected, field_set, &description)
                }
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                    assert_fixture_element(
                        prefs,
                        Hijri::new_tabular(
                            hijri::TabularAlgorithmLeapYears::TypeII,
                            hijri::TabularAlgorithmEpoch::Friday,
                        ),
                        input,
                        &expected,
                        field_set,
                        &description,
                    )
                }
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Rgsa)) => {
                    assert_fixture_element(
                        prefs,
                        Hijri::new_simulated_mecca(),
                        input,
                        &expected,
                        field_set,
                        &description,
                    )
                }
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                    assert_fixture_element(
                        prefs,
                        Hijri::new_tabular(
                            hijri::TabularAlgorithmLeapYears::TypeII,
                            hijri::TabularAlgorithmEpoch::Thursday,
                        ),
                        input,
                        &expected,
                        field_set,
                        &description,
                    )
                }
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                    assert_fixture_element(
                        prefs,
                        Hijri::new_umm_al_qura(),
                        input,
                        &expected,
                        field_set,
                        &description,
                    )
                }
                CalendarAlgorithm::Japanese => assert_fixture_element(
                    prefs,
                    Japanese::new(),
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Persian => assert_fixture_element(
                    prefs,
                    Persian,
                    input,
                    &expected,
                    field_set,
                    &description,
                ),
                CalendarAlgorithm::Roc => {
                    assert_fixture_element(prefs, Roc, input, &expected, field_set, &description)
                }
                _ => panic!("datetime test does not support locale {prefs:?}"),
            }
        }
    }
}

fn assert_fixture_element<C>(
    prefs: DateTimeFormatterPreferences,
    calendar: C,
    input: &str,
    expected: &TestOutputItem,
    field_set: CompositeDateTimeFieldSet,
    description: &str,
) where
    C: icu_calendar::Calendar + CldrCalendar + icu_calendar::IntoAnyCalendar + Clone,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::YearNamesV1>,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::MonthNamesV1>,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::SkeletaV1>,
{
    let iso_input = DateTime::try_from_str(input, Iso).unwrap();
    let input = DateTime::try_from_str(input, calendar).unwrap();

    let input = ZonedDateTime {
        date: input.date.clone(),
        time: input.time,
        zone: TimeZoneInfo::utc(),
    };
    let any_input = ZonedDateTime {
        date: input.date.clone().to_any(),
        time: input.time,
        zone: TimeZoneInfo::utc(),
    };
    let iso_any_input = ZonedDateTime {
        date: iso_input.date.to_any(),
        time: iso_input.time,
        zone: TimeZoneInfo::utc(),
    };

    let dtf = FixedCalendarDateTimeFormatter::try_new(prefs, field_set).expect(description);
    let any_dtf = DateTimeFormatter::try_new(prefs, field_set).expect(description);

    let output = dtf.format(&input);
    assert_writeable_eq!(output, expected.expectation(), "{}", description);
    let pattern = output.pattern();

    if let Some(expected_pattern) = expected.pattern() {
        assert_writeable_eq!(pattern, expected_pattern);
    }

    if matches!(
        input.date.calendar().kind(),
        AnyCalendarKind::HijriSimulatedMecca
    ) {
        // Not supported with FormattableAnyCalendar
        return;
    }

    let any_output = any_dtf.format_same_calendar(&any_input).unwrap();
    assert_writeable_eq!(
        any_output,
        expected.expectation(),
        "(DateTimeFormatter) {}",
        description
    );
    assert_eq!(pattern, any_output.pattern());

    let iso_any_output = any_dtf.format(&iso_any_input);
    assert_writeable_eq!(
        iso_any_output,
        expected.expectation(),
        "(DateTimeFormatter iso conversion) {}",
        description
    );
    assert_eq!(pattern, iso_any_output.pattern());
}

fn test_fixture_with_time_zones(fixture_name: &str, file: &str) {
    for fx in serde_json::from_str::<fixtures::Fixture>(file)
        .expect("Unable to get fixture.")
        .0
    {
        let fset = match fx.input.options.semantic {
            Some(semantic) => semantic.build_composite().unwrap(),
            None => {
                eprintln!("Warning: Skipping test with no semantic skeleton: {fx:?}");
                continue;
            }
        };

        let zoned_datetime =
            ZonedDateTime::try_lenient_from_str(&fx.input.value, Gregorian, IanaParser::new())
                .expect(&fx.input.value);

        let description = match fx.description {
            Some(description) => {
                format!("\n  test: {description:?}\n  file: {fixture_name}.json\n")
            }
            None => format!("\n  file: {fixture_name}.json\n"),
        };
        for (locale, output_value) in fx.output.values {
            let locale: Locale = locale.parse().unwrap();
            let dtf = {
                FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(locale.into(), fset)
                    .unwrap()
            };
            assert_writeable_eq!(
                dtf.format(&zoned_datetime),
                output_value.expectation(),
                "{}",
                description,
            );
        }
    }
}

#[test]
fn test_dayperiod_patterns() {
    for test in
        serde_json::from_str::<DayPeriodTests>(include_str!("patterns/tests/dayperiods.json"))
            .unwrap()
            .0
    {
        let locale: Locale = test.locale.parse().unwrap();
        for test_case in &test.test_cases {
            for dt_input in &test_case.datetimes {
                let datetime = DateTime::try_from_str(dt_input, Gregorian).unwrap();
                for DayPeriodExpectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        let parsed_pattern =
                            DateTimePattern::try_from_pattern_str(pattern_input).unwrap();
                        let mut pattern_formatter = FixedCalendarDateTimeNames::<
                            Gregorian,
                            CompositeDateTimeFieldSet,
                        >::try_new(
                            (&locale).into()
                        )
                        .unwrap();
                        let formatted_datetime = pattern_formatter
                            .include_for_pattern(&parsed_pattern)
                            .unwrap()
                            .format(&datetime);
                        assert_try_writeable_eq!(
                            formatted_datetime,
                            *expected,
                            Ok(()),
                            "\n\
                            locale:   `{}`,\n\
                            datetime: `{}`,\n\
                            pattern:  `{}`",
                            locale,
                            dt_input,
                            pattern_input,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_time_zone_format_configs() {
    for test in
        serde_json::from_str::<TimeZoneTests>(include_str!("patterns/tests/time_zones.json"))
            .unwrap()
            .0
    {
        let prefs: DateTimeFormatterPreferences = test.locale.parse::<Locale>().unwrap().into();
        let zoned_datetime =
            ZonedDateTime::try_lenient_from_str(&test.datetime, Gregorian, IanaParser::new())
                .expect(&test.datetime);
        for (pattern_input, expect) in &test.expectations {
            let Some(skeleton) = patterns::time_zones::pattern_to_semantic_skeleton(pattern_input)
            else {
                continue;
            };
            let tzf =
                FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, skeleton).unwrap();
            assert_writeable_eq!(
                tzf.format(&zoned_datetime.zone),
                *expect,
                "\n\
                    prefs:  `{:?}`,\n\
                    datetime: `{}`,\n\
                    config: `{:?}`,\n
                    ",
                prefs,
                test.datetime,
                pattern_input,
            );
        }
    }
}

#[test]
fn test_time_zone_format_offset_seconds() {
    use icu_datetime::fieldsets::zone::LocalizedOffsetLong;

    let tzf =
        FixedCalendarDateTimeFormatter::<(), _>::try_new(locale!("en").into(), LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(
        tzf.format(&UtcOffset::try_from_seconds(12).unwrap()),
        "GMT+00:00:12",
    );
}

#[test]
fn test_time_zone_format_offset_fallback() {
    use icu_datetime::fieldsets::zone::LocalizedOffsetLong;

    let tzf =
        FixedCalendarDateTimeFormatter::<(), _>::try_new(locale!("en").into(), LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(
        tzf.format(
            &IanaParser::new()
                .parse("America/Los_Angeles")
                .with_offset(None)
        ),
        "GMT+?",
    );
}

#[test]
fn test_time_zone_patterns() {
    for test in
        serde_json::from_str::<TimeZoneTests>(include_str!("patterns/tests/time_zones.json"))
            .unwrap()
            .0
    {
        let prefs: DateTimeFormatterPreferences = test.locale.parse::<Locale>().unwrap().into();
        let zoned_datetime =
            ZonedDateTime::try_lenient_from_str(&test.datetime, Gregorian, IanaParser::new())
                .expect(&test.datetime);

        for (pattern_input, expect) in &test.expectations {
            let parsed_pattern = DateTimePattern::try_from_pattern_str(pattern_input).unwrap();
            let mut pattern_formatter =
                FixedCalendarDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(prefs).unwrap();
            let formatted_datetime = pattern_formatter
                .include_for_pattern(&parsed_pattern)
                .unwrap()
                .format(&zoned_datetime);
            assert_writeable_eq!(
                writeable::adapters::LossyWrap(formatted_datetime),
                *expect,
                "\n\
                    prefs:  `{:?}`,\n\
                    datetime: `{}`,\n\
                    pattern:  `{}`",
                prefs,
                test.datetime,
                pattern_input,
            );
        }
    }
}

#[test]
fn test_length_fixtures() {
    test_fixture("lengths", include_str!("fixtures/tests/lengths.json"));
    test_fixture_with_time_zones(
        "lengths_with_zones",
        include_str!("fixtures/tests/lengths_with_zones.json"),
    );
    test_fixture_with_time_zones(
        "lengths_with_zones_from_pdt",
        include_str!("fixtures/tests/lengths_with_zones_from_pdt.json"),
    );
}

#[test]
fn test_japanese() {
    test_fixture("japanese", include_str!("fixtures/tests/japanese.json"));
}

#[test]
fn test_lengths_with_preferences() {
    test_fixture(
        "lengths_with_preferences",
        include_str!("fixtures/tests/lengths_with_preferences.json"),
    );
}

/// Tests simple component::Bag.
#[test]
fn test_components() {
    test_fixture("components", include_str!("fixtures/tests/components.json"));
}

/// Tests component::Bag configurations that have exact matches to CLDR skeletons.
#[test]
fn test_components_exact_matches() {
    test_fixture(
        "components-exact-matches",
        include_str!("fixtures/tests/components-exact-matches.json"),
    );
}

#[test]
fn test_components_hour_cycle() {
    test_fixture(
        "components_hour_cycle",
        include_str!("fixtures/tests/components_hour_cycle.json"),
    );
}

/// Tests that time zones are included, which rely on the append items mechanism.
#[test]
fn test_components_with_zones() {
    test_fixture_with_time_zones(
        "components_with_zones",
        include_str!("fixtures/tests/components_with_zones.json"),
    );
}

/// Tests that component::Bags can adjust for width differences in the final pattern.
#[test]
fn test_components_width_differences() {
    test_fixture(
        "components-width-differences",
        include_str!("fixtures/tests/components-width-differences.json"),
    );
}

/// Tests that combine component::Bags options that don't exactly match a pattern.
#[test]
fn test_components_partial_matches() {
    test_fixture(
        "components-partial-matches",
        include_str!("fixtures/tests/components-partial-matches.json"),
    );
}

/// Tests that component::Bags can combine a date skeleton, and a time skeleton.
#[test]
fn test_components_combine_datetime() {
    test_fixture(
        "components-combine-datetime",
        include_str!("fixtures/tests/components-combine-datetime.json"),
    );
}
