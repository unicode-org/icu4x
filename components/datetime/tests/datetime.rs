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
    japanese::{Japanese, JapaneseExtended},
    AsCalendar, DateTime, Gregorian, Iso,
};
use icu_datetime::provider::time_zones::{
    ExemplarCitiesV1Marker, MetaZoneGenericNamesLongV1Marker, MetaZoneGenericNamesShortV1Marker,
    MetaZoneId, MetaZoneSpecificNamesLongV1Marker, MetaZoneSpecificNamesShortV1Marker,
    TimeZoneBcp47Id, TimeZoneFormatsV1Marker,
};
use icu_datetime::time_zone::TimeZoneFormatterConfig;
use icu_datetime::{
    mock::{parse_gregorian_from_str, parse_zoned_gregorian_from_str},
    pattern::runtime,
    provider::{calendar::*, week_data::WeekDataV1Marker},
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    CldrCalendar, DateTimeFormatter, DateTimeFormatterOptions, TimeFormatter, TypedDateFormatter,
    TypedDateTimeFormatter, TypedZonedDateTimeFormatter,
};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_locid::{
    extensions_unicode_key as key, extensions_unicode_value as value, LanguageIdentifier, Locale,
};
use icu_provider::prelude::*;
use icu_provider_adapters::any_payload::AnyPayloadProvider;
use icu_provider_adapters::fork::MultiForkByKeyProvider;
use icu_timezone::{CustomTimeZone, TimeVariant};
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
    let provider = icu_testdata::unstable();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let japanese = Japanese::try_new_unstable(&provider).expect("Cannot load japanese data");
        let japanext =
            JapaneseExtended::try_new_unstable(&provider).expect("Cannot load japanese data");
        let options_base = match fixtures::get_options(&fx.input.options) {
            Some(o) => o,
            #[cfg(feature = "experimental")]
            None => unreachable!(),
            #[cfg(not(feature = "experimental"))]
            None => continue,
        };
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
            let options = options_base.clone();
            let locale = Locale::from_str(&locale).expect("Expected parseable locale in fixture");
            if let Ok(kind) = AnyCalendarKind::from_locale(&locale) {
                match kind {
                    AnyCalendarKind::Buddhist => assert_fixture_element(
                        &locale,
                        &input_buddhist,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Japanese => assert_fixture_element(
                        &locale,
                        &input_japanese,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::JapaneseExtended => assert_fixture_element(
                        &locale,
                        &input_japanext,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Coptic => assert_fixture_element(
                        &locale,
                        &input_coptic,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Indian => assert_fixture_element(
                        &locale,
                        &input_indian,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Ethiopic => assert_fixture_element(
                        &locale,
                        &input_ethiopic,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Ethioaa => assert_fixture_element(
                        &locale,
                        &input_ethioaa,
                        &input_iso,
                        &output_value,
                        &provider,
                        options,
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
                    options,
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
    options: DateTimeFormatterOptions,
    description: &str,
) where
    A: AsCalendar,
    A::Calendar: CldrCalendar,
    A::Calendar: IncludedInAnyCalendar,
    D: BufferProvider,
{
    let any_input = input_value.to_any();
    let iso_any_input = input_iso.to_any();
    let dtf = TypedDateTimeFormatter::<A::Calendar>::try_new_with_buffer_provider(
        provider,
        &locale.into(),
        options.clone(),
    )
    .expect(description);
    let result = dtf.format_to_string(input_value);

    assert_eq!(result, output_value, "{}", description);

    let any_dtf =
        DateTimeFormatter::try_new_with_buffer_provider(provider, &locale.into(), options.clone())
            .expect(description);
    let result = any_dtf.format_to_string(&any_input).unwrap();

    assert_eq!(result, output_value, "(DateTimeFormatter) {}", description);

    let result = any_dtf.format_to_string(&iso_any_input).unwrap();

    assert_eq!(
        result, output_value,
        "(DateTimeFormatter iso conversion) {}",
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
            let df = TypedDateFormatter::<A::Calendar>::try_new_with_buffer_provider(
                provider,
                &locale.into(),
                bag.date.unwrap(),
            )
            .unwrap();
            let tf = TimeFormatter::try_new_with_buffer_provider(
                provider,
                &locale.into(),
                bag.time.unwrap(),
            )
            .unwrap();

            let dtf = TypedDateTimeFormatter::try_from_date_and_time(df, tf).unwrap();
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
            let df = TypedDateFormatter::<A::Calendar>::try_new_with_buffer_provider(
                provider,
                &locale.into(),
                bag.date.unwrap(),
            )
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
            let tf = TimeFormatter::try_new_with_buffer_provider(
                provider,
                &locale.into(),
                bag.time.unwrap(),
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
    let provider = icu_testdata::unstable();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let options = match fixtures::get_options(&fx.input.options) {
            Some(o) => o,
            #[cfg(feature = "experimental")]
            None => unreachable!(),
            #[cfg(not(feature = "experimental"))]
            None => continue,
        };

        let (input_date, mut time_zone) = parse_zoned_gregorian_from_str(&fx.input.value).unwrap();
        time_zone.time_zone_id = config.time_zone_id.map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.map(MetaZoneId);
        time_zone.time_variant = config.time_variant.map(TimeVariant);

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
            let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new_unstable(
                &provider,
                &locale.into(),
                options.clone(),
                TimeZoneFormatterOptions::default(),
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
    let provider = icu_testdata::unstable();
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
        let mut date_patterns_data: DataPayload<GregorianDateLengthsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });
        let mut time_patterns_data: DataPayload<TimeLengthsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.long = "{0}".parse().unwrap();
        });
        let date_symbols_data: DataPayload<GregorianDateSymbolsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        let time_symbols_data: DataPayload<TimeSymbolsV1Marker> =
            provider.load(req).unwrap().take_payload().unwrap();
        #[cfg(feature = "experimental")]
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
                                key: GregorianDateSymbolsV1Marker::KEY,
                                data: date_symbols_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: TimeSymbolsV1Marker::KEY,
                                data: time_symbols_data.clone().wrap_into_any_payload(),
                            },
                            #[cfg(feature = "experimental")]
                            AnyPayloadProvider {
                                key: DateSkeletonPatternsV1Marker::KEY,
                                data: skeleton_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: GregorianDateLengthsV1Marker::KEY,
                                data: date_patterns_data.clone().wrap_into_any_payload(),
                            },
                            AnyPayloadProvider {
                                key: TimeLengthsV1Marker::KEY,
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
                        let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_unstable(
                            &local_provider.as_downcasting(),
                            &data_locale,
                            format_options.clone(),
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
    let zone_provider = icu_testdata::unstable();

    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let data_locale: DataLocale = test.locale.parse::<LanguageIdentifier>().unwrap().into();
        let mut config = test.config;
        let (_, mut time_zone) = parse_zoned_gregorian_from_str(&test.datetime).unwrap();
        time_zone.time_zone_id = config.time_zone_id.take().map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.take().map(MetaZoneId);
        time_zone.time_variant = config.time_variant.take().map(TimeVariant);
        for TimeZoneExpectation {
            patterns: _,
            configs,
            fallback_formats,
            expected,
        } in &test.expectations
        {
            for &config_input in configs {
                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let tzf = TimeZoneFormatter::try_from_config_unstable(
                        &zone_provider,
                        &data_locale,
                        config_input.into(),
                        fallback_format.into(),
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
                        data_locale,
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
#[cfg(debug_assertions)]
#[should_panic(expected = "using last-resort time zone fallback")]
fn test_time_zone_format_gmt_offset_not_set_debug_assert_panic() {
    let zone_provider = icu_testdata::unstable();
    let langid: LanguageIdentifier = "en".parse().unwrap();
    let time_zone = CustomTimeZone::new(
        None,
        Some(TimeZoneBcp47Id(tinystr!(8, "uslax"))),
        Some(MetaZoneId(tinystr!(4, "ampa"))),
        Some(TimeVariant::daylight()),
    );
    let tzf = TimeZoneFormatter::try_from_config_unstable(
        &zone_provider,
        &langid.into(),
        TimeZoneFormatterConfig::LocalizedGMT,
        TimeZoneFormatterOptions::default(),
    )
    .unwrap();
    let mut buffer = String::new();
    tzf.format_to_write(&mut buffer, &time_zone).unwrap();
}

#[test]
#[cfg(not(debug_assertions))]
fn test_time_zone_format_gmt_offset_not_set_no_debug_assert() {
    let zone_provider = icu_testdata::unstable();
    let langid: LanguageIdentifier = "en".parse().unwrap();
    let time_zone = MockTimeZone::new(
        None,
        Some(TimeZoneBcp47Id(tinystr!(8, "uslax"))),
        Some(MetaZoneId(tinystr!(4, "ampa"))),
        Some(tinystr!(8, "daylight")),
    );
    let tzf = TimeZoneFormatter::try_from_config(
        &zone_provider,
        &langid.into(),
        TimeZoneFormatterConfig::LocalizedGMT,
        TimeZoneFormatterOptions::default(),
    )
    .unwrap();
    let mut buffer = String::new();
    tzf.format_to_write(&mut buffer, &time_zone).unwrap();
    assert_eq!(buffer.to_string(), "GMT+?".to_string());
}

#[test]
fn test_time_zone_patterns() {
    let date_provider = icu_testdata::unstable();
    let decimal_provider = icu_testdata::unstable();
    let zone_provider = icu_testdata::unstable();
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
        time_zone.time_variant = config.time_variant.take().map(TimeVariant);

        let mut date_patterns_data: DataPayload<GregorianDateLengthsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let mut time_patterns_data: DataPayload<TimeLengthsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        #[cfg(feature = "experimental")]
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let symbols_data: DataPayload<GregorianDateSymbolsV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let week_data: DataPayload<WeekDataV1Marker> =
            date_provider.load(req).unwrap().take_payload().unwrap();
        let decimal_data: DataPayload<DecimalSymbolsV1Marker> =
            decimal_provider.load(req).unwrap().take_payload().unwrap();
        let time_zone_formats_data: DataPayload<TimeZoneFormatsV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();
        let meta_zone_specific_short_data: DataPayload<MetaZoneSpecificNamesShortV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();
        let meta_zone_specific_long_data: DataPayload<MetaZoneSpecificNamesLongV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();
        let meta_zone_generic_short_data: DataPayload<MetaZoneGenericNamesShortV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();
        let meta_zone_generic_long_data: DataPayload<MetaZoneGenericNamesLongV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();
        let exemplar_cities_data: DataPayload<ExemplarCitiesV1Marker> =
            zone_provider.load(req).unwrap().take_payload().unwrap();

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
                        key: GregorianDateSymbolsV1Marker::KEY,
                        data: symbols_data.clone().wrap_into_any_payload(),
                    },
                    #[cfg(feature = "experimental")]
                    AnyPayloadProvider {
                        key: DateSkeletonPatternsV1Marker::KEY,
                        data: skeleton_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: GregorianDateLengthsV1Marker::KEY,
                        data: date_patterns_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: TimeLengthsV1Marker::KEY,
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
                    AnyPayloadProvider {
                        key: TimeZoneFormatsV1Marker::KEY,
                        data: time_zone_formats_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: MetaZoneSpecificNamesShortV1Marker::KEY,
                        data: meta_zone_specific_short_data
                            .clone()
                            .wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: MetaZoneSpecificNamesLongV1Marker::KEY,
                        data: meta_zone_specific_long_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: MetaZoneGenericNamesShortV1Marker::KEY,
                        data: meta_zone_generic_short_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: MetaZoneGenericNamesLongV1Marker::KEY,
                        data: meta_zone_generic_long_data.clone().wrap_into_any_payload(),
                    },
                    AnyPayloadProvider {
                        key: ExemplarCitiesV1Marker::KEY,
                        data: exemplar_cities_data.clone().wrap_into_any_payload(),
                    },
                ]);

                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new_unstable(
                        &local_provider.as_downcasting(),
                        &data_locale,
                        format_options.clone(),
                        fallback_format.into(),
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
            time_variant: Some(tinystr!(2, "dt")),
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

    let provider = icu_testdata::unstable();
    let result = TypedDateTimeFormatter::<Gregorian>::try_new_unstable(
        &provider,
        &locale!("en").into(),
        options,
    );

    assert!(result.is_err());
}
