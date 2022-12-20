// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An enum for Parser errors.
#[non_exhaustive]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ParseError {
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
    TimeZone,
    TimeZonePart,
    TimeNumOffset,
    TimeZoneName,
}

// An enum for date time separator.
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

// An enum for decimal separator.
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

/// [`ParsedDateTime`] is the parsed result from the DateTimeParser.
///
/// The structure contains all the information needed for IXDTF. Now it only supports date and time
/// fields.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDateTime {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub nano_second: Option<i32>,
    pub time_zone: Option<Vec<u8>>,
}

/// [`DateTimeParser`] is the parser to parse IXDTF bytes.
///
/// # Examples
/// ```
/// use ixdtf::parser::DateTimeParser;
///
/// let dt = "2022-11-08".as_bytes();
/// let parsed = DateTimeParser::new(dt).parse();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTimeParser<'a> {
    bytes: &'a [u8],
}

impl<'a> DateTimeParser<'a> {
    /// Create a new instance of [`DateTimeParser`].
    pub fn new(bytes: &'a [u8]) -> DateTimeParser {
        return DateTimeParser { bytes };
    }

    fn is_lcalpha(letter: u8) -> bool {
        return (b'a'..=b'z').contains(&letter);
    }

    fn is_ucalpha(letter: u8) -> bool {
        return (b'A'..=b'Z').contains(&letter);
    }

    fn is_alpha(letter: u8) -> bool {
        return Self::is_lcalpha(letter) || Self::is_ucalpha(letter);
    }

    fn is_digit(letter: u8) -> bool {
        return (b'0'..=b'9').contains(&letter);
    }

    fn is_critical_flag(letter: u8) -> bool {
        return letter == b'!';
    }

    fn is_time_zone_initial(letter: u8) -> bool {
        return Self::is_alpha(letter) || letter == b'.' || letter == b'_';
    }

    fn is_time_zone_char(letter: u8) -> bool {
        return Self::is_time_zone_initial(letter)
            || Self::is_digit(letter)
            || letter == b'-'
            || letter == b'+';
    }

    fn parse_time_zone_part(&mut self) -> Result<Option<Vec<u8>>, ParseError> {
        let mut time_zone_part = Vec::new();
        if let Some((first, remains)) = self.bytes.split_first() {
            if Self::is_time_zone_initial(*first) {
                time_zone_part.push(*first);
                let mut cnt = 0;
                let mut mut_inner_remains = remains;
                while cnt < 13 {
                    if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                        if Self::is_time_zone_char(*inner_first) {
                            time_zone_part.push(*inner_first);
                            mut_inner_remains = inner_remains;
                            cnt += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if (time_zone_part.len() == 1 && time_zone_part.get(0) == Some(&b'.'))
                    || (time_zone_part.len() == 2
                        && time_zone_part.get(0) == Some(&b'.')
                        && time_zone_part.get(1) == Some(&b'.'))
                {
                    return Err(ParseError::TimeZonePart);
                }
                self.bytes = mut_inner_remains;
                return Ok(Some(time_zone_part));
            } else {
                return Err(ParseError::TimeZonePart);
            }
        } else {
            return Err(ParseError::TimeZonePart);
        }
    }

    fn parse_time_numoffset(&mut self) -> Result<Option<Vec<u8>>, ParseError> {
        let mut time_numoffset = Vec::new();
        if let Some((first, remains)) = self.bytes.split_first() {
            if first == &b'+' || first == &b'-' {
                time_numoffset.push(*first);
                let mut hour: u8 = 0;
                let mut minute: u8 = 0;
                let mut cnt = 0;
                let mut mut_inner_remains = remains;
                while cnt < 2 {
                    if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                        if inner_first >= &b'0' && inner_first <= &b'9' {
                            hour = hour * 10 + *inner_first - b'0';
                            time_numoffset.push(*inner_first);
                            mut_inner_remains = inner_remains;
                        } else {
                            return Err(ParseError::TimeNumOffset);
                        }
                    } else {
                        return Err(ParseError::TimeNumOffset);
                    }
                    cnt += 1;
                }
                if !(0..=23).contains(&hour) {
                    return Err(ParseError::TimeNumOffset);
                }
                if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                    if inner_first >= &b':' {
                        time_numoffset.push(*inner_first);
                        mut_inner_remains = inner_remains;
                    } else {
                        return Err(ParseError::TimeNumOffset);
                    }
                } else {
                    return Err(ParseError::TimeNumOffset);
                }
                cnt = 0;
                while cnt < 2 {
                    if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                        if inner_first >= &b'0' && inner_first <= &b'9' {
                            minute = minute * 10 + *inner_first - b'0';
                            time_numoffset.push(*inner_first);
                            mut_inner_remains = inner_remains;
                        } else {
                            return Err(ParseError::TimeNumOffset);
                        }
                    } else {
                        return Err(ParseError::TimeNumOffset);
                    }
                    cnt += 1;
                }
                if !(0..=59).contains(&minute) {
                    return Err(ParseError::TimeNumOffset);
                }
                self.bytes = mut_inner_remains;
                return Ok(Some(time_numoffset));
            } else {
                return Err(ParseError::TimeNumOffset);
            }
        } else {
            return Ok(None);
        }
    }

    fn parse_time_zone_name(&mut self) -> Result<Option<Vec<u8>>, ParseError> {
        let mut time_zone_name = Vec::new();
        let time_zone_part_result = self.parse_time_zone_part();
        if let Ok(Some(time_zone_part)) = time_zone_part_result {
            time_zone_name.append(&mut time_zone_part.to_vec());
        } else {
            return Err(ParseError::TimeZoneName);
        }
        loop {
            if let Some((first, remains)) = self.bytes.split_first() {
                if first == &b'/' {
                    time_zone_name.push(*first);
                    self.bytes = remains;
                    let time_zone_part_next_result = self.parse_time_zone_part();
                    if let Ok(Some(time_zone_part)) = time_zone_part_next_result {
                        time_zone_name.append(&mut time_zone_part.to_vec());
                    } else {
                        return Err(ParseError::TimeZoneName);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return Ok(Some(time_zone_name));
    }

    fn parse_time_zone(&mut self) -> Result<Option<Vec<u8>>, ParseError> {
        if let Some((first, remains)) = self.bytes.split_first() {
            if first == &b'[' {
                let mut mut_inner_remains = remains;
                if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                    if inner_first == &b'!' {
                        mut_inner_remains = remains
                    }
                }
                self.bytes = mut_inner_remains;
                let mut time_zone_result;
                if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                    if inner_first == &b'+' || inner_first == &b'-' {
                        time_zone_result = self.parse_time_numoffset();
                    } else {
                        time_zone_result = self.parse_time_zone_name();
                    }
                } else {
                    return Err(ParseError::TimeZone);
                }
                if time_zone_result.is_err() {
                    return time_zone_result;
                }
                if let Some((inner_first, inner_remains)) = self.bytes.split_first() {
                    if inner_first == &b']' {
                        self.bytes = inner_remains;
                    } else {
                        return Err(ParseError::TimeZone);
                    }
                } else {
                    return Err(ParseError::TimeZone);
                }
                return time_zone_result;
            } else {
                return Err(ParseError::TimeZone);
            }
        } else {
            return Ok(None);
        }
    }

    fn parse_date_extended_year(&mut self) -> Result<Option<i32>, ParseError> {
        if let Some((first, remains)) = self.bytes.split_first() {
            if first == &b'+' || first == &b'-' {
                // TODO: value assigned to `sign` is never read
                #[allow(unused_assignments)]
                let mut sign: i32 = 0;
                if first == &b'+' {
                    sign = 1;
                } else {
                    sign = -1;
                }
                let mut cnt = 0;
                let mut year: i32 = 0;
                let mut mut_inner_remains = remains;
                while cnt < 6 {
                    if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                        if (&b'0'..=&b'9').contains(&inner_first) {
                            year = year * 10 + *inner_first as i32 - b'0' as i32;
                            mut_inner_remains = inner_remains;
                        } else {
                            return Err(ParseError::DateExtendedYear);
                        }
                    } else {
                        return Err(ParseError::DateExtendedYear);
                    }
                    cnt += 1;
                }
                self.bytes = mut_inner_remains;
                return Ok(Some(year * sign));
            } else {
                return Err(ParseError::DateExtendedYear);
            }
        } else {
            return Err(ParseError::DateExtendedYear);
        }
    }

    fn parse_date_four_digit_year(&mut self) -> Result<Option<i32>, ParseError> {
        let mut cnt = 0;
        let mut year: i32 = 0;
        let mut mut_inner_remains = self.bytes;
        while cnt < 4 {
            if let Some((inner_first, inner_remains)) = mut_inner_remains.split_first() {
                if (&b'0'..=&b'9').contains(&inner_first) {
                    year = year * 10 + *inner_first as i32 - b'0' as i32;
                    mut_inner_remains = inner_remains;
                } else {
                    return Err(ParseError::DateFourDigitYear);
                }
            } else {
                return Err(ParseError::DateFourDigitYear);
            }
            cnt += 1;
        }
        self.bytes = mut_inner_remains;
        Ok(Some(year))
    }

    fn parse_date_year(&mut self) -> Result<Option<i32>, ParseError> {
        if let Some((first, _)) = self.bytes.split_first() {
            if first == &b'+' || first == &b'-' {
                return self.parse_date_extended_year();
            }
            return self.parse_date_four_digit_year();
        }
        Err(ParseError::DateYear)
    }

    fn parse_date_month(&mut self) -> Result<Option<u8>, ParseError> {
        let mut month: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(month))
    }

    fn parse_date_day(&mut self) -> Result<Option<u8>, ParseError> {
        let mut day: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(day))
    }

    fn parse_time_hour(&mut self) -> Result<Option<u8>, ParseError> {
        let mut hour: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(hour))
    }

    fn parse_time_minute(&mut self) -> Result<Option<u8>, ParseError> {
        if self.bytes.is_empty() {
            return Ok(None);
        }
        let mut minute: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(minute))
    }

    fn parse_time_second(&mut self) -> Result<Option<u8>, ParseError> {
        if self.bytes.is_empty() {
            return Ok(None);
        }
        let mut second: u8 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(second))
    }

    fn parse_fraction_part(&mut self) -> Result<Option<i32>, ParseError> {
        let mut fraction: i32 = 0;
        let mut cnt = 0;
        let mut mut_inner_remains = self.bytes;
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
        self.bytes = mut_inner_remains;
        Ok(Some(fraction))
    }

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

    /// Parse the IXDTF bytes to human readable results, stored in [`ParsedDateTime`].
    pub fn parse(&mut self) -> Result<ParsedDateTime, ParseError> {
        let mut result = ParsedDateTime {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            nano_second: None,
            time_zone: None,
        };
        if self.bytes.is_empty() {
            return Ok(result);
        }
        result.year = match self.parse_date_year() {
            Ok(year) => year,
            Err(e) => return Err(e),
        };
        let had_first_date_separator = {
            if let Some((first, inner_remains)) = self.bytes.split_first() {
                if first == &b'-' {
                    self.bytes = inner_remains;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        result.month = match self.parse_date_month() {
            Ok(month) => month,
            Err(e) => return Err(e),
        };

        let had_second_date_separator = {
            if let Some((first, inner_remains)) = self.bytes.split_first() {
                if first == &b'-' {
                    self.bytes = inner_remains;
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
        result.day = match self.parse_date_day() {
            Ok(day) => day,
            Err(e) => return Err(e),
        };
        if let Some((first, remains)) = self.bytes.split_first() {
            if Self::is_date_time_separator(first) {
                self.bytes = remains;
                result.hour = match self.parse_time_hour() {
                    Ok(hour) => hour,
                    Err(e) => return Err(e),
                };
                let had_first_time_separator = {
                    if let Some((first, inner_remains)) = self.bytes.split_first() {
                        if first == &b':' {
                            self.bytes = inner_remains;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if had_first_time_separator && self.bytes.is_empty() {
                    return Err(ParseError::TimeSeparator);
                }
                result.minute = match self.parse_time_minute() {
                    Ok(minute) => minute,
                    Err(e) => return Err(e),
                };
                let had_second_time_separator = {
                    if let Some((first, inner_remains)) = self.bytes.split_first() {
                        if first == &b':' {
                            self.bytes = inner_remains;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if had_second_time_separator && self.bytes.is_empty()
                    || had_first_time_separator != had_second_time_separator
                        && !(had_first_time_separator && self.bytes.is_empty())
                {
                    return Err(ParseError::TimeSeparator);
                }
                result.second = match self.parse_time_second() {
                    Ok(second) => second,
                    Err(e) => return Err(e),
                };
                let had_decimal_separator = {
                    if let Some((first, inner_remains)) = self.bytes.split_first() {
                        if first == &DecimalSeparator::Dot.value()
                            || first == &DecimalSeparator::Comma.value()
                        {
                            self.bytes = inner_remains;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if had_decimal_separator {
                    result.nano_second = match self.parse_fraction_part() {
                        Ok(nano_second) => nano_second,
                        Err(e) => return Err(e),
                    };
                }
            }
        }
        result.time_zone = match self.parse_time_zone() {
            Ok(time_zone) => time_zone,
            Err(e) => return Err(e),
        };
        if !self.bytes.is_empty() {
            return Err(ParseError::DateUnexpectedEnd);
        }
        return Ok(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_datetime() {
        let dt = "2022-11-08[+05:12]".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(11),
                day: Some(8),
                hour: None,
                minute: None,
                second: None,
                nano_second: None,
                time_zone: Some(vec![43, 48, 53, 58, 49, 50]),
            })
        );

        let dt = "2022-11-08[America/Chicago]".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(11),
                day: Some(8),
                hour: None,
                minute: None,
                second: None,
                nano_second: None,
                time_zone: Some(vec![
                    65, 109, 101, 114, 105, 99, 97, 47, 67, 104, 105, 99, 97, 103, 111
                ]),
            })
        );

        let dt = "2022-11-08".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(11),
                day: Some(8),
                hour: None,
                minute: None,
                second: None,
                nano_second: None,
                time_zone: None,
            })
        );

        let dt = "20220605".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: None,
                minute: None,
                second: None,
                nano_second: None,
                time_zone: None,
            })
        );

        let dt = "2022-06-05T04".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: None,
                second: None,
                nano_second: None,
                time_zone: None,
            })
        );

        let dt = "2022-06-05t04:34".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(34),
                second: None,
                nano_second: None,
                time_zone: None,
            })
        );

        let dt = "2022-06-05 04:34:22".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(34),
                second: Some(22),
                nano_second: None,
                time_zone: None,
            })
        );

        let dt = "2022-06-05 04:34:22.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(34),
                second: Some(22),
                nano_second: Some(0),
                time_zone: None,
            })
        );

        let dt = "2022-06-05 043422.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(34),
                second: Some(22),
                nano_second: Some(0),
                time_zone: None,
            })
        );
    }

    #[test]
    fn test_bad_date() {
        let dt = "-2022-06-05".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::DateExtendedYear));

        let dt = "!2022-06-05".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::DateFourDigitYear));

        let dt = "20-06-05".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::DateFourDigitYear));

        let dt = "2022-0605".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::DateSeparator));

        let dt = "202206-05".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::DateSeparator));

        let dt = "2022-06-05e".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeZone));
    }

    #[test]
    fn test_bad_time_spec_separator() {
        let dt = "2022-06-05  043422.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeHour));

        let dt = "2022-06-05 04:3422.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeSeparator));

        let dt = "2022-06-05 0434:22.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeSeparator));

        let dt = "2022-06-05 03422.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeSecond));

        let dt = "2022-06-05 3:42:22.000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeHour));

        let dt = "2022-06-05 03:42:22;000".as_bytes();
        let parsed = DateTimeParser::new(dt).parse();
        assert_eq!(parsed, Err(ParseError::TimeZone));
    }
}
