// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::CompactDecimal;
use icu_decimal::{options::FixedDecimalFormatterOptions, FixedDecimalFormatter};
use icu_plurals::PluralRules;
use icu_provider::{DataLocale, DataPayload, DataProvider, DataRequest};

use crate::{
    format::FormattedCompactDecimal,
    provider::{Count, ShortCompactDecimalFormatDataV1Marker},
    CompactDecimalError,
};

/// TODO words
/// # Examples
///
/// ```
/// use fixed_decimal::CompactDecimal;
/// use icu_compactdecimal::CompactDecimalFormatter;
/// use icu_decimal::options::FixedDecimalFormatterOptions;
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
/// use std::str::FromStr;
///
/// let formatter = CompactDecimalFormatter::try_new_short_unstable(
///     &icu_testdata::unstable(),
///     &locale!("fr").into(),
///     FixedDecimalFormatterOptions::default(),
/// ).unwrap();
///
/// assert_writeable_eq!(formatter.format(&CompactDecimal::from_str("1.2c6").unwrap()).unwrap(), "12:34 PM");
/// ```
///
/// [data provider]: icu_provider
pub struct CompactDecimalFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) fixed_decimal_format: FixedDecimalFormatter,
    pub(crate) compact_data: DataPayload<ShortCompactDecimalFormatDataV1Marker>,  // TODO(egg): Erased nonsense.
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
    ///     &locale!("en").into());
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
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
            compact_data: data_provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?,
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

    /// TODO(egg): meow
    pub fn format<'l>(
        &'l self,
        value: &'l CompactDecimal,
    ) -> Result<FormattedCompactDecimal<'l>, CompactDecimalError> {
        let significand = value.clone().into_significand();
        let exponent = value.exponent();
        let log10_type = significand.nonzero_magnitude_start() + exponent;

        let plural_map = self
            .compact_data
            .get()
            .patterns
            .iter0()
            .filter(|cursor| i16::from(*cursor.key0()) <= log10_type)
            .last();
        let expected_exponent = plural_map.as_ref()
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
