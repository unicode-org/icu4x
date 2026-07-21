// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dimension::provider::currency::{essentials::*, extended::*, patterns::*, symbols::*};
use icu_decimal::CompactDecimalFormatter;
use icu_provider::prelude::*;

use super::CurrencyCode;
use super::formatter::{CurrencyFormatter, CurrencyFormatterPreferences};

impl CurrencyFormatter<CompactDecimalFormatter> {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_compact_short: skip,
            try_new_compact_short_with_buffer_provider,
            try_new_compact_short_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_compact_narrow: skip,
            try_new_compact_narrow_with_buffer_provider,
            try_new_compact_narrow_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for short compact formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_compact_short(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            CompactDecimalFormatter::try_new_short((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    /// Creates a new [`CurrencyFormatter`] for narrow compact formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_compact_narrow(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            CompactDecimalFormatter::try_new_short((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_compact_short)]
    pub fn try_new_compact_short_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencySymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalCompactShortV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        Self::try_new_essential_unstable(
            provider,
            CompactDecimalFormatter::try_new_short_unstable(
                provider,
                (&prefs).into(),
                Default::default(),
            )?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_compact_narrow)]
    pub fn try_new_compact_narrow_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencySymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalCompactShortV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        Self::try_new_essential_unstable(
            provider,
            CompactDecimalFormatter::try_new_short_unstable(
                provider,
                (&prefs).into(),
                Default::default(),
            )?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CurrencyFormatterPreferences,
            currency_code: &CurrencyCode
        ) -> error: DataError,
        functions: [
            try_new_compact_long: skip,
            try_new_compact_long_with_buffer_provider,
            try_new_compact_long_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_compact_long(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_long_internal(
            CompactDecimalFormatter::try_new_long((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_compact_long)]
    pub fn try_new_compact_long_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyExtendedDataV1>
            + DataProvider<CurrencyPatternsDataV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>
            + DataProvider<icu_decimal::provider::DecimalCompactLongV1>,
    {
        Self::try_new_long_internal_unstable(
            provider,
            CompactDecimalFormatter::try_new_long_unstable(
                provider,
                (&prefs).into(),
                Default::default(),
            )?,
            prefs,
            *currency_code,
        )
    }
}
