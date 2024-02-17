// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_pattern` is a utility crate of the [`ICU4X`] project.
//!
//! It includes a [`NumericPlaceholderPattern`] struct which allows for parsing and interpolation
//! of ICU placeholder patterns, like "{0} days" or "{0}, {1}" with custom values.
//!
//! # Placeholders & Elements
//!
//! The [`Parser`] is generic over any `Placeholder` which implements [`FromStr`]
//! allowing the consumer to parse placeholder patterns such as "{0}, {1}",
//! "{date}, {time}" or any other.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`FromStr`]: std::str::FromStr

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        // TODO(#1668): enable clippy::exhaustive_structs,
        // TODO(#1668): enable clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod builder;
mod frontend;
mod num_pattern;
#[cfg(feature = "alloc")]
mod parser;
mod single;

use alloc::borrow::Cow;

pub use frontend::Pattern;
pub use frontend::PlaceholderValueProvider;
pub use num_pattern::{
    NumericPlaceholderPattern, NumericPlaceholderPatternItem, NumericPlaceholderProvider,
};
#[cfg(feature = "alloc")]
pub use parser::{Parser, ParserError, ParserOptions, PatternToken};
pub use single::{SinglePlaceholder, SinglePlaceholderKey};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternItem<'a, T> {
    Placeholder(T),
    Literal(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternItemCow<'a, T> {
    Placeholder(T),
    Literal(Cow<'a, str>),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PatternError {
    InvalidPattern,
}

pub trait PatternBackend {
    type PlaceholderKey;
    type Store: ToOwned + ?Sized;
    type Iter<'a>: Iterator<Item = PatternItem<'a, Self::PlaceholderKey>>
    where
        Self: 'a;

    fn validate_store(store: &Self::Store) -> Result<(), PatternError>;
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, PatternError>>,
    >(
        items: I,
    ) -> Result<Cow<'a, Self::Store>, PatternError>
    where
        Self: 'a;
    fn iter_items<'a>(store: &'a Self::Store) -> Self::Iter<'a>;
}

/// # Examples
///
/// ```
/// use icu_pattern::SinglePlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// // Create a pattern from the string syntax:
/// let pattern = SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
///
/// // Interpolate some values into the pattern:
/// assert_writeable_eq!(
///     pattern.interpolate(["Alice"]),
///     "Hello, Alice!"
/// );
/// ```
pub type SinglePlaceholderPattern<Store> = Pattern<SinglePlaceholder, Store>;
