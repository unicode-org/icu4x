// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Relative time formatting

mod format;
pub mod options;
pub mod provider;
mod relativetime;

pub use format::FormattedRelativeTime;
pub use format::parts;
pub use options::RelativeTimeFormatterOptions;
pub use relativetime::RelativeTimeFormatter;
pub use relativetime::RelativeTimeFormatterPreferences;
pub use relativetime::preferences;

/// Types that can be fed to [`RelativeTimeFormatter`] and their utilities.
///
/// This module contains re-exports from the [`fixed_decimal`] crate.
pub mod input {
    pub use fixed_decimal::Decimal;
}
