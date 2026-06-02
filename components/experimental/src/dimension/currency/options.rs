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
pub struct CurrencyFormatterOptions {
    /// The width of the currency format.
    pub width: Width,

    /// The sign style for negative currency values.
    /// Default is [`CurrencySign::Standard`].
    pub currency_sign: CurrencySign,
}

impl From<Width> for CurrencyFormatterOptions {
    fn from(width: Width) -> Self {
        Self {
            width,
            currency_sign: CurrencySign::default(),
        }
    }
}

impl From<CurrencySign> for CurrencyFormatterOptions {
    fn from(currency_sign: CurrencySign) -> Self {
        Self {
            width: Width::default(),
            currency_sign,
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

/// Controls how negative currency values are rendered.
///
/// This corresponds to ECMA-402 `Intl.NumberFormat`'s `currencySign` option.
///
/// - [`CurrencySign::Standard`] uses the locale's standard currency pattern for
///   negative values, typically with a minus sign, for example `-$1,234.56`.
/// - [`CurrencySign::Accounting`] uses the locale's accounting currency pattern
///   for negative values, which in many locales is displayed with parentheses,
///   for example `($1,234.56)`.
///
/// This option only affects currency formatting. Positive values are typically
/// formatted the same for both variants unless the locale data specifies
/// otherwise.
///
/// See also:
/// - ECMA-402 `currencySign`
/// - CLDR `currencyFormats.standard`
/// - CLDR `currencyFormats.accounting`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CurrencySign {
    /// Use the locale's standard currency pattern.
    #[default]
    Standard,

    /// Use the locale's accounting currency pattern.
    Accounting,
}
