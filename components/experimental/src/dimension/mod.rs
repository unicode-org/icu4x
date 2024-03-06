// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::{options::FixedDecimalFormatterOptions, FixedDecimalFormatter};
use icu_provider::prelude::*;
use options::CurrencyFormatterOptions;
use tinystr::TinyAsciiStr;

use self::{error::DimensionError, format::FormattedCurrency};

extern crate alloc;

pub mod error;
pub mod format;
pub mod options;
pub mod provider;
pub mod ule;

/// A formatter for monetary values.
///
/// [`CurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`options`] module.
pub struct CurrencyFormatter {
    /// Options bag for the currency formatter to determine the behavior of the formatter.
    /// for example: currency width.
    options: CurrencyFormatterOptions,

    /// Essential data for the currency formatter.
    essential: DataPayload<provider::CurrencyEssentialsV1Marker>,
    fixed_decimal_formatter: FixedDecimalFormatter,
}

/// A currency code, such as "USD" or "EUR".
#[derive(Clone, Copy)]
pub struct CurrencyCode(pub TinyAsciiStr<3>);

impl CurrencyFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: options::CurrencyFormatterOptions,
        error: DimensionError,
        #[cfg(skip)]
    );

    /// Creates a new [`FixedDecimalFormatterOptions`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        options: options::CurrencyFormatterOptions,
    ) -> Result<Self, DimensionError> {
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())?;
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
        options: options::CurrencyFormatterOptions,
    ) -> Result<Self, DimensionError>
    where
        D: ?Sized
            + DataProvider<provider::CurrencyEssentialsV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )?;
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
