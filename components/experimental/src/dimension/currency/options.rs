// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for [`CurrencyFormatter`](crate::dimension::currency::formatter::CurrencyFormatter).

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Whether to use CLDR **standard** or **accounting** currency format strings.
///
/// This corresponds to ECMA-402 / LDML `currencySign` (`standard` vs `accounting`) for
/// short/narrow currency formatting backed by [`CurrencyEssentials`](crate::dimension::provider::currency::essentials::CurrencyEssentials).
#[derive(Copy, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CurrencyDisplaySign {
    /// CLDR `standard` currency patterns (`currencyFormats` `standard` / `standard-alphaNextToNumber`).
    #[default]
    Standard,
    /// CLDR `accounting` currency patterns (`accounting` / `accounting-alphaNextToNumber`).
    Accounting,
}

/// A collection of configuration options that determine the formatting behavior of
/// [`CurrencyFormatter`](crate::dimension::currency::formatter::CurrencyFormatter).
#[derive(Copy, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CurrencyFormatterOptions {
    /// The width of the currency format.
    pub width: Width,
    /// Standard vs accounting currency patterns from locale data.
    pub currency_display_sign: CurrencyDisplaySign,
}

/// Options for [`LongCurrencyFormatter`](crate::dimension::currency::long_formatter::LongCurrencyFormatter)
/// and [`LongCompactCurrencyFormatter`](crate::dimension::currency::long_compact_formatter::LongCompactCurrencyFormatter).
#[derive(Copy, Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct LongCurrencyFormatterOptions {
    /// Standard vs accounting currency patterns (same CLDR keys as short currency formatting).
    pub currency_display_sign: CurrencyDisplaySign,
}

impl From<Width> for CurrencyFormatterOptions {
    fn from(width: Width) -> Self {
        Self {
            width,
            currency_display_sign: CurrencyDisplaySign::Standard,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum Width {
    /// Format the currency with the standard (short) currency symbol.
    ///
    /// For example, 1 USD formats as "$1.00" in en-US and "US$1" in most other locales.
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    #[default]
    Short,

    /// Format the currency with the narrow currency symbol.
    ///
    /// The narrow symbol may be ambiguous, so it should be evident from context which
    /// currency is being represented.
    ///
    /// For example, 1 USD formats as "$1.00" in most locales.
    #[cfg_attr(feature = "serde", serde(rename = "narrow"))]
    Narrow,
}
