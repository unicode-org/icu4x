// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
mod tests;

pub mod aliases;
pub mod fetch;
pub mod parse;

pub use parse::{PosixLocale, PosixParseError};
use std::env;

// TODO: this is intentionally incomplete, to be replaced once the
// cross-platform category API is finalized
pub fn get_raw_locales() -> Result<Vec<String>, crate::RetrievalError> {
    Ok([
        env::var("LC_ALL"),
        env::var("LANG"),
        env::var("LANGUAGE"),
        env::var("C"),
    ]
    .into_iter()
    .filter_map(Result::ok)
    .collect())
}
