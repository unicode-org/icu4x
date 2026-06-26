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
use icu_plurals::PluralRulesPreferences;
use icu_provider::prelude::*;
use writeable::Writeable;

use super::super::provider::currency::essentials::CurrencyEssentialsV1;
use super::CurrencyCode;
use super::options::{CurrencyFormatterOptions, Width};
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

/// A formatter for monetary values.
///
/// [`CurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
#[derive(Debug)]
pub struct CurrencyFormatter<V: ValueRepresentation> {
    /// Options bag for the currency formatter.
    ///
    /// Internal options (such as the currency width) are set automatically
    /// by the constructors (e.g., `try_new_short` sets the width to `Short`).
    options: CurrencyFormatterOptions,

    /// Essential data for the currency formatter.
    essential: DataPayload<CurrencyEssentialsV1>,

    /// A [`DecimalFormatter`] to format the currency value.
    decimal_formatter: DecimalFormatter,

    _marker: PhantomData<V>,
}

impl CurrencyFormatter<Decimal> {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences) -> error: DataError,
        functions: [
            try_new_short: skip,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences) -> error: DataError,
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
    pub fn try_new_short(prefs: CurrencyFormatterPreferences) -> Result<Self, DataError> {
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
            options: Width::Short.into(),
            essential,
            decimal_formatter,
            _marker: PhantomData,
        })
    }

    /// Creates a new [`CurrencyFormatter`] for narrow formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_narrow(prefs: CurrencyFormatterPreferences) -> Result<Self, DataError> {
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
            options: Width::Narrow.into(),
            essential,
            decimal_formatter,
            _marker: PhantomData,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
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
            options: Width::Short.into(),
            essential,
            decimal_formatter,
            _marker: PhantomData,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_narrow)]
    pub fn try_new_narrow_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
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
            options: Width::Narrow.into(),
            essential,
            decimal_formatter,
            _marker: PhantomData,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given currency code.
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
    /// let fmt = CurrencyFormatter::<Decimal>::try_new_short(locale).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// assert_writeable_eq!(
    ///     fmt.format_fixed_decimal(&value, &currency_code),
    ///     "$12,345.67"
    /// );
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        currency_code: &'l CurrencyCode,
    ) -> impl Writeable + Display + 'l {
        // TODO(#6064): Support plural-specific patterns and full currency formatting spec.
        let (currency_str, pattern, _pattern_selection) = self
            .essential
            .get()
            .name_and_pattern(self.options.width, currency_code);

        let pattern = pattern.unwrap_or_else(|| <&DoublePlaceholderPattern>::default());

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
