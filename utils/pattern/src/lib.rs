// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_pattern` is a utility crate of the [`ICU4X`] project.
//!
//! It includes a [`Pattern`] struct which wraps a paid of [`Parser`] and [`Interpolator`] allowing for parsing and interpolation of ICU placeholder patterns, like "{0} days" or
//! "{0}, {1}" with custom elements and string literals.
//!
//! # Placeholders & Elements
//!
//! The [`Parser`] is generic over any `Placeholder` which implements [`FromStr`]
//! allowing the consumer to parse placeholder patterns such as "{0}, {1}",
//! "{date}, {time}" or any other.
//!
//! The [`Interpolator`] can interpolate the [`Pattern`] against any
//! iterator over `Element`.
//!
//! # Examples
//!
//! In the following example we're going to use a custom `Token` type,
//! and an `Element` type which will be either a `Token` or a string slice.
//!
//! For the purpose of the example, a higher level
//! [`interpolate_to_string`](Pattern::interpolate_to_string) method
//! is being used.
//!
//! ```
//! use icu_pattern::Pattern;
//! use std::{borrow::Cow, convert::TryInto, fmt::Display};
//!
//! #[derive(Debug, PartialEq)]
//! enum ExampleToken {
//!     Year,
//!     Month,
//!     Day,
//!     Hour,
//!     Minute,
//! }
//!
//! impl Display for ExampleToken {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "[{:?}]", self)
//!     }
//! }
//!
//! #[derive(Debug, PartialEq)]
//! enum ExampleElement<'s> {
//!     Token(ExampleToken),
//!     Literal(Cow<'s, str>),
//! }
//!
//! impl Display for ExampleElement<'_> {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         match self {
//!             Self::Token(token) => token.fmt(f),
//!             Self::Literal(lit) => lit.fmt(f),
//!         }
//!     }
//! }
//!
//! let pattern: Pattern<usize> = "{0}, {1}".try_into().expect("Failed to parse a pattern.");
//!
//! let replacements = vec![
//!     vec![
//!         ExampleElement::Token(ExampleToken::Year),
//!         ExampleElement::Literal("-".into()),
//!         ExampleElement::Token(ExampleToken::Month),
//!         ExampleElement::Literal("-".into()),
//!         ExampleElement::Token(ExampleToken::Day),
//!     ],
//!     vec![
//!         ExampleElement::Token(ExampleToken::Hour),
//!         ExampleElement::Literal(":".into()),
//!         ExampleElement::Token(ExampleToken::Minute),
//!     ],
//! ];
//!
//! assert_eq!(
//!     pattern
//!         .interpolate_to_string::<ExampleElement, _>(&replacements)
//!         .expect("Failed to interpolate a pattern."),
//!     "[Year]-[Month]-[Day], [Hour]:[Minute]"
//! );
//! ```
//!
//! # Combinators
//!
//! In the example above, the replacements will be parsed at compile time and stored on a [`Vec`],
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

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        // TODO(#1668): enable clippy::exhaustive_structs,
        // TODO(#1668): enable clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]

mod interpolator;
mod parser;
mod pattern;
mod replacement;
mod token;

pub use interpolator::{InterpolatedKind, Interpolator, InterpolatorError};
pub use parser::{Parser, ParserError, ParserOptions};
pub use pattern::{InterpolatedPattern, Pattern, PatternError};
pub use replacement::ReplacementProvider;
pub use token::PatternToken;
