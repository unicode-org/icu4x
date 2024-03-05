// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for [`CurrencyFormatter`](crate::CurrencyFormatter).

/// A bag of options defining how currencies will be formatted by
/// [`CurrencyFormatter`](crate::CurrencyFormatter).
#[derive(Debug, Eq, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct CurrencyFormatterOptions {
    /// The width of the currency format.
    pub width: Width,
}

impl From<Width> for CurrencyFormatterOptions {
    fn from(width: Width) -> Self {
        Self { width }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum Width {
    /// Represents the short currency format.
    /// For example, "$1.00" in English. (TODO: is this correct?)
    Short,

    /// Represents the narrow currency format.
    /// For example, "$1" in English. (TODO: is this correct?)
    Narrow,
}

impl Default for Width {
    fn default() -> Self {
        Self::Short
    }
}
