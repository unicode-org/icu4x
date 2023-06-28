// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use core::convert::TryFrom;
use fixed_decimal::{CompactDecimal, FixedDecimal};
use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    FixedDecimalFormatter,
};
use icu_plurals::PluralRules;
use icu_provider::prelude::*;
use zerovec::maps::ZeroMap2dCursor;

use crate::{
    format::FormattedCompactDecimal,
    provider::{
        Count, ErasedCompactDecimalFormatDataV1Marker, LongCompactDecimalFormatDataV1Marker,
        PatternULE, ShortCompactDecimalFormatDataV1Marker,
    },
    CompactDecimalError,
};

/// A bag of options defining how numbers will be formatted by
/// [`CompactDecimalFormatter`](crate::CompactDecimalFormatter).
#[derive(Debug, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub struct CompactDecimalFormatterOptions {
    /// Options to configure the inner [`FixedDecimalFormatter`].
    pub fixed_decimal_formatter_options: FixedDecimalFormatterOptions,
}

impl Default for CompactDecimalFormatterOptions {
    fn default() -> Self {
        Self {
            fixed_decimal_formatter_options: GroupingStrategy::Min2.into(),
        }
    }
}

impl From<FixedDecimalFormatterOptions> for CompactDecimalFormatterOptions {
    fn from(fixed_decimal_formatter_options: FixedDecimalFormatterOptions) -> Self {
        Self {
            fixed_decimal_formatter_options,
        }
    }
}

impl From<GroupingStrategy> for CompactDecimalFormatterOptions {
    fn from(grouping_strategy: GroupingStrategy) -> Self {
        Self {
            fixed_decimal_formatter_options: grouping_strategy.into(),
        }
    }
}

/// A formatter that renders locale-sensitive compact numbers.
///
/// # Examples
///
/// ```
/// use icu_compactdecimal::CompactDecimalFormatter;
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let short_french = CompactDecimalFormatter::try_new_short(
///    &locale!("fr").into(),
///    Default::default(),
/// ).unwrap();
///
/// let [long_french, long_japanese, long_bangla] = [locale!("fr"), locale!("ja"), locale!("bn")]
///     .map(|locale| {
///         CompactDecimalFormatter::try_new_long(
///             &locale.into(),
///             Default::default(),
///         )
///         .unwrap()
///     });
///
/// /// Supports short and long notations:
/// # // The following line contains U+00A0 NO-BREAK SPACE.
/// assert_writeable_eq!(short_french.format_i64(35_357_670), "35 M");
/// assert_writeable_eq!(long_french.format_i64(35_357_670), "35 millions");
/// /// The powers of ten used are locale-dependent:
/// assert_writeable_eq!(long_japanese.format_i64(3535_7670), "3536万");
/// /// So are the digits:
/// assert_writeable_eq!(long_bangla.format_i64(3_53_57_670), "৩.৫ কোটি");
///
/// /// The output does not always contain digits:
/// assert_writeable_eq!(long_french.format_i64(1000), "mille");
/// ```
#[derive(Debug)]
pub struct CompactDecimalFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) fixed_decimal_format: FixedDecimalFormatter,
    pub(crate) compact_data: DataPayload<ErasedCompactDecimalFormatDataV1Marker>,
}

impl CompactDecimalFormatter {
    /// Constructor that takes a selected locale and a list of preferences,
    /// then collects all data necessary to format numbers in short compact
    /// decimal notation for the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_short(
    ///     &locale!("sv").into(),
    ///     Default::default(),
    /// );
    /// ```
    #[cfg(feature = "data")]
    pub fn try_new_short(
        locale: &DataLocale,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, CompactDecimalError> {
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new(
                locale,
                options.fixed_decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal(locale)?,
            compact_data: DataProvider::<ShortCompactDecimalFormatDataV1Marker>::load(
                &crate::provider::Baked,
                DataRequest {
                    locale,
                    metadata: Default::default(),
                },
            )?
            .take_payload()?
            .cast(),
        })
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: CompactDecimalFormatterOptions,
        error: CompactDecimalError,
        #[cfg(skip_new)]
        functions: [
            try_new_short,
            try_new_short_with_any_provider,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, CompactDecimalError>
    where
        D: DataProvider<ShortCompactDecimalFormatDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>
            + ?Sized,
    {
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                options.fixed_decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal_unstable(data_provider, locale)?,
            compact_data: DataProvider::<ShortCompactDecimalFormatDataV1Marker>::load(
                data_provider,
                DataRequest {
                    locale,
                    metadata: Default::default(),
                },
            )?
            .take_payload()?
            .cast(),
        })
    }

    /// Constructor that takes a selected locale and a list of preferences,
    /// then collects all data necessary to format numbers in short compact
    /// decimal notation for the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_long(
    ///     &locale!("sv").into(),
    ///     Default::default(),
    /// );
    /// ```
    #[cfg(feature = "data")]
    pub fn try_new_long(
        locale: &DataLocale,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, CompactDecimalError> {
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new(
                locale,
                options.fixed_decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal(locale)?,
            compact_data: DataProvider::<LongCompactDecimalFormatDataV1Marker>::load(
                &crate::provider::Baked,
                DataRequest {
                    locale,
                    metadata: Default::default(),
                },
            )?
            .take_payload()?
            .cast(),
        })
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: CompactDecimalFormatterOptions,
        error: CompactDecimalError,
        #[cfg(skip)]
        functions: [
            try_new_long,
            try_new_long_with_any_provider,
            try_new_long_with_buffer_provider,
            try_new_long_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_long)]
    pub fn try_new_long_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: CompactDecimalFormatterOptions,
    ) -> Result<Self, CompactDecimalError>
    where
        D: DataProvider<LongCompactDecimalFormatDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>
            + ?Sized,
    {
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                options.fixed_decimal_formatter_options,
            )?,
            plural_rules: PluralRules::try_new_cardinal_unstable(data_provider, locale)?,
            compact_data: DataProvider::<LongCompactDecimalFormatDataV1Marker>::load(
                data_provider,
                DataRequest {
                    locale,
                    metadata: Default::default(),
                },
            )?
            .take_payload()?
            .cast(),
        })
    }

    /// Formats an integer in compact decimal notation using the default
    /// precision settings.
    ///
    /// The result may have a fractional digit only if it is compact and its
    /// significand is less than 10. Trailing fractional 0s are omitted, and
    /// a sign is shown only for negative values.
    /// ```
    /// # use icu_compactdecimal::CompactDecimalFormatter;
    /// # use icu_locid::locale;
    /// # use writeable::assert_writeable_eq;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short(
    /// #    &locale!("en").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// assert_writeable_eq!(short_english.format_i64(0), "0");
    /// assert_writeable_eq!(short_english.format_i64(2), "2");
    /// assert_writeable_eq!(short_english.format_i64(843), "843");
    /// assert_writeable_eq!(short_english.format_i64(2207), "2.2K");
    /// assert_writeable_eq!(short_english.format_i64(15_127), "15K");
    /// assert_writeable_eq!(short_english.format_i64(3_010_349), "3M");
    /// assert_writeable_eq!(short_english.format_i64(-13_132), "-13K");
    /// ```
    /// The result is the nearest such compact number, with halfway cases-
    /// rounded towards the number with an even least significant digit.
    /// ```
    /// # use icu_compactdecimal::CompactDecimalFormatter;
    /// # use icu_locid::locale;
    /// # use writeable::assert_writeable_eq;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short(
    /// #    &locale!("en").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// assert_writeable_eq!(short_english.format_i64(999_499), "999K");
    /// assert_writeable_eq!(short_english.format_i64(999_500), "1M");
    /// assert_writeable_eq!(short_english.format_i64(1650), "1.6K");
    /// assert_writeable_eq!(short_english.format_i64(1750), "1.8K");
    /// assert_writeable_eq!(short_english.format_i64(1950), "2K");
    /// assert_writeable_eq!(short_english.format_i64(-1_172_700), "-1.2M");
    /// ```
    pub fn format_i64(&self, value: i64) -> FormattedCompactDecimal<'_> {
        let unrounded = FixedDecimal::from(value);
        let log10_type = unrounded.nonzero_magnitude_start();
        let (mut plural_map, mut exponent) = self.plural_map_and_exponent_for_magnitude(log10_type);
        let mut significand = unrounded.multiplied_pow10(-i16::from(exponent));
        // If we have just one digit before the decimal point…
        if significand.nonzero_magnitude_start() == 0 {
            // …round to one fractional digit…
            significand.half_even(-1);
        } else {
            // …otherwise, we have at least 2 digits before the decimal point,
            // so round to eliminate the fractional part.
            significand.half_even(0);
        }
        let rounded_magnitude = significand.nonzero_magnitude_start() + i16::from(exponent);
        if rounded_magnitude > log10_type {
            // We got bumped up a magnitude by rounding.
            // This means that `significand` is a power of 10.
            let old_exponent = exponent;
            // NOTE(egg): We could inline `plural_map_and_exponent_for_magnitude`
            // to avoid iterating twice (we only need to look at the next key),
            // but this obscures the logic and the map is tiny.
            (plural_map, exponent) = self.plural_map_and_exponent_for_magnitude(rounded_magnitude);
            significand =
                significand.multiplied_pow10(i16::from(old_exponent) - i16::from(exponent));
            // There is no need to perform any rounding: `significand`, being
            // a power of 10, is as round as it gets, and since `exponent` can
            // only have become larger, it is already the correct rounding of
            // `unrounded` to the precision we want to show.
        }
        significand.trim_end();
        FormattedCompactDecimal {
            formatter: self,
            plural_map,
            value: Cow::Owned(CompactDecimal::from_significand_and_exponent(
                significand,
                exponent,
            )),
        }
    }

    /// Formats a [`CompactDecimal`] object according to locale data.
    ///
    /// This is an advanced API; prefer using [`Self::format_i64()`] in simple
    /// cases.
    ///
    /// Since the caller specifies the exact digits that are displayed, this
    /// allows for arbitrarily complex rounding rules.
    /// However, contrary to [`FixedDecimalFormatter::format()`], this operation
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
    /// ```
    /// # use icu_compactdecimal::CompactDecimalFormatter;
    /// # use icu_locid::locale;
    /// # use writeable::assert_writeable_eq;
    /// # use std::str::FromStr;
    /// use fixed_decimal::CompactDecimal;
    ///
    /// # let short_french = CompactDecimalFormatter::try_new_short(
    /// #    &locale!("fr").into(),
    /// #    Default::default(),
    /// # ).unwrap();
    /// # let [long_french, long_bangla] = [locale!("fr"), locale!("bn")]
    /// #     .map(|locale| {
    /// #         CompactDecimalFormatter::try_new_long(
    /// #             &locale.into(),
    /// #             Default::default(),
    /// #         )
    /// #         .unwrap()
    /// #     });
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
    ) -> Result<FormattedCompactDecimal<'l>, CompactDecimalError> {
        let log10_type = value.significand().nonzero_magnitude_start() + value.exponent();

        let (plural_map, expected_exponent) =
            self.plural_map_and_exponent_for_magnitude(log10_type);
        if value.exponent() != i16::from(expected_exponent) {
            return Err(CompactDecimalError::Exponent {
                actual: value.exponent(),
                expected: i16::from(expected_exponent),
                log10_type,
            });
        }

        Ok(FormattedCompactDecimal {
            formatter: self,
            plural_map,
            value: Cow::Borrowed(value),
        })
    }

    /// Returns the compact decimal exponent that should be used for a number of
    /// the given magnitude when using this formatter.
    ///
    /// # Examples
    /// ```
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_locid::locale;
    ///
    /// let [long_french, long_japanese, long_bangla] =
    ///     [locale!("fr"), locale!("ja"), locale!("bn")].map(|locale| {
    ///         CompactDecimalFormatter::try_new_long(
    ///             &locale.into(),
    ///             Default::default(),
    ///         )
    ///         .unwrap()
    ///     });
    /// /// French uses millions.
    /// assert_eq!(long_french.compact_exponent_for_magnitude(6), 6);
    /// /// Bangla uses lakhs.
    /// assert_eq!(long_bangla.compact_exponent_for_magnitude(6), 5);
    /// /// Japanese uses myriads.
    /// assert_eq!(long_japanese.compact_exponent_for_magnitude(6), 4);
    /// ```
    pub fn compact_exponent_for_magnitude(&self, magnitude: i16) -> u8 {
        let (_, exponent) = self.plural_map_and_exponent_for_magnitude(magnitude);
        exponent
    }

    fn plural_map_and_exponent_for_magnitude(
        &self,
        magnitude: i16,
    ) -> (Option<ZeroMap2dCursor<i8, Count, PatternULE>>, u8) {
        let plural_map = self
            .compact_data
            .get()
            .patterns
            .iter0()
            .filter(|cursor| i16::from(*cursor.key0()) <= magnitude)
            .last();
        let exponent = plural_map
            .as_ref()
            .and_then(|map| {
                map.get1(&Count::Other)
                    .and_then(|pattern| u8::try_from(pattern.exponent).ok())
            })
            .unwrap_or(0);
        (plural_map, exponent)
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;
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
                CompactDecimalFormatter::try_new_short(&locale!("en").into(), case.options.clone())
            } else {
                CompactDecimalFormatter::try_new_long(&locale!("en").into(), case.options.clone())
            }
            .unwrap();
            let result1T = formatter.format_i64(1_000_000_000_000_000);
            assert_writeable_eq!(result1T, case.expected1T, "{:?}", case);
            let result10T = formatter.format_i64(10_000_000_000_000_000);
            assert_writeable_eq!(result10T, case.expected10T, "{:?}", case);
        }
    }
}
