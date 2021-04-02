// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Debug;
use thiserror::Error;

/// An error which can be returned when parsing a pattern.
///
/// # Examples
/// ```
/// use icu_pattern::{Parser, ParserOptions, ParserError};
///
/// let mut parser = Parser::new("{0", ParserOptions {
///     allow_raw_letters: false,
/// });
/// assert_eq!(Err(ParserError::UnclosedPlaceholder), parser.try_next::<usize>());
/// ```
///
/// # Type parameters
///
/// - `P`: A replacement type which implements [`FromStr`].
///
/// [`FromStr`]: std::str::FromStr
#[derive(Error, Debug, PartialEq)]
pub enum ParserError<P>
where
    P: Debug,
{
    /// Encountered an illegal character.
    #[error("Illegal character: {0}.")]
    IllegalCharacter(char),

    /// Placeholder hould not be parsed from the given string slice.
    #[error("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(P),

    /// The pattern contains an unclosed placeholder.
    #[error("Unclosed placeholder")]
    UnclosedPlaceholder,

    /// The pattern contains an unclosed quoted literal.
    #[error("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
}

impl<R> From<R> for ParserError<R>
where
    R: Debug,
{
    fn from(input: R) -> Self {
        Self::InvalidPlaceholder(input)
    }
}
