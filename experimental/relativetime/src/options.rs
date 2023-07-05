// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for configuring [`RelativeTimeFormatter`](crate::RelativeTimeFormatter).

/// A bag of options for defining how to format time using
/// [`RelativeTimeFormatter`](crate::RelativeTimeFormatter).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RelativeTimeFormatterOptions {
    /// Whether to always use numeric formatting for time.
    pub numeric: Numeric,
}

/// Configures whether to always use numeric formatting even when special formatting is available.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Numeric {
    /// Always use numeric formatting.
    Always,

    /// Automatically select special formatting if available else fallback to numeric formatting.
    Auto,
}

impl Default for Numeric {
    fn default() -> Self {
        Numeric::Always
    }
}
