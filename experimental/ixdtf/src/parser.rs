// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;

#[derive(PartialEq, Clone, Copy, Debug)]
enum ParseError {
    DateExtendedYear,
    DateFourDigitYear,
    DateMonth,
    DateDay,
    TimeHour,
    TimeMinute,
    TimeSecond,
    FractionPart,
    DateSeparator,
    TimeSeparator,
    TimeZone,
    TimeZonePart,
    TimeNumOffset,
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

#[derive(Debug, Clone, PartialEq)]
struct ParsedDateTime {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub nano_second: Option<i32>,
    pub time_zone: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DateTimeParser {}

impl DateTimeParser {
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

    fn parse_time_zone_part<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<Vec<u8>>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut time_zone_part = Vec::new();
        if let Some(first_letter) = iter.peek() {
            match Self::is_time_zone_initial(**first_letter) {
                true => {
                    time_zone_part.push(**first_letter);
                }
                false => return Err(ParseError::TimeZonePart),
            }
        }
        iter.next();
        let mut cnt = 0;
        while cnt <= 13 {
            if let Some(time_zone_char) = iter.peek() {
                if Self::is_time_zone_char(**time_zone_char) {
                    time_zone_part.push(**time_zone_char);
                } else {
                    break;
                }
            } else {
                break;
            }
            iter.next();
            cnt += 1;
        }
        if (time_zone_part.len() == 1 && time_zone_part.get(0) == Some(&b'.'))
            || (time_zone_part.len() == 2
                && time_zone_part.get(0) == Some(&b'.')
                && time_zone_part.get(1) == Some(&b'.'))
        {
            return Err(ParseError::TimeZonePart);
        }
        return Ok(Some(time_zone_part));
    }

    fn parse_time_num_offset<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<Vec<u8>>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut time_num_offset = Vec::new();
        if let Some(first_letter) = iter.peek() {
            if first_letter == &&b'+' || first_letter == &&b'-' {
                time_num_offset.push(**first_letter);
            } else {
                return Err(ParseError::TimeNumOffset);
            }
        } else {
            return Ok(None);
        }
        iter.next();
        let mut hour: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(h) = iter.peek() {
                if **h >= b'0' && **h <= b'9' {
                    hour = hour * 10 + **h - b'0';
                    time_num_offset.push(**h);
                } else {
                    return Err(ParseError::TimeNumOffset);
                }
            } else {
                return Err(ParseError::TimeNumOffset);
            }
            iter.next();
            cnt += 1;
        }
        if !(0..=23).contains(&hour) {
            return Err(ParseError::TimeNumOffset);
        }

        if iter.peek() == Some(&&b':') {
            time_num_offset.push(b':');
        } else {
            return Err(ParseError::TimeNumOffset);
        }
        iter.next();

        let mut minute: u8 = 0;
        cnt = 0;
        while cnt < 2 {
            if let Some(m) = iter.peek() {
                if **m >= b'0' && **m <= b'9' {
                    minute = minute * 10 + **m - b'0';
                    time_num_offset.push(**m);
                } else {
                    return Err(ParseError::TimeNumOffset);
                }
            } else {
                return Err(ParseError::TimeNumOffset);
            }
            iter.next();
            cnt += 1;
        }
        if !(0..=59).contains(&minute) {
            return Err(ParseError::TimeNumOffset);
        }

        return Ok(Some(time_num_offset));
    }

    fn parse_time_zone_name<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<Vec<u8>>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut time_zone_name = Vec::new();
        let time_zone_part_result = Self::parse_time_zone_part(iter);
        if let Ok(time_zone_part_option) = time_zone_part_result {
            if let Some(time_zone_part) = time_zone_part_option {
                time_zone_name.append(&mut time_zone_part.to_vec());
            }
        } else {
            return time_zone_part_result;
        }
        loop {
            if iter.peek() == Some(&&b'/') {
                time_zone_name.push(b'/');
                iter.next();
                let time_zone_part_next_result = Self::parse_time_zone_part(iter);
                if let Ok(time_zone_part_next_option) = time_zone_part_next_result {
                    if let Some(time_zone_part_next) = time_zone_part_next_option {
                        time_zone_name.append(&mut time_zone_part_next.to_vec());
                    }
                } else {
                    return time_zone_part_next_result;
                }
            } else {
                break;
            }
        }
        return Ok(Some(time_zone_name));
    }

    fn parse_time_zone<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<Vec<u8>>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }

        if iter.peek() == Some(&&b'[') {
            iter.next();
        } else {
            return Err(ParseError::TimeZone);
        }
        if iter.peek() == Some(&&b'!') {
            iter.next();
        }

        let time_zone_result = match iter.peek() {
            Some(&&b'+') | Some(&&b'-') => Self::parse_time_num_offset(iter),
            _ => Self::parse_time_zone_name(iter),
        };

        if time_zone_result.is_err() {
            return time_zone_result;
        }
        if iter.peek() == Some(&&b']') {
            iter.next();
        } else {
            return Err(ParseError::TimeZone);
        }
        return time_zone_result;
    }

    fn parse_date_extended_year<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<i32>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut year: i32 = 0;
        // TODO: value assigned to `sign` is never read
        #[allow(unused_assignments)]
        let mut sign = 1;
        if iter.peek() == Some(&&b'+') {
            sign = 1;
        } else if iter.peek() == Some(&&b'-') {
            sign = -1
        } else {
            return Err(ParseError::DateExtendedYear);
        }
        iter.next();
        let mut cnt = 0;
        while cnt < 6 {
            if let Some(y) = iter.peek() {
                if **y >= b'0' && **y <= b'9' {
                    year = year * 10 + **y as i32 - b'0' as i32;
                } else {
                    return Err(ParseError::DateExtendedYear);
                }
            } else {
                return Err(ParseError::DateExtendedYear);
            }
            iter.next();
            cnt += 1;
        }
        return Ok(Some(year * sign));
    }

    fn parse_date_four_digit_year<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<i32>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut year: i32 = 0;
        let mut cnt = 0;
        while cnt < 4 {
            if let Some(y) = iter.peek() {
                if **y >= b'0' && **y <= b'9' {
                    year = year * 10 + **y as i32 - b'0' as i32;
                } else {
                    return Err(ParseError::DateFourDigitYear);
                }
            } else {
                return Err(ParseError::DateFourDigitYear);
            }
            iter.next();
            cnt += 1;
        }
        return Ok(Some(year));
    }

    fn parse_date_year<'a, I>(iter: &mut core::iter::Peekable<I>) -> Result<Option<i32>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == Some(&&b'+') || iter.peek() == Some(&&b'-') {
            return Self::parse_date_extended_year(iter);
        }
        return Self::parse_date_four_digit_year(iter);
    }

    fn parse_date_separator<'a, I>(
        iter: &mut core::iter::Peekable<I>,
        had_date_separator: &bool,
    ) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        // Whether current position has data separator.
        let has_date_separator = iter.peek() == Some(&&b'-');
        if *had_date_separator != has_date_separator {
            return false;
        }
        if has_date_separator {
            iter.next();
        }
        return true;
    }

    fn parse_date_month<'a, I>(iter: &mut core::iter::Peekable<I>) -> Result<Option<u8>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut month: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(m) = iter.peek() {
                if **m >= b'0' && **m <= b'9' {
                    month = month * 10 + **m - b'0';
                } else {
                    return Err(ParseError::DateMonth);
                }
            } else {
                return Err(ParseError::DateMonth);
            }
            iter.next();
            cnt += 1;
        }
        if !(1..=12).contains(&month) {
            return Err(ParseError::DateMonth);
        }
        return Ok(Some(month));
    }

    fn parse_date_day<'a, I>(iter: &mut core::iter::Peekable<I>) -> Result<Option<u8>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut day: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(d) = iter.peek() {
                if **d >= b'0' && **d <= b'9' {
                    day = day * 10 + **d - b'0';
                } else {
                    return Err(ParseError::DateDay);
                }
            } else {
                return Err(ParseError::DateDay);
            }
            iter.next();
            cnt += 1;
        }
        if !(1..=31).contains(&day) {
            return Err(ParseError::DateDay);
        }
        return Ok(Some(day));
    }

    fn parse_date_time_separator<'a, I>(iter: &mut core::iter::Peekable<I>) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        // Whether current position has data separator.
        if iter.peek() == Some(&&DateTimeSeparator::CapitalT.value())
            || iter.peek() == Some(&&DateTimeSeparator::LowCaseT.value())
            || iter.peek() == Some(&&DateTimeSeparator::Space.value())
        {
            iter.next();
            return true;
        }
        return false;
    }

    fn parse_time_hour<'a, I>(iter: &mut core::iter::Peekable<I>) -> Result<Option<u8>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut hour: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(h) = iter.peek() {
                if **h >= b'0' && **h <= b'9' {
                    hour = hour * 10 + **h - b'0';
                } else {
                    return Err(ParseError::TimeHour);
                }
            } else {
                return Err(ParseError::TimeHour);
            }
            iter.next();
            cnt += 1;
        }
        if !(0..=23).contains(&hour) {
            return Err(ParseError::TimeHour);
        }
        return Ok(Some(hour));
    }

    fn parse_time_separator<'a, I>(
        iter: &mut core::iter::Peekable<I>,
        had_time_separator: &bool,
    ) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        // Whether current position has time separator.
        let has_time_separator = iter.peek() == Some(&&b':');
        if *had_time_separator != has_time_separator {
            return false;
        }
        if has_time_separator {
            iter.next();
        }
        return true;
    }

    fn parse_time_minute<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<u8>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut minute: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(m) = iter.peek() {
                if **m >= b'0' && **m <= b'9' {
                    minute = minute * 10 + **m - b'0';
                } else {
                    return Err(ParseError::TimeMinute);
                }
            } else {
                return Err(ParseError::TimeMinute);
            }
            iter.next();
            cnt += 1;
        }
        if !(0..=59).contains(&minute) {
            return Err(ParseError::TimeMinute);
        }
        return Ok(Some(minute));
    }

    fn parse_time_second<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<u8>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut second: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(s) = iter.peek() {
                if **s >= b'0' && **s <= b'9' {
                    second = second * 10 + **s - b'0';
                } else {
                    return Err(ParseError::TimeSecond);
                }
            } else {
                return Err(ParseError::TimeSecond);
            }
            iter.next();
            cnt += 1;
        }
        if !(1..=60).contains(&second) {
            return Err(ParseError::TimeSecond);
        }
        return Ok(Some(second));
    }

    fn parse_decimal_separator<'a, I>(iter: &mut core::iter::Peekable<I>) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        // Whether current position has decimal separator.
        if iter.peek() == Some(&&DecimalSeparator::Dot.value())
            || iter.peek() == Some(&&DecimalSeparator::Comma.value())
        {
            iter.next();
            return true;
        }
        return false;
    }

    fn parse_fraction_part<'a, I>(
        iter: &mut core::iter::Peekable<I>,
    ) -> Result<Option<i32>, ParseError>
    where
        I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == None {
            return Ok(None);
        }
        let mut fraction: i32 = 0;
        let mut cnt = 0;

        while cnt < 9 {
            if let Some(f) = iter.peek() {
                if **f >= b'0' && **f <= b'9' {
                    fraction = fraction * 10 + **f as i32 - b'0' as i32;
                } else {
                    break;
                }
            } else {
                break;
            }
            iter.next();
            cnt += 1;
        }
        if cnt == 0 {
            return Err(ParseError::FractionPart);
        }
        return Ok(Some(fraction));
    }

    pub fn parse(s: &[u8]) -> Result<ParsedDateTime, ParseError> {
        let mut iter = s.iter().peekable();
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
        if iter.peek().is_none() {
            return Ok(result);
        }
        // Process DataYear.
        result.year = match Self::parse_date_year(&mut iter) {
            Ok(year) => year,
            Err(e) => return Err(e),
        };
        // Whether input string had data separator previously.
        let had_date_separator = iter.peek() == Some(&&b'-');
        if !Self::parse_date_separator(&mut iter, &had_date_separator) {
            return Err(ParseError::DateSeparator);
        }
        result.month = match Self::parse_date_month(&mut iter) {
            Ok(month) => month,
            Err(e) => return Err(e),
        };
        if !Self::parse_date_separator(&mut iter, &had_date_separator) {
            return Err(ParseError::DateSeparator);
        }
        result.day = match Self::parse_date_day(&mut iter) {
            Ok(day) => day,
            Err(e) => return Err(e),
        };
        if iter.peek() != None && Self::parse_date_time_separator(&mut iter) {
            result.hour = match Self::parse_time_hour(&mut iter) {
                Ok(hour) => hour,
                Err(e) => return Err(e),
            };
            // Whether input string had time separator previously.
            let had_time_separator = iter.peek() == Some(&&b':');
            if iter.peek() != None
                && iter.peek() != Some(&&b'[')
                && Self::parse_time_separator(&mut iter, &had_time_separator)
            {
                result.minute = match Self::parse_time_minute(&mut iter) {
                    Ok(minute) => minute,
                    Err(e) => return Err(e),
                };
                if iter.peek() != None
                    && iter.peek() != Some(&&b'[')
                    && Self::parse_time_separator(&mut iter, &had_time_separator)
                {
                    result.second = match Self::parse_time_second(&mut iter) {
                        Ok(second) => second,
                        Err(e) => return Err(e),
                    };
                    if iter.peek() != None
                        && iter.peek() != Some(&&b'[')
                        && Self::parse_decimal_separator(&mut iter)
                    {
                        result.nano_second = match Self::parse_fraction_part(&mut iter) {
                            Ok(nano_second) => nano_second,
                            Err(e) => return Err(e),
                        };
                    }
                }
            }
        }
        result.time_zone = match Self::parse_time_zone(&mut iter) {
            Ok(time_zone) => time_zone,
            Err(e) => return Err(e),
        };
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
                year: Some(2022),
                month: Some(11),
                day: Some(8),
                hour: None,
                minute: None,
                second: None,
                nano_second: None,
                time_zone: None
            })
        );

        let dt = "20220605".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
            })
        );

        let dt = "2022-06-05T04".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
            })
        );

        let dt = "2022-06-05t04:34".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
            })
        );

        let dt = "2022-06-05 04:34:22".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
            })
        );

        let dt = "2022-06-05 04:34:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
            })
        );

        let dt = "2022-06-05 043422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: None
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
    }

    #[test]
    fn test_bad_time_spec_separator() {
        let dt = "2022-06-05  043422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeHour));

        let dt = "2022-06-05 04:3422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeZone));

        let dt = "2022-06-05 0434:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeZone));

        let dt = "2022-06-05 03422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeSecond));

        let dt = "2022-06-05 3:42:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(parsed, Err(ParseError::TimeHour));
    }

    #[test]
    fn test_time_zone() {
        let dt = "2022-06-05 04:34:22.000[America/Chicago]".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: Some("America/Chicago".as_bytes().to_vec())
            })
        );

        let dt = "2022-06-05[America/Chicago]".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: Some("America/Chicago".as_bytes().to_vec())
            })
        );

        let dt = "2022-06-05 04[America/Chicago]".as_bytes();
        let parsed = DateTimeParser::parse(dt);
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
                time_zone: Some("America/Chicago".as_bytes().to_vec())
            })
        );

        let dt = "2022-06-05 04:05[America/Chicago]".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(5),
                second: None,
                nano_second: None,
                time_zone: Some("America/Chicago".as_bytes().to_vec())
            })
        );

        let dt = "2022-06-05 04:05[+08:00]".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            Ok(ParsedDateTime {
                year: Some(2022),
                month: Some(6),
                day: Some(5),
                hour: Some(4),
                minute: Some(5),
                second: None,
                nano_second: None,
                time_zone: Some("+08:00".as_bytes().to_vec())
            })
        );
    }
}
