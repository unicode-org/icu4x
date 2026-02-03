// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale preferences used by this crate

use icu_locale_core::preferences::define_preferences;

define_preferences!(
    /// The preferences for fixed decimal formatting.
    [Copy]
    DecimalFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        ///
        /// To get the resolved numbering system, you can inspect the data provider.
        /// See the [`provider`] module for an example.
        numbering_system: NumberingSystem
    }
);

/// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
#[doc = "\n"] // prevent autoformatting
pub use icu_locale_core::preferences::extensions::unicode::keywords::NumberingSystem;
