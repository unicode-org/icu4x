// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for configuring [`RelativeTimeFormatter`](crate::relativetime::RelativeTimeFormatter).

/// A bag of options for defining how to format time using
/// [`RelativeTimeFormatter`](crate::relativetime::RelativeTimeFormatter).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RelativeTimeFormatterOptions {
    /// Whether to always use numeric formatting for time.
    pub numeric: Numeric,
    /// The width of the relative time format.
    pub width: Width,
}

impl RelativeTimeFormatterOptions {
    /// Constructs a new [`RelativeTimeFormatterOptions`] with default values.
    pub const fn default() -> Self {
        Self {
            numeric: Numeric::Always,
            width: Width::Long,
        }
    }

    /// Sets the [`Width`] option.
    pub const fn with_width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    /// Sets the [`Numeric`] option.
    pub const fn with_numeric(mut self, numeric: Numeric) -> Self {
        self.numeric = numeric;
        self
    }
}

impl From<Width> for RelativeTimeFormatterOptions {
    fn from(width: Width) -> Self {
        Self::default().with_width(width)
    }
}

impl From<Numeric> for RelativeTimeFormatterOptions {
    fn from(numeric: Numeric) -> Self {
        Self::default().with_numeric(numeric)
    }
}

/// Configures whether to always use numeric formatting even when special formatting is available.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum Numeric {
    /// Always use numeric formatting.
    #[default]
    Always,

    /// Automatically select special formatting if available else fallback to numeric formatting.
    Auto,
}

/// The width of the relative time format.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum Width {
    /// Format the relative time with the long format.
    ///
    /// For example, "in 5 seconds" in `en`.
    #[default]
    Long,

    /// Format the relative time with the short format.
    ///
    /// For example, "in 5 sec." in `en`.
    Short,

    /// Format the relative time with the narrow format.
    ///
    /// For example, "in 5s" in `en`.
    Narrow,
}
