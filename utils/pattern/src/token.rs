// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

/// A token returned by the [`Parser`].
///
/// # Examples
///
/// ```
/// use icu_pattern::{Parser, ParserOptions, PatternToken};
///
/// let input = "{0}, {1}";
///
/// let mut parser = Parser::new(
///     input,
///     ParserOptions {
///         allow_raw_letters: false,
///     },
/// );
///
/// let mut result = vec![];
///
/// while let Some(element) =
///     parser.try_next().expect("Failed to advance iterator")
/// {
///     result.push(element);
/// }
///
/// assert_eq!(
///     result,
///     &[
///         PatternToken::Placeholder(0),
///         PatternToken::Literal {
///             content: ", ".into(),
///             quoted: false
///         },
///         PatternToken::Placeholder(1),
///     ]
/// );
/// ```
///
/// # Type parameters
///
/// - `P`: A placeholder type which implements [`FromStr`].
///
/// # Lifetimes
///
/// - `s`: The life time of an input string slice being parsed.
///
/// [`Parser`]: crate::Parser
/// [`FromStr`]: std::str::FromStr
#[derive(PartialEq, Debug, Clone)]
pub enum PatternToken<'s, P> {
    Placeholder(P),
    Literal { content: Cow<'s, str>, quoted: bool },
}

impl<'s, P> From<(&'s str, bool)> for PatternToken<'s, P> {
    fn from(input: (&'s str, bool)) -> Self {
        Self::Literal {
            content: Cow::Borrowed(input.0),
            quoted: input.1,
        }
    }
}
