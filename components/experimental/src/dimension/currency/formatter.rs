// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;
use either::Either;

use fixed_decimal::{
    Decimal as FixedDecimal, RoundingIncrement, SignedRoundingMode, UnsignedRoundingMode,
};
use icu_decimal::preferences::CompactDecimalFormatterPreferences;
use icu_decimal::{AbstractFormatter, DecimalFormatter, DecimalFormatterPreferences};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::{PluralRules, PluralRulesPreferences};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::Writeable;

use super::super::provider::currency::{
    essentials::CurrencyEssentialsV1,
    extended::CurrencyExtendedDataV1,
    fractions::{CurrencyFractionsV1, FractionInfo, Rounding},
    patterns::CurrencyPatternsDataV1,
    symbols::CurrencySymbolsV1,
};
use super::CurrencyCode;

extern crate alloc;

define_preferences!(
    /// The preferences for currency formatting.
    [Copy]
    CurrencyFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        numbering_system: crate::dimension::preferences::NumberingSystem
    }
);

prefs_convert!(CurrencyFormatterPreferences, DecimalFormatterPreferences, {
    numbering_system
});
prefs_convert!(CurrencyFormatterPreferences, PluralRulesPreferences);
prefs_convert!(
    CurrencyFormatterPreferences,
    CompactDecimalFormatterPreferences,
    { numbering_system }
);

#[derive(Debug)]
pub(crate) enum CurrencyFormatterData {
    IsoSymbol {
        essential: DataPayload<CurrencyEssentialsV1>,
        currency: CurrencyCode,
    },
    IsoName {
        patterns: DataPayload<CurrencyPatternsDataV1>,
        currency: CurrencyCode,
    },
    Symbol {
        essential: DataPayload<CurrencyEssentialsV1>,
        symbol: DataPayload<CurrencySymbolsV1>,
    },
    Name {
        extended: DataPayload<CurrencyExtendedDataV1>,
        patterns: DataPayload<CurrencyPatternsDataV1>,
        plural_rules: PluralRules,
    },
    NoCurrency,
}

/// A formatter for monetary values.
///
/// [`CurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
#[derive(Debug)]
pub struct CurrencyFormatter<V: AbstractFormatter> {
    value_formatter: V,
    currency_data: CurrencyFormatterData,
    fraction_info: FractionInfo,
}

impl<V: AbstractFormatter> CurrencyFormatter<V> {
    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new_essential(
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
        width: TinyAsciiStr<1>,
    ) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids.clone())?
                .payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            crate::provider::Baked.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);
        #[allow(const_item_mutation)]
        let currency_data = match DataProvider::<CurrencySymbolsV1>::load(
            &crate::provider::Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    CurrencySymbolsV1::make_attributes(currency, width, &mut TinyAsciiStr::EMPTY),
                    &locale,
                ),
                ..Default::default()
            },
        )
        .allow_identifier_not_found()?
        {
            Some(res) => CurrencyFormatterData::Symbol {
                essential,
                symbol: res.payload,
            },
            None => CurrencyFormatterData::IsoSymbol {
                essential,
                currency,
            },
        };

        Ok(Self {
            value_formatter,
            currency_data,
            fraction_info,
        })
    }

    pub(crate) fn try_new_essential_unstable<D>(
        provider: &D,
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
        width: TinyAsciiStr<1>,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencySymbolsV1>
            + DataProvider<CurrencyFractionsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids.clone())?.payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            provider.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);
        #[allow(const_item_mutation)]
        let currency_data = match provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    CurrencySymbolsV1::make_attributes(currency, width, &mut TinyAsciiStr::EMPTY),
                    &locale,
                ),
                ..Default::default()
            })
            .allow_identifier_not_found()?
        {
            Some(res) => CurrencyFormatterData::Symbol {
                essential,
                symbol: res.payload,
            },
            None => CurrencyFormatterData::IsoSymbol {
                essential,
                currency,
            },
        };

        Ok(Self {
            value_formatter,
            currency_data,
            fraction_info,
        })
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new_code_internal(
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids.clone())?
                .payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            crate::provider::Baked.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::IsoSymbol {
                essential,
                currency,
            },
            fraction_info,
        })
    }

    pub(crate) fn try_new_code_internal_unstable<D>(
        provider: &D,
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<CurrencyEssentialsV1> + DataProvider<CurrencyFractionsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids.clone())?.payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            provider.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::IsoSymbol {
                essential,
                currency,
            },
            fraction_info,
        })
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new_name_internal(
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError> {
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let marker_attributes =
            DataMarkerAttributes::try_from_str(currency.0.as_str()).map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;
        let extended_opt = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    &locale,
                ),
                ..Default::default()
            })
            .allow_identifier_not_found()?
            .map(|res| res.payload);

        let patterns = crate::provider::Baked.load(Default::default())?.payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            crate::provider::Baked.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        let currency_data = match extended_opt {
            Some(extended) => {
                let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;
                CurrencyFormatterData::Name {
                    extended,
                    patterns,
                    plural_rules,
                }
            }
            None => CurrencyFormatterData::IsoName { patterns, currency },
        };

        Ok(Self {
            value_formatter,
            currency_data,
            fraction_info,
        })
    }

    pub(crate) fn try_new_name_internal_unstable<D>(
        provider: &D,
        value_formatter: V,
        prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyExtendedDataV1>
            + DataProvider<CurrencyPatternsDataV1>
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let marker_attributes =
            DataMarkerAttributes::try_from_str(currency.0.as_str()).map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;
        let extended_opt = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    &locale,
                ),
                ..Default::default()
            })
            .allow_identifier_not_found()?
            .map(|res| res.payload);

        let patterns = provider.load(Default::default())?.payload;
        let fractions: DataPayload<CurrencyFractionsV1> =
            provider.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        let currency_data = match extended_opt {
            Some(extended) => {
                let plural_rules =
                    PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;
                CurrencyFormatterData::Name {
                    extended,
                    patterns,
                    plural_rules,
                }
            }
            None => CurrencyFormatterData::IsoName { patterns, currency },
        };

        Ok(Self {
            value_formatter,
            currency_data,
            fraction_info,
        })
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new_no_currency_internal(
        value_formatter: V,
        _prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError> {
        let fractions: DataPayload<CurrencyFractionsV1> =
            crate::provider::Baked.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::NoCurrency,
            fraction_info,
        })
    }

    pub(crate) fn try_new_no_currency_internal_unstable<D>(
        provider: &D,
        value_formatter: V,
        _prefs: CurrencyFormatterPreferences,
        currency: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<CurrencyFractionsV1>,
    {
        let fractions: DataPayload<CurrencyFractionsV1> =
            provider.load(Default::default())?.payload;
        let fraction_info = fractions.get().resolve(currency);

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::NoCurrency,
            fraction_info,
        })
    }
}

impl CurrencyFormatter<DecimalFormatter> {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: CurrencyCode) -> error: DataError,
        functions: [
            try_new_symbol: skip,
            try_new_symbol_with_buffer_provider,
            try_new_symbol_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: CurrencyCode) -> error: DataError,
        functions: [
            try_new_symbol_narrow: skip,
            try_new_symbol_narrow_with_buffer_provider,
            try_new_symbol_narrow_unstable,
            Self
        ]
    );

    // We manually implement the compiled constructors because of the cross-crate dependency
    // on `icu_decimal` markers (which are not present in `icu_experimental`'s local `Baked` provider).
    // TODO: When CurrencyFormatter is migrated out of experimental, check if we can use the
    // macro-generated versions instead of these manual implementations.

    /// Creates a new [`CurrencyFormatter`] for formatting with short currency symbols from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_symbol(
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    /// Creates a new [`CurrencyFormatter`] for formatting with narrow currency symbols from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_symbol_narrow(
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_symbol)]
    pub fn try_new_symbol_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencySymbolsV1>
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        Self::try_new_essential_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_symbol_narrow)]
    pub fn try_new_symbol_narrow_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencySymbolsV1>
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        Self::try_new_essential_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: CurrencyCode) -> error: DataError,
        functions: [
            try_new_code: skip,
            try_new_code_with_buffer_provider,
            try_new_code_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for formatting using the 3-letter ISO currency code from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_code(currency_preferences, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "USD\u{a0}12,345.67");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_code(
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_code_internal(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_code)]
    pub fn try_new_code_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        Self::try_new_code_internal_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: CurrencyCode) -> error: DataError,
        functions: [
            try_new_name: skip,
            try_new_name_with_buffer_provider,
            try_new_name_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for formatting with full currency display names from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_name(currency_preferences, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12,345.67 US dollars");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_name(
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_name_internal(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_name)]
    pub fn try_new_name_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyExtendedDataV1>
            + DataProvider<CurrencyPatternsDataV1>
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        Self::try_new_name_internal_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: CurrencyCode) -> error: DataError,
        functions: [
            try_new_no_currency: skip,
            try_new_no_currency_with_buffer_provider,
            try_new_no_currency_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for formatting currency amounts without a currency symbol, code, or name from compiled locale data.
    ///
    /// The currency code is used to determine the number of decimal digits and rounding for the format.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_no_currency(currency_preferences, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12,345.67");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_no_currency(
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_no_currency_internal(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_no_currency)]
    pub fn try_new_no_currency_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyFractionsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        Self::try_new_no_currency_internal_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            currency_code,
        )
    }

}

struct FormattedNoCurrency<'a, V> {
    formatter: &'a V,
    value: FixedDecimal,
}

impl<V: AbstractFormatter> Writeable for FormattedNoCurrency<'_, V> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        V::format_sign(
            self.formatter,
            V::format_unsigned(self.formatter, &self.value.absolute),
            self.value.sign,
        )
        .write_to(sink)
    }
}

impl<V: AbstractFormatter> Display for FormattedNoCurrency<'_, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.write_to(f)
    }
}

struct FormattedCurrencyPattern<'a, V> {
    formatter: &'a V,
    value: FixedDecimal,
    currency_data: &'a CurrencyFormatterData,
}

impl<V: AbstractFormatter> Writeable for FormattedCurrencyPattern<'_, V> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let formatted_value = V::format_unsigned(self.formatter, &self.value.absolute);

        let (pattern, currency_str) = match self.currency_data {
            CurrencyFormatterData::IsoSymbol {
                essential,
                currency,
            } => {
                let pattern = essential.get().get_positive(true, true);
                (pattern, currency.0.as_str())
            }
            CurrencyFormatterData::IsoName { patterns, currency } => {
                let currency_str = currency.0.as_str();
                let pattern = patterns.get().elements.get_default().1;
                (pattern, currency_str)
            }
            CurrencyFormatterData::Symbol { essential, symbol } => {
                let symbol = symbol.get();
                let pattern = essential
                    .get()
                    .get_positive(symbol.starts_with_letter(), symbol.ends_with_letter());
                (pattern, symbol.as_str())
            }
            CurrencyFormatterData::Name {
                extended,
                patterns,
                plural_rules,
            } => {
                let operands = V::plural_operands(&formatted_value);
                let currency_str = extended.get().get(operands, plural_rules);
                let pattern = patterns.get().get(operands, plural_rules);
                (pattern, currency_str)
            }
            CurrencyFormatterData::NoCurrency => unreachable!(),
        };

        V::format_sign(
            self.formatter,
            pattern.interpolate((formatted_value, currency_str)),
            self.value.sign,
        )
        .write_to(sink)
    }
}

impl<V: AbstractFormatter> Display for FormattedCurrencyPattern<'_, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.write_to(f)
    }
}

impl<V: AbstractFormatter> CurrencyFormatter<V> {
    /// Formats a [`FixedDecimal`] value.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_symbol(locale, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(
    ///     fmt.format_fixed_decimal(&value),
    ///     "$12,345.67"
    /// );
    /// ```
    ///
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_compact_symbol(locale, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "$12K");
    /// ```
    ///
    /// ```
    /// use icu::experimental::dimension::currency::formatter::CurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_prefs = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_compact_long_symbol(currency_prefs, currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "$12 thousand");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
    ) -> impl Writeable + Display + 'l {
        let rounded_value = apply_precision(value.clone(), self.fraction_info);

        if let CurrencyFormatterData::NoCurrency = &self.currency_data {
            Either::Left(FormattedNoCurrency {
                formatter: &self.value_formatter,
                value: rounded_value,
            })
        } else {
            Either::Right(FormattedCurrencyPattern {
                formatter: &self.value_formatter,
                value: rounded_value,
                currency_data: &self.currency_data,
            })
        }
    }
}

pub(crate) fn apply_precision(mut value: FixedDecimal, fraction_info: FractionInfo) -> FixedDecimal {
    let precision = fraction_info.digits as i16;
    let rounding = fraction_info.rounding;

    let (magnitude, increment) = match rounding {
        Rounding::R50 => (-precision + 1, RoundingIncrement::MultiplesOf5),
        Rounding::R20 => (-precision + 1, RoundingIncrement::MultiplesOf2),
        Rounding::R5 => (-precision, RoundingIncrement::MultiplesOf5),
        Rounding::R1 => (-precision, RoundingIncrement::MultiplesOf1),
    };

    value = value.rounded_with_mode_and_increment(
        magnitude,
        SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
        increment,
    );
    value.pad_end(-precision);
    value
}

// TODO: Discuss reusing the `load_with_fallback` helper from `icu_decimal`
// (or moving it to a shared location) instead of duplicating it here.
pub(crate) fn load_with_fallback<'a, M: DataMarker>(
    provider: &(impl DataProvider<M> + ?Sized),
    ids: impl Iterator<Item = DataIdentifierBorrowed<'a>>,
) -> Result<DataResponse<M>, DataError> {
    let mut ids = ids.peekable();

    while let Some(id) = ids.next() {
        if ids.peek().is_some() {
            if let Some(r) = provider
                .load(DataRequest {
                    id,
                    metadata: {
                        let mut m = DataRequestMetadata::default();
                        m.silent = true;
                        m
                    },
                })
                .allow_identifier_not_found()?
            {
                return Ok(r);
            }
        } else {
            return provider.load(DataRequest {
                id,
                metadata: DataRequestMetadata::default(),
            });
        }
    }

    Err(DataErrorKind::InvalidRequest.into_error())
}
