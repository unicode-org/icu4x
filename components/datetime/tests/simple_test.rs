// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::cal::Hebrew;
use icu_calendar::{Date, Gregorian};
use icu_datetime::fieldsets;
use icu_datetime::fieldsets::enums::{
    CompositeDateTimeFieldSet, DateAndTimeFieldSet, DateFieldSet,
};
use icu_datetime::{DateTimeFormatterPreferences, FixedCalendarDateTimeFormatter};
use icu_locale_core::{locale, Locale};
use icu_time::{DateTime, Time};
use writeable::assert_writeable_eq;

const EXPECTED_DATETIME: &[&str] = &[
    "Friday, December 22, 2023 at 9:22:53\u{202f}PM",
    "vendredi 22 décembre 2023 à 21:22:53",
    "2023年12月22日星期五 21:22:53",
    "श\u{941}क\u{94d}रवार, 22 दिस\u{902}बर 2023 को 9:22:53 pm बजे",
    "Friday, December 22, 2023 at 9:22\u{202f}PM",
    "vendredi 22 décembre 2023 à 21:22",
    "2023年12月22日星期五 21:22",
    "श\u{941}क\u{94d}रवार, 22 दिस\u{902}बर 2023 को 9:22 pm बजे",
    "December 22, 2023 at 9:22:53\u{202f}PM",
    "22 décembre 2023 à 21:22:53",
    "2023年12月22日 21:22:53",
    "22 दिस\u{902}बर 2023 को 9:22:53 pm बजे",
    "December 22, 2023 at 9:22\u{202f}PM",
    "22 décembre 2023 à 21:22",
    "2023年12月22日 21:22",
    "22 दिस\u{902}बर 2023 को 9:22 pm बजे",
    "Dec 22, 2023, 9:22:53\u{202f}PM",
    "22 déc. 2023, 21:22:53",
    "2023年12月22日 21:22:53",
    "22 दिस॰ 2023, 9:22:53 pm",
    "Dec 22, 2023, 9:22\u{202f}PM",
    "22 déc. 2023, 21:22",
    "2023年12月22日 21:22",
    "22 दिस॰ 2023, 9:22 pm",
    "12/22/23, 9:22:53\u{202f}PM",
    "22/12/2023 21:22:53",
    "2023/12/22 21:22:53",
    "22/12/23, 9:22:53 pm",
    "12/22/23, 9:22\u{202f}PM",
    "22/12/2023 21:22",
    "2023/12/22 21:22",
    "22/12/23, 9:22 pm",
];

const EXPECTED_DATE: &[&str] = &[
    "Friday, December 22, 2023",
    "vendredi 22 décembre 2023",
    "2023年12月22日星期五",
    "शुक्रवार, 22 दिसंबर 2023",
    "December 22, 2023",
    "22 décembre 2023",
    "2023年12月22日",
    "22 दिसंबर 2023",
    "Dec 22, 2023",
    "22 déc. 2023",
    "2023年12月22日",
    "22 दिस॰ 2023",
    "12/22/23",
    "22/12/2023",
    "2023/12/22",
    "22/12/23",
];

#[test]
fn neo_datetime_lengths() {
    let datetime = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(21, 22, 53, 0).unwrap(),
    };
    let mut expected_iter = EXPECTED_DATETIME.iter();
    use icu_datetime::options::TimePrecision::Minute as HM;
    for field_set in [
        DateAndTimeFieldSet::YMDET(fieldsets::YMDET::long()),
        DateAndTimeFieldSet::YMDET(fieldsets::YMDET::long().with_time_precision(HM)),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::long()),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::long().with_time_precision(HM)),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::medium()),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::medium().with_time_precision(HM)),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::short()),
        DateAndTimeFieldSet::YMDT(fieldsets::YMDT::short().with_time_precision(HM)),
    ] {
        for locale in [locale!("en"), locale!("fr"), locale!("zh"), locale!("hi")] {
            let prefs = DateTimeFormatterPreferences::from(&locale);
            let skeleton = CompositeDateTimeFieldSet::DateTime(field_set);
            let formatter = FixedCalendarDateTimeFormatter::try_new(prefs, skeleton).unwrap();
            let formatted = formatter.format(&datetime);
            let expected = expected_iter.next().unwrap();
            assert_writeable_eq!(formatted, *expected, "{skeleton:?} {locale:?}");
        }
    }
}

#[test]
fn neo_date_lengths() {
    let datetime = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(21, 22, 53, 0).unwrap(),
    };
    let mut expected_iter = EXPECTED_DATE.iter();
    for field_set in [
        DateFieldSet::YMDE(fieldsets::YMDE::long()),
        DateFieldSet::YMD(fieldsets::YMD::long()),
        DateFieldSet::YMD(fieldsets::YMD::medium()),
        DateFieldSet::YMD(fieldsets::YMD::short()),
    ] {
        let date_skeleton = CompositeDateTimeFieldSet::Date(field_set);
        for locale in [locale!("en"), locale!("fr"), locale!("zh"), locale!("hi")] {
            let prefs = DateTimeFormatterPreferences::from(&locale);
            let formatter = FixedCalendarDateTimeFormatter::try_new(prefs, date_skeleton).unwrap();
            let formatted = formatter.format(&datetime);
            let expected = expected_iter.next().unwrap();
            assert_writeable_eq!(formatted, *expected, "{date_skeleton:?} {locale:?}");
        }
    }
}

#[test]
fn overlap_patterns() {
    let datetime = DateTime {
        date: Date::try_new_gregorian(2024, 8, 9).unwrap(),
        time: Time::try_new(20, 40, 7, 250).unwrap(),
    };
    struct TestCase {
        locale: Locale,
        skeleton: CompositeDateTimeFieldSet,
        expected: &'static str,
    }
    let cases = [
        // Note: in en-US, there is no comma in the overlap pattern
        TestCase {
            locale: locale!("en-US"),
            skeleton: CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(
                fieldsets::ET::medium(),
            )),
            expected: "Fri 8:40:07\u{202f}PM",
        },
        TestCase {
            locale: locale!("en-US"),
            skeleton: CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::MDET(
                fieldsets::MDET::medium(),
            )),
            expected: "Fri, Aug 9, 8:40:07\u{202f}PM",
        },
        // Note: in ru, the standalone weekday name is used when it is the only one in the pattern
        // (but the strings are the same in data)
        TestCase {
            locale: locale!("ru"),
            skeleton: CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(
                fieldsets::ET::medium(),
            )),
            expected: "пт 20:40:07",
        },
        TestCase {
            locale: locale!("ru"),
            skeleton: CompositeDateTimeFieldSet::Date(DateFieldSet::E(fieldsets::E::medium())),
            expected: "пт",
        },
    ];
    for TestCase {
        locale,
        skeleton,
        expected,
    } in cases
    {
        let prefs = DateTimeFormatterPreferences::from(&locale);
        let formatter = FixedCalendarDateTimeFormatter::try_new(prefs, skeleton).unwrap();
        let formatted = formatter.format(&datetime);
        assert_writeable_eq!(formatted, expected, "{locale:?} {skeleton:?}");
    }
}

#[test]
fn hebrew_months() {
    let datetime = DateTime {
        date: Date::try_new_iso(2011, 4, 3).unwrap().to_calendar(Hebrew),
        time: Time::try_new(14, 15, 7, 0).unwrap(),
    };
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("en").into(), fieldsets::YMD::medium())
            .unwrap();

    let formatted_datetime = formatter.format(&datetime);

    assert_writeable_eq!(formatted_datetime, "28 Adar II 5771");
}

#[test]
fn test_5387() {
    let datetime = DateTime {
        date: Date::try_new_gregorian(2024, 8, 16).unwrap(),
        time: Time::try_new(14, 15, 16, 0).unwrap(),
    };
    let formatter_auto = FixedCalendarDateTimeFormatter::try_new(
        locale!("en").into(),
        CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(fieldsets::ET::medium())),
    )
    .unwrap();
    let formatter_h12 = FixedCalendarDateTimeFormatter::try_new(
        locale!("en-u-hc-h12").into(),
        CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(fieldsets::ET::medium())),
    )
    .unwrap();
    let formatter_h24 = FixedCalendarDateTimeFormatter::try_new(
        locale!("en-u-hc-h23").into(),
        CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(fieldsets::ET::medium())),
    )
    .unwrap();

    // TODO(#5387): All of these should resolve to a pattern without a comma
    assert_writeable_eq!(formatter_auto.format(&datetime), "Fri 2:15:16\u{202f}PM");
    assert_writeable_eq!(formatter_h12.format(&datetime), "Fri, 2:15:16\u{202f}PM");
    assert_writeable_eq!(formatter_h24.format(&datetime), "Fri, 14:15:16");
}

/// Verify that formatting with no explicit hour cycle preference defaults to the
/// locale's region-based hour cycle per CLDR data.
///
/// When an explicit `-u-hc-*` extension is provided, it should override the locale default.
///
/// Regression test for <https://github.com/unicode-org/icu4x/issues/594>.
#[cfg(feature = "unstable")]
#[test]
fn test_locale_default_hour_cycle() {
    use icu_datetime::provider::fields::components;
    use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

    // Use hour=21 (9 PM) — hour > 12 ensures h12 and h23 produce visibly different output.
    let datetime = DateTime {
        date: Date::try_new_gregorian(2024, 1, 15).unwrap(),
        time: Time::try_new(21, 22, 0, 0).unwrap(),
    };

    // (locale string, expected hour cycle)
    //
    // Note: the field set below uses `fieldsets::T::short()` with no explicit hour cycle.
    // The `DateTimeFormatterPreferences::from(&locale)` does NOT inject a default hour_cycle
    // preference unless the locale string contains `-u-hc-*`. This ensures we are testing
    // the "no preference" path where the locale's baked-in default takes effect.
    //
    // NOTE: es-US/es-ES pair omitted — baked test data does not include
    // both variants. The en/en-GB pair covers the same region-resolution logic.
    let cases: &[(&str, HourCycle)] = &[
        // h12 locales — expect AM/PM patterns
        ("en", HourCycle::H12), // US English → h12
        ("ko", HourCycle::H12), // Korean → h12
        // h23 locales — expect 24-hour patterns
        ("fr", HourCycle::H23),    // French → h23
        ("ja", HourCycle::H23),    // Japanese → h23
        ("de", HourCycle::H23),    // German → h23
        ("en-GB", HourCycle::H23), // British English → h23 (region override)
        // Explicit -u-hc- extension overrides locale default
        ("fr-u-hc-h12", HourCycle::H12), // French forced to h12
        ("en-u-hc-h23", HourCycle::H23), // English forced to h23
    ];

    for (locale_str, expected_hour_cycle) in cases {
        let locale: Locale = locale_str.parse().unwrap();
        let prefs = DateTimeFormatterPreferences::from(&locale);
        let formatter =
            FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, fieldsets::T::short())
                .unwrap();
        let formatted = formatter.format(&datetime);
        let resolved_pattern = formatted.pattern();
        let bag = components::Bag::from(&resolved_pattern);

        assert_eq!(
            bag.hour_cycle,
            Some(*expected_hour_cycle),
            "Locale {locale_str}: expected hour cycle {expected_hour_cycle:?}, got {:?}",
            bag.hour_cycle,
        );
    }

    // Edge case: midnight (hour=0) — verifies h11/h12 vs h23 distinction at boundary
    let midnight_datetime = DateTime {
        date: Date::try_new_gregorian(2024, 1, 15).unwrap(),
        time: Time::try_new(0, 0, 0, 0).unwrap(),
    };
    for (locale_str, expected_hour_cycle) in &[("en", HourCycle::H12), ("fr", HourCycle::H23)] {
        let locale: Locale = locale_str.parse().unwrap();
        let prefs = DateTimeFormatterPreferences::from(&locale);
        let formatter =
            FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, fieldsets::T::short())
                .unwrap();
        let formatted = formatter.format(&midnight_datetime);
        let resolved_pattern = formatted.pattern();
        let bag = components::Bag::from(&resolved_pattern);

        assert_eq!(
            bag.hour_cycle,
            Some(*expected_hour_cycle),
            "Midnight edge case — Locale {locale_str}: expected {expected_hour_cycle:?}, got {:?}",
            bag.hour_cycle,
        );
    }
}
