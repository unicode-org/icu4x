#[derive(Clone, Copy)]
enum  DateTimeSeparator {
    CapitalT,
    LowCaseT,
    Space,
}

impl DateTimeSeparator {
    fn value(&self) -> String {
        match *self {
            DateTimeSeparator::CapitalT => "T".to_string(),
            DateTimeSeparator::LowCaseT => "t".to_string(),
            DateTimeSeparator::Space => " ".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
enum  DecimalSeparator {
    Dot,
    Comma,
}

impl DecimalSeparator {
    fn value(&self) -> String {
        match *self {
            DecimalSeparator::Dot => ".".to_string(),
            DecimalSeparator::Comma => ",".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DateTimeParser {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub nano_second: Option<i32>,
}

impl DateTimeParser {

    fn empty() -> Self {
        DateTimeParser {
            year : None,
            month : None,
            day : None,
            hour : None,
            minute : None,
            second : None,
            nano_second : None
        }
    }

    fn parse_date_year(idx :usize, s: &str) -> (usize, Option<i32>) {
        match &s[idx..(idx + 1)] {
            "+" | "-" => {
                match s[idx..(idx + 7)].parse::<i32>() {
                    Ok(year) => ((idx + 7), Some(year)),
                    Err(_) => (idx, None)
                }
            }
            _ => match s[idx..(idx + 4)].parse::<i32>() {
                Ok(year) => ((idx + 4), Some(year)),
                Err(_) => (idx, None)
            }
        }
    }

    fn parse_date_month(idx :usize, s: &str) -> (usize, Option<u8>) {
        match s[idx..(idx + 2)].parse::<u8>() {
            Ok(month) => {
                if month > 0 && month <= 12 {
                    ((idx + 2), Some(month))
                } else {
                    (0, None)
                }
            },
            Err(_) => (0, None)
        }
    }

    fn parse_date_day(idx :usize, s: &str) -> (usize, Option<u8>) {
        match s[idx..(idx + 2)].parse::<u8>() {
            Ok(day) => {
                if day > 0 && day <= 31 {
                    ((idx + 2), Some(day))
                } else {
                    (0, None)
                }
            },
            Err(_) => (0, None)
        }
    }

    fn parse_time_hour(idx :usize, s: &str) -> (usize, Option<u8>) {
        match s[idx..(idx + 2)].parse::<u8>() {
            Ok(hour) => {
                if hour <= 24 {
                    ((idx + 2), Some(hour))
                } else {
                    (0, None)
                }
            },
            Err(_) => (0, None)
        }
    }

    fn parse_time_minute(idx :usize, s: &str) -> (usize, Option<u8>) {
        match s[idx..(idx + 2)].parse::<u8>() {
            Ok(minute) => {
                if minute < 60 {
                    ((idx + 2), Some(minute))
                } else {
                    (0, None)
                }
            },
            Err(_) => (0, None)
        }
    }

    fn parse_time_second(idx :usize, s: &str) -> (usize, Option<u8>) {
        match s[idx..(idx + 2)].parse::<u8>() {
            Ok(second) => {
                if second <= 60 {
                    ((idx + 2), Some(second))
                } else {
                    (0, None)
                }
            },
            Err(_) => (0, None)
        }
    }

    fn parse_time_fractional_part(idx :usize, s: &str) -> (usize, Option<i32>) {
        match s[idx..].parse::<i32>() {
            Ok(fraction) => {
                (s.len(), Some(fraction))
            },
            Err(_) => (0, None)
        }
    }


    pub fn parse(s: String) -> Self {
        let len = s.len();
        let mut year : Option<i32> = None;
        let mut month : Option<u8> = None;
        let mut day : Option<u8> = None;
        let mut hour : Option<u8> = None;
        let mut minute : Option<u8> = None;
        let mut second : Option<u8> = None;
        let mut nano_second : Option<i32> = None;
        let mut idx : usize = 0;

        (idx, year) = Self::parse_date_year(idx, &s);
        if (idx == 0) {
            return Self::empty()
        }
        if idx < len && &s[idx..(idx+1)] == "-" {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        }
        (idx, month) = Self::parse_date_month(idx, &s);
        if (idx == 0) {
            return Self::empty()
        }
        if idx < len && &s[idx..(idx+1)] == "-" {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        }
        (idx, day) = Self::parse_date_day(idx, &s);
        if (idx == 0) {
            return Self::empty()
        }
        if idx < len && (&s[idx..(idx + 1)] == DateTimeSeparator::CapitalT.value() || &s[idx..(idx + 1)] == DateTimeSeparator::LowCaseT.value() || &s[idx..(idx + 1)] == DateTimeSeparator::Space.value()) {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        } else {
            return Self::empty()
        }
        (idx, hour) = Self::parse_time_hour(idx, &s);
        if (idx == 0) {
            return Self::empty()
        }
        if idx < len && &s[idx..(idx + 1)] == ":" {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        }
        (idx, minute) = Self::parse_time_minute(idx, &s);
        if (idx == 0) {
            return Self::empty()
        }
        if idx < len && &s[idx..(idx + 1)] == ":" {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        }
        (idx, second) = Self::parse_time_second(idx, &s);
        if idx < len && (&s[idx..(idx + 1)] == DecimalSeparator::Dot.value() || &s[idx..(idx + 1)] == DecimalSeparator::Comma.value()) {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        } else {
            return Self::empty()
        }

        if idx < len && &s[idx..(idx + 1)] == "." {
            idx += 1;
        } else if idx == len {
            return DateTimeParser {
                year,
                month,
                day,
                hour,
                minute,
                second,
                nano_second
            }
        }
        (idx, nano_second) = Self::parse_time_fractional_part(idx, &s);
        DateTimeParser {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nano_second
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_datetime() {
        let dt = "2022-06-05".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : None,
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "20220605".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : None,
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05T04".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : None,
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05t04:34".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : None,
                nano_second : None
            },
        );

        let dt = "2022-06-05 04:34:22".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : Some(22),
                nano_second : None
            },
        );

        let dt = "2022-06-05 04:34:22.000".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
                year : Some(2022),
                month : Some(6),
                day : Some(5),
                hour : Some(4),
                minute : Some(34),
                second : Some(22),
                nano_second : Some(0)
            },
        );

        let dt = "2022-06-05 043422.000".to_string();
        let parsed = DateTimeParser::parse(dt);
        assert_eq!(
            parsed,
            DateTimeParser {
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
