// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::{
    currency::CurrencyEssentialsV1Marker, currency_compact::ShortCurrencyCompactV1Marker,
};
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;

use super::compact_options::CompactCurrencyFormatterOptions;

/// A formatter for monetary values.
///
/// [`CompactCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
pub struct CompactCurrencyFormatter {
    /// Short currency compact data for the compact currency formatter.
    short_currency_compact: DataPayload<ShortCurrencyCompactV1Marker>,

    /// Essential data for the compact currency formatter.
    essential: DataPayload<CurrencyEssentialsV1Marker>,

    /// A [`FixedDecimalFormatter`] to format the currency value.
    fixed_decimal_formatter: FixedDecimalFormatter,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,

    /// Options bag for the compact currency formatter to determine the behavior of the formatter.
    /// for example: width.
    options: CompactCurrencyFormatterOptions,
}

impl CompactCurrencyFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale, options: super::options::CompactCurrencyFormatterOptions) -> error: DataError,
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
        locale: &DataLocale,
        options: super::options::CompactCurrencyFormatterOptions,
    ) -> Result<Self, DataError> {
        let short_currency_compact = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())?;
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        let plural_rules = PluralRules::try_new_cardinal(locale)?;

        Ok(Self {
            short_currency_compact,
            essential,
            fixed_decimal_formatter,
            plural_rules,
            options,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        options: super::options::CompactCurrencyFormatterOptions,
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
        )?;

        let short_currency_compact = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        let essential = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(locale),
                ..Default::default()
            })?
            .payload;

        let plural_rules = PluralRules::try_new_cardinal(locale)?;

        Ok(Self {
            short_currency_compact,
            essential,
            fixed_decimal_formatter,
            plural_rules,
            options,
        })
    }
}
