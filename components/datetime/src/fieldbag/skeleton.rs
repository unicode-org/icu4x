// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DateTimeFieldBag;
use super::tokenizer::Uts35DateTimePatternTokenizer;
use super::tokenizer::Token;

#[non_exhaustive]
pub enum DateTimeFieldBagParseError {
    DuplicateField,
    UnexpectedLiteral,
}

pub(crate) fn uts35_to_fieldbag(
    skeleton: &str,
) -> (DateTimeFieldBag, Option<DateTimeFieldBagParseError>) {
    use super::fields::*;
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
            Token::Symbol("G") => {
                put_field!(&mut error, &mut bag.era, Era::Short);
            }
            Token::Symbol("GGGG") => {
                put_field!(&mut error, &mut bag.era, Era::Long);
            }
            Token::Symbol("GGGGG") => {
                put_field!(&mut error, &mut bag.era, Era::Narrow);
            }
            Token::Symbol(_) => {
                todo!()
            }
            Token::Literal(_) => {
                error.get_or_insert(UnexpectedLiteral);
            }
        }
    }
    (bag, error)
}

pub(crate) fn fieldbag_to_uts35(fieldbag: &DateTimeFieldBag) {
    
}
