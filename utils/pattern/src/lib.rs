// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_pattern` is a utility crate of the [`ICU4X`] project.
//!
//! It includes a [`Pattern`] type which supports patterns with various storage backends.
//!
//! The types are tightly coupled with the [`writeable`] crate.
//!
//! # Examples
//!
//! Parsing and interpolating with a single-placeholder pattern:
//!
//! ```
//! use icu_pattern::SinglePlaceholderPattern;
//! use writeable::assert_writeable_eq;
//!
//! // Parse a pattern string:
//! let pattern = "Hello, {0}!"
//!     .parse::<SinglePlaceholderPattern<_>>()
//!     .unwrap();
//!
//! // Interpolate into the pattern string:
//! assert_writeable_eq!(pattern.interpolate(["World"]), "Hello, World!");
//!
//! // Introspect the serialized form of the pattern string:
//! assert_eq!(pattern.take_store(), "\x08Hello, !");
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [`FromStr`]: std::str::FromStr

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod builder;
mod common;
mod double;
mod error;
mod frontend;
#[cfg(feature = "alloc")]
mod parser;
mod single;

pub use common::PatternBackend;
pub use common::PatternItem;
#[cfg(feature = "alloc")]
pub use common::PatternItemCow;
pub use common::PlaceholderValueProvider;
pub use double::DoublePlaceholder;
pub use double::DoublePlaceholderKey;
pub use error::PatternError;
pub use frontend::Pattern;
#[cfg(feature = "alloc")]
pub use parser::ParsedPatternItem;
#[cfg(feature = "alloc")]
pub use parser::Parser;
#[cfg(feature = "alloc")]
pub use parser::ParserError;
#[cfg(feature = "alloc")]
pub use parser::ParserOptions;
pub use single::SinglePlaceholder;
pub use single::SinglePlaceholderKey;
#[doc(no_inline)]
pub use PatternError as Error;

mod private {
    pub trait Sealed {}
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

/// # Examples
///
/// ```
/// use icu_pattern::DoublePlaceholderPattern;
/// use writeable::assert_writeable_eq;
///
/// // Create a pattern from the string syntax:
/// let pattern =
///     DoublePlaceholderPattern::try_from_str("Hello, {0} and {1}!").unwrap();
///
/// // Interpolate some values into the pattern:
/// assert_writeable_eq!(pattern.interpolate(["Alice", "Bob"]), "Hello, Alice and Bob!");
/// ```
pub type DoublePlaceholderPattern<Store> = Pattern<DoublePlaceholder, Store>;

#[test]
#[cfg(feature = "alloc")]
fn test_single_placeholder_pattern_impls() {
    let a = SinglePlaceholderPattern::try_from_str("{0}").unwrap();
    let b = SinglePlaceholderPattern::try_from_str("{0}").unwrap();
    assert_eq!(a, b);
    let c = b.clone();
    assert_eq!(a, c);
}
