// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod error;

use crate::{pattern::PatternIterator, token::PatternToken};
pub use error::ParserError;
use std::{fmt::Debug, str::FromStr};

#[derive(PartialEq)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::Default
    }
}

macro_rules! advance_state {
    ($self:ident, $idx:expr, $next_state:path) => {{
        let range = $self.start_idx..$idx;
        $self.idx = $idx + 1;
        $self.start_idx = $self.idx;
        $self.state = $next_state;
        range
    }};
}

macro_rules! handle_literal {
    ($self:ident, $quoted:expr, $next_state:path) => {{
        let range = advance_state!($self, $self.idx, $next_state);
        if !range.is_empty() {
            return Ok(Some(PatternToken::Literal {
                content: &$self.input[range],
                quoted: $quoted,
            }));
        } else {
            continue;
        }
    }};
}

/// Placeholder pattern parser.
///
/// The parser allows for handling flexible range of generic patterns
/// with placeholders.
/// A placeholder may be anything that can be parsed from an `&str` and
/// must be enclosed in `{` and `}` characters in the input pattern string.
///
/// At the moment the parser is written as a custom fallible iterator.
///
/// # Examples
///
/// ```
/// use icu_pattern::{Parser, PatternToken};
///
/// let input = "{0}, {1}";
///
/// let mut parser = Parser::new(input);
///
/// let mut result = vec![];
///
/// while let Some(element) = parser.try_next().expect("Failed to advance iterator") {
///     result.push(element);
/// }
///
/// assert_eq!(result, &[
///     PatternToken::Placeholder(0),
///     PatternToken::Literal { content: ", ", quoted: false },
///     PatternToken::Placeholder(1),
/// ]);
/// ```
///
/// # Named placeholders
///
/// The parser is also capable of parsing different placeholder types such as strings.
///
/// ## Examples
/// ```
/// use icu_pattern::{Parser, PatternToken};
///
/// let input = "{start}, {end}";
///
/// let mut parser = Parser::new(input);
///
/// let mut result = vec![];
///
/// while let Some(element) = parser.try_next().expect("Failed to advance iterator") {
///     result.push(element);
/// }
///
/// assert_eq!(result, &[
///     PatternToken::Placeholder("start".to_string()),
///     PatternToken::Literal { content: ", ", quoted: false },
///     PatternToken::Placeholder("end".to_string()),
/// ]);
/// ```
///
/// # Lifetimes
///
/// - `p`: The life time of an input string slice to be parsed.
///
/// # Design Decisions
///
/// The parser is written in an intentionally generic way to enable use against wide range
/// of potential placeholder pattern models and use cases.
///
/// Serveral design decisions have been made that the reader should be aware of when using the API.
///
/// ## Zero copy
///
/// The parser is intended for runtime use and is optimized for performance and low memory overhad.
///
/// Zero copy parsing is a model which allows the parser to produce tokens that are de-facto
/// slices of the input without ever having to modify the input or copy from it.
///
/// In case of ICU patterns that decision brings a trade-off around handling of quoted literals.
/// A parser that copies bytes from the input when generating the output can take a pattern literal
/// that contains a quoted portion and concatenace the parts, effectively generating a single
/// literal out of a series of syntactical literal quoted and unquoted nodes.
/// A zero copy parser sacrifices that convenience for marginal performance gains.
///
/// The rationale for the decision is that many placeholder patterns do not contain ASCII letters
/// and therefore can benefit from this design decision.
/// Secondly, even in scenarios where ASCII letters, or other quoted literals, are used, the
/// zero-copy design still maintains high performance, only increasing the number of tokens
/// returned by the parser, but without increase to allocations.
///
/// ### Examples
/// ```
/// use icu_pattern::{Parser, PatternToken};
///
/// let input = "{0} 'and' {1}";
///
/// let mut parser = Parser::new(input);
///
/// let mut result = vec![];
///
/// while let Some(element) = parser.try_next().expect("Failed to advance iterator") {
///     result.push(element);
/// }
///
/// assert_eq!(result, &[
///     PatternToken::Placeholder(0),
///     PatternToken::Literal { content: " ", quoted: false },
///     PatternToken::Literal { content: "and", quoted: true },
///     PatternToken::Literal { content: " ", quoted: false },
///     PatternToken::Placeholder(1),
/// ]);
/// ```
///
/// ## Fallible Iterator
///
/// Rust providers a strong support for iterators and iterator combinators, which
/// fits very well into the design of this parser/interpolator model.
///
/// Unfortunately, Rust iterators at the moment are infallible, while parsers are inhereantely
/// fallible. As such, the decision has been made to design the API in line with what
/// we hope will become a trait signature of a fallible iterator in the future, rather
/// than implementing a reversed infallible iterator (where the [`Item`] would be
/// `Option<Result<Item>>`).
///
/// That decision impacts the ergonomics of operating on the parser, on one hand making
/// the fallible iteration more ergonomic, at a trade-off of losing access to the wide
/// range of Rust iterator traits.
///
/// ## Generic Placeholder
///
/// To handle generic placeholder design, the only constrain necessary in the parser
/// is that a placeholder must be parsed from a string slice.
/// At the moment of writing, Rust is [preparing to deprecate][`RFC 2924`] [`FromStr`] in favor of
/// [`TryFrom<&str>`][`TryFrom`].
/// Among many banfits of such transition would be the auto-trait behavior of [`From`] and
/// a [`TryFrom<&str>`][`TryFrom`] for [`&str`] allowing for placeholders to be [`&str`] themselves.
///
/// Unfortunately, at the moment [`TryFrom<&str>`][`TryFrom`] for [`usize`] is not implemented, which would
/// impact the core use case of placeholder patterns.
///
/// In result, the decision has been made to use [`FromStr`] for the time being, until
/// [`TryFrom<&str>`][`TryFrom`] gets implemented on all types that support [`FromStr`].
///
/// [`TR35 2.6.1]: https://unicode.org/reports/tr35/tr35-dates.html#dateTimeFormat
/// [`RFC 2924`]: https://github.com/rust-lang/rfcs/pull/2924
/// [`Item`]: std::iter::Iterator::Item
/// [`TryFrom`]: std::convert::TryFrom
pub struct Parser<'p> {
    input: &'p str,
    len: usize,

    start_idx: usize,
    idx: usize,

    state: ParserState,
}

impl<'p> Parser<'p> {
    /// Creates a new `Parser`.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::Parser;
    /// let mut parser = Parser::new("{0}, {1}");
    /// ```
    pub fn new(input: &'p str) -> Self {
        Self {
            input,
            len: input.len(),

            start_idx: 0,
            idx: 0,

            state: ParserState::default(),
        }
    }

    /// An iterator method that advances the iterator and returns the result of an attempt to parse
    /// the next token.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Parser, PatternToken};
    ///
    /// let mut parser = Parser::new("{0}, {1}");
    ///
    /// // A call to try_next() returns the next value…
    /// assert_eq!(Ok(Some(PatternToken::Placeholder(0))), parser.try_next());
    /// assert_eq!(Ok(Some(PatternToken::Literal { content: ", ", quoted: false})), parser.try_next::<usize>());
    /// assert_eq!(Ok(Some(PatternToken::Placeholder(1))), parser.try_next());
    ///
    /// // … and then None once it's over.
    /// assert_eq!(Ok(None), parser.try_next::<usize>());
    /// ```
    pub fn try_next<P>(
        &mut self,
    ) -> Result<Option<PatternToken<'p, P>>, ParserError<<P as FromStr>::Err>>
    where
        P: FromStr,
        P::Err: Debug,
    {
        while let Some(b) = self.input.as_bytes().get(self.idx) {
            match self.state {
                ParserState::Placeholder if *b == b'}' => {
                    let range = advance_state!(self, self.idx, ParserState::Default);
                    return self.input[range]
                        .parse()
                        .map(|ret| Some(PatternToken::Placeholder(ret)))
                        .map_err(ParserError::InvalidPlaceholder);
                }
                ParserState::QuotedLiteral if *b == b'\'' => {
                    handle_literal!(self, true, ParserState::Default)
                }
                ParserState::Default if *b == b'{' => {
                    handle_literal!(self, false, ParserState::Placeholder)
                }
                ParserState::Default if *b == b'\'' => {
                    handle_literal!(self, false, ParserState::QuotedLiteral)
                }
                ParserState::Default if b.is_ascii_alphabetic() => {
                    return Err(ParserError::IllegalCharacter(*b as char));
                }
                _ => self.idx += 1,
            }
        }
        match self.state {
            ParserState::Placeholder => Err(ParserError::UnclosedPlaceholder),
            ParserState::QuotedLiteral => Err(ParserError::UnclosedQuotedLiteral),
            ParserState::Default => {
                let range = self.start_idx..self.len;
                if !range.is_empty() {
                    self.start_idx = self.len;
                    Ok(Some(PatternToken::Literal {
                        content: &self.input[range],
                        quoted: false,
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl<'p, P> PatternIterator<'p, P> for Parser<'p> {
    fn try_next(
        &mut self,
    ) -> std::result::Result<Option<PatternToken<'p, P>>, ParserError<<P as FromStr>::Err>>
    where
        P: FromStr,
        <P as FromStr>::Err: Debug,
    {
        Parser::try_next(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse() {
        let input = "'PRE' {0} 'and' {1} 'POST'";
        let mut iter = Parser::new(&input);
        let mut result = vec![];
        while let Some(elem) = iter.try_next().unwrap() {
            result.push(elem);
        }
        assert_eq!(
            result,
            vec![
                ("PRE", true).into(),
                (" ", false).into(),
                PatternToken::Placeholder(0),
                (" ", false).into(),
                ("and", true).into(),
                (" ", false).into(),
                PatternToken::Placeholder(1),
                (" ", false).into(),
                ("POST", true).into(),
            ]
        );
    }
}
