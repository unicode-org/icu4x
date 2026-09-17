// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use super::DateTimeFieldBag;
use super::tokenizer::Token;
use super::tokenizer::Uts35DateTimePatternTokenizer;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DateTimeFieldBagParseError {
    DuplicateField,
    UnexpectedLiteral,
}

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
    match (hour, hour_kind) {
        (Some(Hour::Numeric), None) => sink.write_char('j')?,
        (Some(Hour::TwoDigit), None) => sink.write_str("jj")?,
        (Some(Hour::Numeric), Some(HourKind::Clock12)) => sink.write_char('h')?,
        (Some(_), Some(_)) => todo!(),
        (None, _) => (),
    }
    Ok(())
}
