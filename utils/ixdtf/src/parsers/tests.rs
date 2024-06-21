// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;
use alloc::vec::Vec;

use crate::{
    parsers::{
        records::{
            Annotation, DateRecord, IxdtfParseRecord, TimeRecord, TimeZoneAnnotation,
            TimeZoneRecord,
        },
        IxdtfParser,
    },
    ParserError,
};

#[test]
fn temporal_parser_basic() {
    let basic_result = IxdtfParser::from_str("20201108").parse().unwrap();
    let sep_result = IxdtfParser::from_str("2020-11-08").parse().unwrap();

    assert_eq!(
        basic_result.date,
        Some(DateRecord {
            year: 2020,
            month: 11,
            day: 8,
        })
    );
    assert_eq!(basic_result.date, sep_result.date,);
}

#[test]
fn temporal_date_time_max() {
    let result = IxdtfParser::from_str(
        "+002020-11-08T12:28:32.329402834[!America/Argentina/ComodRivadavia][!u-ca=iso8601]",
    )
    .parse()
    .unwrap();

    assert_eq!(
        result.time,
        Some(TimeRecord {
            hour: 12,
            minute: 28,
            second: 32,
            nanosecond: 329402834,
        })
    );
}

#[test]
fn good_zoned_date_time() {
    let result = IxdtfParser::from_str("2020-04-08[America/Chicago]")
        .parse()
        .unwrap();
    assert_eq!(
        result.date,
        Some(DateRecord {
            year: 2020,
            month: 4,
            day: 8,
        })
    );
    let tz_annotation = result.tz.unwrap();
    assert_eq!(
        tz_annotation,
        TimeZoneAnnotation {
            critical: false,
            tz: TimeZoneRecord::Name("America/Chicago".as_bytes())
        }
    );
}

#[test]
fn bad_zoned_date_time() {
    let bad_value = "2020-04-08(America/Chicago]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[America/Chicago)";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::IanaChar),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[America/ Chicago)";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::IanaCharPostSeparator),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[Amer";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::AbruptEnd {
            location: "IANATimeZoneName"
        }),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[u-ca=iso8601][Europe/London]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::AnnotationKeyLeadingChar),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn good_extended_year_parsing() {
    let extended_year = "+002020-11-08";
    let result = IxdtfParser::from_str(extended_year).parse().unwrap();
    assert_eq!(
        result.date,
        Some(DateRecord {
            year: 2020,
            month: 11,
            day: 8,
        }),
        "Extended year \"{extended_year}\" should pass."
    );

    let extended_year = "-002020-11-08";
    let result = IxdtfParser::from_str(extended_year).parse().unwrap();
    assert_eq!(
        result.date,
        Some(DateRecord {
            year: -2020,
            month: 11,
            day: 8,
        }),
        "Extended year \"{extended_year}\" should pass."
    );
}

#[test]
fn bad_extended_year() {
    let bad_year = "-000000-11-08";
    let err = IxdtfParser::from_str(bad_year).parse();
    assert_eq!(
        err,
        Err(ParserError::DateExtendedYear),
        "Invalid extended year parsing: \"{bad_year}\" should fail to parse."
    );

    let bad_year = "-1000000-11-08";
    let err = IxdtfParser::from_str(bad_year).parse();
    assert_eq!(
        err,
        Err(ParserError::DateMonth),
        "Invalid year range parsing: \"{bad_year}\" should fail to parse."
    );

    let bad_year = "+10000001108";
    let err = IxdtfParser::from_str(bad_year).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid year range parsing: \"{bad_year}\" should fail to parse."
    );
}

#[test]
fn good_annotations_date_time() {
    let result = IxdtfParser::from_str(
        "2020-11-08[!America/Argentina/ComodRivadavia][u-ca=iso8601][foo=bar]",
    )
    .parse()
    .unwrap();

    let tz_annotation = result.tz.unwrap();

    assert_eq!(
        tz_annotation,
        TimeZoneAnnotation {
            critical: true,
            tz: TimeZoneRecord::Name("America/Argentina/ComodRivadavia".as_bytes()),
        }
    );

    assert_eq!(result.calendar, Some("iso8601".as_bytes()));

    let omit_result = IxdtfParser::from_str("+0020201108[!u-ca=iso8601][f-1a2b=a0sa-2l4s]")
        .parse()
        .unwrap();

    assert!(omit_result.tz.is_none());

    assert_eq!(omit_result.calendar, Some("iso8601".as_bytes()));
}

#[test]
fn invalid_day_for_month() {
    let bad_value = "2021-02-29";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "1900-02-29";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-04-31";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-04-00";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn invalid_month() {
    let bad_value = "2021-00-29";
    let mut ixdtf = IxdtfParser::from_str(bad_value);
    let err = ixdtf.parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidMonthRange),
        "Invalid month range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "1900-13-29";
    let mut ixdtf = IxdtfParser::from_str(bad_value);
    let err = ixdtf.parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidMonthRange),
        "Invalid month range parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn invalid_annotations() {
    let bad_value = "2021-01-29 02:12:48+01:00:00(u-ca=iso8601]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=iso8601)";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::AnnotationValueChar),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u][u-ca=iso8601]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::InvalidAnnotation),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=iso8601][!foo=bar]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::UnrecognizedCritical),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn invalid_calendar_annotations() {
    let bad_value = "2021-01-29 02:12:48+01:00:00[!u-ca=iso8601][u-ca=japanese]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::CriticalDuplicateCalendar),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=japanese][u-ca=iso8601][!u-ca=gregorian]";
    let err = IxdtfParser::from_str(bad_value).parse();
    assert_eq!(
        err,
        Err(ParserError::CriticalDuplicateCalendar),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn duplicate_same_calendar() {
    let duplicate_calendars = [
        "2020-11-11[!u-ca=iso8601][u-ca=iso8601]",
        "2020-11-11[u-ca=iso8601][!u-ca=iso8601]",
    ];

    for duplicate in duplicate_calendars {
        let result = IxdtfParser::from_str(duplicate).parse().unwrap();
        let calendar = result.calendar.unwrap();
        assert_eq!(
            calendar,
            "iso8601".as_bytes(),
            "Invalid Ixdtf parsing: \"{duplicate}\" should fail parsing."
        );
    }
}

#[test]
fn valid_calendar_annotations() {
    let value = "2021-01-29 02:12:48+01:00:00[u-ca=japanese][u-ca=iso8601][u-ca=gregorian]";
    let mut annotations = Vec::default();
    let result = IxdtfParser::from_str(value)
        .parse_with_annotation_handler(|annotation| {
            annotations.push(annotation.clone());
            Some(annotation)
        })
        .unwrap();
    assert_eq!(
        result.calendar,
        Some("japanese".as_bytes()),
        "Valid annotation parsing: \"{value}\" should parse calendar as 'japanese'."
    );

    assert_eq!(
        annotations[1],
        Annotation {
            critical: false,
            key: "u-ca".as_bytes(),
            value: "iso8601".as_bytes()
        },
        "Valid annotation parsing: \"{value}\" should parse first annotation as 'iso8601'."
    );

    assert_eq!(
        annotations[2],
        Annotation {
            critical: false,
            key: "u-ca".as_bytes(),
            value: "gregorian".as_bytes(),
        },
        "Valid annotation parsing: \"{value}\" should parse second annotation as 'gregorian'."
    );

    let value = "2021-01-29 02:12:48+01:00:00[u-ca=gregorian][u-ca=iso8601][u-ca=japanese]";
    let mut annotations = Vec::default();
    let result = IxdtfParser::from_str(value)
        .parse_with_annotation_handler(|annotation| {
            annotations.push(annotation.clone());
            Some(annotation)
        })
        .unwrap();
    assert_eq!(
        result.calendar,
        Some("gregorian".as_bytes()),
        "Valid annotation parsing: \"{value}\" should parse calendar as 'gregorian'."
    );

    assert_eq!(
        annotations[1],
        Annotation {
            critical: false,
            key: "u-ca".as_bytes(),
            value: "iso8601".as_bytes()
        },
        "Valid annotation parsing: \"{value}\" should parse first annotation as 'iso8601'."
    );

    assert_eq!(
        annotations[2],
        Annotation {
            critical: false,
            key: "u-ca".as_bytes(),
            value: "japanese".as_bytes(),
        },
        "Valid annotation parsing: \"{value}\" should parse second annotation as 'japanese'."
    );
}

#[test]
fn temporal_year_month() {
    let possible_year_months = [
        "+002020-11",
        "2020-11[u-ca=iso8601]",
        "+00202011",
        "202011[u-ca=iso8601]",
    ];

    for ym in possible_year_months {
        let result = IxdtfParser::from_str(ym).parse_year_month().unwrap();

        let date = result.date.unwrap();

        assert_eq!(date.year, 2020);
        assert_eq!(date.month, 11);
    }
}

#[test]
fn invalid_year_month() {
    // Valid AnnotatedDateTime, but not a valid AnnotatedYearMonth.
    let bad_value = "+002020-11T12:28:32[!u-ca=iso8601]";
    let err = IxdtfParser::from_str(bad_value).parse_year_month();
    assert_eq!(err, Err(ParserError::InvalidEnd));

    let bad_value = "-202011[!u-ca=iso8601]";
    let err = IxdtfParser::from_str(bad_value).parse_year_month();
    assert_eq!(err, Err(ParserError::DateMonth));

    let bad_value = "-00202011Z[Europe/Berlin]";
    let err = IxdtfParser::from_str(bad_value).parse_year_month();
    assert_eq!(err, Err(ParserError::InvalidEnd));
}

#[test]
fn temporal_month_day() {
    let possible_month_day = ["11-07", "1107[+04:00]", "--11-07", "--1107[+04:00]"];

    for md in possible_month_day {
        let result = IxdtfParser::from_str(md).parse_month_day().unwrap();

        let date = result.date.unwrap();

        assert_eq!(date.month, 11);
        assert_eq!(date.day, 7);
    }
}

#[test]
fn invalid_month_day() {
    let bad_value = "-11-07";
    let err = IxdtfParser::from_str(bad_value).parse_month_day();
    assert_eq!(err, Err(ParserError::MonthDayHyphen))
}

#[test]
fn temporal_time() {
    let possible_times = [
        "T12:01:04",
        "t12:01:04",
        "12:01:04",
        "12:01:04[u-ca=iso8601]",
        "12:01:04[+04:00][u-ca=iso8601]",
        "12:01:04-05:00[America/from_str_York][u-ca=iso8601]",
    ];

    for time in possible_times {
        let result = IxdtfParser::from_str(time).parse_time().unwrap();
        let time = result.time.unwrap();
        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 1);
        assert_eq!(time.second, 4);
    }
}

#[test]
fn invalid_time() {
    let bad_value = "20240801";
    let err = IxdtfParser::from_str(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "24-12-08";
    let err = IxdtfParser::from_str(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::TimeHour),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    // Attempts to parse UTC offset: -12, leaving -08 on end as junk.
    let bad_value = "T19-12-08";
    let err = IxdtfParser::from_str(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "T19:12-089";
    let err = IxdtfParser::from_str(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "T19:120-08";
    let err = IxdtfParser::from_str(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::TimeSeparator),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn temporal_valid_instant_strings() {
    let instants = [
        "1970-01-01T00:00+00:00[!Africa/Abidjan]",
        "1970-01-01T00:00+00:00[UTC]",
        "1970-01-01T00:00Z[!Europe/Vienna]",
    ];

    for test in instants {
        let result = IxdtfParser::from_str(test).parse();
        assert!(result.is_ok());
    }
}

#[test]
#[cfg(feature = "duration")]
fn temporal_duration_parsing() {
    use crate::parsers::{
        records::{DateDurationRecord, DurationParseRecord, Sign, TimeDurationRecord},
        IsoDurationParser,
    };

    let durations = [
        "p1y1m1dt1h1m1s",
        "P1Y1M1W1DT1H1M1.1S",
        "-P1Y1M1W1DT1H1M1.123456789S",
        "-P1Y3wT0,5H",
    ];

    for dur in durations {
        let ok_result = IsoDurationParser::from_str(dur).parse();
        assert!(
            ok_result.is_ok(),
            "Failing to parse a valid ISO 8601 target: \"{dur}\" should pass."
        );
    }

    let sub_second = IsoDurationParser::from_str(durations[2]).parse().unwrap();
    assert_eq!(
        sub_second,
        DurationParseRecord {
            sign: Sign::Negative,
            date: Some(DateDurationRecord {
                years: 1,
                months: 1,
                weeks: 1,
                days: 1,
            }),
            time: Some(TimeDurationRecord::Seconds {
                hours: 1,
                minutes: 1,
                seconds: 1,
                fraction: 123456789
            })
        },
        "Failing to parse a valid Duration string: \"{}\" should pass.",
        durations[2]
    );

    let test_result = IsoDurationParser::from_str(durations[3]).parse().unwrap();
    assert_eq!(
        test_result,
        DurationParseRecord {
            sign: Sign::Negative,
            date: Some(DateDurationRecord {
                years: 1,
                months: 0,
                weeks: 3,
                days: 0,
            }),
            time: Some(TimeDurationRecord::Hours {
                hours: 0,
                fraction: 1_800_000_000_000,
            })
        }
    );
}

#[test]
#[cfg(feature = "duration")]
fn temporal_invalid_durations() {
    use crate::parsers::IsoDurationParser;

    let invalids = [
        "P1Y1M1W0,5D",
        "P1Y1M1W1DT1H1M1.123456789123S",
        "+PT",
        "P1Y1M1W1DT1H0.5M0.5S",
    ];

    for test in invalids {
        let err = IsoDurationParser::from_str(test).parse();
        assert!(
            err.is_err(),
            "Invalid ISO8601 Duration target: \"{test}\" should fail duration parsing."
        );
    }
}

#[test]
#[cfg(feature = "duration")]
fn maximum_duration_fraction() {
    use crate::parsers::IsoDurationParser;

    let test = "P1Y1DT1.999999999H";
    let result = IsoDurationParser::from_str(test).parse();
    assert!(result.is_ok());

    let test = "P1Y1DT1H1.999999999M";
    let result = IsoDurationParser::from_str(test).parse();
    assert!(result.is_ok());

    let test = "P1Y1DT1H1M1.999999999S";
    let result = IsoDurationParser::from_str(test).parse();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "duration")]
fn duration_exceeds_range() {
    use crate::parsers::IsoDurationParser;

    let test = "P1000000000000000000000000000000000000000YT1H";
    let err = IsoDurationParser::from_str(test).parse();
    assert_eq!(err, Err(ParserError::DurationValueExceededRange));

    let test = "P1YT1000000000000000000000000000000000000000H";
    let err = IsoDurationParser::from_str(test).parse();
    assert_eq!(err, Err(ParserError::DurationValueExceededRange));
}

#[test]
fn temporal_invalid_iso_datetime_strings() {
    // NOTE: The below tests were initially pulled from test262's `argument-string-invalid`
    const INVALID_DATETIME_STRINGS: [&str; 32] = [
        "", // 1
        "invalid iso8601",
        "2020-01-00",
        "2020-01-32",
        "2020-02-30",
        "2021-02-29",
        "2020-00-01",
        "2020-13-01",
        "2020-01-01T",
        "2020-01-01T25:00:00",
        "2020-01-01T01:60:00",
        "2020-01-01T01:60:61",
        "2020-01-01junk",
        "2020-01-01T00:00:00junk",
        "2020-01-01T00:00:00+00:00junk",
        "2020-01-01T00:00:00+00:00[UTC]junk",
        "2020-01-01T00:00:00+00:00[UTC][u-ca=iso8601]junk",
        "02020-01-01",
        "2020-001-01",
        "2020-01-001",
        "2020-01-01T001",
        "2020-01-01T01:001",
        "2020-01-01T01:01:001",
        "2020-W01-1",
        "2020-001",
        "+0002020-01-01",
        "2020-01",
        "+002020-01",
        "01-01",
        "2020-W01",
        "P1Y",
        "-P12Y",
    ];

    for invalid_target in INVALID_DATETIME_STRINGS {
        let error_result = IxdtfParser::from_str(invalid_target).parse();
        assert!(
            error_result.is_err(),
            "Invalid ISO8601 `DateTime` target: \"{invalid_target}\" should fail parsing."
        );
    }
}

#[test]
fn test_correct_datetime() {
    let dt = "2022-11-08";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 11,
                day: 8,
            }),
            time: None,
            offset: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "20220605";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            offset: None,
            time: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05T04";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            time: Some(TimeRecord {
                hour: 4,
                minute: 0,
                second: 0,
                nanosecond: 0,
            }),
            offset: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05t04:34";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 0,
                nanosecond: 0,
            }),
            offset: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 04:34:22";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                nanosecond: 0,
            }),
            offset: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 04:34:22.000";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                nanosecond: 0,
            }),
            offset: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 043422.000";
    let parsed = IxdtfParser::from_str(dt).parse();
    assert_eq!(
        parsed,
        Ok(IxdtfParseRecord {
            date: Some(DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            }),
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                nanosecond: 0,
            }),
            offset: None,
            tz: None,
            calendar: None,
        })
    );
}

#[test]
fn test_bad_date() {
    let dt = "-2022-06-05";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::DateExtendedYear));

    let dt = "!2022-06-05";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::DateYear));

    let dt = "20-06-05";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::DateYear));

    let dt = "2022-0605";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::DateSeparator));

    let dt = "202206-05";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::DateSeparator));

    let dt = "2022-06-05e";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::InvalidEnd));
}

#[test]
fn test_bad_time_spec_separator() {
    let dt = "2022-06-05  043422.000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::TimeHour));

    let dt = "2022-06-05 04:3422.000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::TimeSeparator));

    let dt = "2022-06-05 0434:22.000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::TimeSeparator));

    let dt = "2022-06-05 03422.000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::TimeSecond));

    let dt = "2022-06-05 3:42:22.000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::TimeHour));

    let dt = "2022-06-05 03:42:22;000";
    let err = IxdtfParser::from_str(dt).parse();
    assert_eq!(err, Err(ParserError::InvalidEnd));
}
