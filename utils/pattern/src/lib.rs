// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_pattern` is a utility crate of the [`ICU4X`] project.
//!
//! It includes a [`Parser`]/[`Interpolator`] combinator. The pair can be
//! used to parse and interpolatore ICU placeholder patterns with custom
//! elements and string literals.
//!
//! # Placeholders & Elements
//!
//! An `Element` may be any type that implements [`From<&str>`][`From`].
//! A `Placeholder` may be any type that implements [`FromStr`].
//!
//! # Examples
//!
//! In the following example we're going to use a custom `Token` type,
//! and an `Element` type which will be either a `Token` or a string slice.
//!
//! ```
//! use icu_pattern::{Parser, Interpolator};
//! use std::convert::TryInto;
//!
//! #[derive(Debug, PartialEq)]
//! enum Token {
//!     Variant1,
//!     Variant2
//! }
//!
//! #[derive(Debug, PartialEq)]
//! enum Element<'s> {
//!     Token(Token),
//!     Literal(&'s str),
//! }
//!
//! impl<'s> From<&'s str> for Element<'s> {
//!     fn from(input: &'s str) -> Self {
//!         Self::Literal(input)
//!     }
//! }
//!
//! let pattern: Vec<_> = Parser::new("{0}, {1}", true).try_into().unwrap();
//!
//! let replacements = vec![
//!     vec![
//!         Element::Token(Token::Variant1),
//!         Element::Literal(" foo "),
//!         Element::Token(Token::Variant2),
//!     ],
//!     vec![
//!         Element::Token(Token::Variant2),
//!         Element::Literal(" bar "),
//!         Element::Token(Token::Variant1),
//!     ],
//! ];
//!
//! let mut interpolator = Interpolator::<_, Element>::new(&pattern, replacements);
//!
//! let mut result = vec![];
//!
//! while let Some(element) = interpolator.try_next().expect("Failed to advance iterator") {
//!     result.push(element);
//! }
//!
//! assert_eq!(result, &[
//!     Element::Token(Token::Variant1),
//!     Element::Literal(" foo "),
//!     Element::Token(Token::Variant2),
//!     Element::Literal(", "),
//!     Element::Token(Token::Variant2),
//!     Element::Literal(" bar "),
//!     Element::Token(Token::Variant1),
//! ]);
//! ```
//!
//! # Combinators
//!
//! In the example above, the replacements will be pre-computed and stored on a [`Vec`],
//! which is a collection type that has an implementation for [`ReplacementProvider`]
//! trait.
//!
//! In real use, the consumer may want to use different models of replacement provider,
//! and different element schemas.
//! Because the replacement is an iterator itself, it allows for other, more specialized parsers,
//! to be used to lazily parse particular patterns that are meant to replace the placeholders.
//! This allows for lazy parsing of those specialized patterns to be triggered
//! only if the placeholder pattern encounters a placeholder key that requires given
//! pattern to be used.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`FromStr`]: std::str::FromStr
mod interpolator;
mod parser;
mod replacement;
mod token;

pub use interpolator::{Interpolator, InterpolatorError};
pub use parser::{Parser, ParserError};
pub use replacement::ReplacementProvider;
pub use token::PatternToken;
