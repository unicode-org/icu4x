// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for [`CurrencyFormatter`](crate::dimension::currency::formatter::CurrencyFormatter).

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A collection of configuration options that determine the formatting behavior of
/// [`CurrencyFormatter`](crate::dimension::currency::formatter::CurrencyFormatter).
#[derive(Copy, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CurrencyFormatterOptions {}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Width {
    /// Format the currency with the standard (short) currency symbol.
    ///
    /// For example, 1 USD formats as "$1.00" in en-US and "US$1" in most other locales.
    Short,

    /// Format the currency with the narrow currency symbol.
    ///
    /// The narrow symbol may be ambiguous, so it should be evident from context which
    /// currency is being represented.
    ///
    /// For example, 1 USD formats as "$1.00" in most locales.
    Narrow,
}
