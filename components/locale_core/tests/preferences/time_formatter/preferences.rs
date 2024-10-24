// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::preferences::{define_preferences, extensions::unicode::keywords};

define_preferences!(
    /// The locale preferences for time formatting.
    TimeFormatterPreferences,
    {
        /// The hour cycle
        hour_cycle: keywords::HourCycle
    }
);
