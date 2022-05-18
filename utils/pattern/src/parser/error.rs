// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use std::fmt::Debug;

/// An error returned when parsing a pattern.
///
/// # Examples
/// ```
/// use icu_pattern::{Parser, ParserError, ParserOptions};
///
/// let mut parser = Parser::<usize>::new(
///     "{0",
///     ParserOptions {
///         allow_raw_letters: false,
///     },
/// );
/// assert_eq!(Err(ParserError::UnclosedPlaceholder), parser.try_next());
/// ```
///
/// # Type parameters
///
/// - `E`: An error of the replacement type which implements [`FromStr`].
///
/// [`FromStr`]: std::str::FromStr
#[derive(Display, Debug, PartialEq)]
pub enum ParserError<E>
where
    E: Debug,
{
    /// Encountered an illegal character.
    #[displaydoc("Illegal character: {0}.")]
    IllegalCharacter(char),

    /// Placeholder hould not be parsed from the given string slice.
    #[displaydoc("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(E),

    /// The pattern contains an unclosed placeholder.
    #[displaydoc("Unclosed placeholder")]
    UnclosedPlaceholder,

    /// The pattern contains an unclosed quoted literal.
    #[displaydoc("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
}

impl<E: Debug> std::error::Error for ParserError<E> {}
