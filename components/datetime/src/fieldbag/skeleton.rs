// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use super::DateTimeFieldBag;
use crate::provider::pattern::reference::tokenizer::Token;
use crate::provider::pattern::reference::tokenizer::Uts35DateTimePatternTokenizer;

/// An error returned when parsing a UTS #35 skeleton string into a [`DateTimeFieldBag`].
///
/// Strict parsing via [`DateTimeFieldBag::try_from_skeleton`] and [`FromStr`](core::str::FromStr)
/// returns this error if the skeleton contains duplicate fields, literals, non-canonical symbols
/// or widths, or unknown fields. For lenient parsing that recovers from these conditions, use
/// [`DateTimeFieldBag::from_skeleton`].
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, displaydoc::Display)]
#[ignore_extra_doc_attributes]
pub enum DateTimeFieldBagParseError {
    /// A field category was specified more than once in the skeleton.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("yyMMy"),
    ///     Err(DateTimeFieldBagParseError::DuplicateField)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("yyMMy"), "yyMM");
    /// ```
    DuplicateField,
    /// The skeleton contained literal characters (such as punctuation, whitespace, or quoted text).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("y-MM-dd"),
    ///     Err(DateTimeFieldBagParseError::UnexpectedLiteral)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("y-MM-dd"), "yMMdd");
    /// ```
    UnexpectedLiteral,
    /// The skeleton contained a syntax error, such as an unclosed quoted literal.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("GGGGy'foo"),
    ///     Err(DateTimeFieldBagParseError::SyntaxError)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("GGGGy'foo"), "GGGGy");
    /// ```
    SyntaxError,
    /// A field symbol had a non-canonical or unsupported repetition width (such as `GGG`, `yyyy`, or `MMMMMMM`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("yyyyMMdd"),
    ///     Err(DateTimeFieldBagParseError::InvalidFieldLength)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("yyyyMMdd"), "yMMdd");
    /// ```
    InvalidFieldLength,
    /// A non-canonical field symbol was used instead of its canonical skeleton equivalent (such as `L` instead of `M`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("yLLLL"),
    ///     Err(DateTimeFieldBagParseError::NonCanonicalField)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("yLLLL"), "yMMMM");
    /// ```
    NonCanonicalField,
    /// An unknown or unsupported field symbol was encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::DateTimeFieldBagParseError;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_eq!(
    ///     DateTimeFieldBag::try_from_skeleton("yQ"),
    ///     Err(DateTimeFieldBagParseError::UnknownField)
    /// );
    /// assert_writeable_eq!(DateTimeFieldBag::from_skeleton("yQ"), "y");
    /// ```
    UnknownField,
}

impl core::error::Error for DateTimeFieldBagParseError {}

pub(crate) fn uts35_to_fieldbag(
    skeleton: &str,
) -> (DateTimeFieldBag, Option<DateTimeFieldBagParseError>) {
    use super::field::*;
    use DateTimeFieldBagParseError::*;

    macro_rules! put_field {
        ($error:expr, $field:expr, $value:expr) => {{
            if ($field).is_some() {
                ($error).get_or_insert(DuplicateField);
            } else {
                *($field) = Some($value);
            }
        }};
    }

    let mut tokenizer = Uts35DateTimePatternTokenizer(skeleton);
    let mut bag = DateTimeFieldBag::default();
    let mut error = None;
    while let Some(token) = tokenizer.step() {
        let mut token = token;
        if let Token::Symbol(ch, _) = &mut token {
            *ch = match *ch {
                'Y' | 'u' | 'U' | 'r' => {
                    error.get_or_insert(NonCanonicalField);
                    'y'
                }
                'L' => {
                    error.get_or_insert(NonCanonicalField);
                    'M'
                }
                'c' | 'e' => {
                    error.get_or_insert(NonCanonicalField);
                    'E'
                }
                'C' | 'J' => {
                    error.get_or_insert(NonCanonicalField);
                    'j'
                }
                'K' => {
                    error.get_or_insert(NonCanonicalField);
                    'h'
                }
                'k' => {
                    error.get_or_insert(NonCanonicalField);
                    'H'
                }
                'Z' | 'x' | 'X' => {
                    error.get_or_insert(NonCanonicalField);
                    'O'
                }
                'V' => {
                    error.get_or_insert(NonCanonicalField);
                    'v'
                }
                other => other,
            };
        }
        match token {
            Token::Symbol('G', 1) => {
                put_field!(&mut error, &mut bag.era, Era::Short);
            }
            Token::Symbol('G', 4) => {
                put_field!(&mut error, &mut bag.era, Era::Long);
            }
            Token::Symbol('G', 5) => {
                put_field!(&mut error, &mut bag.era, Era::Narrow);
            }
            Token::Symbol('G', _) => {
                put_field!(&mut error, &mut bag.era, Era::Short);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('y', 1) => {
                put_field!(&mut error, &mut bag.year, Year::Numeric);
            }
            Token::Symbol('y', 2) => {
                put_field!(&mut error, &mut bag.year, Year::TwoDigit);
            }
            Token::Symbol('y', _) => {
                put_field!(&mut error, &mut bag.year, Year::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('M', 1) => {
                put_field!(&mut error, &mut bag.month, Month::Numeric);
            }
            Token::Symbol('M', 2) => {
                put_field!(&mut error, &mut bag.month, Month::TwoDigit);
            }
            Token::Symbol('M', 3) => {
                put_field!(&mut error, &mut bag.month, Month::Short);
            }
            Token::Symbol('M', 4) => {
                put_field!(&mut error, &mut bag.month, Month::Long);
            }
            Token::Symbol('M', 5) => {
                put_field!(&mut error, &mut bag.month, Month::Narrow);
            }
            Token::Symbol('M', _) => {
                put_field!(&mut error, &mut bag.month, Month::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('d', 1) => {
                put_field!(&mut error, &mut bag.day, Day::Numeric);
            }
            Token::Symbol('d', 2) => {
                put_field!(&mut error, &mut bag.day, Day::TwoDigit);
            }
            Token::Symbol('d', _) => {
                put_field!(&mut error, &mut bag.day, Day::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('E', 1) => {
                put_field!(&mut error, &mut bag.weekday, Weekday::Short);
            }
            Token::Symbol('E', 4) => {
                put_field!(&mut error, &mut bag.weekday, Weekday::Long);
            }
            Token::Symbol('E', 5) => {
                put_field!(&mut error, &mut bag.weekday, Weekday::Narrow);
            }
            Token::Symbol('E', _) => {
                put_field!(&mut error, &mut bag.weekday, Weekday::Short);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('B', 1) => {
                put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleShort);
            }
            Token::Symbol('B', 4) => {
                put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleLong);
            }
            Token::Symbol('B', 5) => {
                put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleNarrow);
            }
            Token::Symbol('B', _) => {
                put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleShort);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('j', 1) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
            }
            Token::Symbol('j', 2) => {
                put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
            }
            Token::Symbol('j', _) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('h', 1) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
            }
            Token::Symbol('h', 2) => {
                put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
            }
            Token::Symbol('h', _) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('H', 1) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
            }
            Token::Symbol('H', 2) => {
                put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
            }
            Token::Symbol('H', _) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('m', 1) => {
                put_field!(&mut error, &mut bag.minute, Minute::Numeric);
            }
            Token::Symbol('m', 2) => {
                put_field!(&mut error, &mut bag.minute, Minute::TwoDigit);
            }
            Token::Symbol('m', _) => {
                put_field!(&mut error, &mut bag.minute, Minute::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('s', 1) => {
                put_field!(&mut error, &mut bag.second, Second::Numeric);
            }
            Token::Symbol('s', 2) => {
                put_field!(&mut error, &mut bag.second, Second::TwoDigit);
            }
            Token::Symbol('s', _) => {
                put_field!(&mut error, &mut bag.second, Second::Numeric);
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('S', 1) => {
                put_field!(
                    &mut error,
                    &mut bag.fractional_second_digits,
                    FractionalSecondDigits::F1
                );
            }
            Token::Symbol('S', 2) => {
                put_field!(
                    &mut error,
                    &mut bag.fractional_second_digits,
                    FractionalSecondDigits::F2
                );
            }
            Token::Symbol('S', 3) => {
                put_field!(
                    &mut error,
                    &mut bag.fractional_second_digits,
                    FractionalSecondDigits::F3
                );
            }
            Token::Symbol('S', _) => {
                put_field!(
                    &mut error,
                    &mut bag.fractional_second_digits,
                    FractionalSecondDigits::F1
                );
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('z', 1) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortSpecific
                );
            }
            Token::Symbol('z', 4) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::LongSpecific
                );
            }
            Token::Symbol('z', _) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortSpecific
                );
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('O', 1) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortOffset
                );
            }
            Token::Symbol('O', 4) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::LongOffset
                );
            }
            Token::Symbol('O', _) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortOffset
                );
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol('v', 1) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortGeneric
                );
            }
            Token::Symbol('v', 4) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::LongGeneric
                );
            }
            Token::Symbol('v', _) => {
                put_field!(
                    &mut error,
                    &mut bag.time_zone_name,
                    TimeZoneName::ShortGeneric
                );
                error.get_or_insert(InvalidFieldLength);
            }
            Token::Symbol(_, _) => {
                error.get_or_insert(UnknownField);
            }
            Token::Literal(_) => {
                error.get_or_insert(UnexpectedLiteral);
            }
            Token::UnclosedLiteral(_) => {
                error.get_or_insert(SyntaxError);
            }
        }
    }
    (bag, error)
}

pub(crate) fn fieldbag_to_uts35<W: ?Sized + fmt::Write>(
    fieldbag: &DateTimeFieldBag,
    sink: &mut W,
) -> fmt::Result {
    use super::field::*;

    let DateTimeFieldBag {
        era,
        year,
        month,
        day,
        weekday,
        day_period,
        hour_kind,
        hour,
        minute,
        second,
        fractional_second_digits,
        time_zone_name,
    } = fieldbag;
    match era {
        Some(Era::Short) => sink.write_char('G')?,
        Some(Era::Long) => sink.write_str("GGGG")?,
        Some(Era::Narrow) => sink.write_str("GGGGG")?,
        None => (),
    }
    match year {
        Some(Year::Numeric) => sink.write_char('y')?,
        Some(Year::TwoDigit) => sink.write_str("yy")?,
        None => (),
    }
    match month {
        Some(Month::Numeric) => sink.write_char('M')?,
        Some(Month::TwoDigit) => sink.write_str("MM")?,
        Some(Month::Short) => sink.write_str("MMM")?,
        Some(Month::Long) => sink.write_str("MMMM")?,
        Some(Month::Narrow) => sink.write_str("MMMMM")?,
        None => (),
    }
    match day {
        Some(Day::Numeric) => sink.write_char('d')?,
        Some(Day::TwoDigit) => sink.write_str("dd")?,
        None => (),
    }
    match weekday {
        Some(Weekday::Short) => sink.write_char('E')?,
        Some(Weekday::Long) => sink.write_str("EEEE")?,
        Some(Weekday::Narrow) => sink.write_str("EEEEE")?,
        None => (),
    }
    match day_period {
        Some(DayPeriod::FlexibleShort) => sink.write_char('B')?,
        Some(DayPeriod::FlexibleLong) => sink.write_str("BBBB")?,
        Some(DayPeriod::FlexibleNarrow) => sink.write_str("BBBBB")?,
        None => (),
    }
    match (hour, hour_kind) {
        (Some(Hour::Numeric), None) => sink.write_char('j')?,
        (Some(Hour::TwoDigit), None) => sink.write_str("jj")?,
        (Some(Hour::Numeric), Some(HourKind::Clock12)) => sink.write_char('h')?,
        (Some(Hour::TwoDigit), Some(HourKind::Clock12)) => sink.write_str("hh")?,
        (Some(Hour::Numeric), Some(HourKind::Clock24)) => sink.write_char('H')?,
        (Some(Hour::TwoDigit), Some(HourKind::Clock24)) => sink.write_str("HH")?,
        (None, _) => (),
    }
    match minute {
        Some(Minute::Numeric) => sink.write_char('m')?,
        Some(Minute::TwoDigit) => sink.write_str("mm")?,
        None => (),
    }
    match second {
        Some(Second::Numeric) => sink.write_char('s')?,
        Some(Second::TwoDigit) => sink.write_str("ss")?,
        None => (),
    }
    match fractional_second_digits {
        Some(FractionalSecondDigits::F1) => sink.write_char('S')?,
        Some(FractionalSecondDigits::F2) => sink.write_str("SS")?,
        Some(FractionalSecondDigits::F3) => sink.write_str("SSS")?,
        None => (),
    }
    match time_zone_name {
        Some(TimeZoneName::ShortSpecific) => sink.write_char('z')?,
        Some(TimeZoneName::LongSpecific) => sink.write_str("zzzz")?,
        Some(TimeZoneName::ShortOffset) => sink.write_char('O')?,
        Some(TimeZoneName::LongOffset) => sink.write_str("OOOO")?,
        Some(TimeZoneName::ShortGeneric) => sink.write_char('v')?,
        Some(TimeZoneName::LongGeneric) => sink.write_str("vvvv")?,
        None => (),
    }
    Ok(())
}

#[test]
fn test_skeleton_parse_errors_and_lenient() {
    use writeable::assert_writeable_eq;

    let cases = [
        (
            "GGGGy'foo'",
            DateTimeFieldBagParseError::UnexpectedLiteral,
            "GGGGy",
        ),
        ("GG", DateTimeFieldBagParseError::InvalidFieldLength, "G"),
        ("GGG", DateTimeFieldBagParseError::InvalidFieldLength, "G"),
        (
            "MMMMMMM",
            DateTimeFieldBagParseError::InvalidFieldLength,
            "M",
        ),
        ("L", DateTimeFieldBagParseError::NonCanonicalField, "M"),
        ("K", DateTimeFieldBagParseError::NonCanonicalField, "h"),
        ("k", DateTimeFieldBagParseError::NonCanonicalField, "H"),
        ("Q", DateTimeFieldBagParseError::UnknownField, ""),
        ("yQw", DateTimeFieldBagParseError::UnknownField, "y"),
    ];
    for (input, expected_err, expected_canonical) in cases {
        assert_eq!(
            DateTimeFieldBag::try_from_skeleton(input),
            Err(expected_err),
            "{input:?}"
        );
        assert_writeable_eq!(
            DateTimeFieldBag::from_skeleton(input),
            expected_canonical,
            "{input:?}"
        );
    }
}
