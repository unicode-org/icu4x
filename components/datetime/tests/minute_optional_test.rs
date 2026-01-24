use icu_calendar::Gregorian;
use icu_datetime::fieldsets::T;
use icu_datetime::input::Time;
use icu_datetime::options::TimePrecision;
use icu_datetime::FixedCalendarDateTimeFormatter;
use icu_locale_core::locale;
use writeable::assert_writeable_eq;

#[test]
fn test_minute_optional() {
    let t_nominal = Time::try_new(15, 0, 0, 0).unwrap();
    let formatter_us = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("en-US").into(),
        T::short().with_time_precision(TimePrecision::MinuteOptional),
    )
    .unwrap();

    assert_writeable_eq!(formatter_us.format(&t_nominal), "3\u{202f}PM");

    let formatter_gb = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("en-GB").into(),
        T::short().with_time_precision(TimePrecision::MinuteOptional),
    )
    .unwrap();

    assert_writeable_eq!(formatter_gb.format(&t_nominal), "15:00");
}

#[test]
fn test_minute_optional_h23_explicit() {
    let t_nominal = Time::try_new(15, 0, 0, 0).unwrap();

    let formatter_us_h23 = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("en-US-u-hc-h23").into(),
        T::short().with_time_precision(TimePrecision::MinuteOptional),
    )
    .unwrap();

    assert_writeable_eq!(formatter_us_h23.format(&t_nominal), "15:00");
}

#[test]
fn test_minute_optional_h24() {
    let t_nominal = Time::try_new(15, 0, 0, 0).unwrap();

    let formatter_h24 = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("en-US-u-hc-h24").into(),
        T::short().with_time_precision(TimePrecision::MinuteOptional),
    )
    .unwrap();

    let output = formatter_h24.format(&t_nominal).to_string();

    // If h24 is not available, falls back to locale default (h12 for en-US)
    assert!(
        output == "15:00" || output == "24:00" || output == "3\u{202f}PM",
        "Unexpected output: {}",
        output
    );
}
