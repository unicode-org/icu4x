// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE )

use icu::locale::LanguageIdentifier;

/// Implements ECMA-402 [`Intl.PluralRules`][link].
///
/// [link]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRulres/PluralRules
pub mod pluralrules;

/// An adapter between `icu_locale` and `ecma402_traits`.
///
/// Specifically, adds an implementation of [ecma402_traits::Locale], which is
/// rudimentary at the moment.
#[derive(Debug, Hash, Clone, PartialEq)]
pub enum Locale {
    /// An ECMA402 compatible [Locale] created from icu4x [LanguageIdentifier].
    FromLangid(LanguageIdentifier),
    /// An ECMA402 [Locale] created from icu4x [icu_locale::Locale].
    FromLocale(icu::locale::Locale),
}

impl ecma402_traits::Locale for crate::Locale {}

impl std::fmt::Display for crate::Locale {
    /// Delegates the implementation to one of the underlying constituent elements
    /// without any custom additions.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::FromLangid(ref l) => write!(f, "{}", l),
            Locale::FromLocale(ref l) => write!(f, "{}", l),
        }
    }
}
