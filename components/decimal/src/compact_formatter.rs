// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

use crate::{
    error::ExponentError, options::CompactDecimalFormatterOptions,
    preferences::CompactDecimalFormatterPreferences, provider::*, DecimalFormatter,
};
use alloc::borrow::Cow;
use fixed_decimal::{CompactDecimal, Decimal, UnsignedDecimal};
use icu_pattern::{Pattern, PatternBackend, SinglePlaceholder};
use icu_plurals::PluralRules;
use icu_provider::DataError;
use icu_provider::{marker::ErasedMarker, prelude::*};
use writeable::Writeable;

/// A formatter that renders locale-sensitive compact numbers.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #7161](https://github.com/unicode-org/icu4x/issues/7161).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
///
/// # Examples
///
/// ```
/// use icu::decimal::CompactDecimalFormatter;
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let short_french = CompactDecimalFormatter::try_new_short(
///    locale!("fr").into(),
///    Default::default(),
/// ).unwrap();
///
/// let [long_french, long_japanese, long_bangla] = [locale!("fr"), locale!("ja"), locale!("bn")]
///     .map(|locale| {
///         CompactDecimalFormatter::try_new_long(
///             locale.into(),
///             Default::default(),
///         )
///         .unwrap()
///     });
///
/// /// Supports short and long notations:
/// # // The following line contains U+00A0 NO-BREAK SPACE.
/// assert_writeable_eq!(short_french.format_fixed_decimal(&35_357_670i64.into()), "35 M");
/// assert_writeable_eq!(long_french.format_fixed_decimal(&35_357_670i64.into()), "35 millions");
/// /// The powers of ten used are locale-dependent:
/// assert_writeable_eq!(long_japanese.format_fixed_decimal(&3535_7670i64.into()), "3536万");
/// /// So are the digits:
/// assert_writeable_eq!(long_bangla.format_fixed_decimal(&3_53_57_670i64.into()), "৩.৫ কোটি");
///
/// /// The output does not always contain digits:
/// assert_writeable_eq!(long_french.format_fixed_decimal(&1000i64.into()), "mille");
/// ```
#[derive(Debug)]
pub struct CompactDecimalFormatter {
    plural_rules: PluralRules,
    decimal_formatter: DecimalFormatter,
    compact_data:
        DataPayload<ErasedMarker<<DecimalCompactLongV1 as DynamicDataMarker>::DataStruct>>,
}

impl CompactDecimalFormatter {
    /// Constructor that takes a selected locale and a list of preferences,
    /// then collects all compiled data necessary to format numbers in short compact
    /// decimal notation for the given locale.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::decimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    ///
    /// CompactDecimalFormatter::try_new_short(
    ///     locale!("sv").into(),
    ///     Default::default(),
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_short(
        prefs: CompactDecimalFormatterPreferences,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, DataError> {
        let locale = DecimalCompactShortV1::make_locale(prefs.locale_preferences);
        Ok(Self {
            decimal_formatter: DecimalFormatter::try_new(
                (&prefs).into(),
                options.decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal((&prefs).into())?,
            compact_data: DataProvider::<DecimalCompactShortV1>::load(
                &Baked,
                DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    ..Default::default()
                },
            )?
            .payload
            .cast(),
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CompactDecimalFormatterPreferences, options: CompactDecimalFormatterOptions) -> error: DataError,
        functions: [
            try_new_short: skip,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: CompactDecimalFormatterPreferences,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<DecimalCompactShortV1>
            + DataProvider<DecimalSymbolsV1>
            + DataProvider<DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>
            + ?Sized,
    {
        let locale = DecimalCompactShortV1::make_locale(prefs.locale_preferences);
        Ok(Self {
            decimal_formatter: DecimalFormatter::try_new_unstable(
                provider,
                (&prefs).into(),
                options.decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?,
            compact_data: DataProvider::<DecimalCompactShortV1>::load(
                provider,
                DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    ..Default::default()
                },
            )?
            .payload
            .cast(),
        })
    }

    /// Constructor that takes a selected locale and a list of preferences,
    /// then collects all compiled data necessary to format numbers in short compact
    /// decimal notation for the given locale.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::decimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    ///
    /// CompactDecimalFormatter::try_new_long(
    ///     locale!("sv").into(),
    ///     Default::default(),
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_long(
        prefs: CompactDecimalFormatterPreferences,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, DataError> {
        let locale = DecimalCompactLongV1::make_locale(prefs.locale_preferences);
        Ok(Self {
            decimal_formatter: DecimalFormatter::try_new(
                (&prefs).into(),
                options.decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal((&prefs).into())?,
            compact_data: DataProvider::<DecimalCompactLongV1>::load(
                &Baked,
                DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    ..Default::default()
                },
            )?
            .payload
            .cast(),
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CompactDecimalFormatterPreferences, options: CompactDecimalFormatterOptions) -> error: DataError,
        functions: [
            try_new_long: skip,
            try_new_long_with_buffer_provider,
            try_new_long_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_long)]
    pub fn try_new_long_unstable<D>(
        provider: &D,
        prefs: CompactDecimalFormatterPreferences,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<DecimalCompactLongV1>
            + DataProvider<DecimalSymbolsV1>
            + DataProvider<DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>
            + ?Sized,
    {
        let locale = DecimalCompactLongV1::make_locale(prefs.locale_preferences);
        Ok(Self {
            decimal_formatter: DecimalFormatter::try_new_unstable(
                provider,
                (&prefs).into(),
                options.decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?,
            compact_data: DataProvider::<DecimalCompactLongV1>::load(
                provider,
                DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    ..Default::default()
                },
            )?
            .payload
            .cast(),
        })
    }

    /// Formats a floating-point number in compact decimal notation using the default
    /// precision settings.
    ///
    /// The result may have a fractional digit only if it is compact and its
    /// significand is less than 10. Trailing fractional 0s are omitted, and
    /// a sign is shown only for negative values.
    ///
    /// ✨ *Enabled with the `ryu` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::decimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let short_english = CompactDecimalFormatter::try_new_short(
    ///     locale!("en").into(),
    ///     Default::default(),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(short_english.format_f64(0.0).unwrap(), "0");
    /// assert_writeable_eq!(short_english.format_f64(2.0).unwrap(), "2");
    /// assert_writeable_eq!(short_english.format_f64(843.0).unwrap(), "843");
    /// assert_writeable_eq!(short_english.format_f64(2207.0).unwrap(), "2.2K");
    /// assert_writeable_eq!(short_english.format_f64(15_127.0).unwrap(), "15K");
    /// assert_writeable_eq!(short_english.format_f64(3_010_349.0).unwrap(), "3M");
    /// assert_writeable_eq!(short_english.format_f64(-13_132.0).unwrap(), "-13K");
    /// ```
    ///
    /// The result is the nearest such compact number, with halfway cases-
    /// rounded towards the number with an even least significant digit.
    ///
    /// ```
    /// # use icu::decimal::CompactDecimalFormatter;
    /// # use icu::locale::locale;
    /// # use writeable::assert_writeable_eq;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short(
    /// #    locale!("en").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// assert_writeable_eq!(short_english.format_f64(999_499.99).unwrap(), "999K");
    /// assert_writeable_eq!(short_english.format_f64(999_500.00).unwrap(), "1M");
    /// assert_writeable_eq!(short_english.format_f64(1650.0).unwrap(), "1.6K");
    /// assert_writeable_eq!(short_english.format_f64(1750.0).unwrap(), "1.8K");
    /// assert_writeable_eq!(short_english.format_f64(1950.0).unwrap(), "2K");
    /// assert_writeable_eq!(
    ///     short_english.format_f64(-1_172_700.0).unwrap(),
    ///     "-1.2M"
    /// );
    /// ```
    #[cfg(feature = "ryu")]
    pub fn format_f64(
        &self,
        value: f64,
    ) -> Result<impl Writeable + Display + '_, fixed_decimal::LimitError> {
        use fixed_decimal::FloatPrecision::RoundTrip;
        // NOTE: This first gets the shortest representation of the f64, which
        // manifests as double rounding.
        let partly_rounded = Decimal::try_from_f64(value, RoundTrip)?;
        Ok(self.format_fixed_decimal(&partly_rounded))
    }

    /// Formats a [`Decimal`] by automatically scaling and rounding it.
    ///
    /// The result may have a fractional digit only if it is compact and its
    /// significand is less than 10. Trailing fractional 0s are omitted.
    ///
    /// Because the Decimal is mutated before formatting, this function
    /// takes ownership of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::Decimal;
    /// use icu::decimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let short_english = CompactDecimalFormatter::try_new_short(
    ///     locale!("en").into(),
    ///     Default::default(),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(0)),
    ///     "0"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(2)),
    ///     "2"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(843)),
    ///     "843"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(2207)),
    ///     "2.2K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(15127)),
    ///     "15K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(3010349)),
    ///     "3M"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&Decimal::from(-13132)),
    ///     "-13K"
    /// );
    ///
    /// // The sign display on the Decimal is respected:
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(
    ///         &Decimal::from(2500)
    ///             .with_sign_display(fixed_decimal::SignDisplay::ExceptZero)
    ///     ),
    ///     "+2.5K"
    /// );
    /// ```
    ///
    /// The result is the nearest such compact number, with halfway cases-
    /// rounded towards the number with an even least significant digit.
    ///
    /// ```
    /// # use icu::decimal::CompactDecimalFormatter;
    /// # use icu::locale::locale;
    /// # use writeable::assert_writeable_eq;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short(
    /// #    locale!("en").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"999499.99".parse().unwrap()),
    ///     "999K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"999500.00".parse().unwrap()),
    ///     "1M"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"1650".parse().unwrap()),
    ///     "1.6K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"1750".parse().unwrap()),
    ///     "1.8K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"1950".parse().unwrap()),
    ///     "2K"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"-1172700".parse().unwrap()),
    ///     "-1.2M"
    /// );
    /// assert_writeable_eq!(
    ///     short_english.format_fixed_decimal(&"0.2222".parse().unwrap()),
    ///     "0.22"
    /// );
    /// ```
    pub fn format_fixed_decimal(&self, value: &Decimal) -> impl Writeable + Display + '_ {
        let (compact_pattern, significand) = self
            .compact_data
            .get()
            .get_pattern_and_significand(&value.absolute, &self.plural_rules);

        self.decimal_formatter.format_sign(
            value.sign,
            compact_pattern
                .unwrap_or(Pattern::<SinglePlaceholder>::PASS_THROUGH)
                .interpolate([self
                    .decimal_formatter
                    .format_unsigned(Cow::Owned(significand))]),
        )
    }

    /// Formats a [`CompactDecimal`] object according to locale data.
    ///
    /// This is an advanced API; prefer using [`Self::format_fixed_decimal()`] in simple
    /// cases.
    ///
    /// Since the caller specifies the exact digits that are displayed, this
    /// allows for arbitrarily complex rounding rules.
    /// However, contrary to [`DecimalFormatter::format()`], this operation
    /// can fail, because the given [`CompactDecimal`] can be inconsistent with
    /// the locale data; for instance, if the locale uses lakhs and crores and
    /// millions are requested, or vice versa, this function returns an error.
    ///
    /// The given [`CompactDecimal`] should be constructed using
    /// [`Self::compact_exponent_for_magnitude()`] on the same
    /// [`CompactDecimalFormatter`] object.
    /// Specifically, `formatter.format_compact_decimal(n)` requires that `n.exponent()`
    /// be equal to `formatter.compact_exponent_for_magnitude(n.significand().nonzero_magnitude_start() + n.exponent())`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu::decimal::CompactDecimalFormatter;
    /// # use icu::locale::locale;
    /// # use writeable::assert_writeable_eq;
    /// # use std::str::FromStr;
    /// use fixed_decimal::CompactDecimal;
    ///
    /// # let short_french = CompactDecimalFormatter::try_new_short(
    /// #    locale!("fr").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// # let long_french = CompactDecimalFormatter::try_new_long(
    /// #    locale!("fr").into(),
    /// #    Default::default()
    /// # ).unwrap();
    /// # let long_bangla = CompactDecimalFormatter::try_new_long(
    /// #    locale!("bn").into(),
    /// #    Default::default()
    /// # ).unwrap();
    /// #
    /// let about_a_million = CompactDecimal::from_str("1.20c6").unwrap();
    /// let three_million = CompactDecimal::from_str("+3c6").unwrap();
    /// let ten_lakhs = CompactDecimal::from_str("10c5").unwrap();
    /// # // The following line contains U+00A0 NO-BREAK SPACE.
    /// assert_writeable_eq!(
    ///     short_french
    ///         .format_compact_decimal(&about_a_million)
    ///         .unwrap(),
    ///     "1,20 M"
    /// );
    /// assert_writeable_eq!(
    ///     long_french
    ///         .format_compact_decimal(&about_a_million)
    ///         .unwrap(),
    ///     "1,20 million"
    /// );
    ///
    /// # // The following line contains U+00A0 NO-BREAK SPACE.
    /// assert_writeable_eq!(
    ///     short_french.format_compact_decimal(&three_million).unwrap(),
    ///     "+3 M"
    /// );
    /// assert_writeable_eq!(
    ///     long_french.format_compact_decimal(&three_million).unwrap(),
    ///     "+3 millions"
    /// );
    ///
    /// assert_writeable_eq!(
    ///     long_bangla.format_compact_decimal(&ten_lakhs).unwrap(),
    ///     "১০ লাখ"
    /// );
    ///
    /// assert_eq!(
    ///     long_bangla
    ///         .format_compact_decimal(&about_a_million)
    ///         .err()
    ///         .unwrap()
    ///         .to_string(),
    ///     "Expected compact exponent 5 for 10^6, got 6",
    /// );
    /// assert_eq!(
    ///     long_french
    ///         .format_compact_decimal(&ten_lakhs)
    ///         .err()
    ///         .unwrap()
    ///         .to_string(),
    ///     "Expected compact exponent 6 for 10^6, got 5",
    /// );
    ///
    /// /// Some patterns omit the digits; in those cases, the output does not
    /// /// contain the sequence of digits specified by the CompactDecimal.
    /// let a_thousand = CompactDecimal::from_str("1c3").unwrap();
    /// assert_writeable_eq!(
    ///     long_french.format_compact_decimal(&a_thousand).unwrap(),
    ///     "mille"
    /// );
    /// ```
    pub fn format_compact_decimal<'l>(
        &'l self,
        value: &'l CompactDecimal,
    ) -> Result<impl Writeable + Display + 'l, ExponentError> {
        let log10_type =
            value.significand().absolute.nonzero_magnitude_start() + i16::from(value.exponent());

        let (pattern, expected_exponent) = self
            .compact_data
            .get()
            .0
            .iter()
            .filter(|&t| log10_type >= i16::from(t.sized))
            .last()
            .map(|t| {
                (
                    t.variable
                        .get((&value.significand().absolute).into(), &self.plural_rules)
                        .1,
                    t.sized - t.variable.get_default().0.get(),
                )
            })
            .unwrap_or((Pattern::<SinglePlaceholder>::PASS_THROUGH, 0));

        if value.exponent() != expected_exponent {
            return Err(ExponentError {
                actual: value.exponent(),
                expected: expected_exponent,
                log10_type,
            });
        }

        Ok(self.decimal_formatter.format_sign(
            value.significand().sign,
            pattern.interpolate([self
                .decimal_formatter
                .format_unsigned(Cow::Borrowed(&value.significand().absolute))]),
        ))
    }

    /// Returns the compact decimal exponent that should be used for a number of
    /// the given magnitude when using this formatter.
    ///
    /// # Examples
    /// ```
    /// use icu::decimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    ///
    /// let [long_french, long_japanese, long_bangla] = [
    ///     locale!("fr").into(),
    ///     locale!("ja").into(),
    ///     locale!("bn").into(),
    /// ]
    /// .map(|locale| {
    ///     CompactDecimalFormatter::try_new_long(locale, Default::default())
    ///         .unwrap()
    /// });
    /// /// French uses millions.
    /// assert_eq!(long_french.compact_exponent_for_magnitude(6), 6);
    /// /// Bangla uses lakhs.
    /// assert_eq!(long_bangla.compact_exponent_for_magnitude(6), 5);
    /// /// Japanese uses myriads.
    /// assert_eq!(long_japanese.compact_exponent_for_magnitude(6), 4);
    /// ```
    pub fn compact_exponent_for_magnitude(&self, magnitude: i16) -> u8 {
        self.compact_data
            .get()
            .0
            .iter()
            .filter(|t| magnitude >= i16::from(t.sized))
            .last()
            .map(|t| t.sized - t.variable.get_default().0.get())
            .unwrap_or_default()
    }
}

impl<'a, P: PatternBackend> CompactPatterns<'a, P> {
    /// Gets the compact pattern and significand for the given decimal
    pub fn get_pattern_and_significand(
        &'a self,
        value: &UnsignedDecimal,
        rules: &PluralRules,
    ) -> (Option<&'a Pattern<P>>, UnsignedDecimal) {
        let log10_type = value.nonzero_magnitude_start();

        let entry = self
            .0
            .iter()
            .enumerate()
            .filter(|&(_, t)| i16::from(t.sized) <= log10_type)
            .last();

        let exponent = entry
            .map(|(_, t)| t.sized - t.variable.get_default().0.get())
            .unwrap_or_default();

        let rounding_magnitude = if log10_type > i16::from(exponent) {
            // If we have at least 2 digits before the decimal point,
            // round to eliminate the fractional part.
            i16::from(exponent)
        } else {
            // …otherwise, round to two significant digits
            log10_type - 1
        };

        if let Some(t) = self
            .0
            .get(entry.map(|(idx, _)| idx + 1).unwrap_or_default())
        {
            let next_exponent = t.sized - t.variable.get_default().0.get();

            let rounds_to_next_exponent = log10_type + 1 == i16::from(next_exponent)
                && value.digit_at(rounding_magnitude - 1) >= 5
                && (rounding_magnitude..=log10_type).all(|m| value.digit_at(m) == 9);

            // We got bumped up a magnitude by rounding.
            if rounds_to_next_exponent {
                return (
                    Some(t.variable.get(1.into(), rules).1),
                    UnsignedDecimal::ONE,
                );
            }
        }

        let significand = value
            .clone()
            .rounded(rounding_magnitude)
            .multiplied_pow10(-i16::from(exponent))
            .trimmed_end();

        (
            entry.map(|(_, t)| t.variable.get((&significand).into(), rules).1),
            significand,
        )
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::GroupingStrategy;
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    #[allow(non_snake_case)]
    #[test]
    fn test_grouping() {
        // https://unicode-org.atlassian.net/browse/ICU-22254
        #[derive(Debug)]
        struct TestCase<'a> {
            short: bool,
            options: CompactDecimalFormatterOptions,
            expected1T: &'a str,
            expected10T: &'a str,
        }
        let cases = [
            TestCase {
                short: true,
                options: Default::default(),
                expected1T: "1000T",
                expected10T: "10,000T",
            },
            TestCase {
                short: true,
                options: GroupingStrategy::Always.into(),
                expected1T: "1,000T",
                expected10T: "10,000T",
            },
            TestCase {
                short: true,
                options: GroupingStrategy::Never.into(),
                expected1T: "1000T",
                expected10T: "10000T",
            },
            TestCase {
                short: false,
                options: Default::default(),
                expected1T: "1000 trillion",
                expected10T: "10,000 trillion",
            },
            TestCase {
                short: false,
                options: GroupingStrategy::Always.into(),
                expected1T: "1,000 trillion",
                expected10T: "10,000 trillion",
            },
            TestCase {
                short: false,
                options: GroupingStrategy::Never.into(),
                expected1T: "1000 trillion",
                expected10T: "10000 trillion",
            },
        ];
        for case in cases {
            let formatter = if case.short {
                CompactDecimalFormatter::try_new_short(locale!("en").into(), case.options.clone())
            } else {
                CompactDecimalFormatter::try_new_long(locale!("en").into(), case.options.clone())
            }
            .unwrap();
            let result1T = formatter.format_fixed_decimal(&1_000_000_000_000_000i64.into());
            assert_writeable_eq!(result1T, case.expected1T, "{:?}", case);
            let result10T = formatter.format_fixed_decimal(&10_000_000_000_000_000i64.into());
            assert_writeable_eq!(result10T, case.expected10T, "{:?}", case);
        }
    }

    #[test]
    fn regression_7387() {
        let formatter =
            CompactDecimalFormatter::try_new_short(locale!("ar").into(), Default::default())
                .unwrap();

        assert_writeable_eq!(
            formatter.format_fixed_decimal(&3_000_000i64.into()),
            "3\u{a0}مليون"
        );
    }
}
