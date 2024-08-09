// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::{options::FixedDecimalFormatterOptions, FixedDecimalFormatter};
use icu_plurals::PluralRules;
use icu_provider::prelude::*;

use crate::dimension::provider::{
    currency_patterns::CurrencyPatternsDataV1Marker,
    extended_currency::CurrencyExtendedDataV1Marker,
};

use super::{long_format::LongFormattedCurrency, CurrencyCode};

extern crate alloc;

/// A formatter for monetary values.
///
/// [`LongCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
pub struct LongCurrencyFormatter {
    /// Extended data for the currency formatter.
    extended: DataPayload<CurrencyExtendedDataV1Marker>,

    /// Formatting patterns for each currency plural category.
    patterns: DataPayload<CurrencyPatternsDataV1Marker>,

    /// A [`FixedDecimalFormatter`] to format the currency value.
    fixed_decimal_formatter: FixedDecimalFormatter,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,
}

impl LongCurrencyFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale: &DataLocale, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`LongCurrencyFormatter`] from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, currency_code: &CurrencyCode) -> Result<Self, DataError> {
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())?;

        let marker_attributes = DataMarkerAttributes::try_from_str(currency_code.0.as_str())
            .map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;

        let extended = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal(locale)?;

        Ok(Self {
            extended,
            patterns,
            fixed_decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<super::super::provider::extended_currency::CurrencyExtendedDataV1Marker>
            + DataProvider<super::super::provider::currency_patterns::CurrencyPatternsDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )?;

        let marker_attributes = DataMarkerAttributes::try_from_str(currency_code.0.as_str())
            .map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;
        let extended = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = provider.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, locale)?;

        Ok(Self {
            extended,
            patterns,
            fixed_decimal_formatter,
            plural_rules,
        })
    }

    /// Formats in the long format a [`FixedDecimal`] value for the given currency code.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::long_formatter::LongCurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::Writeable;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = LongCurrencyFormatter::try_new(&locale, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
    /// let mut sink = String::new();
    /// formatted_currency.write_to(&mut sink).unwrap();
    /// assert_eq!(sink.as_str(), "12,345.67 US dollars");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        currency_code: CurrencyCode,
    ) -> LongFormattedCurrency<'l> {
        LongFormattedCurrency {
            value,
            currency_code,
            extended: self.extended.get(),
            patterns: self.patterns.get(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            plural_rules: &self.plural_rules,
        }
    }
}
