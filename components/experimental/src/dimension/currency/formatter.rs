// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use fixed_decimal::Decimal as FixedDecimal;
use icu_decimal::preferences::CompactDecimalFormatterPreferences;
use icu_decimal::{AbstractFormatter, DecimalFormatter, DecimalFormatterPreferences};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::{PluralRules, PluralRulesPreferences};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::Writeable;

use super::super::provider::currency::{
    essentials::CurrencyEssentialsV1, extended::CurrencyExtendedDataV1,
    patterns::CurrencyPatternsDataV1, symbols::CurrencySymbolsV1,
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
    Iso {
        essential: DataPayload<CurrencyEssentialsV1>,
        currency: CurrencyCode,
    },
    Essential {
        essential: DataPayload<CurrencyEssentialsV1>,
        symbol: DataPayload<CurrencySymbolsV1>,
    },
    Long {
        extended: DataPayload<CurrencyExtendedDataV1>,
        patterns: DataPayload<CurrencyPatternsDataV1>,
        plural_rules: PluralRules,
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
            },
            None => CurrencyFormatterData::Iso {
                essential,
                currency,
            },
        };

        Ok(Self {
            value_formatter,
            currency_data,
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
        D: ?Sized + DataProvider<CurrencyEssentialsV1> + DataProvider<CurrencySymbolsV1>,
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
            },
            None => CurrencyFormatterData::Iso {
                essential,
                currency,
            },
        };

        Ok(Self {
            value_formatter,
            currency_data,
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
        let extended = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            },
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
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let marker_attributes =
            DataMarkerAttributes::try_from_str(currency.0.as_str()).map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;
        let extended = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = provider.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        Ok(Self {
            value_formatter,
            currency_data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            },
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

    // We manually implement the compiled constructors because of the cross-crate dependency
    // on `icu_decimal` markers (which are not present in `icu_experimental`'s local `Baked` provider).
    // TODO: When CurrencyFormatter is migrated out of experimental, check if we can use the
    // macro-generated versions instead of these manual implementations.

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

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_long: skip,
            try_new_long_with_buffer_provider,
            try_new_long_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for long formatting from compiled locale data.
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
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::try_new_short(locale, &currency_code).unwrap();
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
    /// let fmt = CurrencyFormatter::try_new_compact_short(locale, &currency_code).unwrap();
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
    /// let fmt = CurrencyFormatter::try_new_compact_long(currency_prefs, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12 thousand US dollars");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
    ) -> impl Writeable + Display + 'l {
        let formatted_value = V::format_unsigned(&self.value_formatter, &value.absolute);

        // TODO(#8146): Evaluate if FixedDecimal is the correct input type or if we should use
        // an exact decimal/money representation.
        let (pattern, currency_str) = match &self.currency_data {
            CurrencyFormatterData::Iso {
                essential,
                currency,
            } => {
                let pattern = essential.get().get_positive(true, true);
                (pattern, currency.0.as_str())
            }
            CurrencyFormatterData::Essential { essential, symbol } => {
                let symbol = symbol.get();
                let pattern = essential
                    .get()
                    .get_positive(symbol.starts_with_letter(), symbol.ends_with_letter());
                (pattern, symbol.as_str())
            }
            CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            } => {
                let operands = V::plural_operands(&formatted_value);
                let currency_str = extended.get().get(operands, plural_rules);
                let pattern = patterns.get().get(operands, plural_rules);
                (pattern, currency_str)
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
            value.sign,
        )
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
