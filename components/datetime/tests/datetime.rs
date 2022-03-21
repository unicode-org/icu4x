// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serialize")]

mod fixtures;
mod patterns;

use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, japanese::Japanese, AsCalendar, DateTime, Gregorian,
};
use icu_datetime::{
    mock::{parse_gregorian_from_str, zoned_datetime::MockZonedDateTime},
    pattern::runtime::Pattern,
    provider::{
        calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
        week_data::WeekDataV1Marker,
    },
    time_zone::TimeZoneFormat,
    CldrCalendar, DateTimeFormat, DateTimeFormatOptions, ZonedDateTimeFormat,
};
use icu_locid::{LanguageIdentifier, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::prelude::*;
use icu_provider::struct_provider::AnyPayloadProvider;
use patterns::{
    get_dayperiod_tests, get_time_zone_tests,
    structs::{
        dayperiods::DayPeriodExpectation,
        time_zones::{TimeZoneConfig, TimeZoneExpectation},
    },
};
use std::fmt::Write;
use tinystr::tinystr;

fn test_fixture(fixture_name: &str) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let japanese = Japanese::try_new(&provider).expect("Cannot load japanese data");
        let options = fixtures::get_options(&fx.input.options);
        let input_value = parse_gregorian_from_str(&fx.input.value).unwrap();
        let input_buddhist = input_value.to_calendar(Buddhist);
        let input_japanese = input_value.to_calendar(japanese);
        let input_coptic = input_value.to_calendar(Coptic);

        let description = match fx.description {
            Some(description) => {
                format!(
                    "\n  test: {:?}\n  file: {}.json\n",
                    description, fixture_name
                )
            }
            None => format!("\n  file: {}.json\n", fixture_name),
        };
        for (locale, output_value) in fx.output.values.into_iter() {
            if let Some(locale) = locale.strip_prefix("buddhist/") {
                assert_fixture_element(
                    locale,
                    &input_buddhist,
                    &output_value,
                    &provider,
                    &options,
                    &description,
                )
            } else if let Some(locale) = locale.strip_prefix("japanese/") {
                assert_fixture_element(
                    locale,
                    &input_japanese,
                    &output_value,
                    &provider,
                    &options,
                    &description,
                )
            } else if let Some(locale) = locale.strip_prefix("coptic/") {
                assert_fixture_element(
                    locale,
                    &input_coptic,
                    &output_value,
                    &provider,
                    &options,
                    &description,
                )
            } else {
                assert_fixture_element(
                    &locale,
                    &input_value,
                    &output_value,
                    &provider,
                    &options,
                    &description,
                )
            }
        }
    }
}

fn assert_fixture_element<A, D>(
    locale: &str,
    input_value: &DateTime<A>,
    output_value: &str,
    provider: &D,
    options: &DateTimeFormatOptions,
    description: &str,
) where
    A: AsCalendar,
    A::Calendar: CldrCalendar,
    D: ResourceProvider<DateSymbolsV1Marker>
        + ResourceProvider<DatePatternsV1Marker>
        + ResourceProvider<DateSkeletonPatternsV1Marker>
        + ResourceProvider<OrdinalV1Marker>
        + ResourceProvider<WeekDataV1Marker>,
{
    let locale: Locale = locale.parse().unwrap();
    let dtf = DateTimeFormat::<A::Calendar>::try_new(locale, provider, options).unwrap();
    let result = dtf.format_to_string(input_value);

    assert_eq!(result, output_value, "{}", description);

    let mut s = String::new();
    dtf.format_to_write(&mut s, input_value).unwrap();
    assert_eq!(s, output_value, "{}", description);

    let fdt = dtf.format(input_value);
    let s = fdt.to_string();
    assert_eq!(s, output_value, "{}", description);

    let mut s = String::new();
    write!(s, "{}", fdt).unwrap();
    assert_eq!(s, output_value, "{}", description);
}

fn test_fixture_with_time_zones(fixture_name: &str, config: TimeZoneConfig) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let options = fixtures::get_options(&fx.input.options);

        let mut input_value: MockZonedDateTime = fx.input.value.parse().unwrap();
        input_value.time_zone.time_zone_id = config.time_zone_id.clone();
        input_value.time_zone.metazone_id = config.metazone_id.clone();
        input_value.time_zone.time_variant = config.time_variant;

        let description = match fx.description {
            Some(description) => {
                format!(
                    "\n  test: {:?}\n  file: {}.json\n",
                    description, fixture_name
                )
            }
            None => format!("\n  file: {}.json\n", fixture_name),
        };
        for (locale, output_value) in fx.output.values.into_iter() {
            let locale: Locale = locale.parse().unwrap();
            let dtf = ZonedDateTimeFormat::<Gregorian>::try_new(
                locale, &provider, &provider, &provider, &options,
            )
            .unwrap();
            let result = dtf.format_to_string(&input_value);

            assert_eq!(result, output_value, "{}", description);

            let mut s = String::new();
            dtf.format_to_write(&mut s, &input_value).unwrap();
            assert_eq!(s, output_value, "{}", description);

            let fdt = dtf.format(&input_value);
            let s = fdt.to_string();
            assert_eq!(s, output_value, "{}", description);

            let mut s = String::new();
            write!(s, "{}", fdt).unwrap();
            assert_eq!(s, output_value, "{}", description);
        }
    }
}

#[test]
fn test_dayperiod_patterns() {
    let provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatOptions::default();
    for test in get_dayperiod_tests("dayperiods").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut patterns_data: DataPayload<DatePatternsV1Marker> = provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });
        let symbols_data: DataPayload<DateSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> = provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let week_data: DataPayload<WeekDataV1Marker> = provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: langid.region.map(|r| r.as_str().to_string().into()),
                    langid: None,
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        for test_case in &test.test_cases {
            for dt_input in &test_case.datetimes {
                let datetime = parse_gregorian_from_str(dt_input).unwrap();
                for DayPeriodExpectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        let new_pattern1: Pattern = pattern_input.parse().unwrap();
                        let new_pattern2: Pattern = pattern_input.parse().unwrap();
                        patterns_data.with_mut(move |data| {
                            data.time_h11_h12.long = new_pattern1;
                            data.time_h23_h24.long = new_pattern2;
                        });
                        let local_provider = MultiForkByKeyProvider {
                            providers: vec![
                                AnyPayloadProvider {
                                    key: DateSymbolsV1Marker::KEY,
                                    data: symbols_data.clone().wrap_into_any_payload(),
                                },
                                AnyPayloadProvider {
                                    key: DateSkeletonPatternsV1Marker::KEY,
                                    data: skeleton_data.clone().wrap_into_any_payload(),
                                },
                                AnyPayloadProvider {
                                    key: DatePatternsV1Marker::KEY,
                                    data: patterns_data.clone().wrap_into_any_payload(),
                                },
                                AnyPayloadProvider {
                                    key: WeekDataV1Marker::KEY,
                                    data: week_data.clone().wrap_into_any_payload(),
                                },
                            ],
                        };
                        let dtf = DateTimeFormat::<Gregorian>::try_new(
                            langid.clone(),
                            &local_provider.as_downcasting(),
                            &format_options,
                        )
                        .unwrap();
                        assert_eq!(
                            dtf.format(&datetime).to_string(),
                            *expected,
                            "\n\
                            locale:   `{}`,\n\
                            datetime: `{}`,\n\
                            pattern:  `{}`",
                            langid,
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
    let zone_provider = icu_testdata::get_provider();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut config = test.config;
        let mut datetime: MockZonedDateTime = test.datetime.parse().unwrap();
        datetime.time_zone.time_zone_id = config.time_zone_id.take();
        datetime.time_zone.metazone_id = config.metazone_id.take();
        datetime.time_zone.time_variant = config.time_variant.take();
        for TimeZoneExpectation {
            patterns: _,
            configs,
            expected,
        } in &test.expectations
        {
            for &config_input in configs {
                let tzf = TimeZoneFormat::try_from_config(
                    langid.clone(),
                    config_input.into(),
                    &zone_provider,
                )
                .unwrap();
                let mut buffer = String::new();
                tzf.format_to_write(&mut buffer, &datetime).unwrap();
                assert_eq!(
                    buffer.to_string(),
                    *expected,
                    "\n\
                    locale:   `{}`,\n\
                    datetime: `{}`,\n\
                    config: `{:?}`,\n\
                    ",
                    langid,
                    test.datetime,
                    config_input
                );
            }
        }
    }
}

#[test]
fn test_time_zone_patterns() {
    let date_provider = icu_testdata::get_provider();
    let plural_provider = icu_testdata::get_provider();
    let zone_provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatOptions::default();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut config = test.config;
        let mut datetime: MockZonedDateTime = test.datetime.parse().unwrap();
        datetime.time_zone.time_zone_id = config.time_zone_id.take();
        datetime.time_zone.metazone_id = config.metazone_id.take();
        datetime.time_zone.time_variant = config.time_variant.take();

        let mut patterns_data: DataPayload<DatePatternsV1Marker> = date_provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> = date_provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let symbols_data: DataPayload<DateSymbolsV1Marker> = date_provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid.clone()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let week_data: DataPayload<WeekDataV1Marker> = date_provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: langid.region.map(|r| r.as_str().to_string().into()),
                    langid: None,
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });

        for TimeZoneExpectation {
            patterns,
            configs: _,
            expected,
        } in &test.expectations
        {
            for pattern_input in patterns {
                let new_pattern1: Pattern = pattern_input.parse().unwrap();
                let new_pattern2: Pattern = pattern_input.parse().unwrap();
                patterns_data.with_mut(move |data| {
                    data.time_h11_h12.long = new_pattern1;
                    data.time_h23_h24.long = new_pattern2;
                });
                let local_provider = MultiForkByKeyProvider {
                    providers: vec![
                        AnyPayloadProvider {
                            key: DateSymbolsV1Marker::KEY,
                            data: symbols_data.clone().wrap_into_any_payload(),
                        },
                        AnyPayloadProvider {
                            key: DateSkeletonPatternsV1Marker::KEY,
                            data: skeleton_data.clone().wrap_into_any_payload(),
                        },
                        AnyPayloadProvider {
                            key: DatePatternsV1Marker::KEY,
                            data: patterns_data.clone().wrap_into_any_payload(),
                        },
                        AnyPayloadProvider {
                            key: WeekDataV1Marker::KEY,
                            data: week_data.clone().wrap_into_any_payload(),
                        },
                    ],
                };

                let dtf = ZonedDateTimeFormat::<Gregorian>::try_new(
                    langid.clone(),
                    &local_provider.as_downcasting(),
                    &zone_provider,
                    &plural_provider,
                    &format_options,
                )
                .unwrap();

                assert_eq!(
                    dtf.format(&datetime).to_string(),
                    *expected,
                    "\n\
                    locale:   `{}`,\n\
                    datetime: `{}`,\n\
                    pattern:  `{}`",
                    langid,
                    test.datetime,
                    pattern_input,
                );
            }
        }
    }
}

#[test]
fn test_length_fixtures() {
    // components/datetime/tests/fixtures/tests/lengths.json
    test_fixture("lengths");
    test_fixture_with_time_zones("lengths_with_zones", TimeZoneConfig::default());
    test_fixture_with_time_zones(
        "lengths_with_zones_from_pdt",
        TimeZoneConfig {
            metazone_id: Some(String::from("America_Pacific")),
            time_variant: Some(tinystr!(8, "daylight")),
            ..TimeZoneConfig::default()
        },
    );
}

#[test]
fn test_japanese() {
    // components/datetime/tests/fixtures/tests/japanese.json
    test_fixture("japanese");
}

#[test]
fn test_lengths_with_preferences() {
    // components/datetime/tests/fixtures/tests/lengths_with_preferences.json
    test_fixture("lengths_with_preferences");
}

/// Tests simple component::Bag.
#[test]
fn test_components() {
    // components/datetime/tests/fixtures/tests/components.json
    test_fixture("components");
}

/// Tests component::Bag configurations that have exact matches to CLDR skeletons.
#[test]
fn test_components_exact_matches() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-exact-matches");
}

#[test]
fn test_components_hour_cycle() {
    // components/datetime/tests/fixtures/tests/components_hour_cycle.json
    test_fixture("components_hour_cycle");
}

/// Tests that time zones are included, which rely on the append items mechanism.
#[test]
fn test_components_with_zones() {
    // components/datetime/tests/fixtures/tests/components_with_zones.json
    test_fixture_with_time_zones("components_with_zones", TimeZoneConfig::default());
}

/// Tests that component::Bags can adjust for width differences in the final pattern.
#[test]
fn test_components_width_differences() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-width-differences");
}

/// Tests that combine component::Bags options that don't exactly match a pattern.
#[test]
fn test_components_partial_matches() {
    // components/datetime/tests/fixtures/tests/components-partial-matches.json
    test_fixture("components-partial-matches");
}

/// Tests that component::Bags can combine a date skeleton, and a time skeleton.
#[test]
fn test_components_combine_datetime() {
    // components/datetime/tests/fixtures/tests/components-combine-datetime.json
    test_fixture("components-combine-datetime");
}

#[test]
fn constructing_datetime_format_with_time_zone_pattern_symbols_is_err() {
    use icu_datetime::{
        options::length::{Bag, Time},
        DateTimeFormatOptions,
    };
    use icu_locid::Locale;
    use icu_locid_macros::langid;

    let options = DateTimeFormatOptions::Length(Bag {
        // Full has time-zone symbols.
        time: Some(Time::Full),
        ..Default::default()
    });

    let locale: Locale = langid!("en").into();
    let provider = icu_testdata::get_provider();
    let result = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options);

    assert!(result.is_err());
}
