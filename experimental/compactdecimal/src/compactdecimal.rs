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
use icu_provider::{DataLocale, DataPayload, DataProvider, DataRequest};
use zerovec::maps::ZeroMap2dCursor;

use crate::{
    format::FormattedCompactDecimal,
    provider::{
        Count, ErasedCompactDecimalFormatDataV1Marker, LongCompactDecimalFormatDataV1Marker,
        PatternULE, ShortCompactDecimalFormatDataV1Marker,
    },
    CompactDecimalError,
};

/// A formatter that renders locale-sensitive compact numbers.
///
/// # Examples
///
/// ```
/// use icu_compactdecimal::CompactDecimalFormatter;
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
/// use std::str::FromStr;
/// use fixed_decimal::CompactDecimal;
///
/// let short_french = CompactDecimalFormatter::try_new_short_unstable(
///    &icu_testdata::unstable(),
///    &locale!("fr").into(),
/// ).unwrap();
///
/// let [long_french, long_japanese, long_bangla] = [locale!("fr"), locale!("ja"), locale!("bn")]
///     .map(|locale| {
///         CompactDecimalFormatter::try_new_long_unstable(
///             &icu_testdata::unstable(),
///             &locale.into(),
///         )
///         .unwrap()
///     });
///
/// /// Supports short and long notations:
/// # // The following line contains U+00A0 NO-BREAK SPACE.
/// assert_writeable_eq!(short_french.format(35_357_670), "35 M");
/// assert_writeable_eq!(long_french.format(35_357_670), "35 millions");
/// /// The powers of ten used are locale-dependent:
/// assert_writeable_eq!(long_japanese.format(3535_7670), "3536万");
/// /// So are the digits:
/// assert_writeable_eq!(long_bangla.format(3_53_57_670), "৩.৫ কোটি");
/// ```
pub struct CompactDecimalFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) fixed_decimal_format: FixedDecimalFormatter,
    pub(crate) compact_data: DataPayload<ErasedCompactDecimalFormatDataV1Marker>,
}

impl CompactDecimalFormatter {
    /// Constructor that takes a selected locale, reference to a
    /// [data provider] and a list of preferences, then collects all data
    /// necessary to format numbers in short compact decimal notation for
    /// the given locale.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_short_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("sv").into());
    /// ```
    ///
    /// [data provider]: icu_provider
    pub fn try_new_short_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
    ) -> Result<Self, CompactDecimalError>
    where
        D: DataProvider<ShortCompactDecimalFormatDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>
            + ?Sized,
    {
        let mut options = FixedDecimalFormatterOptions::default();
        options.grouping_strategy = GroupingStrategy::Min2;
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                options,
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: skip,
        error: CompactDecimalError,
        functions: [
            Self::try_new_short_unstable,
            try_new_short_with_any_provider,
            try_new_short_with_buffer_provider
        ]
    );

    /// Constructor that takes a selected locale, reference to a
    /// [data provider] and a list of preferences, then collects all data
    /// necessary to format numbers in short compact decimal notation for
    /// the given locale.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_long_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("sv").into());
    /// ```
    ///
    /// [data provider]: icu_provider
    pub fn try_new_long_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
    ) -> Result<Self, CompactDecimalError>
    where
        D: DataProvider<LongCompactDecimalFormatDataV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>
            + ?Sized,
    {
        let mut options = FixedDecimalFormatterOptions::default();
        options.grouping_strategy = GroupingStrategy::Min2;
        Ok(Self {
            fixed_decimal_format: FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                options,
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: skip,
        error: CompactDecimalError,
        functions: [
            Self::try_new_long_unstable,
            try_new_long_with_any_provider,
            try_new_long_with_buffer_provider
        ]
    );

    /// Formats an integer in compact decimal notation using the default
    /// precision settings.
    ///
    /// The result may have a fractional digit only if it is compact and its
    /// significand is less than 10. Trailing fractional 0s are omitted.
    /// ```
    /// # use fixed_decimal::CompactDecimal;
    /// # use icu_compactdecimal::CompactDecimalFormatter;
    /// # use icu_locid::locale;
    /// # use writeable::assert_writeable_eq;
    /// # use std::str::FromStr;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short_unstable(
    /// #    &icu_testdata::unstable(),
    /// #    &locale!("en").into()
    /// # ).unwrap();
    /// assert_writeable_eq!(short_english.format(2), "2");
    /// assert_writeable_eq!(short_english.format(843), "843");
    /// assert_writeable_eq!(short_english.format(2207), "2.2K");
    /// assert_writeable_eq!(short_english.format(15_127), "15K");
    /// assert_writeable_eq!(short_english.format(3_010_349), "3M");
    /// ```
    /// The result is the nearest such compact number, with halfway cases-
    /// rounded towards the number with an even least significant digit.
    /// ```
    /// # use fixed_decimal::CompactDecimal;
    /// # use icu_compactdecimal::CompactDecimalFormatter;
    /// # use icu_locid::locale;
    /// # use writeable::assert_writeable_eq;
    /// # use std::str::FromStr;
    /// #
    /// # let short_english = CompactDecimalFormatter::try_new_short_unstable(
    /// #    &icu_testdata::unstable(),
    /// #    &locale!("en").into(),
    /// # ).unwrap();
    /// assert_writeable_eq!(short_english.format(999_499), "999K");
    /// assert_writeable_eq!(short_english.format(999_500), "1M");
    /// assert_writeable_eq!(short_english.format(1650), "1.6K");
    /// assert_writeable_eq!(short_english.format(1750), "1.8K");
    /// assert_writeable_eq!(short_english.format(1950), "2K");
    /// ```
    pub fn format<'l>(&'l self, value: i64) -> FormattedCompactDecimal<'l> {
        fn round(significand: &mut FixedDecimal) {
            if significand.nonzero_magnitude_start() == 0 {
                significand.half_even(-1);
            } else {
                significand.half_even(0);
            }
        }

        let unrounded = FixedDecimal::from(value);
        let log10_type = unrounded.nonzero_magnitude_start();
        let (mut plural_map, mut exponent) = self.plural_map_and_exponent_for_magnitude(log10_type);
        if exponent == 0 {
            return FormattedCompactDecimal {
                formatter: self,
                plural_map,
                value: Cow::Owned(CompactDecimal::noncompact(unrounded)),
            };
        }
        let mut significand = unrounded.clone().multiplied_pow10(-i16::from(exponent));
        round(&mut significand);
        let rounded_magnitude = significand.nonzero_magnitude_start() + i16::from(exponent);
        if rounded_magnitude > log10_type {
            // We got bumped up a magnitude by rounding.
            // NOTE(egg): We could inline `plural_map_and_exponent_for_magnitude`
            // to avoid iterating twice (we only need to look at the next key),
            // but this obscures the logic and the map is tiny.
            (plural_map, exponent) = self.plural_map_and_exponent_for_magnitude(rounded_magnitude);
            // TODO(egg): Discuss awkwardness.
            if exponent == 0 {
                return FormattedCompactDecimal {
                    formatter: self,
                    plural_map,
                    value: Cow::Owned(CompactDecimal::noncompact(unrounded)),
                };
            }
            significand = unrounded.multiplied_pow10(-i16::from(exponent));
            // TODO(egg): Discuss correctness.
            round(&mut significand);
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
    /// This is an advanced API; prefer using [`format()`] in simple cases.
    ///
    /// Since the caller specifies the exact digits that are displayed, this
    /// allows for arbitrarily complex rounding rules.
    /// However, contrary to [`FixedDecimalFormatter::format()`], this operation
    /// can fail, because the given [`CompactDecimal`] can be inconsistent with
    /// the locale data; for instance, if the locale uses lakhs and crores and
    /// millions are requested, or vice versa, this function returns an error.
    ///
    /// The given [`CompactDecimal`] should be constructed using
    /// [`compact_exponent_for_magnitude`] on the same
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
    /// # let short_french = CompactDecimalFormatter::try_new_short_unstable(
    /// #    &icu_testdata::unstable(),
    /// #    &locale!("fr").into(),
    /// # ).unwrap();
    /// # let [long_french, long_bangla] = [locale!("fr"), locale!("bn")]
    /// #     .map(|locale| {
    /// #         CompactDecimalFormatter::try_new_long_unstable(
    /// #             &icu_testdata::unstable(),
    /// #             &locale.into(),
    /// #         )
    /// #         .unwrap()
    /// #     });
    /// #
    /// let about_a_million = CompactDecimal::from_str("1.20c6").unwrap();
    /// let three_million = CompactDecimal::from_str("3c6").unwrap();
    /// let ten_lakhs = CompactDecimal::from_str("10c5").unwrap();
    /// # // The following line contains U+00A0 NO-BREAK SPACE.
    /// assert_writeable_eq!(short_french.format_compact_decimal(&about_a_million).unwrap(), "1,20 M");
    /// assert_writeable_eq!(long_french.format_compact_decimal(&about_a_million).unwrap(), "1,20 million");
    ///
    /// # // The following line contains U+00A0 NO-BREAK SPACE.
    /// assert_writeable_eq!(short_french.format_compact_decimal(&three_million).unwrap(), "3 M");
    /// assert_writeable_eq!(long_french.format_compact_decimal(&three_million).unwrap(), "3 millions");
    ///
    /// assert_writeable_eq!(long_bangla.format_compact_decimal(&ten_lakhs).unwrap(), "১০ লাখ");
    ///
    /// assert_eq!(
    ///     long_bangla.format_compact_decimal(&about_a_million).err().unwrap().to_string(),
    ///     "Expected compact exponent 5 for 10^6, got 6",
    /// );
    /// assert_eq!(
    ///     long_french.format_compact_decimal(&ten_lakhs).err().unwrap().to_string(),
    ///     "Expected compact exponent 6 for 10^6, got 5",
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
