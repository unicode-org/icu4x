// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use super::super::provider::currency::{
    essentials::{CurrencyEssentials, CurrencyEssentialsV1},
    extended::CurrencyExtendedDataV1,
    fractions::{CurrencyFractionsV1, FractionInfo, Rounding},
    patterns::CurrencyPatternsDataV1,
    symbols::CurrencySymbolsV1,
};
use super::CurrencyCode;
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
    Iso {
        essential: DataPayload<CurrencyEssentialsV1>,
        currency: CurrencyCode,
    },
    Essential {
        essential: DataPayload<CurrencyEssentialsV1>,
        symbol: DataPayload<CurrencySymbolsV1>,
        currency: CurrencyCode,
    },
    Long {
        extended: Option<DataPayload<CurrencyExtendedDataV1>>,
        patterns: DataPayload<CurrencyPatternsDataV1>,
        plural_rules: PluralRules,
        currency: CurrencyCode,
    },
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
    fractions: DataPayload<CurrencyFractionsV1>,
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
            Some(res) => CurrencyFormatterData::Essential {
                essential,
                symbol: res.payload,
                currency,
            },
            None => CurrencyFormatterData::Iso {
                essential,
                currency,
            },
        };

        let fractions = crate::provider::Baked.load(Default::default())?.payload;

        Ok(Self {
            value_formatter,
            currency_data,
            fractions,
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
            Some(res) => CurrencyFormatterData::Essential {
                essential,
                symbol: res.payload,
                currency,
            },
            None => CurrencyFormatterData::Iso {
                essential,
                currency,
            },
        };

        let fractions = provider.load(Default::default())?.payload;

        Ok(Self {
            value_formatter,
            currency_data,
            fractions,
        })
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new_long_internal(
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
        // According to UTS #35, if no displayName is found, the currency code itself should be used.
        // https://www.unicode.org/reports/tr35/tr35-numbers.html#Plural_Rules_in_Currency_Formatting
        let extended = crate::provider::Baked
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

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        let fractions = crate::provider::Baked.load(Default::default())?.payload;

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
                currency,
            },
            fractions,
        })
    }

    pub(crate) fn try_new_long_internal_unstable<D>(
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
        // According to UTS #35, if no displayName is found, the currency code itself should be used.
        // https://www.unicode.org/reports/tr35/tr35-numbers.html#Plural_Rules_in_Currency_Formatting
        let extended = provider
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

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        let fractions = provider.load(Default::default())?.payload;

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
                currency,
            },
            fractions,
        })
    }
}

impl CurrencyFormatter<DecimalFormatter> {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_short: skip,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_narrow: skip,
            try_new_narrow_with_buffer_provider,
            try_new_narrow_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_long: skip,
            try_new_long_with_buffer_provider,
            try_new_long_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for short formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_short(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    /// Creates a new [`CurrencyFormatter`] for narrow formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_narrow(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_essential(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
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
            *currency_code,
            CurrencySymbolsV1::SHORT,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_narrow)]
    pub fn try_new_narrow_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
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
            *currency_code,
            CurrencySymbolsV1::NARROW,
        )
    }

    /// Creates a new [`CurrencyFormatter`] for long formatting from compiled locale data.
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
    /// let fmt = CurrencyFormatter::try_new_long(currency_preferences, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12,345.67 US dollars");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_long(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        Self::try_new_long_internal(
            DecimalFormatter::try_new((&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
        )
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_long)]
    pub fn try_new_long_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
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
        Self::try_new_long_internal_unstable(
            provider,
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?,
            prefs,
            *currency_code,
        )
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
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_short(currency_preferences, &currency_code).unwrap();
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
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_compact_short(currency_preferences, &currency_code).unwrap();
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
    /// let currency_preferences = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_compact_long(currency_preferences, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12 thousand US dollars");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
    ) -> impl Writeable + Display + 'l {
        // TODO(#8146): Evaluate if FixedDecimal is the correct input type or if we should use
        // an exact decimal/money representation.
        // Per UTS #35 (LDML Part 3: Numbers, Section 3.8, Rule 3 in Compact Number Formatting), for uncompacted
        // fallback values below 1,000 ("0" pattern), significant and maximum fractional digits are adjusted by default
        // (typically stripping trailing zeros, e.g., "$12" rather than "$12.01"). Rather than gating precision application
        // at the type/trait level, currency fraction rounding is applied uniformly, relying on the underlying formatter
        // (such as CompactDecimalFormatter) to trim trailing fractional zeros and format according to magnitude.
        let (pattern, currency_str, formatted_value, sign) = match &self.currency_data {
            CurrencyFormatterData::Iso {
                essential,
                currency,
            } => {
                let pattern = essential.get().get_positive(true, true);
                // Note: we assume that all currency formatting variants within a locale
                // share the same fraction digits as the standard pattern (verified during datagen).
                let fraction_info = self.resolve_fraction_info(*currency, Some(essential.get()));
                let rounded_value = apply_precision(value.clone(), fraction_info);
                let formatted_value =
                    V::format_unsigned(&self.value_formatter, rounded_value.absolute);
                (
                    pattern,
                    currency.0.as_str(),
                    formatted_value,
                    rounded_value.sign,
                )
            }
            CurrencyFormatterData::Essential {
                essential,
                symbol,
                currency,
            } => {
                let symbol = symbol.get();
                let pattern = essential
                    .get()
                    .get_positive(symbol.starts_with_letter(), symbol.ends_with_letter());
                // Note: we assume that all currency formatting variants within a locale
                // share the same fraction digits as the standard pattern (verified during datagen).
                let fraction_info = self.resolve_fraction_info(*currency, Some(essential.get()));
                let rounded_value = apply_precision(value.clone(), fraction_info);
                let formatted_value =
                    V::format_unsigned(&self.value_formatter, rounded_value.absolute);
                (
                    pattern,
                    symbol.as_str(),
                    formatted_value,
                    rounded_value.sign,
                )
            }
            CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
                currency,
            } => {
                let fraction_info = self.resolve_fraction_info(*currency, None);
                let rounded_value = apply_precision(value.clone(), fraction_info);
                let formatted_value =
                    V::format_unsigned(&self.value_formatter, rounded_value.absolute);

                let operands = V::plural_operands(&formatted_value);
                let currency_str = extended
                    .as_ref()
                    .map(|ext| ext.get().get(operands, plural_rules))
                    .unwrap_or(currency.0.as_str());
                let pattern = patterns.get().get(operands, plural_rules);

                (pattern, currency_str, formatted_value, rounded_value.sign)
            }
        };

        // Per UTS #35 (LDML / TR35 Part 3: Numbers, Section 3.2.1), when a pattern does not specify an
        // explicit negative subpattern, the default negative format is formed by prepending the localized
        // minus sign to the entire positive pattern (e.g., `-¤#,##0` producing `-$12K`).
        // Therefore, `format_sign` is applied as the outermost wrapper around the glued currency string so
        // that the minus sign modifies the full monetary expression rather than just the numeric significand.
        V::format_sign(
            &self.value_formatter,
            pattern.interpolate((formatted_value, currency_str)),
            sign,
        )
    }

    /// Resolves the fraction/precision information for a given currency.
    ///
    /// The resolution follows a 3-step fallback:
    /// 1. Try currency-specific override in the global map (e.g., JPY = 0 decimals).
    /// 2. Try locale-specific standard fraction digits from `CurrencyEssentials` (if available).
    /// 3. Fallback to the global default.
    fn resolve_fraction_info(
        &self,
        currency_code: CurrencyCode,
        essentials: Option<&CurrencyEssentials>,
    ) -> FractionInfo {
        let iso_code = currency_code.0.to_unvalidated();

        // 1. Try currency-specific override in global map
        if let Some(ule) = self.fractions.get().fractions.get(&iso_code) {
            return zerovec::ule::AsULE::from_unaligned(*ule);
        }

        // 2. Try locale-specific standard fraction digits
        if let Some(essentials) = essentials {
            return FractionInfo {
                digits: essentials.standard_fractions,
                rounding: Rounding::R1,
                cash_digits: None,
                cash_rounding: None,
            };
        }

        // 3. Fallback to global default
        self.fractions.get().default
    }
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

pub(crate) fn apply_precision(value: FixedDecimal, fraction_info: FractionInfo) -> FixedDecimal {
    let precision = fraction_info.digits as i16;
    let rounding = fraction_info.rounding;

    let (magnitude, increment) = match rounding {
        Rounding::R50 => (-precision + 1, RoundingIncrement::MultiplesOf5),
        Rounding::R20 => (-precision + 1, RoundingIncrement::MultiplesOf2),
        Rounding::R5 => (-precision, RoundingIncrement::MultiplesOf5),
        Rounding::R1 => (-precision, RoundingIncrement::MultiplesOf1),
    };

    value.rounded_with_mode_and_increment(
        magnitude,
        SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
        increment,
    )
}
