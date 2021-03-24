// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::Extensions;
use crate::parser::errors::ParserError;
use crate::parser::{get_subtag_iterator, parse_language_identifier_from_iter, ParserMode};
use crate::Locale;

pub fn parse_locale(t: &[u8]) -> Result<Locale, ParserError> {
    let mut iter = get_subtag_iterator(t).peekable();

    let id = parse_language_identifier_from_iter(&mut iter, ParserMode::Locale)?;
    let extensions = if iter.peek().is_some() {
        Extensions::try_from_iter(&mut iter)?
    } else {
        Extensions::default()
    };
    Ok(Locale { id, extensions })
}
