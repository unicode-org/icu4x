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
#[cfg(feature = "alloc")]
mod parser;
mod single;

use alloc::borrow::Cow;

pub use frontend::Pattern;
pub use frontend::PlaceholderValueProvider;
#[cfg(feature = "alloc")]
pub use parser::{Parser, ParserError, ParserOptions, PatternToken};
pub use single::{SinglePlaceholder, SinglePlaceholderKey};

/// A borrowed item in a [`Pattern`]. Items are either string literals or placeholders.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternItem<'a, T> {
    /// A placeholder of the type specified on this [`PatternItemCow`].
    Placeholder(T),
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(&'a str),
}

/// A borrowed-or-owned item in a [`Pattern`]. Items are either string literals or placeholders.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternItemCow<'a, T> {
    /// A placeholder of the type specified on this [`PatternItemCow`].
    Placeholder(T),
    /// A string literal. This can occur in one of three places:
    ///
    /// 1. Between the start of the string and the first placeholder (prefix)
    /// 2. Between two placeholders (infix)
    /// 3. Between the final placeholder and the end of the string (suffix)
    Literal(Cow<'a, str>),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PatternError {
    InvalidPattern,
}

trait Sealed {}

/// Types that implement backing data models for [`Pattern`] implement this trait.
///
/// The trait has no public methods and is not implementable outside of this crate.
#[allow(private_bounds)]
pub trait PatternBackend: Sealed {
    /// The type to be used as the placeholder key in code.
    type PlaceholderKey;

    /// The unsized type of the store required for this backend, usually `str` or `[u8]`.
    type Store: ?Sized;

    #[doc(hidden)] // TODO(#4467): Should be internal
    type Iter<'a>: Iterator<Item = PatternItem<'a, Self::PlaceholderKey>>
    where
        Self: 'a;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn validate_store(store: &Self::Store) -> Result<(), PatternError>;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, PatternError>>,
    >(
        items: I,
    ) -> Result<<Self::Store as ToOwned>::Owned, PatternError>
    where
        Self: 'a,
        Self::Store: ToOwned;

    #[doc(hidden)] // TODO(#4467): Should be internal
    fn iter_items<'a>(store: &'a Self::Store) -> Self::Iter<'a>;
}

/// # Examples
///
/// ```
/// use icu_pattern::SinglePlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// // Create a pattern from the string syntax:
/// let pattern =
///     SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
///
/// // Interpolate some values into the pattern:
/// assert_writeable_eq!(pattern.interpolate(["Alice"]), "Hello, Alice!");
/// ```
pub type SinglePlaceholderPattern<Store> = Pattern<SinglePlaceholder, Store>;
