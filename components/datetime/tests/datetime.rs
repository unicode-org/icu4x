// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod patterns;

use icu_calendar::{
    any_calendar::{AnyCalendarKind, IntoAnyCalendar},
    buddhist::Buddhist,
    chinese::Chinese,
    coptic::Coptic,
    dangi::Dangi,
    ethiopian::{Ethiopian, EthiopianEraStyle},
    hebrew::Hebrew,
    indian::Indian,
    islamic::IslamicCivil,
    islamic::IslamicObservational,
    islamic::IslamicTabular,
    islamic::IslamicUmmAlQura,
    japanese::{Japanese, JapaneseExtended},
    persian::Persian,
    provider::WeekDataV1Marker,
    roc::Roc,
    AsCalendar, DateTime, Gregorian, Iso,
};
use icu_datetime::provider::time_zones::{
    ExemplarCitiesV1Marker, MetazoneGenericNamesLongV1Marker, MetazoneGenericNamesShortV1Marker,
    MetazoneId, MetazoneSpecificNamesLongV1Marker, MetazoneSpecificNamesShortV1Marker,
    TimeZoneBcp47Id, TimeZoneFormatsV1Marker,
};
use icu_datetime::{
    pattern::runtime,
    provider::calendar::*,
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    CldrCalendar, DateTimeFormatter, DateTimeFormatterOptions, TimeFormatter, TypedDateFormatter,
    TypedDateTimeFormatter, TypedZonedDateTimeFormatter,
};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_locid::{
    extensions::unicode::{key, value},
    langid, locale, LanguageIdentifier, Locale,
};
use icu_provider::prelude::*;
use icu_provider_adapters::any_payload::AnyPayloadProvider;
use icu_provider_adapters::fork::MultiForkByKeyProvider;
use icu_timezone::{CustomTimeZone, ZoneVariant};
use patterns::{
    get_dayperiod_tests, get_time_zone_tests,
    structs::{
        dayperiods::DayPeriodExpectation,
        time_zones::{TimeZoneConfig, TimeZoneExpectation},
    },
};
use std::str::FromStr;
use tinystr::tinystr;
use writeable::{assert_writeable_eq, Writeable};

mod mock;

fn test_fixture(fixture_name: &str) {
    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let japanese = Japanese::new();
        let japanext = JapaneseExtended::new();
        let options_base = match fixtures::get_options(&fx.input.options) {
            Some(o) => o,
            #[cfg(feature = "experimental")]
            None => unreachable!(),
            #[cfg(not(feature = "experimental"))]
            None => continue,
        };
        let input_value = mock::parse_gregorian_from_str(&fx.input.value).unwrap();
        let input_buddhist = input_value.to_calendar(Buddhist);
        let input_chinese = input_value.to_calendar(Chinese);
        let input_coptic = input_value.to_calendar(Coptic);
        let input_dangi = input_value.to_calendar(Dangi);
        let input_ethiopian = input_value.to_calendar(Ethiopian::new());
        let input_ethioaa =
            input_value.to_calendar(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem));
        let input_hebrew = input_value.to_calendar(Hebrew);
        let input_indian = input_value.to_calendar(Indian);
        let input_islamic_civil = input_value.to_calendar(IslamicCivil);
        let input_islamic_observational = input_value.to_calendar(IslamicObservational);
        let input_islamic_tabular = input_value.to_calendar(IslamicTabular);
        let input_islamic_umm_al_qura = input_value.to_calendar(IslamicUmmAlQura);
        let input_iso = input_value.to_calendar(Iso);
        let input_japanese = input_value.to_calendar(japanese);
        let input_japanext = input_value.to_calendar(japanext);
        let input_persian = input_value.to_calendar(Persian);
        let input_roc = input_value.to_calendar(Roc);

        let description = match fx.description {
            Some(description) => {
                format!("\n  test: {description:?}\n  file: {fixture_name}.json\n")
            }
            None => format!("\n  file: {fixture_name}.json\n"),
        };
        for (locale, output_value) in fx.output.values.into_iter() {
            let options = options_base.clone();
            let locale = Locale::from_str(&locale).expect("Expected parseable locale in fixture");
            if let Some(kind) = AnyCalendarKind::get_for_locale(&locale) {
                match kind {
                    AnyCalendarKind::Buddhist => assert_fixture_element(
                        &locale,
                        &input_buddhist,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Chinese => assert_fixture_element(
                        &locale,
                        &input_chinese,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Coptic => assert_fixture_element(
                        &locale,
                        &input_coptic,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Dangi => assert_fixture_element(
                        &locale,
                        &input_dangi,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Ethiopian => assert_fixture_element(
                        &locale,
                        &input_ethiopian,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::EthiopianAmeteAlem => assert_fixture_element(
                        &locale,
                        &input_ethioaa,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Hebrew => assert_fixture_element(
                        &locale,
                        &input_hebrew,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Indian => assert_fixture_element(
                        &locale,
                        &input_indian,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::IslamicCivil => assert_fixture_element(
                        &locale,
                        &input_islamic_civil,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::IslamicObservational => assert_fixture_element(
                        &locale,
                        &input_islamic_observational,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::IslamicTabular => assert_fixture_element(
                        &locale,
                        &input_islamic_tabular,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::IslamicUmmAlQura => assert_fixture_element(
                        &locale,
                        &input_islamic_umm_al_qura,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Japanese => assert_fixture_element(
                        &locale,
                        &input_japanese,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::JapaneseExtended => assert_fixture_element(
                        &locale,
                        &input_japanext,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Persian => assert_fixture_element(
                        &locale,
                        &input_persian,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    AnyCalendarKind::Roc => assert_fixture_element(
                        &locale,
                        &input_roc,
                        &input_iso,
                        &output_value,
                        options,
                        &description,
                    ),
                    _ => panic!("datetime test does not support locale {locale:?}"),
                }
            } else {
                assert_fixture_element(
                    &locale,
                    &input_value,
                    &input_iso,
                    &output_value,
                    options,
                    &description,
                )
            }
        }
    }
}

fn assert_fixture_element<A>(
    locale: &Locale,
    input_value: &DateTime<A>,
    input_iso: &DateTime<Iso>,
    output_value: &str,
    options: DateTimeFormatterOptions,
    description: &str,
) where
    A: AsCalendar,
    A::Calendar: CldrCalendar,
    A::Calendar: IntoAnyCalendar,
    icu_datetime::provider::Baked: DataProvider<<A::Calendar as CldrCalendar>::DateSymbolsV1Marker>,
    icu_datetime::provider::Baked: DataProvider<<A::Calendar as CldrCalendar>::DateLengthsV1Marker>,
{
    let any_input = input_value.to_any();
    let iso_any_input = input_iso.to_any();
    #[cfg(feature = "experimental")]
    let (dtf, any_dtf) = {
        (
            TypedDateTimeFormatter::<A::Calendar>::try_new_experimental(
                &locale.into(),
                options.clone(),
            )
            .expect(description),
            DateTimeFormatter::try_new_experimental(&locale.into(), options.clone())
                .expect(description),
        )
    };
    #[cfg(not(feature = "experimental"))]
    let (dtf, any_dtf) = {
        (
            TypedDateTimeFormatter::<A::Calendar>::try_new(&locale.into(), options.clone())
                .expect(description),
            DateTimeFormatter::try_new(&locale.into(), options.clone()).expect(description),
        )
    };

    assert_writeable_eq!(dtf.format(input_value), output_value, "{}", description);

    assert_writeable_eq!(
        any_dtf.format(&any_input).unwrap(),
        output_value,
        "(DateTimeFormatter) {}",
        description
    );

    assert_writeable_eq!(
        any_dtf.format(&iso_any_input).unwrap(),
        output_value,
        "(DateTimeFormatter iso conversion) {}",
        description
    );

    if let DateTimeFormatterOptions::Length(bag) = options {
        if bag.date.is_some() && bag.time.is_some() {
            let df = TypedDateFormatter::<A::Calendar>::try_new_with_length(
                &locale.into(),
                bag.date.unwrap(),
            )
            .unwrap();
            let tf = TimeFormatter::try_new_with_length(&locale.into(), bag.time.unwrap()).unwrap();

            let dtf = TypedDateTimeFormatter::try_from_date_and_time(df, tf).unwrap();
            assert_writeable_eq!(dtf.format(input_value), output_value, "{}", description);
        } else if bag.date.is_some() {
            let df = TypedDateFormatter::<A::Calendar>::try_new_with_length(
                &locale.into(),
                bag.date.unwrap(),
            )
            .unwrap();

            assert_writeable_eq!(df.format(input_value), output_value, "{}", description);
        } else if bag.time.is_some() {
            let tf = TimeFormatter::try_new_with_length(&locale.into(), bag.time.unwrap()).unwrap();

            assert_writeable_eq!(tf.format(input_value), output_value, "{}", description);
        }
    }
}

fn test_fixture_with_time_zones(fixture_name: &str, config: TimeZoneConfig) {
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

        let (input_date, mut time_zone) =
            mock::parse_zoned_gregorian_from_str(&fx.input.value).unwrap();
        time_zone.time_zone_id = config.time_zone_id.map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.map(MetazoneId);
        time_zone.zone_variant = config.zone_variant.map(ZoneVariant);

        let description = match fx.description {
            Some(description) => {
                format!("\n  test: {description:?}\n  file: {fixture_name}.json\n")
            }
            None => format!("\n  file: {fixture_name}.json\n"),
        };
        for (locale, output_value) in fx.output.values.into_iter() {
            let locale: Locale = locale.parse().unwrap();
            #[cfg(feature = "experimental")]
            let dtf = {
                TypedZonedDateTimeFormatter::<Gregorian>::try_new_experimental(
                    &locale.into(),
                    options.clone(),
                    TimeZoneFormatterOptions::default(),
                )
                .unwrap()
            };
            #[cfg(not(feature = "experimental"))]
            let dtf = {
                TypedZonedDateTimeFormatter::<Gregorian>::try_new(
                    &locale.into(),
                    options.clone(),
                    TimeZoneFormatterOptions::default(),
                )
                .unwrap()
            };
            assert_writeable_eq!(
                dtf.format(&input_date, &time_zone),
                output_value,
                "{}",
                description
            );
        }
    }
}

#[test]
fn test_dayperiod_patterns() {
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
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.medium = "{0}".parse().unwrap();
        });
        let mut time_patterns_data: DataPayload<TimeLengthsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        date_patterns_data.with_mut(|data| {
            data.length_combinations.medium = "{0}".parse().unwrap();
        });
        let date_symbols_data: DataPayload<GregorianDateSymbolsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let time_symbols_data: DataPayload<TimeSymbolsV1Marker> = icu_datetime::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        #[cfg(feature = "experimental")]
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let week_data: DataPayload<WeekDataV1Marker> = icu_calendar::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        data_locale.retain_unicode_ext(|_| false);
        let decimal_data: DataPayload<DecimalSymbolsV1Marker> = icu_decimal::provider::Baked
            .load(DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        for test_case in &test.test_cases {
            for dt_input in &test_case.datetimes {
                let datetime = mock::parse_gregorian_from_str(dt_input).unwrap();
                for DayPeriodExpectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        let new_pattern1: runtime::Pattern = pattern_input.parse().unwrap();
                        let new_pattern2: runtime::Pattern = pattern_input.parse().unwrap();
                        time_patterns_data.with_mut(move |data| {
                            data.time_h11_h12.medium = new_pattern1;
                            data.time_h23_h24.medium = new_pattern2;
                        });
                        let local_provider = MultiForkByKeyProvider::new(vec![
                            AnyPayloadProvider::from_payload::<GregorianDateSymbolsV1Marker>(
                                date_symbols_data.clone(), //
                            ),
                            AnyPayloadProvider::from_payload::<TimeSymbolsV1Marker>(
                                time_symbols_data.clone(), //
                            ),
                            #[cfg(feature = "experimental")]
                            AnyPayloadProvider::from_payload::<DateSkeletonPatternsV1Marker>(
                                skeleton_data.clone(), //
                            ),
                            AnyPayloadProvider::from_payload::<GregorianDateLengthsV1Marker>(
                                date_patterns_data.clone(), //
                            ),
                            AnyPayloadProvider::from_payload::<TimeLengthsV1Marker>(
                                time_patterns_data.clone(), //
                            ),
                            AnyPayloadProvider::from_payload::<WeekDataV1Marker>(
                                week_data.clone(), //
                            ),
                            AnyPayloadProvider::from_payload::<DecimalSymbolsV1Marker>(
                                decimal_data.clone(), //
                            ),
                        ]);
                        let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_unstable(
                            &local_provider.as_downcasting(),
                            &data_locale,
                            Default::default(),
                        )
                        .unwrap();
                        assert_writeable_eq!(
                            dtf.format(&datetime),
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
    for test in get_time_zone_tests("time_zones").unwrap().0 {
        let data_locale: DataLocale = test.locale.parse::<LanguageIdentifier>().unwrap().into();
        let mut config = test.config;
        let (_, mut time_zone) = mock::parse_zoned_gregorian_from_str(&test.datetime).unwrap();
        time_zone.time_zone_id = config.time_zone_id.take().map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.take().map(MetazoneId);
        time_zone.zone_variant = config.zone_variant.take().map(ZoneVariant);
        for TimeZoneExpectation {
            patterns: _,
            configs,
            fallback_formats,
            expected,
        } in &test.expectations
        {
            for &config_input in configs {
                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let mut tzf =
                        TimeZoneFormatter::try_new(&data_locale, fallback_format.into()).unwrap();
                    config_input.set_on_formatter(&mut tzf).unwrap();
                    assert_writeable_eq!(
                        tzf.format(&time_zone),
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
    let time_zone = CustomTimeZone {
        gmt_offset: None,
        time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uslax"))),
        metazone_id: Some(MetazoneId(tinystr!(4, "ampa"))),
        zone_variant: Some(ZoneVariant::daylight()),
    };
    let tzf = TimeZoneFormatter::try_new(&langid!("en").into(), Default::default()).unwrap();
    tzf.format(&time_zone).write_to_string();
}

#[test]
#[cfg(not(debug_assertions))]
fn test_time_zone_format_gmt_offset_not_set_no_debug_assert() {
    let time_zone = MockTimeZone::new(
        None,
        Some(TimeZoneBcp47Id(tinystr!(8, "uslax"))),
        Some(MetazoneId(tinystr!(4, "ampa"))),
        Some(tinystr!(8, "daylight")),
    );
    let tzf = TimeZoneFormatter::try_new(&langid!("en").into(), Default::default()).unwrap();
    assert_writeable_eq!(tzf.format(&time_zone).unwrap(), "GMT+?");
}

#[test]
fn test_time_zone_patterns() {
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
        let (datetime, mut time_zone) =
            mock::parse_zoned_gregorian_from_str(&test.datetime).unwrap();
        time_zone.time_zone_id = config.time_zone_id.take().map(TimeZoneBcp47Id);
        time_zone.metazone_id = config.metazone_id.take().map(MetazoneId);
        time_zone.zone_variant = config.zone_variant.take().map(ZoneVariant);

        let mut date_patterns_data: DataPayload<GregorianDateLengthsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let mut time_patterns_data: DataPayload<TimeLengthsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        #[cfg(feature = "experimental")]
        let skeleton_data: DataPayload<DateSkeletonPatternsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let symbols_data: DataPayload<GregorianDateSymbolsV1Marker> = icu_datetime::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let week_data: DataPayload<WeekDataV1Marker> = icu_calendar::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let decimal_data: DataPayload<DecimalSymbolsV1Marker> = icu_decimal::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let time_zone_formats_data: DataPayload<TimeZoneFormatsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let metazone_specific_short_data: DataPayload<MetazoneSpecificNamesShortV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let metazone_specific_long_data: DataPayload<MetazoneSpecificNamesLongV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let metazone_generic_short_data: DataPayload<MetazoneGenericNamesShortV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let metazone_generic_long_data: DataPayload<MetazoneGenericNamesLongV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();
        let exemplar_cities_data: DataPayload<ExemplarCitiesV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .unwrap()
                .take_payload()
                .unwrap();

        date_patterns_data.with_mut(|data| {
            data.length_combinations.medium = "{0}".parse().unwrap();
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
                    data.time_h11_h12.medium = new_pattern1;
                    data.time_h23_h24.medium = new_pattern2;
                });
                let local_provider = MultiForkByKeyProvider::new(vec![
                    AnyPayloadProvider::from_payload::<GregorianDateSymbolsV1Marker>(
                        symbols_data.clone(), //
                    ),
                    #[cfg(feature = "experimental")]
                    AnyPayloadProvider::from_payload::<DateSkeletonPatternsV1Marker>(
                        skeleton_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<GregorianDateLengthsV1Marker>(
                        date_patterns_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<TimeLengthsV1Marker>(
                        time_patterns_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<WeekDataV1Marker>(
                        week_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<DecimalSymbolsV1Marker>(
                        decimal_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<TimeZoneFormatsV1Marker>(
                        time_zone_formats_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<MetazoneSpecificNamesShortV1Marker>(
                        metazone_specific_short_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<MetazoneSpecificNamesLongV1Marker>(
                        metazone_specific_long_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<MetazoneGenericNamesShortV1Marker>(
                        metazone_generic_short_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<MetazoneGenericNamesLongV1Marker>(
                        metazone_generic_long_data.clone(), //
                    ),
                    AnyPayloadProvider::from_payload::<ExemplarCitiesV1Marker>(
                        exemplar_cities_data.clone(), //
                    ),
                ]);

                for (&fallback_format, expect) in fallback_formats.iter().zip(expected.iter()) {
                    let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new_unstable(
                        &local_provider.as_downcasting(),
                        &data_locale,
                        Default::default(),
                        fallback_format.into(),
                    )
                    .unwrap();

                    assert_writeable_eq!(
                        dtf.format(&datetime, &time_zone),
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
            zone_variant: Some(tinystr!(2, "dt")),
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

    let mut length_bag = Bag::default();
    length_bag.time = Some(Time::Full); // Full has timezone symbols
    let options = DateTimeFormatterOptions::Length(length_bag);

    let result = TypedDateTimeFormatter::<Gregorian>::try_new(&locale!("en").into(), options);

    assert!(result.is_err());
}
