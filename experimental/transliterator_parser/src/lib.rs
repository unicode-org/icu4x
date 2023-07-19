// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_transliterator_parser` is a utility crate of the [`ICU4X`] project.
//!
//! This crate provides parsing functionality for [UTS #35 - Transliterators](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
//!
//! See [`parse`](crate::parse) for more information.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
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
#![warn(missing_docs)]

mod parse;

pub use parse::*;
