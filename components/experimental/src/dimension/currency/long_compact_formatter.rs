// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use fixed_decimal::Decimal;
use icu_plurals::PluralRules;
use icu_provider::prelude::*;
use writeable::Writeable;

use crate::dimension::currency::compact_formatter::CompactCurrencyFormatterPreferences;
use crate::{
    compactdecimal::options::CompactDecimalFormatterOptions,
    compactdecimal::CompactDecimalFormatter,
    dimension::provider::currency::{
        extended::CurrencyExtendedDataV1, patterns::CurrencyPatternsDataV1,
    },
};

use super::CurrencyCode;

extern crate alloc;

/// A formatter for monetary values.
///
/// [`LongCompactCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
#[derive(Debug)]
pub struct LongCompactCurrencyFormatter {
    /// Extended data for the currency formatter.
    extended: DataPayload<CurrencyExtendedDataV1>,

    /// Formatting patterns for each currency plural category.
    patterns: DataPayload<CurrencyPatternsDataV1>,

    /// A [`CompactDecimalFormatter`] to format the currency value in compact form.
    compact_decimal_formatter: CompactDecimalFormatter,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,
}

impl LongCompactCurrencyFormatter {
    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CompactCurrencyFormatterPreferences,
            currency_code: &CurrencyCode
        ) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`LongCompactCurrencyFormatter`] from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: CompactCurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        let compact_decimal_formatter = CompactDecimalFormatter::try_new_long(
            (&prefs).into(),
            CompactDecimalFormatterOptions::default(),
        )?;

        let marker_attributes = DataMarkerAttributes::try_from_str(currency_code.0.as_str())
            .map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;

        let locale = &CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);

        let extended = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            extended,
            patterns,
            compact_decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: CompactCurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyExtendedDataV1>
            + DataProvider<CurrencyPatternsDataV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>
            + DataProvider<crate::compactdecimal::provider::DecimalCompactLongV1>,
    {
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);

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

        let compact_decimal_formatter = CompactDecimalFormatter::try_new_long_unstable(
            provider,
            (&prefs).into(),
            CompactDecimalFormatterOptions::default(),
        )?;

        Ok(Self {
            extended,
            patterns,
            compact_decimal_formatter,
            plural_rules,
        })
    }

    /// Formats in the long format a [`Decimal`] value for the given currency code.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::long_compact_formatter::LongCompactCurrencyFormatter;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_prefs = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = LongCompactCurrencyFormatter::try_new(currency_prefs, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "12 thousand US dollars");
    /// ```
    pub fn format_fixed_decimal<'l>(&'l self, value: &'l Decimal) -> impl Writeable + Display + 'l {
        let operands = value.into();

        let display_name = self
            .extended
            .get()
            .display_names
            .get(operands, &self.plural_rules);

        let pattern = self
            .patterns
            .get()
            .patterns
            .get(operands, &self.plural_rules);

        let (compact_pattern, significand) = self
            .compact_decimal_formatter
            .compact_data
            .get()
            .get_pattern_and_significand(
                &value.absolute,
                &self.compact_decimal_formatter.plural_rules,
            );

        self.compact_decimal_formatter
            .decimal_formatter
            .format_sign(
                value.sign,
                pattern.interpolate((
                    compact_pattern
                        .unwrap_or(icu_pattern::SinglePlaceholderPattern::PASS_THROUGH)
                        .interpolate([self
                            .compact_decimal_formatter
                            .decimal_formatter
                            .format_unsigned(icu_decimal::Cow::Owned(significand))]),
                    display_name,
                )),
            )
    }
}
