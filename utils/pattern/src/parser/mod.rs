// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod error;

use crate::token::PatternToken;
pub use error::ParserError;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData, str::FromStr};

#[derive(PartialEq, Debug)]
enum ParserState {
    Default,
    Placeholder,
    QuotedLiteral,
    Apostrophe { quoted: bool },
}

impl Default for ParserState {
    fn default() -> Self {
        Self::Default
    }
}

macro_rules! handle_literal {
    ($self:ident, $quoted:expr, $next_state:expr) => {{
        let range = $self.advance_state($self.idx, $next_state);
        if !range.is_empty() {
            #[allow(clippy::indexing_slicing)]
            // TODO(#1668) Clippy exceptions need docs or fixing.
            return Ok(Some(PatternToken::Literal {
                content: Cow::Borrowed(&$self.input[range]),
                quoted: $quoted,
            }));
        } else {
            continue;
        }
    }};
}

/// Options passed to the constructor of [`Parser`].
#[derive(Debug)]
pub struct ParserOptions {
    /// Controls whether ASCII letters can appear in the raw
    /// pattern.
    ///
    /// If set to `true`, ASCII letters can be used directly in the pattern,
    /// like "{0} days".
    ///
    /// If set to `false`, ASCII letters can only appear in quoted literals,
    /// like "{0} 'days'".
    pub allow_raw_letters: bool,
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
/// # Named placeholders
///
/// The parser is also capable of parsing different placeholder types such as strings.
///
/// ## Examples
/// ```
/// use icu_pattern::{Parser, ParserOptions, PatternToken};
///
/// let input = "{start}, {end}";
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
///         PatternToken::Placeholder("start".to_owned()),
///         PatternToken::Literal {
///             content: ", ".into(),
///             quoted: false
///         },
///         PatternToken::Placeholder("end".to_owned()),
///     ]
/// );
/// ```
///
/// # Type parameters
///
/// - `P`: The type of the placeholder used as a key for the [`ReplacementProvider`].
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
/// use icu_pattern::{Parser, ParserOptions, PatternToken};
///
/// let input = "{0} 'and' {1}";
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
///             content: " ".into(),
///             quoted: false
///         },
///         PatternToken::Literal {
///             content: "and".into(),
///             quoted: true
///         },
///         PatternToken::Literal {
///             content: " ".into(),
///             quoted: false
///         },
///         PatternToken::Placeholder(1),
///     ]
/// );
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
/// Among many benfits of such transition would be the auto-trait behavior of [`From`] and
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
/// [`ReplacementProvider`]: crate::ReplacementProvider
#[derive(Debug)]
pub struct Parser<'p, P> {
    input: &'p str,
    len: usize,

    allow_raw_letters: bool,

    start_idx: usize,
    idx: usize,

    state: ParserState,
    marker: PhantomData<P>,
}

impl<'p, P> Parser<'p, P> {
    /// Creates a new `Parser`.
    ///
    /// The `allow_raw_letters` controls whether the parser will support
    /// ASCII letters without quotes.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Parser, ParserOptions};
    /// let mut parser = Parser::<usize>::new(
    ///     "{0}, {1}",
    ///     ParserOptions {
    ///         allow_raw_letters: false,
    ///     },
    /// );
    /// ```
    pub fn new(input: &'p str, options: ParserOptions) -> Self {
        Self {
            input,
            len: input.len(),

            allow_raw_letters: options.allow_raw_letters,

            start_idx: 0,
            idx: 0,

            state: ParserState::default(),
            marker: PhantomData,
        }
    }

    /// An iterator method that advances the iterator and returns the result of an attempt to parse
    /// the next token.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Parser, ParserOptions, PatternToken};
    ///
    /// let mut parser = Parser::<usize>::new(
    ///     "{0}, {1}",
    ///     ParserOptions {
    ///         allow_raw_letters: false,
    ///     },
    /// );
    ///
    /// // A call to try_next() returns the next value…
    /// assert_eq!(Ok(Some(PatternToken::Placeholder(0))), parser.try_next());
    /// assert_eq!(
    ///     Ok(Some(PatternToken::Literal {
    ///         content: ", ".into(),
    ///         quoted: false
    ///     })),
    ///     parser.try_next()
    /// );
    /// assert_eq!(Ok(Some(PatternToken::Placeholder(1))), parser.try_next());
    ///
    /// // … and then None once it's over.
    /// assert_eq!(Ok(None), parser.try_next());
    /// ```
    pub fn try_next(
        &mut self,
    ) -> Result<Option<PatternToken<'p, P>>, ParserError<<P as FromStr>::Err>>
    where
        P: FromStr,
        P::Err: Debug,
    {
        while let Some(b) = self.input.as_bytes().get(self.idx) {
            match self.state {
                ParserState::Placeholder if *b == b'}' => {
                    let range = self.advance_state(self.idx, ParserState::Default);
                    #[allow(clippy::indexing_slicing)]
                    // TODO(#1668) Clippy exceptions need docs or fixing.
                    return self.input[range]
                        .parse()
                        .map(|ret| Some(PatternToken::Placeholder(ret)))
                        .map_err(ParserError::InvalidPlaceholder);
                }
                ParserState::QuotedLiteral if *b == b'\'' => {
                    if self.input.as_bytes().get(self.idx + 1) == Some(&b'\'') {
                        handle_literal!(self, true, ParserState::Apostrophe { quoted: true })
                    } else {
                        handle_literal!(self, true, ParserState::Default)
                    }
                }
                ParserState::Default if *b == b'{' => {
                    handle_literal!(self, false, ParserState::Placeholder)
                }
                ParserState::Default if *b == b'\'' => {
                    if self.input.as_bytes().get(self.idx + 1) == Some(&b'\'') {
                        handle_literal!(self, false, ParserState::Apostrophe { quoted: false })
                    } else {
                        handle_literal!(self, false, ParserState::QuotedLiteral)
                    }
                }
                ParserState::Default if !self.allow_raw_letters && b.is_ascii_alphabetic() => {
                    return Err(ParserError::IllegalCharacter(*b as char));
                }
                ParserState::Apostrophe { quoted } => {
                    self.start_idx -= 1;
                    if quoted {
                        handle_literal!(self, true, ParserState::QuotedLiteral)
                    } else {
                        handle_literal!(self, false, ParserState::Default)
                    }
                }
                _ => self.idx += 1,
            }
        }
        match self.state {
            ParserState::Placeholder => Err(ParserError::UnclosedPlaceholder),
            ParserState::QuotedLiteral => Err(ParserError::UnclosedQuotedLiteral),
            ParserState::Apostrophe { .. } => unreachable!(),
            ParserState::Default => {
                let range = self.start_idx..self.len;
                if !range.is_empty() {
                    self.start_idx = self.len;
                    #[allow(clippy::indexing_slicing)]
                    // TODO(#1668) Clippy exceptions need docs or fixing.
                    Ok(Some(PatternToken::Literal {
                        content: Cow::Borrowed(&self.input[range]),
                        quoted: false,
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn advance_state(&mut self, idx: usize, next_state: ParserState) -> std::ops::Range<usize> {
        let range = self.start_idx..idx;
        self.idx = idx + 1;
        self.start_idx = self.idx;
        self.state = next_state;
        range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Pattern;
    use std::{convert::TryInto, ops::Deref};

    #[test]
    fn pattern_parse_placeholders() {
        let samples = vec![
            ("{0}", vec![PatternToken::Placeholder(0)]),
            (
                "{0}{1}",
                vec![PatternToken::Placeholder(0), PatternToken::Placeholder(1)],
            ),
            (
                "{0} 'at' {1}",
                vec![
                    PatternToken::Placeholder(0),
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "at".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Placeholder(1),
                ],
            ),
            (
                "{0}'at'{1}",
                vec![
                    PatternToken::Placeholder(0),
                    PatternToken::Literal {
                        content: "at".into(),
                        quoted: true,
                    },
                    PatternToken::Placeholder(1),
                ],
            ),
            (
                "'{0}' 'at' '{1}'",
                vec![
                    PatternToken::Literal {
                        content: "{0}".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "at".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "{1}".into(),
                        quoted: true,
                    },
                ],
            ),
            (
                "'PRE' {0} 'and' {1} 'POST'",
                vec![
                    PatternToken::Literal {
                        content: "PRE".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Placeholder(0),
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "and".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Placeholder(1),
                    PatternToken::Literal {
                        content: " ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "POST".into(),
                        quoted: true,
                    },
                ],
            ),
            (
                "{0} o''clock and 'o''clock'",
                vec![
                    PatternToken::Placeholder(0),
                    PatternToken::Literal {
                        content: " o".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "'".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "clock and ".into(),
                        quoted: false,
                    },
                    PatternToken::Literal {
                        content: "o".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: "'".into(),
                        quoted: true,
                    },
                    PatternToken::Literal {
                        content: "clock".into(),
                        quoted: true,
                    },
                ],
            ),
        ];

        for (input, expected) in samples {
            let parser = Parser::new(
                input,
                ParserOptions {
                    allow_raw_letters: true,
                },
            );
            let result: Pattern<_> = parser.try_into().expect("Failed to parse a pattern");
            assert_eq!(result.deref(), expected,);
        }

        let broken: Vec<(_, Option<ParserError<std::num::ParseIntError>>)> = vec![
            ("{", Some(ParserError::UnclosedPlaceholder)),
            ("{0", Some(ParserError::UnclosedPlaceholder)),
            ("{01", Some(ParserError::UnclosedPlaceholder)),
            (
                "{date}",
                // This should be:
                // ```
                // ParserError::InvalidPlaceholder(
                //     ParseIntError {
                //         kind: std::num::IntErrorKind::InvalidDigit
                //     }
                // ),
                // ```
                // Pending: https://github.com/rust-lang/rust/issues/22639
                //
                // Once that is fixed, we can stop using an `Option` here.
                None,
            ),
            ("{date} 'days'", None),
            ("'{00}", Some(ParserError::UnclosedQuotedLiteral)),
            ("d", Some(ParserError::IllegalCharacter('d'))),
        ];

        for (input, error) in broken {
            let parser = Parser::<usize>::new(
                input,
                ParserOptions {
                    allow_raw_letters: false,
                },
            );
            let result: Result<Pattern<_>, _> = parser.try_into();
            if let Some(error) = error {
                assert_eq!(result.expect_err("Should have failed."), error,);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
