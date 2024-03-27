// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::{options::FixedDecimalFormatterOptions, FixedDecimalFormatter};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;

use super::super::provider::currency::CurrencyEssentialsV1Marker;
use super::format::FormattedCurrency;
use super::options::CurrencyFormatterOptions;

extern crate alloc;

/// A formatter for monetary values.
///
/// [`CurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
pub struct CurrencyFormatter {
    /// Options bag for the currency formatter to determine the behavior of the formatter.
    /// for example: currency width.
    options: CurrencyFormatterOptions,

    /// Essential data for the currency formatter.
    essential: DataPayload<CurrencyEssentialsV1Marker>,

    // TODO: Remove this allow once the `fixed_decimal_formatter` is used.
    #[allow(dead_code)]
    /// A [`FixedDecimalFormatter`] to format the currency value.
    fixed_decimal_formatter: FixedDecimalFormatter,
}

/// A currency code, such as "USD" or "EUR".
#[derive(Clone, Copy)]
pub struct CurrencyCode(pub TinyAsciiStr<3>);

impl CurrencyFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: super::options::CurrencyFormatterOptions,
        error: DataError,
        #[cfg(skip)]
    );

    /// Creates a new [`CurrencyFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        options: super::options::CurrencyFormatterOptions,
    ) -> Result<Self, DataError> {
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())
                // TODO: replace this `map_err` with `?` once the `FixedDecimalFormatter::try_new` returns a `Result` with `DataError`.
                .map_err(|_| {
                    DataError::custom(
                        "Failed to create a FixedDecimalFormatter for CurrencyFormatter",
                    )
                })?;
        let essential = crate::provider::Baked
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            essential,
            fixed_decimal_formatter,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        options: super::options::CurrencyFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<super::super::provider::currency::CurrencyEssentialsV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )
        // TODO: replace this `map_err` with `?` once the `FixedDecimalFormatter::try_new` returns a `Result` with `DataError`.
        .map_err(|_| {
            DataError::custom("Failed to create a FixedDecimalFormatter for CurrencyFormatter")
        })?;
        let essential = provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            essential,
            fixed_decimal_formatter,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given currency code.
    ///
    /// # Examples
    /// ```
    /// use icu_locid::locale;
    /// use tinystr::*;
    /// use writeable::Writeable;
    /// use icu_experimental::dimension::currency::formatter::{CurrencyCode, CurrencyFormatter};
    ///
    /// let locale = locale!("en-US").into();
    /// let fmt = CurrencyFormatter::try_new(&locale, Default::default()).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
    /// let mut sink = String::new();
    /// formatted_currency.write_to(&mut sink).unwrap();
    /// assert_eq!(sink.as_str(), "$12345.67");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        currency_code: CurrencyCode,
    ) -> FormattedCurrency<'l> {
        FormattedCurrency {
            value,
            currency_code,
            options: &self.options,
            essential: self.essential.get(),
        }
    }
}
