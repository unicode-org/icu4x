// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use super::DateTimeFieldBag;
use crate::provider::pattern::reference::tokenizer::Token;
use crate::provider::pattern::reference::tokenizer::Uts35DateTimePatternTokenizer;

#[allow(missing_docs)] // TODO: Write excellent docs for this
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DateTimeFieldBagParseError {
    DuplicateField,
    UnexpectedLiteral,
    SyntaxError,
}

#[allow(clippy::todo)] // TODO: Resolve the TODOs
pub(crate) fn uts35_to_fieldbag(
    skeleton: &str,
) -> (DateTimeFieldBag, Option<DateTimeFieldBagParseError>) {
    use super::field::*;
    use DateTimeFieldBagParseError::*;

    macro_rules! put_field {
        ($error:expr, $field:expr, $value:expr) => {{
            if ($field).is_some() {
                *($error) = Some(DuplicateField);
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
            Token::Symbol('G', 1) => {
                put_field!(&mut error, &mut bag.era, Era::Short);
            }
            Token::Symbol('G', 4) => {
                put_field!(&mut error, &mut bag.era, Era::Long);
            }
            Token::Symbol('G', 5) => {
                put_field!(&mut error, &mut bag.era, Era::Narrow);
            }
            Token::Symbol('y', 1) => {
                put_field!(&mut error, &mut bag.year, Year::Numeric);
            }
            Token::Symbol('y', 4) => {
                put_field!(&mut error, &mut bag.year, Year::TwoDigit);
            }
            Token::Symbol('j', 1) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
            }
            Token::Symbol('j', 2) => {
                put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
            }
            Token::Symbol('h', 1) => {
                put_field!(&mut error, &mut bag.hour, Hour::Numeric);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
            }
            Token::Symbol('h', 2) => {
                put_field!(&mut error, &mut bag.hour, Hour::TwoDigit);
                put_field!(&mut error, &mut bag.hour_kind, HourKind::Clock12);
            }
            Token::Symbol(_, _) => {
                todo!()
            }
            Token::Literal(_) => {
                error.get_or_insert(UnexpectedLiteral);
            }
            Token::Placeholder(_) | Token::UnclosedLiteral(_) | Token::UnclosedPlaceholder(_) => {
                error.get_or_insert(SyntaxError);
            }
        }
    }
    (bag, error)
}

#[allow(clippy::todo)] // TODO: Resolve the TODOs
pub(crate) fn fieldbag_to_uts35<W: ?Sized + fmt::Write>(
    fieldbag: &DateTimeFieldBag,
    sink: &mut W,
) -> fmt::Result {
    use super::field::*;

    #[allow(unused_variables)] // TODO: Handle all fields
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
    match (hour, hour_kind) {
        (Some(Hour::Numeric), None) => sink.write_char('j')?,
        (Some(Hour::TwoDigit), None) => sink.write_str("jj")?,
        (Some(Hour::Numeric), Some(HourKind::Clock12)) => sink.write_char('h')?,
        (Some(_), Some(_)) => todo!(),
        (None, _) => (),
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
