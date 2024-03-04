// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::ToOwned;

use crate::{
    parsers::{
        records::{DateRecord, IsoParseRecord, TimeRecord, TimeZone},
        IsoDurationParser, IxdtfParser,
    },
    ParserError,
};

#[test]
fn temporal_parser_basic() {
    let basic = "20201108";
    let basic_separated = "2020-11-08";

    let basic_result = IxdtfParser::new(basic).parse_date_time().unwrap();

    let sep_result = IxdtfParser::new(basic_separated).parse_date_time().unwrap();

    assert_eq!(
        basic_result.date,
        DateRecord {
            year: 2020,
            month: 11,
            day: 8,
        }
    );
    assert_eq!(basic_result.date, sep_result.date,);
}

#[test]
fn temporal_date_time_max() {
    let date_time =
        "+002020-11-08T12:28:32.329402834[!America/Argentina/ComodRivadavia][!u-ca=iso8601]";

    let result = IxdtfParser::new(date_time).parse_date_time().unwrap();

    assert_eq!(
        result.time,
        Some(TimeRecord {
            hour: 12,
            minute: 28,
            second: 32,
            millisecond: 329,
            microsecond: 402,
            nanosecond: 834,
        })
    );
}

#[test]
fn good_zoned_date_time() {
    let zdt = "2020-04-08[America/Chicago]";
    let result = IxdtfParser::new(zdt).parse_zoned_date_time().unwrap();
    assert_eq!(
        result.date,
        DateRecord {
            year: 2020,
            month: 4,
            day: 8,
        }
    );
    assert_eq!(
        result.tz,
        Some(TimeZone {
            name: Some("America/Chicago".to_owned()),
            offset: None,
        })
    );
}

#[test]
fn bad_zoned_date_time() {
    let bad_value = "2020-04-08(America/Chicago]";
    let err = IxdtfParser::new(bad_value).parse_zoned_date_time();
    assert_eq!(
        err,
        Err(ParserError::MissingRequiredTzAnnotation),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[America/Chicago)";
    let err = IxdtfParser::new(bad_value).parse_zoned_date_time();
    assert_eq!(
        err,
        Err(ParserError::IanaChar),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[America/ Chicago)";
    let err = IxdtfParser::new(bad_value).parse_zoned_date_time();
    assert_eq!(
        err,
        Err(ParserError::IanaCharPostSeparator),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[Amer";
    let err = IxdtfParser::new(bad_value).parse_zoned_date_time();
    assert_eq!(
        err,
        Err(ParserError::AbruptEnd),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2020-04-08[u-ca=iso8601][Europe/London]";
    let err = IxdtfParser::new(bad_value).parse_zoned_date_time();
    assert_eq!(
        err,
        Err(ParserError::MissingRequiredTzAnnotation),
        "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn good_extended_year_parsing() {
    let extended_year = "+002020-11-08";
    let result = IxdtfParser::new(extended_year).parse_date_time().unwrap();
    assert_eq!(
        result.date,
        DateRecord {
            year: 2020,
            month: 11,
            day: 8,
        },
        "Extended year \"{extended_year}\" should pass."
    );

    let extended_year = "-002020-11-08";
    let result = IxdtfParser::new(extended_year).parse_date_time().unwrap();
    assert_eq!(
        result.date,
        DateRecord {
            year: -2020,
            month: 11,
            day: 8,
        },
        "Extended year \"{extended_year}\" should pass."
    );
}

#[test]
fn bad_extended_year() {
    let bad_year = "-000000-11-08";
    let err = IxdtfParser::new(bad_year).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::DateExtendedYear),
        "Invalid extended year parsing: \"{bad_year}\" should fail to parse."
    );

    let bad_year = "-271821-11-08";
    let err = IxdtfParser::new(bad_year).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidYearRange),
        "Invalid year range parsing: \"{bad_year}\" should fail to parse."
    );

    let bad_year = "+275761-11-08";
    let err = IxdtfParser::new(bad_year).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidYearRange),
        "Invalid year range parsing: \"{bad_year}\" should fail to parse."
    );
}

#[test]
fn good_annotations_date_time() {
    let basic = "2020-11-08[America/Argentina/ComodRivadavia][u-ca=iso8601][foo=bar]";
    let omitted = "+0020201108[u-ca=iso8601][f-1a2b=a0sa-2l4s]";

    let result = IxdtfParser::new(basic).parse_date_time().unwrap();

    assert_eq!(
        result.tz,
        Some(TimeZone {
            name: Some("America/Argentina/ComodRivadavia".to_owned()),
            offset: None,
        })
    );

    assert_eq!(result.calendar, Some("iso8601".to_owned()));

    let omit_result = IxdtfParser::new(omitted).parse_date_time().unwrap();

    assert!(omit_result.tz.is_none());

    assert_eq!(omit_result.calendar, Some("iso8601".to_owned()));
}

#[test]
fn invalid_day_for_month() {
    let bad_value = "2021-02-29";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "1900-02-29";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-04-31";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-04-00";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidDayRange),
        "Invalid day range parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn invalid_month() {
    let bad_value = "2021-00-29";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidMonthRange),
        "Invalid month range parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "1900-13-29";
    let err = IxdtfParser::new(bad_value).parse_date_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidMonthRange),
        "Invalid month range parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn invalid_annotations() {
    let bad_value = "2021-01-29 02:12:48+01:00:00(u-ca=iso8601]";
    let err = IxdtfParser::new(bad_value).parse_instant();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=iso8601)";
    let err = IxdtfParser::new(bad_value).parse_instant();
    assert_eq!(
        err,
        Err(ParserError::AnnotationValueChar),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "2021-01-29 02:12:48+01:00:00[u][u-ca=iso8601]";
    let err = IxdtfParser::new(bad_value).parse_instant();
    assert_eq!(
        err,
        Err(ParserError::InvalidAnnotation),
        "Invalid annotation parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn temporal_year_month() {
    let possible_year_months = [
        "+002020-11",
        "2020-11[u-ca=iso8601]",
        "+00202011",
        "202011[u-ca=iso8601]",
        "+002020-11-07T12:28:32[!u-ca=iso8601]",
    ];

    for ym in possible_year_months {
        let result = IxdtfParser::new(ym).parse_year_month().unwrap();

        assert_eq!(result.date.year, 2020);
        assert_eq!(result.date.month, 11);
    }
}

#[test]
fn temporal_month_day() {
    let possible_month_day = [
        "11-07",
        "1107[+04:00]",
        "--11-07",
        "--1107[+04:00]",
        "+002020-11-07T12:28:32[!u-ca=iso8601]",
    ];

    for md in possible_month_day {
        let result = IxdtfParser::new(md).parse_month_day().unwrap();

        assert_eq!(result.date.month, 11);
        assert_eq!(result.date.day, 7);
    }
}

#[test]
fn temporal_time() {
    let possible_times = [
        "T12:01:04",
        "t12:01:04",
        "12:01:04",
        "12:01:04[u-ca=iso8601]",
        "12:01:04[+04:00][u-ca=iso8601]",
        "12:01:04-05:00[America/New_York][u-ca=iso8601]",
    ];

    for time in possible_times {
        let result = IxdtfParser::new(time).parse_time().unwrap();
        let time = result.time.unwrap();
        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 1);
        assert_eq!(time.second, 4);
    }
}

#[test]
fn invalid_time() {
    let bad_value = "20240801";
    let err = IxdtfParser::new(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "24-12-08";
    let err = IxdtfParser::new(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::DateYear),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    // Attempts to parse UTC offset: -12, leaving -08 on end as junk.
    let bad_value = "T19-12-08";
    let err = IxdtfParser::new(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::InvalidEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "T19:12-089";
    let err = IxdtfParser::new(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::AbruptEnd),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );

    let bad_value = "T19:120-08";
    let err = IxdtfParser::new(bad_value).parse_time();
    assert_eq!(
        err,
        Err(ParserError::TimeSeparator),
        "Invalid time parsing: \"{bad_value}\" should fail to parse."
    );
}

#[test]
fn temporal_invalid_annotations() {
    let invalid_annotations = [
        "2020-11-11[!u-ca=iso8601][u-ca=iso8601]",
        "2020-11-11[u-ca=iso8601][!u-ca=iso8601]",
        "2020-11-11[u-ca=iso8601][!rip=this-invalid-annotation]",
    ];

    for invalid in invalid_annotations {
        let err_result = IxdtfParser::new(invalid).parse_month_day();
        assert!(
            err_result.is_err(),
            "Invalid ISO annotation parsing: \"{invalid}\" should fail parsing."
        );
    }
}

#[test]
fn temporal_valid_instant_strings() {
    let instants = [
        "1970-01-01T00:00+00:00[!Africa/Abidjan]",
        "1970-01-01T00:00+00:00[UTC]",
        "1970-01-01T00:00Z[!Europe/Vienna]",
    ];

    for test in instants {
        let result = IxdtfParser::new(test).parse_instant();
        assert!(result.is_ok());
    }
}

#[test]
fn temporal_duration_parsing() {
    let durations = [
        "p1y1m1dt1h1m1s",
        "P1Y1M1W1DT1H1M1.1S",
        "-P1Y1M1W1DT1H1M1.123456789S",
        "-P1Y3wT0,5H",
    ];

    for dur in durations {
        let ok_result = IsoDurationParser::new(dur).parse();
        assert!(
            ok_result.is_ok(),
            "Failing to parse a valid ISO 8601 target: \"{dur}\" should pass."
        );
    }

    let sub_second = IsoDurationParser::new(durations[2]).parse().unwrap();

    assert_eq!(sub_second.milliseconds as i32, -123);
    assert_eq!(sub_second.microseconds as i32, -456);
    assert_eq!(sub_second.nanoseconds as i32, -789);

    let test_result = IsoDurationParser::new(durations[3]).parse().unwrap();

    assert_eq!(test_result.years, -1);
    assert_eq!(test_result.weeks, -3);
    assert_eq!(test_result.minutes, -30.0);
}

#[test]
fn temporal_invalid_durations() {
    let invalids = [
        "P1Y1M1W0,5D",
        "P1Y1M1W1DT1H1M1.123456789123S",
        "+PT",
        "P1Y1M1W1DT1H0.5M0.5S",
    ];

    for test in invalids {
        let err = IsoDurationParser::new(test).parse();
        assert!(
            err.is_err(),
            "Invalid ISO8601 Duration target: \"{test}\" should fail duration parsing."
        );
    }
}

#[test]
fn temporal_invalid_iso_datetime_strings() {
    // NOTE: The below tests were initially pulled from test262's `argument-string-invalid`
    const INVALID_DATETIME_STRINGS: [&str; 34] = [
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
        // valid, but outside the supported year range:
        "-999999-01-01",
        "+999999-01-01",
    ];

    for invalid_target in INVALID_DATETIME_STRINGS {
        let error_result = IxdtfParser::new(invalid_target).parse_date_time();
        assert!(
            error_result.is_err(),
            "Invalid ISO8601 `DateTime` target: \"{invalid_target}\" should fail parsing."
        );
    }
}

#[test]
fn test_correct_datetime() {
    let dt = "2022-11-08";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 11,
                day: 8,
            },
            time: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "20220605";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: None,
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05T04";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: Some(TimeRecord {
                hour: 4,
                minute: 0,
                second: 0,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            }),
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05t04:34";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 0,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            }),
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 04:34:22";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            }),
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 04:34:22.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            }),
            tz: None,
            calendar: None,
        })
    );

    let dt = "2022-06-05 043422.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(
        parsed,
        Ok(IsoParseRecord {
            date: DateRecord {
                year: 2022,
                month: 6,
                day: 5,
            },
            time: Some(TimeRecord {
                hour: 4,
                minute: 34,
                second: 22,
                millisecond: 0,
                microsecond: 0,
                nanosecond: 0,
            }),
            tz: None,
            calendar: None,
        })
    );
}

#[test]
fn test_bad_date() {
    let dt = "-2022-06-05";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::DateExtendedYear));

    let dt = "!2022-06-05";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::DateYear));

    let dt = "20-06-05";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::DateYear));

    let dt = "2022-0605";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::DateSeparator));

    let dt = "202206-05";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::DateSeparator));

    let dt = "2022-06-05e";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::InvalidEnd));
}

#[test]
fn test_bad_time_spec_separator() {
    let dt = "2022-06-05  043422.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::TimeHour));

    let dt = "2022-06-05 04:3422.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::TimeSeparator));

    let dt = "2022-06-05 0434:22.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::TimeSeparator));

    let dt = "2022-06-05 03422.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::TimeSecond));

    let dt = "2022-06-05 3:42:22.000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::TimeHour));

    let dt = "2022-06-05 03:42:22;000";
    let parsed = IxdtfParser::new(dt).parse_date_time();
    assert_eq!(parsed, Err(ParserError::InvalidEnd));
}
