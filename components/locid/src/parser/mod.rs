// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod errors;
mod langid;
mod locale;

pub use errors::ParserError;
pub use langid::{parse_language_identifier, parse_language_identifier_from_iter, ParserMode};
pub use locale::parse_locale;

pub fn get_subtag_iterator(t: &[u8]) -> impl Iterator<Item = &[u8]> {
    t.split(|c| *c == b'-' || *c == b'_')
}
