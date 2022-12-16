// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::CompactDecimal;
use icu_decimal::{options::FixedDecimalFormatterOptions, FixedDecimalFormatter};
use icu_plurals::PluralRules;
use icu_provider::{DataLocale, DataPayload, DataProvider, DataRequest};

use crate::{
    format::FormattedCompactDecimal,
    provider::{
        Count, ErasedCompactDecimalFormatDataV1Marker, LongCompactDecimalFormatDataV1Marker,
        ShortCompactDecimalFormatDataV1Marker,
    },
    CompactDecimalError,
};

/// TODO words
/// # Examples
///
///
/// [data provider]: icu_provider
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
    /// use icu_decimal::options::FixedDecimalFormatterOptions;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_short_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("sv").into(),
    ///     FixedDecimalFormatterOptions::default());
    /// ```
    ///
    /// [data provider]: icu_provider
    pub fn try_new_short_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
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
        options: FixedDecimalFormatterOptions,
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
    /// use icu_decimal::options::FixedDecimalFormatterOptions;
    /// use icu_locid::locale;
    ///
    /// CompactDecimalFormatter::try_new_long_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("sv").into(),
    ///     FixedDecimalFormatterOptions::default());
    /// ```
    ///
    /// [data provider]: icu_provider
    pub fn try_new_long_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
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

    /// TODO(egg): Explain that this is an advanced API which can fail (and you
    /// should use the API that does not exist instead).
    ///
    /// # Examples
    /// ```
    /// use fixed_decimal::CompactDecimal;
    /// use icu_compactdecimal::CompactDecimalFormatter;
    /// use icu_decimal::options::FixedDecimalFormatterOptions;
    /// use icu_locid::locale;
    /// use writeable::assert_writeable_eq;
    /// use std::str::FromStr;
    ///
    /// let long_bengali = CompactDecimalFormatter::try_new_long_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("bn").into(),
    ///     FixedDecimalFormatterOptions::default(),
    /// ).unwrap();
    ///
    /// let short_french = CompactDecimalFormatter::try_new_short_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("fr").into(),
    ///     FixedDecimalFormatterOptions::default(),
    /// ).unwrap();
    ///
    /// let long_french = CompactDecimalFormatter::try_new_long_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("fr").into(),
    ///     FixedDecimalFormatterOptions::default(),
    /// ).unwrap();
    ///
    /// let about_a_million = CompactDecimal::from_str("1.2c6").unwrap();
    /// let three_millions = CompactDecimal::from_str("3c6").unwrap();
    /// let ten_lakhs = CompactDecimal::from_str("10c5").unwrap();
    ///
    /// assert_writeable_eq!(short_french.format_compact_decimal(&about_a_million).unwrap(), "1,2\u{A0}M");
    /// assert_writeable_eq!(long_french.format_compact_decimal(&about_a_million).unwrap(), "1,2 million");
    ///
    /// assert_writeable_eq!(short_french.format_compact_decimal(&three_millions).unwrap(), "3\u{A0}M");
    /// assert_writeable_eq!(long_french.format_compact_decimal(&three_millions).unwrap(), "3 millions");
    ///
    /// assert_writeable_eq!(long_bengali.format_compact_decimal(&ten_lakhs).unwrap(), "১০ লাখ");
    ///
    /// assert_eq!(
    ///     long_bengali.format_compact_decimal(&about_a_million).err().unwrap().to_string(),
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

        let plural_map = self
            .compact_data
            .get()
            .patterns
            .iter0()
            .filter(|cursor| i16::from(*cursor.key0()) <= log10_type)
            .last();
        let expected_exponent = plural_map
            .as_ref()
            .and_then(|map| {
                map.get1(&Count::Other)
                    .and_then(|pattern| Some(i16::from(pattern.exponent)))
            })
            .unwrap_or(0);
        if value.exponent() != expected_exponent {
            return Err(CompactDecimalError::Exponent {
                actual: value.exponent(),
                expected: expected_exponent,
                log10_type: log10_type,
            });
        }

        Ok(FormattedCompactDecimal {
            formatter: self,
            plural_map,
            value,
        })
    }
}
