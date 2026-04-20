// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use fixed_decimal::{Decimal, Sign};
use icu_decimal::DecimalFormatter;
use icu_plurals::PluralRules;
use icu_provider::prelude::*;
use writeable::adapters::Concat;
use writeable::Writeable;

use crate::dimension::currency::compact_formatter::CompactCurrencyFormatterPreferences;
use crate::dimension::provider::currency::{
    essentials::{outer_literal_affixes_double_placeholder, CurrencyEssentialsV1},
    extended::CurrencyExtendedDataV1,
    patterns::CurrencyPatternsDataV1,
};

use super::{
    options::{CurrencyDisplaySign, LongCurrencyFormatterOptions, Width},
    CurrencyCode,
};

extern crate alloc;

use alloc::string::String;

/// A formatter for monetary values.
///
/// [`LongCompactCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
#[derive(Debug)]
pub struct LongCompactCurrencyFormatter {
    currency_code: CurrencyCode,

    /// Extended data for the currency formatter.
    extended: DataPayload<CurrencyExtendedDataV1>,

    /// Formatting patterns for each currency plural category.
    patterns: DataPayload<CurrencyPatternsDataV1>,

    essential: DataPayload<CurrencyEssentialsV1>,

    decimal_formatter: DecimalFormatter,

    compact_data: DataPayload<icu_decimal::provider::DecimalCompactLongV1>,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,

    options: LongCurrencyFormatterOptions,
}

impl LongCompactCurrencyFormatter {
    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CompactCurrencyFormatterPreferences,
            currency_code: &CurrencyCode,
            options: LongCurrencyFormatterOptions
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
        options: LongCurrencyFormatterOptions,
    ) -> Result<Self, DataError> {
        let decimal_formatter = DecimalFormatter::try_new((&prefs).into(), Default::default())?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactLongV1>::load(
            &icu_decimal::provider::Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(
                    &icu_decimal::provider::DecimalCompactLongV1::make_locale(
                        prefs.locale_preferences,
                    ),
                ),
                ..Default::default()
            },
        )?
        .payload
        .cast();

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

        let essential_locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&essential_locale),
                ..Default::default()
            })?
            .payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            currency_code: *currency_code,
            extended,
            patterns,
            essential,
            decimal_formatter,
            compact_data,
            plural_rules,
            options,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: CompactCurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
        options: LongCurrencyFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyExtendedDataV1>
            + DataProvider<CurrencyPatternsDataV1>
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>
            + DataProvider<icu_decimal::provider::DecimalCompactLongV1>,
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

        let essential_locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let essential = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&essential_locale),
                ..Default::default()
            })?
            .payload;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        let decimal_formatter =
            DecimalFormatter::try_new_unstable(provider, (&prefs).into(), Default::default())?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactLongV1>::load(
            provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(
                    &icu_decimal::provider::DecimalCompactLongV1::make_locale(
                        prefs.locale_preferences,
                    ),
                ),
                ..Default::default()
            },
        )?
        .payload
        .cast();

        Ok(Self {
            currency_code: *currency_code,
            extended,
            patterns,
            essential,
            decimal_formatter,
            compact_data,
            plural_rules,
            options,
        })
    }

    /// Formats in the long format a [`Decimal`] value for the given currency code.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::long_compact_formatter::LongCompactCurrencyFormatter;
    /// use icu::experimental::dimension::currency::options::LongCurrencyFormatterOptions;
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let currency_prefs = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = LongCompactCurrencyFormatter::try_new(
    ///     currency_prefs,
    ///     &currency_code,
    ///     LongCurrencyFormatterOptions::default(),
    /// )
    /// .unwrap();
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
            .compact_data
            .get()
            .get_pattern_and_significand(&value.absolute, &self.plural_rules);

        let inner = pattern.interpolate((
            compact_pattern
                .unwrap_or(icu_pattern::SinglePlaceholderPattern::PASS_THROUGH)
                .interpolate([self
                    .decimal_formatter
                    .format_unsigned(icu_decimal::Cow::Owned(significand))]),
            display_name,
        ));

        let (prefix, suffix, output_sign) = if matches!(
            self.options.currency_display_sign,
            CurrencyDisplaySign::Accounting
        ) && value.sign() == Sign::Negative
        {
            let resolved = self.essential.get().resolve_currency_pattern(
                Width::Short,
                &self.currency_code,
                self.options.currency_display_sign,
                true,
            );
            let (prefix, suffix) = if resolved.sign_encoded_in_pattern {
                outer_literal_affixes_double_placeholder(resolved.pattern)
            } else {
                (String::new(), String::new())
            };
            let output_sign = if resolved.sign_encoded_in_pattern {
                Sign::None
            } else {
                value.sign()
            };
            (prefix, suffix, output_sign)
        } else {
            (String::new(), String::new(), value.sign())
        };

        let wrapped = Concat(Concat(prefix, inner), suffix);
        self.decimal_formatter.format_sign(output_sign, wrapped)
    }
}
