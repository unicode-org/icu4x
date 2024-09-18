// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::hebrew::Hebrew;
use icu_calendar::{Date, DateTime, Time};
use icu_datetime::neo::TypedNeoFormatter;
use icu_datetime::neo_marker::NeoYearMonthDayMarker;
use icu_datetime::neo_skeleton::{
    NeoComponents, NeoDateComponents, NeoDateSkeleton, NeoDateTimeComponents, NeoDayComponents,
    NeoSkeletonLength, NeoTimeComponents,
};
use icu_datetime::options::length;
use icu_locale_core::{locale, Locale};
use icu_timezone::{CustomTimeZone, CustomZonedDateTime};
use writeable::assert_try_writeable_eq;

const EXPECTED_DATETIME: &[&str] = &[
    "Friday, December 22, 2023, 9:22:53 PM",
    "vendredi 22 décembre 2023, 21:22:53",
    "2023年12月22日星期五 21:22:53",
    "शुक्रवार, 22 दिसंबर 2023, 9:22:53 pm",
    "Friday, December 22, 2023, 9:22 PM",
    "vendredi 22 décembre 2023, 21:22",
    "2023年12月22日星期五 21:22",
    "शुक्रवार, 22 दिसंबर 2023, 9:22 pm",
    "December 22, 2023, 9:22:53 PM",
    "22 décembre 2023, 21:22:53",
    "2023年12月22日 21:22:53",
    "22 दिसंबर 2023, 9:22:53 pm",
    "December 22, 2023, 9:22 PM",
    "22 décembre 2023, 21:22",
    "2023年12月22日 21:22",
    "22 दिसंबर 2023, 9:22 pm",
    "Dec 22, 2023, 9:22:53 PM",
    "22 déc. 2023, 21:22:53",
    "2023年12月22日 21:22:53",
    "22 दिस॰ 2023, 9:22:53 pm",
    "Dec 22, 2023, 9:22 PM",
    "22 déc. 2023, 21:22",
    "2023年12月22日 21:22",
    "22 दिस॰ 2023, 9:22 pm",
    "12/22/23, 9:22:53 PM",
    "22/12/2023 21:22:53",
    "2023/12/22 21:22:53",
    "22/12/23, 9:22:53 pm",
    "12/22/23, 9:22 PM",
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
    let datetime = DateTime::try_new_gregorian_datetime(2023, 12, 22, 21, 22, 53).unwrap();
    let mut expected_iter = EXPECTED_DATETIME.iter();
    for date_length in [
        length::Date::Full,
        length::Date::Long,
        length::Date::Medium,
        length::Date::Short,
    ] {
        let (day_components, length) = NeoDateSkeleton::day_from_date_length(date_length);
        for time_length in [length::Time::Medium, length::Time::Short] {
            let time_components = NeoTimeComponents::from_time_length(time_length);
            for locale in [
                locale!("en").into(),
                locale!("fr").into(),
                locale!("zh").into(),
                locale!("hi").into(),
            ] {
                let formatter = TypedNeoFormatter::try_new_with_components(
                    &locale,
                    NeoDateTimeComponents::DateTime(day_components, time_components),
                    length.into(),
                )
                .unwrap();
                let formatted = formatter.format(&datetime);
                let expected = expected_iter.next().unwrap();
                assert_try_writeable_eq!(
                    formatted,
                    *expected,
                    Ok(()),
                    "{day_components:?} {time_components:?} {length:?} {locale:?}"
                );
            }
        }
    }
}

#[test]
fn neo_date_lengths() {
    let datetime = DateTime::try_new_gregorian_datetime(2023, 12, 22, 21, 22, 53).unwrap();
    let mut expected_iter = EXPECTED_DATE.iter();
    for date_length in [
        length::Date::Full,
        length::Date::Long,
        length::Date::Medium,
        length::Date::Short,
    ] {
        let (day_components, length) = NeoDateSkeleton::day_from_date_length(date_length);
        for locale in [
            locale!("en").into(),
            locale!("fr").into(),
            locale!("zh").into(),
            locale!("hi").into(),
        ] {
            let formatter = TypedNeoFormatter::try_new_with_components(
                &locale,
                NeoDateComponents::Day(day_components),
                length.into(),
            )
            .unwrap();
            let formatted = formatter.format(&datetime);
            let expected = expected_iter.next().unwrap();
            assert_try_writeable_eq!(
                formatted,
                *expected,
                Ok(()),
                "{day_components:?} {length:?} {locale:?}"
            );
        }
    }
}

#[test]
fn overlap_patterns() {
    let datetime = CustomZonedDateTime {
        date: Date::try_new_gregorian_date(2024, 8, 9).unwrap(),
        time: Time::try_new(20, 40, 7, 250).unwrap(),
        zone: CustomTimeZone::utc(),
    };
    struct TestCase {
        locale: Locale,
        components: NeoComponents,
        length: NeoSkeletonLength,
        expected: &'static str,
    }
    let cases = [
        // Note: in en-US, there is no comma in the overlap pattern
        TestCase {
            locale: locale!("en-US"),
            components: NeoComponents::DateTime(
                NeoDayComponents::Weekday,
                NeoTimeComponents::HourMinute,
            ),
            length: NeoSkeletonLength::Medium,
            expected: "Fri 8:40\u{202f}PM",
        },
        TestCase {
            locale: locale!("en-US"),
            components: NeoComponents::DateTime(
                NeoDayComponents::MonthDayWeekday,
                NeoTimeComponents::HourMinute,
            ),
            length: NeoSkeletonLength::Medium,
            expected: "Fri, Aug 9, 8:40\u{202f}PM",
        },
        // Note: in ru, the standalone weekday name is used when it is the only one in the pattern
        // (but the strings are the same in data)
        TestCase {
            locale: locale!("ru"),
            components: NeoComponents::DateTime(
                NeoDayComponents::Weekday,
                NeoTimeComponents::HourMinute,
            ),
            length: NeoSkeletonLength::Medium,
            expected: "пт 20:40",
        },
        TestCase {
            locale: locale!("ru"),
            components: NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::Weekday)),
            length: NeoSkeletonLength::Medium,
            expected: "пт",
        },
    ];
    for TestCase {
        locale,
        components,
        length,
        expected,
    } in cases
    {
        let formatter = TypedNeoFormatter::try_new_with_components(
            &(&locale).into(),
            components,
            length.into(),
        )
        .unwrap();
        let formatted = formatter.format(&datetime);
        assert_try_writeable_eq!(
            formatted,
            expected,
            Ok(()),
            "{locale:?} {components:?} {length:?}"
        );
    }
}

#[test]
fn hebrew_months() {
    let datetime = DateTime::try_new_iso_datetime(2011, 4, 3, 14, 15, 7).unwrap();
    let datetime = datetime.to_calendar(Hebrew);
    let formatter = TypedNeoFormatter::<_, NeoYearMonthDayMarker>::try_new(
        &locale!("en").into(),
        NeoSkeletonLength::Long.into(),
    )
    .unwrap();

    let formatted_datetime = formatter.format(&datetime);

    assert_try_writeable_eq!(formatted_datetime, "28 Adar II 5771");
}

#[test]
fn test_5387() {
    let datetime = DateTime::try_new_gregorian_datetime(2024, 8, 16, 14, 15, 16).unwrap();
    let formatter_auto = TypedNeoFormatter::try_new_with_components(
        &locale!("en").into(),
        NeoDateTimeComponents::DateTime(NeoDayComponents::Weekday, NeoTimeComponents::HourMinute),
        NeoSkeletonLength::Medium.into(),
    )
    .unwrap();
    let formatter_h12 = TypedNeoFormatter::try_new_with_components(
        &locale!("en-u-hc-h12").into(),
        NeoDateTimeComponents::DateTime(NeoDayComponents::Weekday, NeoTimeComponents::HourMinute),
        NeoSkeletonLength::Medium.into(),
    )
    .unwrap();
    let formatter_h24 = TypedNeoFormatter::try_new_with_components(
        &locale!("en-u-hc-h23").into(),
        NeoDateTimeComponents::DateTime(NeoDayComponents::Weekday, NeoTimeComponents::HourMinute),
        NeoSkeletonLength::Medium.into(),
    )
    .unwrap();

    // TODO(#5387): All of these should resolve to a pattern without a comma
    assert_try_writeable_eq!(formatter_auto.format(&datetime), "Fri 2:15\u{202f}PM");
    assert_try_writeable_eq!(formatter_h12.format(&datetime), "Fri, 2:15\u{202f}PM");
    assert_try_writeable_eq!(formatter_h24.format(&datetime), "Fri, 14:15");
}
