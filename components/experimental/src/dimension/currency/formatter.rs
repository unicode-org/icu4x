// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use fixed_decimal::Decimal;
use icu_decimal::{
    DecimalFormatter, DecimalFormatterPreferences, options::DecimalFormatterOptions,
};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRulesPreferences;
use icu_provider::prelude::*;
use writeable::Writeable;

use super::super::provider::currency::essentials::CurrencyEssentialsV1;
use super::CurrencyCode;
use super::options::CurrencyFormatterOptions;

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
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let symbols_payload: DataPayload<icu_decimal::provider::DecimalSymbolsV1> =
            essential.clone().map_project(
                |ess: crate::dimension::provider::currency::essentials::CurrencyEssentials<'_>,
                 _| ess.decimal_symbols.clone(),
            );

        let numsys = symbols_payload.get().numsys();
        let digits = icu_decimal::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic(numsys),
                ),
                ..Default::default()
            })?
            .payload;

        let decimal_formatter = DecimalFormatter::try_new_from_payloads(
            DecimalFormatterOptions::default(),
            symbols_payload,
            digits,
        )?;

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
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let req_numsys = prefs.numbering_system.as_ref().map(|s| s.as_str());
        let essential = req_numsys
            .and_then(|nu| {
                provider
                    .load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::from_str_or_panic(nu),
                            &locale,
                        ),
                        metadata: {
                            let mut m = DataRequestMetadata::default();
                            m.silent = true;
                            m
                        },
                    })
                    .allow_identifier_not_found()
                    .ok()
                    .flatten()
            })
            .map(Ok)
            .unwrap_or_else(|| {
                provider.load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    ..Default::default()
                })
            })?
            .payload;

        let symbols_payload: DataPayload<icu_decimal::provider::DecimalSymbolsV1> =
            essential.clone().map_project(
                |ess: crate::dimension::provider::currency::essentials::CurrencyEssentials<'_>,
                 _| ess.decimal_symbols.clone(),
            );

        let default_numsys = symbols_payload.get().numsys();
        let req_numsys = prefs
            .numbering_system
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(default_numsys);

        let digits = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic(req_numsys),
                ),
                ..Default::default()
            })
            .or_else(|_| {
                provider.load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes(
                        DataMarkerAttributes::from_str_or_panic(default_numsys),
                    ),
                    ..Default::default()
                })
            })?
            .payload;

        let decimal_formatter = DecimalFormatter::try_new_from_payloads(
            DecimalFormatterOptions::default(),
            symbols_payload,
            digits,
        )?;

        Ok(Self {
            options,
            essential,
            decimal_formatter,
        })
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
        let (currency_str, pattern, _pattern_selection) = self
            .essential
            .get()
            .name_and_pattern(self.options.width, currency_code);

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
