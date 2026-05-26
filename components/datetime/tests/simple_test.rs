// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::cal::{ChineseTraditional, Hebrew};
use icu_calendar::types::Month;
use icu_calendar::Date;
use icu_datetime::fieldsets;
use icu_datetime::fieldsets::enums::{
    CompositeDateTimeFieldSet, DateAndTimeFieldSet, DateFieldSet,
};
use icu_datetime::input::{DateTime, Time, TimeZone, UtcOffset, ZonedDateTime};
use icu_datetime::pattern::{DateTimePattern, FixedCalendarDateTimeNames};
use icu_datetime::{
    DateTimeFormatter, DateTimeFormatterPreferences, FixedCalendarDateTimeFormatter,
};
use icu_locale_core::{locale, Locale};
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
        // Finnish standalone weekdays tests.
        // CLDR 48.2 has `yMMMMccccd` skeleton with pattern `cccc d. MMMM y` for Finnish.
        // We want to make sure it correctly uses the standalone form "perjantai" (Wide Standalone)
        // instead of "perjantaina" (Wide Format).
        TestCase {
            locale: locale!("fi"),
            skeleton: CompositeDateTimeFieldSet::Date(DateFieldSet::E(fieldsets::E::medium())),
            expected: "pe",
        },
        TestCase {
            locale: locale!("fi"),
            skeleton: CompositeDateTimeFieldSet::Date(DateFieldSet::YMDE(fieldsets::YMDE::long())),
            expected: "perjantai 9. elokuuta 2024",
        },
        TestCase {
            locale: locale!("fi"),
            skeleton: CompositeDateTimeFieldSet::Date(DateFieldSet::MDE(fieldsets::MDE::long())),
            expected: "perjantai 9. elokuuta",
        },
        TestCase {
            locale: locale!("fi"),
            skeleton: CompositeDateTimeFieldSet::Date(DateFieldSet::E(fieldsets::E::long())),
            expected: "perjantai",
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
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("en").into(), fieldsets::YMD::medium())
            .unwrap();

    let formatted_datetime =
        formatter.format(&Date::try_new_iso(2011, 3, 4).unwrap().to_calendar(Hebrew));

    assert_writeable_eq!(formatted_datetime, "28 Adar I 5771");

    let formatted_datetime =
        formatter.format(&Date::try_new_iso(2011, 4, 3).unwrap().to_calendar(Hebrew));

    assert_writeable_eq!(formatted_datetime, "28 Adar II 5771");
}

#[test]
fn hebrew_numbering() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("he").into(), fieldsets::YMD::long())
            .unwrap();

    let formatted_datetime = formatter.format(
        &Date::try_new_hebrew_v2(5771, 3.into(), 17)
            .unwrap()
            .to_calendar(Hebrew),
    );

    assert_writeable_eq!(formatted_datetime, "י״ז בכסלו ה׳תשע״א");
}

/// Pattern numeric overrides should be preferred over user numeric overrides
#[test]
fn hebrew_thai_numbering() {
    let formatter = FixedCalendarDateTimeFormatter::try_new(
        "he-u-ca-hebrew-nu-thai".parse::<Locale>().unwrap().into(),
        fieldsets::YMD::long(),
    )
    .unwrap();

    let formatted_datetime = formatter.format(
        &Date::try_new_hebrew_v2(5771, 3.into(), 17)
            .unwrap()
            .to_calendar(Hebrew),
    );

    assert_writeable_eq!(formatted_datetime, "י״ז בכסלו ה׳תשע״א");
}

#[test]
fn hanidec_numbering() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("ja").into(), fieldsets::YMD::long())
            .unwrap();

    let formatted_datetime =
        formatter.format(&Date::try_new_chinese_traditional(2011, 3.into(), 29).unwrap());

    // Unfortunately the only patterns that currently use hanidec use cyclic years,
    // so we can't see this in action on the years field, but the day here is hanidays.
    assert_writeable_eq!(formatted_datetime, "辛卯年三月二九日");
}

#[test]
fn hanidays_numbering() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("zh").into(), fieldsets::YMD::long())
            .unwrap();

    let formatted_datetime =
        formatter.format(&Date::try_new_chinese_traditional(2011, 12.into(), 29).unwrap());

    assert_writeable_eq!(formatted_datetime, "2011年腊月廿九");
}

#[test]
fn hanidec_ja_chinese_numbering() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("ja").into(), fieldsets::YMD::long())
            .unwrap();

    let formatted_datetime = formatter.format(
        &Date::try_new_iso(2011, 3, 4)
            .unwrap()
            .to_calendar(ChineseTraditional::new()),
    );

    assert_writeable_eq!(formatted_datetime, "辛卯年正月三〇日");
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

#[test]
fn test_vancouver_2026() {
    let date = Date::try_new_gregorian(2026, 12, 1).unwrap();
    let time = Time::try_new(12, 0, 0, 0).unwrap();

    let fmt = DateTimeFormatter::try_new(
        locale!("en-US").into(),
        fieldsets::YMD::long()
            .with_time_hm()
            .with_zone(fieldsets::zone::SpecificShort),
    )
    .unwrap();

    // Vancouver is in PST (UTC-8) normally.
    {
        let offset = UtcOffset::from_seconds_unchecked(-8 * 3600);
        let zone = TimeZone::from_iana_id("America/Vancouver")
            .with_offset(Some(offset))
            .at_date_time(DateTime { date, time });

        let zdt = ZonedDateTime { date, time, zone };

        assert_writeable_eq!(fmt.format(&zdt), "December 1, 2026 at 12:00\u{202f}PM PST");
    }

    // Vancouver might change to permanent DST (UTC-7).
    {
        let offset = UtcOffset::from_seconds_unchecked(-7 * 3600);
        let zone = TimeZone::from_iana_id("America/Vancouver")
            .with_offset(Some(offset))
            .at_date_time(DateTime { date, time });

        let zdt = ZonedDateTime { date, time, zone };

        assert_writeable_eq!(fmt.format(&zdt), "December 1, 2026 at 12:00\u{202f}PM PDT");
    }
}

#[test]
fn test_flexible_dayperiod_formatting() {
    let formatter =
        DateTimeFormatter::try_new(locale!("zh-HK").into(), fieldsets::T::hm()).unwrap();

    for (hour, expected) in (0..24).zip([
        "凌晨12:00",
        "凌晨1:00",
        "凌晨2:00",
        "凌晨3:00",
        "凌晨4:00",
        "早上5:00",
        "早上6:00",
        "早上7:00",
        "上午8:00",
        "上午9:00",
        "上午10:00",
        "上午11:00",
        "中午12:00",
        "下午1:00",
        "下午2:00",
        "下午3:00",
        "下午4:00",
        "下午5:00",
        "下午6:00",
        "晚上7:00",
        "晚上8:00",
        "晚上9:00",
        "晚上10:00",
        "晚上11:00",
    ]) {
        let time = Time::try_new(hour, 0, 0, 0).unwrap();
        let formatted = formatter.format(&time);
        assert_writeable_eq!(formatted, expected);
    }
}

#[test]
fn test_chinese_leap_numeric() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("ja").into(), fieldsets::YMD::short())
            .unwrap();

    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_chinese_traditional(2028, Month::new(5), 23).unwrap()),
        "戊申-5-23",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_chinese_traditional(2028, Month::leap(5), 23).unwrap()),
        "戊申-閏5-23",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_chinese_traditional(2028, Month::new(6), 23).unwrap()),
        "戊申-6-23",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_chinese_traditional(2028, Month::new(7), 23).unwrap()),
        "戊申-7-23",
    );
}

#[test]
fn test_adar_numeric() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("ar").into(), fieldsets::YMD::short())
            .unwrap();

    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(5), 23).unwrap()),
        "23\u{200f}/5\u{200f}/5771 ص",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::leap(5), 23).unwrap()),
        "23\u{200f}/6a\u{200f}/5771 ص",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(6), 23).unwrap()),
        "23\u{200f}/6b\u{200f}/5771 ص",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(7), 23).unwrap()),
        "23\u{200f}/7\u{200f}/5771 ص",
    );
    writeable::assert_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5772, Month::new(6), 23).unwrap()),
        "23\u{200f}/6\u{200f}/5772 ص",
    );
}

#[test]
fn test_adar_narrow() {
    let mut names =
        FixedCalendarDateTimeNames::<Hebrew, DateFieldSet>::try_new(locale!("en").into()).unwrap();

    let pattern = DateTimePattern::try_from_pattern_str("d/MMMMM/y G").unwrap();
    let formatter = names.include_for_pattern(&pattern).unwrap();

    writeable::assert_try_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(5), 23).unwrap()),
        "23/5/5771 AM",
        Ok(())
    );
    writeable::assert_try_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::leap(5), 23).unwrap()),
        "23/6a/5771 AM",
        Ok(())
    );
    writeable::assert_try_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(6), 23).unwrap()),
        "23/6b/5771 AM",
        Ok(())
    );
    writeable::assert_try_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5771, Month::new(7), 23).unwrap()),
        "23/7/5771 AM",
        Ok(())
    );
    writeable::assert_try_writeable_eq!(
        formatter.format(&Date::try_new_hebrew_v2(5772, Month::new(6), 23).unwrap()),
        "23/6/5772 AM",
        Ok(())
    );
}
