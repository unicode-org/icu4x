#[derive(Clone, Copy)]
enum  DateTimeSeparator {
    CapitalT,
    LowCaseT,
    Space,
}

impl DateTimeSeparator {
    fn value(&self) -> u8 {
        match *self {
            DateTimeSeparator::CapitalT => 84,
            DateTimeSeparator::LowCaseT => 116,
            DateTimeSeparator::Space => 32,
        }
    }
}

#[derive(Clone, Copy)]
enum  DecimalSeparator {
    Dot,
    Comma,
}

impl DecimalSeparator {
    fn value(&self) -> u8 {
        match *self {
            DecimalSeparator::Dot => 46,
            DecimalSeparator::Comma => 59,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParsedDateTime {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub nano_second: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DateTimeParser{}

impl DateTimeParser {

    fn parse_date_extented_year<'a, I>(iter:  &mut core::iter::Peekable<I>) -> Option<i32>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut year: i32 = 0;
        let mut sign = 1;
        if iter.peek() == Some(&&43) {
            // Start with "+"
            sign = 1;
        } else if iter.peek() == Some(&&45) {
            // Start with "-"
            sign = -1
        } else {
            return None;
        }
        iter.next();
        let mut cnt = 0;
        while cnt < 6 {
            if let Some(y) = iter.peek() {
                if **y >= 48 && **y <= 57 {
                    year = year * 10 + **y as i32 - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        return Some(year * sign);
    }

    fn parse_date_four_digit_year<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<i32>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut year: i32 = 0;
        let mut cnt = 0;
        while cnt < 4 {
            if let Some(y) = iter.peek() {
                if **y >= 48 && **y <= 57 {
                    year = year * 10 + **y as i32 - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        return Some(year);
    }

    fn parse_date_year<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<i32>
        where
            I: Iterator<Item = &'a u8>,
    {
        if iter.peek() == Some(&&43) || iter.peek() == Some(&&45) {
            return Self::parse_date_extented_year(iter);
        }
        return Self::parse_date_four_digit_year(iter);
    }

    fn parse_date_separator<'a, I>(iter: &mut core::iter::Peekable<I>, had_date_separator: &bool) -> bool
        where
            I: Iterator<Item = &'a u8>,
    {
        // Whether current position has data separator.
        let has_date_separator = iter.peek() == Some(&&45);
        if *had_date_separator != has_date_separator {
            return false;
        }
        if has_date_separator {
            iter.next();
        }
        return true;
    }

    fn parse_date_month<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<u8>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut month: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(m) = iter.peek() {
                if **m >= 48 && **m <= 57 {
                    month = month * 10 + **m - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        if month < 1 || month > 12 {
            return None;
        }
        return Some(month);
    }

    fn parse_date_day<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<u8>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut day: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(d) = iter.peek() {
                if **d >= 48 && **d <= 57 {
                    day = day * 10 + **d - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        if day < 1 || day > 31 {
            return None;
        }
        return Some(day);
    }

    fn parse_date_time_separator<'a, I>(iter: &mut core::iter::Peekable<I>) -> bool
        where
            I: Iterator<Item = &'a u8>,
    {
        // Whether current position has data separator.
        if iter.peek() == Some(&&DateTimeSeparator::CapitalT.value()) || iter.peek() == Some(&&DateTimeSeparator::LowCaseT.value()) || iter.peek() == Some(&&DateTimeSeparator::Space.value()) {
            iter.next();
            return true;
        }
        return false;
    }

    fn parse_time_hour<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<u8>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut hour: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(h) = iter.peek() {
                if **h >= 48 && **h <= 57 {
                    hour = hour * 10 + **h - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        if hour < 1 || hour > 23 {
            return None;
        }
        return Some(hour);
    }

    fn parse_time_separator<'a, I>(iter: &mut core::iter::Peekable<I>, had_time_separator: &bool) -> bool
        where
            I: Iterator<Item = &'a u8>,
    {
        // Whether current position has time separator.
        let has_time_separator = iter.peek() == Some(&&58);
        if *had_time_separator != has_time_separator {
            return false;
        }
        if has_time_separator {
            iter.next();
        }
        return true;
    }

    fn parse_time_minute<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<u8>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut minute: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(m) = iter.peek() {
                if **m >= 48 && **m <= 57 {
                    minute = minute * 10 + **m - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        if minute < 1 || minute > 59 {
            return None;
        }
        return Some(minute);
    }

    fn parse_time_second<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<u8>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut second: u8 = 0;
        let mut cnt = 0;
        while cnt < 2 {
            if let Some(s) = iter.peek() {
                if **s >= 48 && **s <= 57 {
                    second = second * 10 + **s - 48;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            iter.next();
            cnt += 1;
        }
        if second < 1 || second > 60 {
            return None;
        }
        return Some(second);
    }

    fn parse_decimal_separator<'a, I>(iter: &mut core::iter::Peekable<I>) -> bool
        where
            I: Iterator<Item = &'a u8>,
    {
        // Whether current position has decimal separator.
        if iter.peek() == Some(&&DecimalSeparator::Dot.value()) || iter.peek() == Some(&&DecimalSeparator::Comma.value()) {
            iter.next();
            return true;
        }
        return false;
    }

    fn parse_fraction_part<'a, I>(iter: &mut core::iter::Peekable<I>) -> Option<i32>
        where
            I: Iterator<Item = &'a u8>,
    {
        let mut fraction: i32 = 0;
        let mut cnt = 0;

        while cnt < 9 {
            if let Some(f) = iter.peek() {
                if **f >= 48 && **f <= 57 {
                    fraction = fraction * 10 + **f as i32 - 48;
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
            return None;
        }
        return Some(fraction);
    }

    pub fn parse(s: &[u8]) -> ParsedDateTime {
        let mut iter = s.iter().peekable();
        let mut result = ParsedDateTime {
            year : None,
            month : None,
            day : None,
            hour : None,
            minute : None,
            second : None,
            nano_second : None
        };
        if iter.peek().is_none() {
            return result;
        }
        // Process DataYear.
        if let Some(year) = Self::parse_date_year(&mut iter) {
            result.year = Some(year);
        } else {
            return result;
        }
        // Whether input string had data separator previously.
        let had_date_separator = iter.peek() == Some(&&45);
        if !Self::parse_date_separator(&mut iter, &had_date_separator) {
            return result;
        }
        if let Some(month) = Self::parse_date_month(&mut iter) {
            result.month = Some(month);
        } else {
            return result;
        }
        if !Self::parse_date_separator(&mut iter, &had_date_separator) {
            return result;
        }
        if let Some(day) = Self::parse_date_day(&mut iter) {
            result.day = Some(day);
        } else {
            return result;
        }
        if !Self::parse_date_time_separator(&mut iter) {
            return result;
        }
        if let Some(hour) = Self::parse_time_hour(&mut iter) {
            result.hour = Some(hour);
        } else {
            return result;
        }
        // Whether input string had time separator previously.
        let had_time_separator = iter.peek() == Some(&&58);
        if !Self::parse_time_separator(&mut iter, &had_time_separator) {
            return result;
        }
        if let Some(minute) = Self::parse_time_minute(&mut iter) {
            result.minute = Some(minute);
        } else {
            return result;
        }
        if !Self::parse_time_separator(&mut iter, &had_time_separator) {
            return result;
        }
        if let Some(second) = Self::parse_time_second(&mut iter) {
            result.second = Some(second);
        } else {
            return result;
        }
        if !Self::parse_decimal_separator(&mut iter) {
            return result;
        }
        if let Some(nano_second) = Self::parse_fraction_part(&mut iter) {
            result.nano_second = Some(nano_second);
        } else {
            return result;
        }
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_datetime() {
        let dt = "2022-11-08".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(11),
                day : Some(8),
                hour : None,
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "20220605".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : None,
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05T04".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05t04:34".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05 04:34:22".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : Some(22),
                nano_second : None
            },
        );

        let dt = "2022-06-05 04:34:22.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : Some(22),
                nano_second : Some(0)
            },
        );

        let dt = "2022-06-05 043422.000".as_bytes();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            ParsedDateTime {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : Some(22),
                nano_second : Some(0)
            },
        );
    }
}
