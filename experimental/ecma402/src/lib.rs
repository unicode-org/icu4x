// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This crate provides an experimental implementation of the `ECMA-402` traits using `ICU4X` library.

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

#[cfg(test)]
pub mod testing;

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

impl DataLocale {
    /// Creates a `DataLocale` from any other [`ecma402_traits::Locale`]
    fn from_ecma_locale<L: ecma402_traits::Locale>(other: L) -> Self {
        #[allow(clippy::unwrap_used)] // ecma402_traits::Locale::to_string is a valid locale
        Self(
            other
                .to_string()
                .parse::<icu::locid::Locale>()
                .unwrap()
                .into(),
        )
    }
}

impl core::ops::Deref for DataLocale {
    type Target = icu_provider::DataLocale;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ecma402_traits::Locale for crate::DataLocale {}

impl std::fmt::Display for crate::DataLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
