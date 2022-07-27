// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

mod fixtures;
mod patterns;

use icu_calendar::{
    any_calendar::{AnyCalendarKind, IncludedInAnyCalendar},
    buddhist::Buddhist,
    coptic::Coptic,
    ethiopic::{Ethiopic, EthiopicEraStyle},
    indian::Indian,
    japanese::{Japanese, JapaneseEraStyle},
    provider::JapaneseErasV1Marker,
    AsCalendar, DateTime, Gregorian, Iso,
};
use icu_datetime::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};
use icu_datetime::{
    any::AnyDateTimeFormatter,
    mock::{parse_gregorian_from_str, parse_zoned_gregorian_from_str},
    pattern::runtime,
    provider::{
        calendar::{
            DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
            TimePatternsV1Marker, TimeSymbolsV1Marker,
        },
        week_data::WeekDataV1Marker,
    },
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    CldrCalendar, DateFormatter, DateTimeFormatter, DateTimeFormatterOptions, TimeFormatter,
    ZonedDateTimeFormatter,
};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_locid::{
    extensions_unicode_key as key, extensions_unicode_value as value, LanguageIdentifier, Locale,
};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;
use icu_provider_adapters::any_payload::AnyPayloadProvider;
use icu_provider_adapters::fork::MultiForkByKeyProvider;
use patterns::{
    get_dayperiod_tests, get_time_zone_tests,
    structs::{
        dayperiods::DayPeriodExpectation,
        time_zones::{TimeZoneConfig, TimeZoneExpectation},
    },
};
use std::fmt::Write;
use std::str::FromStr;
use tinystr::tinystr;

fn test_fixture(fixture_name: &str) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let japanese = Japanese::try_new(&provider, JapaneseEraStyle::Modern)
            .expect("Cannot load japanese data");
        let japanext =
            Japanese::try_new(&provider, JapaneseEraStyle::All).expect("Cannot load japanese data");
        let options = fixtures::get_options(&fx.input.options);
        let input_value = parse_gregorian_from_str(&fx.input.value).unwrap();
        let input_iso = input_value.to_calendar(Iso);
        let input_buddhist = input_value.to_calendar(Buddhist);
        let input_japanese = input_value.to_calendar(japanese);
        let input_japanext = input_value.to_calendar(japanext);
        let input_coptic = input_value.to_calendar(Coptic);
        let input_indian = input_value.to_calendar(Indian);
        let input_ethiopic = input_value.to_calendar(Ethiopic::new());

        let input_ethioaa =
            input_value.to_calendar(Ethiopic::new_with_era_style(EthiopicEraStyle::AmeteAlem));
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
            let locale = Locale::from_str(&locale).expect("Expected parseable locale in fixture");
            if let Some(kind) = AnyCalendarKind::from_locale(&locale) {
                match kind {
                    AnyCalendarKind::Buddhist => assert_fixture_element(
                        &locale,
                        &input_buddhist,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Japanese => assert_fixture_element(
                        &locale,
                        &input_japanese,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Japanext => assert_fixture_element(
                        &locale,
                        &input_japanext,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Coptic => assert_fixture_element(
                        &locale,
                        &input_coptic,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Indian => assert_fixture_element(
                        &locale,
                        &input_indian,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Ethiopic => assert_fixture_element(
                        &locale,
                        &input_ethiopic,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    AnyCalendarKind::Ethioaa => assert_fixture_element(
                        &locale,
                        &input_ethioaa,
                        &input_iso,
                        &output_value,
                        &provider,
                        &options,
                        &description,
                    ),
                    _ => panic!("datetime test does not support locale {:?}", locale),
                }
            } else {
                assert_fixture_element(
                    &locale,
                    &input_value,
                    &input_iso,
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
    locale: &Locale,
    input_value: &DateTime<A>,
    input_iso: &DateTime<Iso>,
    output_value: &str,
    provider: &D,
    options: &DateTimeFormatterOptions,
    description: &str,
) where
    A: AsCalendar,
    A::Calendar: CldrCalendar,
    A::Calendar: IncludedInAnyCalendar,
    D: DataProvider<DateSymbolsV1Marker>
        + DataProvider<TimeSymbolsV1Marker>
        + DataProvider<DatePatternsV1Marker>
        + DataProvider<TimePatternsV1Marker>
        + DataProvider<DateSkeletonPatternsV1Marker>
        + DataProvider<DecimalSymbolsV1Marker>
        + DataProvider<OrdinalV1Marker>
        + DataProvider<WeekDataV1Marker>
        + DataProvider<JapaneseErasV1Marker>,
{
    let any_input = input_value.to_any();
    let iso_any_input = input_iso.to_any();
    let dtf = DateTimeFormatter::<A::Calendar>::try_new(&locale.into(), provider, options).unwrap();
    let result = dtf.format_to_string(input_value);

    assert_eq!(result, output_value, "{}", description);

    let any_dtf =
        AnyDateTimeFormatter::try_new_unstable(&locale.into(), provider, options).unwrap();
    let result = any_dtf.format_to_string(&any_input).unwrap();

    assert_eq!(
        result, output_value,
        "(AnyDateTimeFormatter) {}",
        description
    );

    let result = any_dtf.format_to_string(&iso_any_input).unwrap();

    assert_eq!(
        result, output_value,
        "(AnyDateTimeFormatter iso conversion) {}",
        description
    );

    let mut s = String::new();
    dtf.format_to_write(&mut s, input_value).unwrap();
    assert_eq!(s, output_value, "{}", description);

    let fdt = dtf.format(input_value);
    let s = fdt.to_string();
    assert_eq!(s, output_value, "{}", description);

    let mut s = String::new();
    write!(s, "{}", fdt).unwrap();
    assert_eq!(s, output_value, "{}", description);

    if let DateTimeFormatterOptions::Length(bag) = options {
        if bag.date.is_some() && bag.time.is_some() {
            let df =
                DateFormatter::<A::Calendar>::try_new(&locale.into(), provider, bag.date.unwrap())
                    .unwrap();
            let tf = TimeFormatter::<A::Calendar>::try_new(
                &locale.into(),
                provider,
                bag.time.unwrap(),
                bag.preferences,
            )
            .unwrap();

            let dtf = DateTimeFormatter::try_from_date_and_time(df, tf).unwrap();
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
        } else if bag.date.is_some() {
            let df =
                DateFormatter::<A::Calendar>::try_new(&locale.into(), provider, bag.date.unwrap())
                    .unwrap();
            let result = df.format_to_string(input_value);

            assert_eq!(result, output_value, "{}", description);

            let mut s = String::new();
            df.format_to_write(&mut s, input_value).unwrap();
            assert_eq!(s, output_value, "{}", description);

            let fdt = df.format(input_value);
            let s = fdt.to_string();
            assert_eq!(s, output_value, "{}", description);

            let mut s = String::new();
            write!(s, "{}", fdt).unwrap();
            assert_eq!(s, output_value, "{}", description);
        } else if bag.time.is_some() {
            let tf = TimeFormatter::<A::Calendar>::try_new(
                &locale.into(),
                provider,
                bag.time.unwrap(),
                bag.preferences,
            )
            .unwrap();

            let result = tf.format_to_string(input_value);

            assert_eq!(result, output_value, "{}", description);

            let mut s = String::new();
            tf.format_to_write(&mut s, input_value).unwrap();
            assert_eq!(s, output_value, "{}", description);

            let fdt = tf.format(input_value);
            let s = fdt.to_string();
            assert_eq!(s, output_value, "{}", description);

            let mut s = String::new();
            write!(s, "{}", fdt).unwrap();
            assert_eq!(s, output_value, "{}", description);
        }
    }
}

fn test_fixture_with_time_zones(fixture_name: &str, config: TimeZoneConfig) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let options = fixtures::get_options(&fx.input.options);

        let (input_date, mut time_zone) = parse_zoned_gregorian_from_str(&fx.input.value).unwrap();
        time_zone.time_zone_id = config.time_zone_id.map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.map(MetaZoneId);
        time_zone.time_variant = config.time_variant;

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
            let dtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
                &locale.into(),
                &provider,
                &provider,
                &provider,
                &provider,
                &options,
                &TimeZoneFormatterOptions::default(),
            )
            .unwrap();
            let result = dtf.format_to_string(&input_date, &time_zone);

            assert_eq!(result, output_value, "{}", description);

            let mut s = String::new();
            dtf.format_to_write(&mut s, &input_date, &time_zone)
                .unwrap();
            assert_eq!(s, output_value, "{}", description);

            let fdt = dtf.format(&input_date, &time_zone);
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
    let format_options = DateTimeFormatterOptions::default();
    for test in get_dayperiod_tests("dayperiods").unwrap().0 {
        let mut locale: Locale = test.locale.parse().unwrap();
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), value!("gregory"));
        let mut data_locale = DataLocale::from(&locale);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let mut date_patterns_data: DataPayload<DatePatternsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });
        let mut time_patterns_data: DataPayload<TimePatternsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });
        let date_symbols_data: DataPayload<DateSymbolsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        let time_symbols_data: DataPayload<TimeSymbolsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        let week_data: DataPayload<WeekDataV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        data_locale.retain_unicode_ext(|_| false);
        let decimal_data: DataPayload<DecimalSymbolsV1Marker> = provider
            .load(DataRequest {
                locale: &data_locale,
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
                        let new_pattern1: runtime::Pattern = pattern_input.parse().unwrap();
                        let new_pattern2: runtime::Pattern = pattern_input.parse().unwrap();
                        time_patterns_data.with_mut(move |data| {
                            data.time_h11_h12.long = new_pattern1;
                            data.time_h23_h24.long = new_pattern2;
                        });
                        let local_provider = MultiForkByKeyProvider::new(vec![
                            AnyPayloadProvider {
                                key: DateSymbolsV1Marker::KEY,
                                data: date_symbols_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: TimeSymbolsV1Marker::KEY,
                                data: time_symbols_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: DateSkeletonPatternsV1Marker::KEY,
                                data: skeleton_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: DatePatternsV1Marker::KEY,
                                data: date_patterns_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: TimePatternsV1Marker::KEY,
                                data: time_patterns_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: WeekDataV1Marker::KEY,
                                data: week_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: DecimalSymbolsV1Marker::KEY,
                                data: decimal_data.clone().wrap_into_any_payload(),
                            },
                        ]);
                        let dtf = DateTimeFormatter::<Gregorian>::try_new(
                            &data_locale,
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
    let zone_provider = icu_testdata::get_provider();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut config = test.config;
        let (_, mut time_zone) = parse_zoned_gregorian_from_str(&test.datetime).unwrap();
        time_zone.time_zone_id = config.time_zone_id.take().map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.take().map(MetaZoneId);
        time_zone.time_variant = config.time_variant.take();
        for TimeZoneExpectation {
            patterns: _,
            configs,
            fallback_formats,
            expected,
        } in &test.expectations
        {
            for &config_input in configs {
                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let tzf = TimeZoneFormatter::try_from_config(
                        langid.clone(),
                        config_input.into(),
                        &zone_provider,
                        &fallback_format.into(),
                    )
                    .unwrap();
                    let mut buffer = String::new();
                    tzf.format_to_write(&mut buffer, &time_zone).unwrap();
                    assert_eq!(
                        buffer.to_string(),
                        *expect,
                        "\n\
                    locale:   `{}`,\n\
                    datetime: `{}`,\n\
                    config: `{:?}`,\n\
                    fallback: `{:?}`\n
                    ",
                        langid,
                        test.datetime,
                        config_input,
                        fallback_format
                    );
                }
            }
        }
    }
}

#[test]
fn test_time_zone_patterns() {
    let date_provider = icu_testdata::get_provider();
    let decimal_provider = icu_testdata::get_provider();
    let plural_provider = icu_testdata::get_provider();
    let zone_provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatterOptions::default();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let mut locale: Locale = test.locale.parse().unwrap();
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), value!("gregory"));
        let data_locale = DataLocale::from(&locale);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let mut config = test.config;
        let (datetime, mut time_zone) = parse_zoned_gregorian_from_str(&test.datetime).unwrap();
        time_zone.time_zone_id = config.time_zone_id.take().map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.take().map(MetaZoneId);
        time_zone.time_variant = config.time_variant.take();

        let mut date_patterns_data: DataPayload<DatePatternsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let mut time_patterns_data: DataPayload<TimePatternsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let symbols_data: DataPayload<DateSymbolsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let week_data: DataPayload<WeekDataV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();

        date_patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });

        for TimeZoneExpectation {
            patterns,
            configs: _,
            fallback_formats,
            expected,
        } in &test.expectations
        {
            for pattern_input in patterns {
                let new_pattern1: runtime::Pattern = pattern_input.parse().unwrap();
                let new_pattern2: runtime::Pattern = pattern_input.parse().unwrap();
                time_patterns_data.with_mut(move |data| {
                    data.time_h11_h12.long = new_pattern1;
                    data.time_h23_h24.long = new_pattern2;
                });
                let local_provider = MultiForkByKeyProvider::new(vec![
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
                        data: date_patterns_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: TimePatternsV1Marker::KEY,
                        data: time_patterns_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: WeekDataV1Marker::KEY,
                        data: week_data.clone().wrap_into_any_payload(),
                    },
                ]);

                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let dtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
                        &data_locale,
                        &local_provider.as_downcasting(),
                        &zone_provider,
                        &plural_provider,
                        &decimal_provider,
                        &format_options,
                        &fallback_format.into(),
                    )
                    .unwrap();

                    assert_eq!(
                        dtf.format(&datetime, &time_zone).to_string(),
                        *expect,
                        "\n\
                    locale:   `{}`,\n\
                    datetime: `{}`,\n\
                    pattern:  `{}`\n
                    fallback: `{:?}`\n",
                        locale,
                        test.datetime,
                        pattern_input,
                        fallback_format,
                    );
                }
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
            metazone_id: Some(tinystr!(4, "ampa")),
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
        DateTimeFormatterOptions,
    };
    use icu_locid::locale;

    let mut length_bag = Bag::default();
    length_bag.time = Some(Time::Full); // Full has timezone symbols
    let options = DateTimeFormatterOptions::Length(length_bag);

    let provider = icu_testdata::get_provider();
    let result =
        DateTimeFormatter::<Gregorian>::try_new(&locale!("en").into(), &provider, &options);

    assert!(result.is_err());
}
