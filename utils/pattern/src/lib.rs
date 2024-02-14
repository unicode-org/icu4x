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

mod num_pattern;
#[cfg(feature = "alloc")]
mod parser;

#[cfg(feature = "alloc")]
pub use parser::{Parser, ParserError, ParserOptions, PatternToken};
