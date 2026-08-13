// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Date;
use icu_datetime::fieldsets;
use icu_datetime::input::{DateTime, Time};
use icu_datetime::range::{DateRangeFormatter, FixedCalendarDateRangeFormatter};
use icu_locale_core::locale;
use writeable::assert_writeable_eq;

#[test]
fn test_date_range_gregorian() {
    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_same_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };
    let end_next_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };
    let end_next_month = DateTime {
        date: Date::try_new_gregorian(2024, 1, 5).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    // 1. Date-only range (YMD medium)
    {
        let fmt = FixedCalendarDateRangeFormatter::try_new(
            locale!("en").into(),
            fieldsets::YMD::medium(),
        )
        .unwrap();

        // Same day: should format as single date
        assert_writeable_eq!(fmt.format(&start, &end_same_day), "Dec 22, 2023");

        // Next day: day diff (uses thin spaces around en-dash)
        assert_writeable_eq!(
            fmt.format(&start, &end_next_day),
            "Dec 22\u{2009}–\u{2009}23, 2023"
        );

        // Next month: month/year diff (falls back to full range fallback since year differs)
        assert_writeable_eq!(
            fmt.format(&start, &end_next_month),
            "Dec 22, 2023\u{2009}–\u{2009}Jan 5, 2024"
        );
    }

    // 2. Time-only range (HM)
    {
        let fmt =
            FixedCalendarDateRangeFormatter::try_new(locale!("en").into(), fieldsets::T::hm())
                .unwrap();

        // Same day, different time (different day period AM/PM)
        assert_writeable_eq!(
            fmt.format(&start.time, &end_same_day.time),
            "9:00\u{202f}AM\u{2009}–\u{2009}5:00\u{202f}PM"
        );

        // Same day, different time (same day period AM)
        let end_same_am = Time::try_new(11, 0, 0, 0).unwrap();
        assert_writeable_eq!(
            fmt.format(&start.time, &end_same_am),
            "9:00\u{2009}–\u{2009}11:00\u{202f}AM"
        );

        // Different day: time-only formatter with date diff.
        // Should fall back to formatting both times fully.
        assert_writeable_eq!(
            fmt.format(&start, &end_next_day),
            "9:00\u{202f}AM\u{2009}–\u{2009}5:00\u{202f}PM"
        );
    }

    // 3. Date and Time range (YMDT medium)
    {
        let fmt = FixedCalendarDateRangeFormatter::try_new(
            locale!("en").into(),
            fieldsets::YMDT::medium(),
        )
        .unwrap();

        // Same day, different time: only time differs.
        // Case 3: Mixed range, only time differs. Date formatted once, glued to time range.
        assert_writeable_eq!(
            fmt.format(&start, &end_same_day),
            "Dec 22, 2023, 9:00\u{202f}AM\u{2009}–\u{2009}5:00\u{202f}PM"
        );

        // Different day, different time: date differs.
        // Case 4: Mixed range, date differs. Falls back to full range fallback.
        assert_writeable_eq!(
            fmt.format(&start, &end_next_day),
            "Dec 22, 2023, 9:00:00\u{202f}AM\u{2009}–\u{2009}Dec 23, 2023, 5:00:00\u{202f}PM"
        );
    }

    // 4. Edge Case: Date-only formatter with time difference
    {
        let fmt = FixedCalendarDateRangeFormatter::try_new(
            locale!("en").into(),
            fieldsets::YMD::medium(),
        )
        .unwrap();

        // Date-only formatter, but input has time diff (date is same).
        // Should format as a single date (no range).
        assert_writeable_eq!(fmt.format(&start, &end_same_day), "Dec 22, 2023");
    }
}

#[test]
fn test_date_range_any_calendar_dynamic_conversion() {
    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_next_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    // Explicitly request Buddhist calendar in the locale
    let fmt =
        DateRangeFormatter::try_new(locale!("th-u-ca-buddhist").into(), fieldsets::YMD::medium())
            .unwrap();

    // Gregorian input should be dynamically converted to Buddhist (2023 -> 2566)
    // Thai day range pattern has no spaces around en-dash: "22–23 ธ.ค. 2566"
    assert_writeable_eq!(fmt.format(&start, &end_next_day), "22–23 ธ.ค. 2566");
}

#[test]
fn test_date_range_timezone() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::Time;
    use icu_datetime::range::FixedCalendarDateRangeFormatter;
    use icu_locale_core::locale;
    use icu_time::zone::UtcOffset;
    use icu_time::{TimeZone, ZonedDateTime};
    use writeable::assert_writeable_eq;

    let zone1 = TimeZone::from_iana_id("Europe/Paris")
        .with_offset(Some(UtcOffset::try_from_seconds(3600).unwrap())); // UTC+1
    let zone2 = TimeZone::from_iana_id("Europe/Athens")
        .with_offset(Some(UtcOffset::try_from_seconds(7200).unwrap())); // UTC+2

    let start = ZonedDateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
        zone: zone1,
    };
    let end = ZonedDateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
        zone: zone2,
    };

    // YMDT with zone
    let fmt = FixedCalendarDateRangeFormatter::try_new(
        locale!("en").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();

    // Since timezones differ, it must fall back to formatting both sides fully and gluing them.
    assert_writeable_eq!(
        fmt.format(&start, &end),
        "Dec 22, 2023, 9:00:00\u{202f}AM GMT+1\u{2009}–\u{2009}Dec 22, 2023, 5:00:00\u{202f}PM GMT+2"
    );
}

#[test]
fn test_date_range_hebrew_leap() {
    use icu_calendar::Date;
    use icu_calendar::cal::Hebrew;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::FixedCalendarDateRangeFormatter;
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    // 2011 was a Hebrew leap year (5771).
    // 2011-03-04 is 28 Adar I 5771.
    // 2011-04-03 is 28 Adar II 5771.
    let start_greg = Date::try_new_gregorian(2011, 3, 4).unwrap();
    let end_greg = Date::try_new_gregorian(2011, 4, 3).unwrap();

    let start = DateTime {
        date: start_greg.to_calendar(Hebrew),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end = DateTime {
        date: end_greg.to_calendar(Hebrew),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    let fmt =
        FixedCalendarDateRangeFormatter::try_new(locale!("en").into(), fieldsets::YMD::medium())
            .unwrap();

    // Should format as a range spanning Adar I and Adar II.
    assert_writeable_eq!(
        fmt.format(&start, &end),
        "28 Adar I\u{2009}–\u{2009}28 Adar II 5771"
    );
}

/// Range version of `test_minute_optional_hour_cycle` from `simple_test` (PR #8237)
#[test]
fn test_minute_optional_hour_cycle() {
    use icu_datetime::options::TimePrecision;
    use icu_datetime::range::NoCalendarRangeFormatter;

    let time = Time::try_new(7, 0, 0, 0).unwrap();
    let time_hour_diff = Time::try_new(8, 0, 0, 0).unwrap();
    let time_minute_diff = Time::try_new(7, 15, 0, 0).unwrap();

    let fs = fieldsets::T::short().with_time_precision(TimePrecision::MinuteOptional);

    // en-US (default h12): zero minutes omitted
    let fmt_en = NoCalendarRangeFormatter::try_new(locale!("en-US").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_en.format(&time, &time_hour_diff),
        "7\u{2009}–\u{2009}8\u{202f}AM"
    );
    assert_writeable_eq!(
        fmt_en.format(&time, &time_minute_diff),
        // The range pattern takes precedence here
        "7:00\u{2009}–\u{2009}7:15\u{202f}AM"
    );

    // fr (default h23): zero minutes retained
    let fmt_fr = NoCalendarRangeFormatter::try_new(locale!("fr").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_fr.format(&time, &time_hour_diff),
        "07:00\u{2009}–\u{2009}08:00"
    );
    assert_writeable_eq!(
        fmt_fr.format(&time, &time_minute_diff),
        "07:00\u{2009}–\u{2009}07:15"
    );

    // en-US with -u-hc-h23 override: zero minutes retained
    let fmt_en_h23 =
        NoCalendarRangeFormatter::try_new(locale!("en-US-u-hc-h23").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_en_h23.format(&time, &time_hour_diff),
        "07:00\u{2009}–\u{2009}08:00"
    );
    assert_writeable_eq!(
        fmt_en_h23.format(&time, &time_minute_diff),
        "07:00\u{2009}–\u{2009}07:15"
    );

    // fr with -u-hc-h12 override: zero minutes omitted
    let fmt_fr_12 = NoCalendarRangeFormatter::try_new(locale!("fr-u-hc-h12").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_fr_12.format(&time, &time_hour_diff),
        "7\u{2009}–\u{2009}8\u{202f}AM"
    );
    assert_writeable_eq!(
        fmt_fr_12.format(&time, &time_minute_diff),
        // The range pattern takes precedence here
        "7:00\u{2009}–\u{2009}7:15\u{202f}AM"
    );

    // fr with -u-hc-h11 override: zero minutes omitted
    let fmt_fr_h11 = NoCalendarRangeFormatter::try_new(locale!("fr-u-hc-h11").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_fr_h11.format(&time, &time_hour_diff),
        "7\u{2009}–\u{2009}8\u{202f}AM"
    );
    assert_writeable_eq!(
        fmt_fr_h11.format(&time, &time_minute_diff),
        // The range pattern takes precedence here
        "7:00\u{2009}–\u{2009}7:15\u{202f}AM"
    );

    // en-US with -u-hc-c24 override: zero minutes retained
    let fmt_en_c24 =
        NoCalendarRangeFormatter::try_new(locale!("en-US-u-hc-c24").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_en_c24.format(&time, &time_hour_diff),
        "07:00\u{2009}–\u{2009}08:00"
    );
    assert_writeable_eq!(
        fmt_en_c24.format(&time, &time_minute_diff),
        "07:00\u{2009}–\u{2009}07:15"
    );

    // fr with -u-hc-c12 override: zero minutes omitted
    let fmt_fr_c12 = NoCalendarRangeFormatter::try_new(locale!("fr-u-hc-c12").into(), fs).unwrap();
    assert_writeable_eq!(
        fmt_fr_c12.format(&time, &time_hour_diff),
        "7\u{2009}–\u{2009}8\u{202f}AM"
    );
    assert_writeable_eq!(
        fmt_fr_c12.format(&time, &time_minute_diff),
        // The range pattern takes precedence here
        "7:00\u{2009}–\u{2009}7:15\u{202f}AM"
    );
}

#[test]
fn test_date_range_ej() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::{DateRangeFormatter, FixedCalendarDateRangeFormatter};
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    let start_greg = Date::try_new_gregorian(2024, 8, 9).unwrap(); // Friday
    let end_greg = Date::try_new_gregorian(2024, 8, 10).unwrap(); // Saturday

    let start = DateTime {
        date: start_greg,
        time: Time::try_new(20, 40, 0, 0).unwrap(),
    };
    let end_same_day = DateTime {
        date: start_greg,
        time: Time::try_new(21, 50, 0, 0).unwrap(),
    };
    let end_next_day = DateTime {
        date: end_greg,
        time: Time::try_new(21, 50, 0, 0).unwrap(),
    };

    let fs = fieldsets::E::short().with_time_hm();

    let fmt = FixedCalendarDateRangeFormatter::try_new(locale!("en-US").into(), fs).unwrap();
    let any_cal_fmt = DateRangeFormatter::try_new(locale!("en-US").into(), fs).unwrap();

    // Overlap patterns like `ej` (weekday + time) operate as unified date-time patterns
    // without independent date and glue separation in `DateTimeZonePatternSelectionData`.
    // When range formatting is invoked, both formatters cleanly produce the expected
    // fallback range output without field conflicts.
    //
    // Note: This behavior relies on the assumption that any locale in CLDR supporting `ej` in
    // date/time patterns also defines corresponding interval range pattern data. This invariant
    // is verified across all locales and calendars in datagen tests:
    // `icu_provider_source::datetime::range_patterns::tests`.
    assert_writeable_eq!(
        fmt.format(&start, &end_same_day),
        "Fri 8:40\u{202f}PM\u{2009}–\u{2009}Fri 9:50\u{202f}PM"
    );
    assert_writeable_eq!(
        fmt.format(&start, &end_next_day),
        "Fri 8:40\u{202f}PM\u{2009}–\u{2009}Sat 9:50\u{202f}PM"
    );

    assert_writeable_eq!(
        any_cal_fmt.format(&start, &end_same_day),
        "Fri 8:40\u{202f}PM\u{2009}–\u{2009}Fri 9:50\u{202f}PM"
    );
    assert_writeable_eq!(
        any_cal_fmt.format(&start, &end_next_day),
        "Fri 8:40\u{202f}PM\u{2009}–\u{2009}Sat 9:50\u{202f}PM"
    );
}

// Tests documenting issues when date range formatting falls back to `root` (`und`).
//
// TODO(#8359): Implement a fix and enable these tests.
//
// When a locale or calendar lacks custom interval patterns in CLDR, locale fallback walks up
// the hierarchy to `root` (`und`). CLDR's `root` defines interval patterns using English/ISO conventions
// (e.g. `y MMM d–d`, `G y-MM-dd–y-MM-dd`). This causes two major categories of problems:
//
// 1. Missing Names: The `root` range pattern often contains
//    symbols/lengths (such as `MMM` or `G`) that were not loaded by the single `DateTimeFormatter`
//    (e.g., `zh` whose single date pattern uses numeric month `M`, or `YMD::long()` which loads
//    `MMMM` but not `MMM`). This can lead to violations of assumptions.
//
// 2. Incorrect patterns: `root` patterns impose YMD order on
//    locales that use Day-Month-Year (such as German, French, Spanish). The results
//    will be incorrect, and, importantly, not consistent with the results of regular date
//    formatting, which can be confusing.

/// Chinese (zh) with `YMD::medium()`:
/// Single date pattern is "y年M月d日" (numeric month 'M').
/// Single `DateTimeFormatter` only loads numeric month data, not abbreviated month names ("MMM").
/// But range pattern falls back to `root` (`und`), which specifies "y MMM d–d".
/// When executing the `root` pattern, `FormattedSingleSide` fails with `NamesNotLoaded`.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "unexpected error in FormattedSingleSide: NamesNotLoaded")]
fn test_root_fallback_names_not_loaded_zh() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::DateRangeFormatter;
    use icu_locale_core::locale;

    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    let fmt_zh =
        DateRangeFormatter::try_new(locale!("zh").into(), fieldsets::YMD::medium()).unwrap();
    let _ = fmt_zh.format(&start, &end_day).to_string();
}

/// Spanish with Hebrew calendar (es-u-ca-hebrew) and `YMD::long()`:
/// Single `DateTimeFormatter` for `YMD::long()` loads full month names ("MMMM"), but range pattern
/// falls back to `root` (`und`), which uses abbreviated month names ("MMM").
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "unexpected error in FormattedSingleSide: NamesNotLoaded")]
fn test_root_fallback_names_not_loaded_es_hebrew() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::DateRangeFormatter;
    use icu_locale_core::locale;

    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    let fmt_es_hebrew =
        DateRangeFormatter::try_new(locale!("es-u-ca-hebrew").into(), fieldsets::YMD::long())
            .unwrap();
    let _ = fmt_es_hebrew.format(&start, &end_day).to_string();
}

/// German with Buddhist calendar (de-u-ca-buddhist) and `YMD::long()`:
/// Single `DateTimeFormatter` for `YMD::long()` loads full month names ("MMMM"), but range pattern
/// falls back to `root` (`und`), which uses abbreviated month names ("MMM").
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "unexpected error in FormattedSingleSide: NamesNotLoaded")]
fn test_root_fallback_names_not_loaded_de_buddhist() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::DateRangeFormatter;
    use icu_locale_core::locale;

    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    let fmt_de_buddhist =
        DateRangeFormatter::try_new(locale!("de-u-ca-buddhist").into(), fieldsets::YMD::long())
            .unwrap();
    let _ = fmt_de_buddhist.format(&start, &end_day).to_string();
}

#[test]
fn test_root_fallback_issues_field_order() {
    use icu_calendar::Date;
    use icu_datetime::fieldsets;
    use icu_datetime::input::{DateTime, Time};
    use icu_datetime::range::DateRangeFormatter;
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    let start = DateTime {
        date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
        time: Time::try_new(9, 0, 0, 0).unwrap(),
    };
    let end_day = DateTime {
        date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
        time: Time::try_new(17, 0, 0, 0).unwrap(),
    };

    // German (de) uses Day.Month.Year order (e.g. "22.12.2023").
    // For Buddhist calendar in YMD::medium(), `de` has no custom range pattern in CLDR.
    // Falling back to `root` uses the root pattern "G y-MM-dd – y-MM-dd" (ISO order with era first).
    let fmt_de_buddhist =
        DateRangeFormatter::try_new(locale!("de-u-ca-buddhist").into(), fieldsets::YMD::medium())
            .unwrap();
    // This produces "BE 2566-12-22 – 2566-12-23" instead of German Day-first order:
    assert_writeable_eq!(
        fmt_de_buddhist.format(&start, &end_day),
        "BE 2566-12-22\u{2009}–\u{2009}2566-12-23"
    );

    // Hebrew calendar in German:
    let fmt_de_hebrew =
        DateRangeFormatter::try_new(locale!("de-u-ca-hebrew").into(), fieldsets::YMD::medium())
            .unwrap();
    // This produces "AM 5784-04-10 – 5784-04-11" instead of German Day-first order:
    assert_writeable_eq!(
        fmt_de_hebrew.format(&start, &end_day),
        "AM 5784-04-10\u{2009}–\u{2009}5784-04-11"
    );
}
