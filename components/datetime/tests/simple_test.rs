// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::DateTime;
use icu_datetime::neo::TypedNeoFormatter;
use icu_datetime::neo_skeleton::{
    NeoDateComponents, NeoDateSkeleton, NeoDateTimeComponents, NeoTimeComponents,
};
use icu_datetime::options::length;
use icu_datetime::{DateTimeFormatterOptions, TypedDateTimeFormatter};
use icu_locale_core::locale;
use writeable::{assert_try_writeable_eq, assert_writeable_eq};

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
                    length,
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
                length,
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
fn old_datetime_lengths() {
    let datetime = DateTime::try_new_gregorian_datetime(2023, 12, 22, 21, 22, 53).unwrap();
    let mut expected_iter = EXPECTED_DATETIME.iter();
    for date_length in [
        length::Date::Full,
        length::Date::Long,
        length::Date::Medium,
        length::Date::Short,
    ] {
        for time_length in [length::Time::Medium, length::Time::Short] {
            for locale in [
                locale!("en").into(),
                locale!("fr").into(),
                locale!("zh").into(),
                locale!("hi").into(),
            ] {
                let formatter = TypedDateTimeFormatter::try_new(
                    &locale,
                    DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
                        date_length,
                        time_length,
                    )),
                )
                .unwrap();
                let formatted = formatter.format(&datetime);
                let expected = expected_iter.next().unwrap();
                assert_writeable_eq!(
                    formatted,
                    *expected,
                    "{date_length:?} {time_length:?} {locale:?}"
                );
            }
        }
    }
}

#[test]
fn old_date_lengths() {
    let datetime = DateTime::try_new_gregorian_datetime(2023, 12, 22, 21, 22, 53).unwrap();
    let mut expected_iter = EXPECTED_DATE.iter();
    for date_length in [
        length::Date::Full,
        length::Date::Long,
        length::Date::Medium,
        length::Date::Short,
    ] {
        for locale in [
            locale!("en").into(),
            locale!("fr").into(),
            locale!("zh").into(),
            locale!("hi").into(),
        ] {
            let formatter = TypedDateTimeFormatter::try_new(
                &locale,
                DateTimeFormatterOptions::Length(length::Bag::from_date_style(date_length)),
            )
            .unwrap();
            let formatted = formatter.format(&datetime);
            let expected = expected_iter.next().unwrap();
            assert_writeable_eq!(formatted, *expected, "{date_length:?} {locale:?}");
        }
    }
}
