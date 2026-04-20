// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use fixed_decimal::{Decimal, Sign};
use icu_decimal::{
    options::DecimalFormatterOptions, DecimalFormatter, DecimalFormatterPreferences,
};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRulesPreferences;
use icu_provider::prelude::*;
use writeable::Writeable;

use super::super::provider::currency::essentials::CurrencyEssentialsV1;
#[cfg(feature = "compiled_data")]
use super::super::provider::currency::essentials::outer_literal_affixes_double_placeholder;
use super::options::{CurrencyDisplaySign, CurrencyFormatterOptions};
use super::CurrencyCode;

extern crate alloc;

#[cfg(feature = "compiled_data")]
use alloc::string::String;

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

/// A formatter for monetary values.
///
/// [`CurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
#[derive(Debug)]
pub struct CurrencyFormatter {
    /// Options bag for the currency formatter to determine the behavior of the formatter.
    /// for example: currency width.
    options: CurrencyFormatterOptions,

    /// Essential data for the currency formatter.
    essential: DataPayload<CurrencyEssentialsV1>,

    /// A [`DecimalFormatter`] to format the currency value.
    decimal_formatter: DecimalFormatter,
}

impl CurrencyFormatter {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, options: CurrencyFormatterOptions) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: CurrencyFormatterPreferences,
        options: CurrencyFormatterOptions,
    ) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_formatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            options,
            essential,
            decimal_formatter,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        options: CurrencyFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            (&prefs).into(),
            DecimalFormatterOptions::default(),
        )?;
        let essential = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            options,
            essential,
            decimal_formatter,
        })
    }

    /// Returns whether a **negative** amount is formatted using a CLDR negative subpattern
    /// (parentheses or similar) for the current [`CurrencyFormatterOptions`], rather than a
    /// leading minus on the number.
    pub fn negative_sign_encoded_in_pattern(&self, currency_code: &CurrencyCode) -> bool {
        self.essential
            .get()
            .resolve_currency_pattern(
                self.options.width,
                currency_code,
                self.options.currency_display_sign,
                true,
            )
            .sign_encoded_in_pattern
    }

    /// When `currencySign=accounting`, `value` is negative, and the resolved CLDR
    /// pattern encodes the sign (e.g. parentheses), returns the outer literal
    /// prefix and suffix around the numeric amount — for composing
    /// `currencyDisplay=code` / `never` amounts with the same accounting shell as
    /// short currency formatting.
    #[cfg(feature = "compiled_data")]
    pub(crate) fn accounting_outer_affixes_if_encoded(
        &self,
        currency_code: &CurrencyCode,
        value: &Decimal,
    ) -> Option<(String, String)> {
        if !matches!(
            self.options.currency_display_sign,
            CurrencyDisplaySign::Accounting
        ) {
            return None;
        }
        if value.sign() != Sign::Negative {
            return None;
        }
        let resolved = self.essential.get().resolve_currency_pattern(
            self.options.width,
            currency_code,
            self.options.currency_display_sign,
            true,
        );
        if !resolved.sign_encoded_in_pattern {
            return None;
        }
        Some(outer_literal_affixes_double_placeholder(resolved.pattern))
    }

    /// Formats a [`Decimal`] value for the given currency code.
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
    /// let fmt = CurrencyFormatter::try_new(locale, Default::default()).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// assert_writeable_eq!(
    ///     fmt.format_fixed_decimal(&value, &currency_code),
    ///     "$12,345.67"
    /// );
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l Decimal,
        currency_code: &'l CurrencyCode,
    ) -> impl Writeable + Display + 'l {
        let value_is_negative_for_resolve = matches!(
            self.options.currency_display_sign,
            CurrencyDisplaySign::Accounting
        ) && value.sign() == Sign::Negative;
        let resolved = self.essential.get().resolve_currency_pattern(
            self.options.width,
            currency_code,
            self.options.currency_display_sign,
            value_is_negative_for_resolve,
        );
        let output_sign = if resolved.sign_encoded_in_pattern {
            Sign::None
        } else {
            value.sign()
        };
        self.decimal_formatter.format_sign(
            output_sign,
            resolved.pattern.interpolate((
                self.decimal_formatter
                    .format_unsigned(icu_decimal::Cow::Borrowed(&value.absolute)),
                resolved.currency,
            )),
        )
    }
}
