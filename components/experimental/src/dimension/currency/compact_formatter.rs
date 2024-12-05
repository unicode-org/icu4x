// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    compactdecimal::{
        CompactDecimalFormatter, CompactDecimalFormatterOptions, CompactDecimalFormatterPreferences,
    },
    dimension::provider::{
        currency::CurrencyEssentialsV1Marker, currency_compact::ShortCurrencyCompactV1Marker,
    },
};
use fixed_decimal::SignedFixedDecimal;
use icu_decimal::FixedDecimalFormatterPreferences;
use icu_locale_core::preferences::{
    define_preferences, extensions::unicode::keywords::NumberingSystem, prefs_convert,
};
use icu_provider::prelude::*;

use super::{
    compact_format::CompactFormattedCurrency, compact_options::CompactCurrencyFormatterOptions,
    CurrencyCode,
};

extern crate alloc;

define_preferences!(
    /// The preferences for currency formatting.
    [Copy]
    CompactCurrencyFormatterPreferences,
    {
        numbering_system: NumberingSystem
    }
);

prefs_convert!(
    CompactCurrencyFormatterPreferences,
    FixedDecimalFormatterPreferences,
    { numbering_system }
);
prefs_convert!(
    CompactCurrencyFormatterPreferences,
    CompactDecimalFormatterPreferences
);

/// A formatter for monetary values.
///
/// [`CompactCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::compact_options`] module.
pub struct CompactCurrencyFormatter {
    /// Short currency compact data for the compact currency formatter.
    short_currency_compact: DataPayload<ShortCurrencyCompactV1Marker>,

    /// Essential data for the compact currency formatter.
    essential: DataPayload<CurrencyEssentialsV1Marker>,

    /// A [`CompactDecimalFormatter`] to format the currency value.
    compact_decimal_formatter: CompactDecimalFormatter,

    /// Options bag for the compact currency formatter to determine the behavior of the formatter.
    /// for example: width.
    options: CompactCurrencyFormatterOptions,
}

impl CompactCurrencyFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (prefs: CompactCurrencyFormatterPreferences, options: super::compact_options::CompactCurrencyFormatterOptions) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`CompactCurrencyFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: CompactCurrencyFormatterPreferences,
        options: super::compact_options::CompactCurrencyFormatterOptions,
    ) -> Result<Self, DataError> {
        let short_locale =
            DataLocale::from_preferences_locale::<ShortCurrencyCompactV1Marker>(prefs.locale_prefs);

        let short_currency_compact = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&short_locale),
                ..Default::default()
            })?
            .payload;

        let essential_locale =
            DataLocale::from_preferences_locale::<CurrencyEssentialsV1Marker>(prefs.locale_prefs);

        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&essential_locale),
                ..Default::default()
            })?
            .payload;

        let compact_decimal_formatter = CompactDecimalFormatter::try_new_short(
            (&prefs).into(),
            CompactDecimalFormatterOptions::default(),
        )?;

        Ok(Self {
            short_currency_compact,
            essential,
            compact_decimal_formatter,
            options,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: CompactCurrencyFormatterPreferences,
        options: CompactCurrencyFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<crate::dimension::provider::currency::CurrencyEssentialsV1Marker>
            + DataProvider<crate::dimension::provider::currency_compact::ShortCurrencyCompactV1Marker>
            + DataProvider<crate::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV2Marker>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>,
    {
        let locale =
            DataLocale::from_preferences_locale::<CurrencyEssentialsV1Marker>(prefs.locale_prefs);

        let compact_decimal_formatter = CompactDecimalFormatter::try_new_short_unstable(
            provider,
            (&prefs).into(),
            CompactDecimalFormatterOptions::default(),
        )?;

        let short_currency_compact = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let essential = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            short_currency_compact,
            essential,
            compact_decimal_formatter,
            options,
        })
    }

    /// Formats in the compact format a [`SignedFixedDecimal`] value for the given currency code.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::compact_formatter::CompactCurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::Writeable;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CompactCurrencyFormatter::try_new(locale, Default::default()).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
    /// let mut sink = String::new();
    /// formatted_currency.write_to(&mut sink).unwrap();
    /// assert_eq!(sink.as_str(), "$12K");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l SignedFixedDecimal,
        currency_code: CurrencyCode,
    ) -> CompactFormattedCurrency<'l> {
        CompactFormattedCurrency {
            value,
            currency_code,
            options: &self.options,
            essential: self.essential.get(),
            _short_currency_compact: self.short_currency_compact.get(),
            compact_decimal_formatter: &self.compact_decimal_formatter,
        }
    }
}
