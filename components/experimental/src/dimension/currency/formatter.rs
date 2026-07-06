// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;
use core::marker::PhantomData;

use fixed_decimal::Decimal as FixedDecimal;
use icu_decimal::{
    DecimalFormatter, DecimalFormatterPreferences, options::DecimalFormatterOptions,
};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::{PluralRules, PluralRulesPreferences};
use icu_provider::prelude::*;
use writeable::Writeable;

use super::super::provider::currency::{
    essentials::CurrencyEssentialsV1, extended::CurrencyExtendedDataV1,
    patterns::CurrencyPatternsDataV1,
};
use super::CurrencyCode;
use super::options::Width;
use icu_pattern::DoublePlaceholderPattern;

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

/// A trait for value representation in currency formatting.
pub trait ValueRepresentation {}

/// Representation for decimal currency formatting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Decimal;

impl ValueRepresentation for Decimal {}

#[derive(Debug)]
enum CurrencyFormatterData {
    Essential {
        essential: DataPayload<CurrencyEssentialsV1>,
        width: Width,
        currency: CurrencyCode,
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
pub struct CurrencyFormatter<V: ValueRepresentation> {
    /// A [`DecimalFormatter`] to format the currency value.
    decimal_formatter: DecimalFormatter,

    data: CurrencyFormatterData,

    _marker: PhantomData<V>,
}

impl CurrencyFormatter<Decimal> {
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
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        // TODO: We should depend on the currency format patterns directly and not depend on
        // the decimal formatter. If we do use the decimal formatter, we need to take care of
        // synchronization of different numbering systems (e.g. if DecimalFormatter falls back
        // to a different numbering system than the one resolved for CurrencyFormatter).
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter =
            DecimalFormatter::try_new(decimal_prefs, DecimalFormatterOptions::default())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Essential {
                essential,
                width: Width::Short,
                currency: *currency_code,
            },
            _marker: PhantomData,
        })
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
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter =
            DecimalFormatter::try_new(decimal_prefs, DecimalFormatterOptions::default())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Essential {
                essential,
                width: Width::Narrow,
                currency: *currency_code,
            },
            _marker: PhantomData,
        })
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
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        // TODO: We should depend on the currency format patterns directly and not depend on
        // the decimal formatter. If we do use the decimal formatter, we need to take care of
        // synchronization of different numbering systems (e.g. if DecimalFormatter falls back
        // to a different numbering system than the one resolved for CurrencyFormatter).
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            decimal_prefs,
            DecimalFormatterOptions::default(),
        )?;
        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Essential {
                essential,
                width: Width::Short,
                currency: *currency_code,
            },
            _marker: PhantomData,
        })
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
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            decimal_prefs,
            DecimalFormatterOptions::default(),
        )?;
        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Essential {
                essential,
                width: Width::Narrow,
                currency: *currency_code,
            },
            _marker: PhantomData,
        })
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
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let decimal_formatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;

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
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            },
            _marker: PhantomData,
        })
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
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            (&prefs).into(),
            DecimalFormatterOptions::default(),
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
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = provider.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        Ok(Self {
            decimal_formatter,
            data: CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    /// Formats a [`FixedDecimal`] value.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::{CurrencyFormatter, Decimal};
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::<Decimal>::try_new_short(locale, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(
    ///     fmt.format_fixed_decimal(&value),
    ///     "$12,345.67"
    /// );
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
    ) -> impl Writeable + Display + 'l {
        // TODO(#8146): Evaluate if FixedDecimal is the correct input type or if we should use
        // an exact decimal/money representation.
        let (pattern, currency_str) = match &self.data {
            CurrencyFormatterData::Essential {
                essential,
                width,
                currency,
            } => {
                // TODO(#6064): Support plural-specific patterns and full currency formatting spec.
                let (currency_str, pattern, _pattern_selection) =
                    essential.get().name_and_pattern(*width, currency);

                let pattern = pattern.unwrap_or_else(|| <&DoublePlaceholderPattern>::default());
                (pattern, currency_str)
            }
            CurrencyFormatterData::Long {
                extended,
                patterns,
                plural_rules,
            } => {
                let operands = value.into();
                let currency_str = extended.get().display_names.get(operands, plural_rules);
                let pattern = patterns.get().patterns.get(operands, plural_rules);
                (pattern, currency_str)
            }
        };

        self.decimal_formatter.format_sign(
            value.sign,
            pattern.interpolate((
                self.decimal_formatter
                    .format_unsigned(icu_decimal::Cow::Borrowed(&value.absolute)),
                currency_str,
            )),
        )
    }
}

// TODO: Discuss reusing the `load_with_fallback` helper from `icu_decimal`
// (or moving it to a shared location) instead of duplicating it here.
fn load_with_fallback<'a, M: DataMarker>(
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
