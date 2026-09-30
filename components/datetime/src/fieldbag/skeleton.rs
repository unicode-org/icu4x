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
    DuplicateField,
    /// The skeleton contained literal characters (such as punctuation, whitespace, or quoted text).
    UnexpectedLiteral,
    /// The skeleton contained a syntax error, such as an unclosed quoted literal.
    SyntaxError,
    /// A field symbol had a non-canonical or unsupported repetition width (such as `GGG`, `yyyy`, or `MMMMMMM`).
    InvalidFieldLength,
    /// A non-canonical field symbol was used instead of its canonical skeleton equivalent (such as `L` instead of `M`).
    NonCanonicalField,
    /// An unknown or unsupported field symbol was encountered.
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
        match token {
            Token::Symbol(ch, len) => {
                let ch = match ch {
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
                    _ => ch,
                };
                match (ch, len) {
                    ('G', 1) => put_field!(&mut error, &mut bag.era, Era::Short),
                    ('G', 4) => put_field!(&mut error, &mut bag.era, Era::Long),
                    ('G', 5) => put_field!(&mut error, &mut bag.era, Era::Narrow),
                    ('G', _) => {
                        put_field!(&mut error, &mut bag.era, Era::Short);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('y', 1) => put_field!(&mut error, &mut bag.year, Year::Numeric),
                    ('y', 2) => put_field!(&mut error, &mut bag.year, Year::TwoDigit),
                    ('y', _) => {
                        put_field!(&mut error, &mut bag.year, Year::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('M', 1) => put_field!(&mut error, &mut bag.month, Month::Numeric),
                    ('M', 2) => put_field!(&mut error, &mut bag.month, Month::TwoDigit),
                    ('M', 3) => put_field!(&mut error, &mut bag.month, Month::Short),
                    ('M', 4) => put_field!(&mut error, &mut bag.month, Month::Long),
                    ('M', 5) => put_field!(&mut error, &mut bag.month, Month::Narrow),
                    ('M', _) => {
                        put_field!(&mut error, &mut bag.month, Month::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('d', 1) => put_field!(&mut error, &mut bag.day, Day::Numeric),
                    ('d', 2) => put_field!(&mut error, &mut bag.day, Day::TwoDigit),
                    ('d', _) => {
                        put_field!(&mut error, &mut bag.day, Day::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('E', 1) => put_field!(&mut error, &mut bag.weekday, Weekday::Short),
                    ('E', 4) => put_field!(&mut error, &mut bag.weekday, Weekday::Long),
                    ('E', 5) => put_field!(&mut error, &mut bag.weekday, Weekday::Narrow),
                    ('E', _) => {
                        put_field!(&mut error, &mut bag.weekday, Weekday::Short);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('B', 1) => {
                        put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleShort)
                    }
                    ('B', 4) => {
                        put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleLong)
                    }
                    ('B', 5) => {
                        put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleNarrow)
                    }
                    ('B', _) => {
                        put_field!(&mut error, &mut bag.day_period, DayPeriod::FlexibleShort);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('j', 1) => put_field!(&mut error, &mut bag.hour, Hour::Numeric),
                    ('j', 2) => put_field!(&mut error, &mut bag.hour, Hour::TwoDigit),
                    ('j', _) => {
                        put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('h', 1) => {
                        put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
                    }
                    ('h', 2) => {
                        put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
                    }
                    ('h', _) => {
                        put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('H', 1) => {
                        put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
                    }
                    ('H', 2) => {
                        put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
                    }
                    ('H', _) => {
                        put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                        put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock24);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('m', 1) => put_field!(&mut error, &mut bag.minute, Minute::Numeric),
                    ('m', 2) => put_field!(&mut error, &mut bag.minute, Minute::TwoDigit),
                    ('m', _) => {
                        put_field!(&mut error, &mut bag.minute, Minute::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('s', 1) => put_field!(&mut error, &mut bag.second, Second::Numeric),
                    ('s', 2) => put_field!(&mut error, &mut bag.second, Second::TwoDigit),
                    ('s', _) => {
                        put_field!(&mut error, &mut bag.second, Second::Numeric);
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('S', 1) => put_field!(
                        &mut error,
                        &mut bag.fractional_second_digits,
                        FractionalSecondDigits::F1
                    ),
                    ('S', 2) => put_field!(
                        &mut error,
                        &mut bag.fractional_second_digits,
                        FractionalSecondDigits::F2
                    ),
                    ('S', 3) => put_field!(
                        &mut error,
                        &mut bag.fractional_second_digits,
                        FractionalSecondDigits::F3
                    ),
                    ('S', _) => {
                        put_field!(
                            &mut error,
                            &mut bag.fractional_second_digits,
                            FractionalSecondDigits::F1
                        );
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('z', 1) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::ShortSpecific
                    ),
                    ('z', 4) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::LongSpecific
                    ),
                    ('z', _) => {
                        put_field!(
                            &mut error,
                            &mut bag.time_zone_name,
                            TimeZoneName::ShortSpecific
                        );
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('O', 1) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::ShortOffset
                    ),
                    ('O', 4) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::LongOffset
                    ),
                    ('O', _) => {
                        put_field!(
                            &mut error,
                            &mut bag.time_zone_name,
                            TimeZoneName::ShortOffset
                        );
                        error.get_or_insert(InvalidFieldLength);
                    }
                    ('v', 1) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::ShortGeneric
                    ),
                    ('v', 4) => put_field!(
                        &mut error,
                        &mut bag.time_zone_name,
                        TimeZoneName::LongGeneric
                    ),
                    ('v', _) => {
                        put_field!(
                            &mut error,
                            &mut bag.time_zone_name,
                            TimeZoneName::ShortGeneric
                        );
                        error.get_or_insert(InvalidFieldLength);
                    }
                    (_, _) => {
                        error.get_or_insert(UnknownField);
                    }
                }
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
fn test_skeleton_literal_errors() {
    assert_eq!(
        DateTimeFieldBag::try_from_skeleton("GGGGy'foo'"),
        Err(DateTimeFieldBagParseError::UnexpectedLiteral)
    );
    assert_eq!(
        DateTimeFieldBag::try_from_skeleton("GGGGy'foo"),
        Err(DateTimeFieldBagParseError::SyntaxError)
    );
}

#[test]
fn test_skeleton_parse_errors_and_lenient() {
    use writeable::assert_writeable_eq;

    let cases = [
        ("GG", DateTimeFieldBagParseError::InvalidFieldLength, "G"),
        ("GGG", DateTimeFieldBagParseError::InvalidFieldLength, "G"),
        ("yyyy", DateTimeFieldBagParseError::InvalidFieldLength, "y"),
        (
            "MMMMMMM",
            DateTimeFieldBagParseError::InvalidFieldLength,
            "M",
        ),
        ("L", DateTimeFieldBagParseError::NonCanonicalField, "M"),
        (
            "LLLL",
            DateTimeFieldBagParseError::NonCanonicalField,
            "MMMM",
        ),
        ("K", DateTimeFieldBagParseError::NonCanonicalField, "h"),
        ("k", DateTimeFieldBagParseError::NonCanonicalField, "H"),
        ("Q", DateTimeFieldBagParseError::UnknownField, ""),
        ("yQw", DateTimeFieldBagParseError::UnknownField, "y"),
        ("yyMMy", DateTimeFieldBagParseError::DuplicateField, "yyMM"),
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
