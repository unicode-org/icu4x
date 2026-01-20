// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
    )
)]
// #![warn(missing_docs)]

//! This crate provides an experimental implementation of the `ECMA-402` traits using `ICU4X` library.

#[cfg(test)]
mod testing;

/// Implements ECMA-402 [`Intl.PluralRules`][link].
///
/// [link]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules
pub mod pluralrules;

/// Implements ECMA-402 [`Intl.ListFormat`][link].
///
/// [link]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat
pub mod list;

/// An adapter between [`DataLocale`] and [`ecma402_traits::Locale`].
#[derive(Debug, Hash, Clone, PartialEq)]
pub struct DataLocale(icu_provider::DataLocale);

impl core::ops::Deref for DataLocale {
    type Target = icu_provider::DataLocale;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ecma402_traits::Locale for crate::DataLocale {}

impl core::fmt::Display for crate::DataLocale {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
