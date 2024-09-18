// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::{
    DataError, DataIdentifierBorrowed, DataLocale, DataPayload, DataProvider, DataRequest,
};

use super::super::provider::percent::PercentEssentialsV1Marker;
use super::format::FormattedPercent;
use super::options::PercentFormatterOptions;

extern crate alloc;

/// A formatter for percent values.
///
/// [`PercentFormatter`] supports:
///   1. Rendering in the locale's percent system.
pub struct PercentFormatter<R> {
    /// Essential data for the percent formatter.
    essential: DataPayload<PercentEssentialsV1Marker>,

    /// Options bag for the percent formatter to determine the behavior of the formatter.
    options: PercentFormatterOptions,

    /// A fixed decimal formatter used to format the percent value.
    fixed_decimal_formatter: R,
}

impl PercentFormatter<FixedDecimalFormatter> {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale, options: PercentFormatterOptions) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`PercentFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        options: PercentFormatterOptions,
    ) -> Result<Self, DataError> {
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())?;

        PercentFormatter::try_new_with_fixed_decimal_formatter(
            locale,
            fixed_decimal_formatter,
            options,
        )
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        options: PercentFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<super::super::provider::percent::PercentEssentialsV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )?;

        PercentFormatter::try_new_with_fixed_decimal_formatter_unstable(
            provider,
            locale,
            fixed_decimal_formatter,
            options,
        )
    }

    /// Formats a [`FixedDecimal`] value for the given percent code.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::percent::formatter::{
    ///     PercentFormatter,
    /// };
    /// use icu::locale::locale;
    /// use writeable::Writeable;
    ///
    /// let locale = locale!("en-US").into();
    /// let fmt = PercentFormatter::try_new(&locale, Default::default()).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let formatted_percent = fmt.format(&value);
    /// let mut sink = String::new();
    /// formatted_percent.write_to(&mut sink).unwrap();
    /// assert_eq!(sink.as_str(), "12,345.67%");
    /// ```
    pub fn format<'l>(&'l self, value: &'l FixedDecimal) -> FormattedPercent<'l> {
        FormattedPercent {
            value,
            essential: self.essential.get(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            options: &self.options,
        }
    }
}

impl<R> PercentFormatter<R>
where
    R: AsRef<FixedDecimalFormatter>,
{
    /// Creates a new [`PercentFormatter`] from compiled locale data, an options bag and fixed decimal formatter.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_fixed_decimal_formatter(
        locale: &DataLocale,
        fixed_decimal_formatter: R,
        options: PercentFormatterOptions,
    ) -> Result<Self, DataError> {
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            essential,
            options,
            fixed_decimal_formatter,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_fixed_decimal_formatter_unstable(
        provider: &(impl DataProvider<PercentEssentialsV1Marker> + ?Sized),
        locale: &DataLocale,
        fixed_decimal_formatter: R,
        options: PercentFormatterOptions,
    ) -> Result<Self, DataError> {
        let essential = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            essential,
            options,
            fixed_decimal_formatter,
        })
    }
}
