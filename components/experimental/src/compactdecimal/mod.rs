// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Compact decimal

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
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
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

mod error;
mod formatter;
/// Options used by this crate
pub mod options;
pub mod provider;

pub use error::ExponentError;
pub use formatter::CompactDecimalFormatter;

/// Locale preferences used by this crate
pub mod preferences {
    use icu_locale::preferences::{define_preferences, prefs_convert};

    #[doc(inline)]
    /// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
    #[doc = "\n"] // prevent autoformatting
    pub use icu_locale_core::preferences::extensions::unicode::keywords::NumberingSystem;

    define_preferences!(
        /// The preferences for compact decimal formatting.
        [Copy]
        CompactDecimalFormatterPreferences,
        {
            /// The user's preferred numbering system.
            ///
            /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
            numbering_system: NumberingSystem
        }
    );

    prefs_convert!(
        CompactDecimalFormatterPreferences,
        icu_decimal::DecimalFormatterPreferences,
        { numbering_system }
    );
    prefs_convert!(
        CompactDecimalFormatterPreferences,
        icu_plurals::PluralRulesPreferences
    );
}
