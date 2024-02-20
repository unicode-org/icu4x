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
mod common;
mod error;
mod frontend;
#[cfg(feature = "alloc")]
mod parser;
mod single;

pub use common::PatternBackend;
pub use common::PatternItem;
pub use common::PatternItemCow;
pub use common::PlaceholderValueProvider;
pub use error::PatternError;
pub use frontend::Pattern;
#[cfg(feature = "alloc")]
pub use parser::Parser;
pub use parser::ParserError;
pub use parser::ParserOptions;
pub use parser::PatternToken;
pub use single::SinglePlaceholder;
pub use single::SinglePlaceholderKey;
#[doc(no_inline)]
pub use PatternError as Error;

trait Sealed {}

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
