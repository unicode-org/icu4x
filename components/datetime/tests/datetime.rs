// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

mod fixtures;
mod patterns;

use icu_datetime::{
    mock::{datetime::MockDateTime, zoned_datetime::MockZonedDateTime},
    DateTimeFormatOptions, ZonedDateTimeFormat,
};
use icu_datetime::{
    provider::{gregory::DatesV1, key::GREGORY_V1},
    DateTimeFormat,
};
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::prelude::*;
use icu_provider::struct_provider::StructProvider;
use patterns::{
    get_dayperiod_tests, get_time_zone_tests,
    structs::{
        dayperiods::DayPeriodExpectation,
        time_zones::{TimeZoneConfig, TimeZoneExpectation},
    },
};
use std::fmt::Write;

fn test_fixture(fixture_name: &str) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let locale: Locale = fx.input.locale.parse().unwrap();
        let options = fixtures::get_options(&fx.input.options);

        let dtf = DateTimeFormat::try_new(locale, &provider, &options).unwrap();
        let value: MockDateTime = fx.input.value.parse().unwrap();

        let result = dtf.format_to_string(&value);
        match fx.description {
            Some(description) => assert_eq!(
                result, fx.output.value,
                "expected {:?} to equal {:?} – {}",
                result, fx.output.value, description
            ),
            None => assert_eq!(result, fx.output.value),
        }

        let mut s = String::new();
        dtf.format_to_write(&mut s, &value).unwrap();
        assert_eq!(s, fx.output.value, "\n file: {}.json\n", fixture_name);

        let fdt = dtf.format(&value);
        let s = fdt.to_string();
        assert_eq!(s, fx.output.value, "\n file: {}.json\n", fixture_name);

        let mut s = String::new();
        write!(s, "{}", fdt).unwrap();
        assert_eq!(s, fx.output.value, "\n file: {}.json\n", fixture_name);
    }
}

fn test_fixture_with_time_zones(fixture_name: &str, config: TimeZoneConfig) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let locale: Locale = fx.input.locale.parse().unwrap();
        let options = fixtures::get_options(&fx.input.options);

        let dtf = ZonedDateTimeFormat::try_new(locale, &provider, &provider, &options).unwrap();

        let mut value: MockZonedDateTime = fx.input.value.parse().unwrap();
        value.time_zone.time_zone_id = config.time_zone_id.clone();
        value.time_zone.metazone_id = config.metazone_id.clone();
        value.time_zone.time_variant = config.time_variant.clone();

        let result = dtf.format_to_string(&value);
        assert_eq!(result, fx.output.value, "\n  file: {}.json\n", fixture_name);

        let mut s = String::new();
        dtf.format_to_write(&mut s, &value).unwrap();
        assert_eq!(s, fx.output.value, "\n  file: {}.json\n", fixture_name);

        let fdt = dtf.format(&value);
        let s = fdt.to_string();
        assert_eq!(s, fx.output.value, "\n  file: {}.json\n", fixture_name);

        let mut s = String::new();
        write!(s, "{}", fdt).unwrap();
        assert_eq!(s, fx.output.value, "\n  file: {}.json\n", fixture_name);
    }
}

#[test]
fn test_dayperiod_patterns() {
    let provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatOptions::default();
    for test in get_dayperiod_tests("dayperiods").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut data: DataPayload<DatesV1> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid.clone()),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        *data
            .cow
            .to_mut()
            .patterns
            .datetime
            .length_patterns
            .long
            .to_mut() = String::from("{0}");
        for test_case in &test.test_cases {
            for dt_input in &test_case.datetimes {
                let datetime: MockDateTime = dt_input.parse().unwrap();
                for DayPeriodExpectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        *data.cow.to_mut().patterns.time.long.to_mut() =
                            String::from(pattern_input);
                        let provider = StructProvider {
                            key: GREGORY_V1,
                            data: data.as_ref(),
                        };
                        let dtf =
                            DateTimeFormat::try_new(langid.clone(), &provider, &format_options)
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
fn test_time_zone_patterns() {
    let date_provider = icu_testdata::get_provider();
    let zone_provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatOptions::default();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut config = test.config;
        let mut datetime: MockZonedDateTime = test.datetime.parse().unwrap();
        datetime.time_zone.time_zone_id = config.time_zone_id.take();
        datetime.time_zone.metazone_id = config.metazone_id.take();
        datetime.time_zone.time_variant = config.time_variant.take();

        let mut data: DataPayload<DatesV1> = date_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid.clone()),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();

        *data
            .cow
            .to_mut()
            .patterns
            .datetime
            .length_patterns
            .long
            .to_mut() = String::from("{0}");

        for TimeZoneExpectation { patterns, expected } in &test.expectations {
            for pattern_input in patterns {
                *data.cow.to_mut().patterns.time.long.to_mut() = String::from(pattern_input);
                let date_provider = StructProvider {
                    key: GREGORY_V1,
                    data: data.as_ref(),
                };

                let dtf = ZonedDateTimeFormat::try_new(
                    langid.clone(),
                    &date_provider,
                    &zone_provider,
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
            time_variant: Some(String::from("daylight")),
            ..TimeZoneConfig::default()
        },
    );
}

/// Tests component::Bag configurations that have exact matches to CLDR skeletons.
#[test]
fn test_components_exact_matches() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-exact-matches");
}

/// Tests that component::Bags can adjust for width differences in the final pattern.
/// TODO(584) - This is unimplemented and will panic.
#[test]
#[should_panic]
fn test_components_width_differences() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-width-differences");
}

/// Tests that component::Bags can combine a date skeleton, and a time skeleton.
#[test]
fn test_components_combine_datetime() {
    // components/datetime/tests/fixtures/tests/components-combine-date-time.json
    test_fixture("components-combine-date-time");
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
    let result = DateTimeFormat::try_new(locale, &provider, &options);

    assert!(result.is_err());
}
