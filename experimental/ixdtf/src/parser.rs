// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(PartialEq, Clone, Copy, Debug)]
enum ParseError {
    DateYear,
    DateExtendedYear,
    DateFourDigitYear,
    DateMonth,
    DateDay,
    DateUnexpectedEnd,
    TimeHour,
    TimeMinute,
    TimeSecond,
    FractionPart,
    DateSeparator,
    TimeSeparator,
    DecimalSeparator,
    TimeUnexpectedEnd,
}

#[derive(Clone, Copy)]
enum DateTimeSeparator {
    CapitalT,
    LowCaseT,
    Space,
}

impl DateTimeSeparator {
    fn value(&self) -> u8 {
        match *self {
            DateTimeSeparator::CapitalT => b'T',
            DateTimeSeparator::LowCaseT => b't',
            DateTimeSeparator::Space => b' ',
        }
    }
}

#[derive(Clone, Copy)]
enum DecimalSeparator {
    Dot,
    Comma,
}

impl DecimalSeparator {
    fn value(&self) -> u8 {
        match *self {
            DecimalSeparator::Dot => b'.',
            DecimalSeparator::Comma => b',',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SegmentIndex {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedDate {
    pub year: Option<SegmentIndex>,
    pub month: Option<SegmentIndex>,
    pub day: Option<SegmentIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedTime {
    pub hour: Option<SegmentIndex>,
    pub minute: Option<SegmentIndex>,
    pub second: Option<SegmentIndex>,
    pub nano_second: Option<SegmentIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedDateTime {
    pub parsed_date: Option<ParsedDate>,
    pub parsed_time: Option<ParsedTime>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DateParser {}

impl DateParser {
    fn parse_date_extended_year(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if let Some((first, remains)) = s.split_first() {
            if first == &b'+' || first == &b'-' {
                let mut cnt = 0;
                let mut mut_inner_remains = remains;
                while cnt < 6 {
                    if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                        if (&b'0'..=&b'9').contains(&inner_first) {
                            mut_inner_remains = inner_remains;
                        } else {
                            return Err(ParseError::DateExtendedYear);
                        }
                    } else {
                        return Err(ParseError::DateExtendedYear);
                    }
                    cnt += 1;
                }
                return Ok((
                    Some(SegmentIndex {
                        start: start_index,
                        end: start_index + 6,
                    }),
                    mut_inner_remains,
                ));
            } else {
                return Err(ParseError::DateExtendedYear);
            }
        } else {
            return Err(ParseError::DateExtendedYear);
        }
    }

    fn parse_date_four_digit_year(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 4 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                } else {
                    return Err(ParseError::DateFourDigitYear);
                }
            } else {
                return Err(ParseError::DateFourDigitYear);
            }
            cnt += 1;
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 3,
            }),
            mut_inner_remains,
        ));
    }

    fn parse_date_year(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if let Some((first, _)) = s.split_first() {
            if first == &b'+' || first == &b'-' {
                return Self::parse_date_extended_year(s, start_index);
            }
            return Self::parse_date_four_digit_year(s, start_index);
        }
        return Err(ParseError::DateYear);
    }

    fn parse_date_month(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        let mut month: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 2 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    month = month * 10 + *inner_first - b'0';
                } else {
                    return Err(ParseError::DateMonth);
                }
            } else {
                return Err(ParseError::DateMonth);
            }
            cnt += 1;
        }
        if !(1..=12).contains(&month) {
            return Err(ParseError::DateMonth);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 1,
            }),
            mut_inner_remains,
        ));
    }

    fn parse_date_day(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        let mut day: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 2 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    day = day * 10 + *inner_first - b'0';
                } else {
                    return Err(ParseError::DateDay);
                }
            } else {
                return Err(ParseError::DateDay);
            }
            cnt += 1;
        }
        if !(1..=31).contains(&day) {
            return Err(ParseError::DateDay);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 1,
            }),
            mut_inner_remains,
        ));
    }

    pub fn parse(s: &[u8]) -> Result<ParsedDate, ParseError> {
        let mut result = ParsedDate {
            year: None,
            month: None,
            day: None,
        };
        if s.is_empty() {
            return Ok(result);
        }
        let mut remains = s;
        let mut start_index = 0;
        (result.year, remains) = match Self::parse_date_year(remains, start_index) {
            Ok((year, remains)) => {
                if let Some(segment_index) = year {
                    start_index = segment_index.end + 1;
                }
                (year, remains)
            }
            Err(e) => return Err(e),
        };
        let had_first_date_separator = {
            if let Some((first, inner_remains)) = remains.split_first() {
                if first == &b'-' {
                    remains = inner_remains;
                    start_index += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        (result.month, remains) = match Self::parse_date_month(remains, start_index) {
            Ok((month, remains)) => {
                if let Some(segment_index) = month {
                    start_index = segment_index.end + 1;
                }
                (month, remains)
            }
            Err(e) => return Err(e),
        };

        let had_second_date_separator = {
            if let Some((first, inner_remains)) = remains.split_first() {
                if first == &b'-' {
                    remains = inner_remains;
                    start_index += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if had_first_date_separator != had_second_date_separator {
            return Err(ParseError::DateSeparator);
        }
        (result.day, remains) = match Self::parse_date_day(remains, start_index) {
            Ok((day, remains)) => (day, remains),
            Err(e) => return Err(e),
        };
        if !remains.is_empty() {
            return Err(ParseError::DateUnexpectedEnd);
        }
        return Ok(result);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TimeParser {}

impl TimeParser {
    fn parse_time_hour(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if s.is_empty() {
            return Ok((None, s));
        }
        let mut hour: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 2 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    hour = hour * 10 + *inner_first - b'0';
                } else {
                    return Err(ParseError::TimeHour);
                }
            } else {
                return Err(ParseError::TimeHour);
            }
            cnt += 1;
        }
        if !(0..=23).contains(&hour) {
            return Err(ParseError::TimeHour);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 1,
            }),
            mut_inner_remains,
        ));
    }

    fn parse_time_minute(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if s.is_empty() {
            return Ok((None, s));
        }
        let mut minute: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 2 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    minute = minute * 10 + *inner_first - b'0';
                } else {
                    return Err(ParseError::TimeMinute);
                }
            } else {
                return Err(ParseError::TimeMinute);
            }
            cnt += 1;
        }
        if !(0..=59).contains(&minute) {
            return Err(ParseError::TimeMinute);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 1,
            }),
            mut_inner_remains,
        ));
    }

    fn parse_time_second(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if s.is_empty() {
            return Ok((None, s));
        }
        let mut second: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 2 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    second = second * 10 + *inner_first - b'0';
                } else {
                    return Err(ParseError::TimeSecond);
                }
            } else {
                return Err(ParseError::TimeSecond);
            }
            cnt += 1;
        }
        if !(1..=60).contains(&second) {
            return Err(ParseError::TimeSecond);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + 1,
            }),
            mut_inner_remains,
        ));
    }

    fn parse_fraction_part(
        s: &[u8],
        start_index: usize,
    ) -> Result<(Option<SegmentIndex>, &[u8]), ParseError> {
        if s.is_empty() {
            return Ok((None, s));
        }
        let mut fraction: i32 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = s;
        while cnt < 9 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    mut_inner_remains = inner_remains;
                    fraction = fraction * 10 + *inner_first as i32 - b'0' as i32;
                } else {
                    return Err(ParseError::FractionPart);
                }
            } else {
                break;
            }
            cnt += 1;
        }
        if cnt == 0 {
            return Err(ParseError::FractionPart);
        }
        return Ok((
            Some(SegmentIndex {
                start: start_index,
                end: start_index + cnt - 1,
            }),
            mut_inner_remains,
        ));
    }

    pub fn parse(s: &[u8], start_index: usize) -> Result<ParsedTime, ParseError> {
        let mut result = ParsedTime {
            hour: None,
            minute: None,
            second: None,
            nano_second: None,
        };
        if s.is_empty() {
            return Ok(result);
        }
        let mut remains = s;
        let mut start_index = start_index;
        (result.hour, remains) = match Self::parse_time_hour(remains, start_index) {
            Ok((hour, remains)) => {
                if let Some(segment_index) = hour {
                    start_index = segment_index.end + 1;
                }
                (hour, remains)
            }
            Err(e) => return Err(e),
        };
        let had_first_time_separator = {
            if let Some((first, inner_remains)) = remains.split_first() {
                if first == &b':' {
                    remains = inner_remains;
                    start_index += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        (result.minute, remains) = match Self::parse_time_minute(remains, start_index) {
            Ok((minute, remains)) => {
                if let Some(segment_index) = minute {
                    start_index = segment_index.end + 1;
                }
                (minute, remains)
            }
            Err(e) => return Err(e),
        };
        let had_second_time_separator = {
            if let Some((first, inner_remains)) = remains.split_first() {
                if first == &b':' {
                    remains = inner_remains;
                    start_index += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if had_second_time_separator && remains.is_empty()
            || had_first_time_separator != had_second_time_separator && !remains.is_empty()
        {
            return Err(ParseError::TimeSeparator);
        }

        (result.second, remains) = match Self::parse_time_second(remains, start_index) {
            Ok((second, remains)) => {
                if let Some(segment_index) = second {
                    start_index = segment_index.end + 1;
                }
                (second, remains)
            }
            Err(e) => return Err(e),
        };
        let had_decimal_separator = {
            if let Some((first, inner_remains)) = remains.split_first() {
                if first == &DecimalSeparator::Dot.value()
                    || first == &DecimalSeparator::Comma.value()
                {
                    remains = inner_remains;
                    start_index += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if !had_decimal_separator && !remains.is_empty() {
            return Err(ParseError::DecimalSeparator);
        }
        if had_decimal_separator {
            (result.nano_second, remains) = match Self::parse_fraction_part(remains, start_index) {
                Ok((nano_second, remains)) => (nano_second, remains),
                Err(e) => return Err(e),
            };
        }

        if !remains.is_empty() {
            return Err(ParseError::TimeUnexpectedEnd);
        }
        return Ok(result);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DateTimeParser {}

impl DateTimeParser {
    fn is_date_time_separator(u: &u8) -> bool {
        // Whether current position has data separator.
        if u == &DateTimeSeparator::CapitalT.value()
            || u == &DateTimeSeparator::LowCaseT.value()
            || u == &DateTimeSeparator::Space.value()
        {
            return true;
        }
        return false;
    }

    pub fn parse(s: &[u8]) -> Result<ParsedDateTime, ParseError> {
        let mut result = ParsedDateTime {
            parsed_date: None,
            parsed_time: None,
        };
        let mut time_slice = s;
        let mut cnt = 0;
        while cnt < s.len() {
            if let Some((first, remains)) = time_slice.split_first() {
                time_slice = remains;
                if Self::is_date_time_separator(first) {
                    break;
                }
            }
            cnt += 1;
        }
        result.parsed_date = match DateParser::parse(&s[..cnt]) {
            Ok(date) => Some(date),
            Err(e) => return Err(e),
        };
        if cnt < s.len() {
            result.parsed_time = match TimeParser::parse(time_slice, cnt + 1) {
                Ok(time) => Some(time),
                Err(e) => return Err(e),
            };
        }
        return Ok(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_datetime() {
        let dt = "2022-11-08".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: None
            })
        );

        let dt = "20220605".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 4, end: 5 }),
                    day: Some(SegmentIndex { start: 6, end: 7 }),
                }),
                parsed_time: None
            })
        );

        let dt = "2022-06-05T04".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: Some(ParsedTime {
                    hour: Some(SegmentIndex { start: 11, end: 12 }),
                    minute: None,
                    second: None,
                    nano_second: None
                })
            })
        );

        let dt = "2022-06-05t04:34".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: Some(ParsedTime {
                    hour: Some(SegmentIndex { start: 11, end: 12 }),
                    minute: Some(SegmentIndex { start: 14, end: 15 }),
                    second: None,
                    nano_second: None
                })
            })
        );

        let dt = "2022-06-05 04:34:22".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: Some(ParsedTime {
                    hour: Some(SegmentIndex { start: 11, end: 12 }),
                    minute: Some(SegmentIndex { start: 14, end: 15 }),
                    second: Some(SegmentIndex { start: 17, end: 18 }),
                    nano_second: None
                })
            })
        );

        let dt = "2022-06-05 04:34:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: Some(ParsedTime {
                    hour: Some(SegmentIndex { start: 11, end: 12 }),
                    minute: Some(SegmentIndex { start: 14, end: 15 }),
                    second: Some(SegmentIndex { start: 17, end: 18 }),
                    nano_second: Some(SegmentIndex { start: 20, end: 22 }),
                })
            })
        );

        let dt = "2022-06-05 043422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                parsed_date: Some(ParsedDate {
                    year: Some(SegmentIndex { start: 0, end: 3 }),
                    month: Some(SegmentIndex { start: 5, end: 6 }),
                    day: Some(SegmentIndex { start: 8, end: 9 }),
                }),
                parsed_time: Some(ParsedTime {
                    hour: Some(SegmentIndex { start: 11, end: 12 }),
                    minute: Some(SegmentIndex { start: 13, end: 14 }),
                    second: Some(SegmentIndex { start: 15, end: 16 }),
                    nano_second: Some(SegmentIndex { start: 18, end: 20 }),
                })
            })
        );
    }

    #[test]
    fn test_bad_date() {
        let dt = "-2022-06-05".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateExtendedYear));

        let dt = "!2022-06-05".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateFourDigitYear));

        let dt = "20-06-05".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateFourDigitYear));

        let dt = "2022-0605".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateSeparator));

        let dt = "202206-05".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateSeparator));

        let dt = "2022-06-05e".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DateUnexpectedEnd));
    }

    #[test]
    fn test_bad_time_spec_separator() {
        let dt = "2022-06-05  043422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeHour));

        let dt = "2022-06-05 04:3422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeSeparator));

        let dt = "2022-06-05 0434:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeSeparator));

        let dt = "2022-06-05 03422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeSecond));

        let dt = "2022-06-05 3:42:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeHour));

        let dt = "2022-06-05 03:42:22;000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::DecimalSeparator));
    }
}
