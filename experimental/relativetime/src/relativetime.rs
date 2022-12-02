// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::{FixedDecimal, Sign};
use icu_decimal::{
    options::FixedDecimalFormatterOptions, provider::DecimalSymbolsV1Marker, FixedDecimalFormatter,
};
use icu_plurals::{provider::CardinalV1Marker, PluralRules};
use icu_provider::{DataLocale, DataPayload, DataProvider, DataRequest};

use crate::format::FormattedRelativeTime;
use crate::provider::*;
use crate::{options::RelativeTimeFormatterOptions, RelativeTimeError};

/// A formatter to render locale-sensitive relative time.
///
/// # Example
///
/// ```
/// use icu_relativetime::{RelativeTimeFormatter, RelativeTimeFormatterOptions};
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
/// use fixed_decimal::FixedDecimal;
///
/// let relative_time_formatter = RelativeTimeFormatter::try_new_long_second_unstable(
///     &icu_testdata::unstable(),
///     &locale!("en").into(),
///     RelativeTimeFormatterOptions::default()
/// )
/// .expect("Data should load successfully.");
///
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(5i8)),
///         "in 5 seconds"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(-10i8)),
///         "10 seconds ago"
/// );
/// ```
///
/// # Example
///
/// ```
/// use icu_relativetime::{RelativeTimeFormatter, RelativeTimeFormatterOptions};
/// use icu_relativetime::options::Numeric;
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
/// use fixed_decimal::FixedDecimal;
///
/// let relative_time_formatter = RelativeTimeFormatter::try_new_long_second_unstable(
///     &icu_testdata::unstable(),
///     &locale!("es").into(),
///     RelativeTimeFormatterOptions { numeric: Numeric::Auto }
/// )
/// .expect("Data should load successfully.");
///
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(0u8)),
///         "ahora"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(2u8)),
///         "dentro de 2 segundos"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(-15i8)),
///         "hace 15 segundos"
/// );
/// ```
pub struct RelativeTimeFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) rt: DataPayload<ErasedRelativeTimeFormatV1Marker>,
    pub(crate) options: RelativeTimeFormatterOptions,
    pub(crate) fixed_decimal_format: FixedDecimalFormatter,
}

macro_rules! constructor {
    ($name: ident, $marker: ty) => {
        #[doc = concat!("Create a new [`RelativeTimeFormatter`]")]
        pub fn $name<D>(
            data_provider: &D,
            locale: &DataLocale,
            options: RelativeTimeFormatterOptions,
        ) -> Result<Self, RelativeTimeError>
        where
            D: DataProvider<CardinalV1Marker>
                + DataProvider<$marker>
                + DataProvider<DecimalSymbolsV1Marker>
                + ?Sized,
        {
            let plural_rules = PluralRules::try_new_cardinal_unstable(data_provider, locale)?;
            // Initialize FixedDecimalFormatter with default options
            let fixed_decimal_format = FixedDecimalFormatter::try_new_unstable(
                data_provider,
                locale,
                FixedDecimalFormatterOptions::default(),
            )?;
            let rt: DataPayload<$marker> = data_provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?;
            let rt = rt.cast();
            Ok(RelativeTimeFormatter {
                plural_rules,
                options,
                rt,
                fixed_decimal_format,
            })
        }
    };
}

impl RelativeTimeFormatter {
    constructor!(
        try_new_long_second_unstable,
        LongSecondRelativeTimeFormatDataV1Marker
    );

    /// Format a `value` according to the locale and formatting options of
    /// [`RelativeTimeFormatter`].
    pub fn format(&self, value: FixedDecimal) -> FormattedRelativeTime<'_> {
        let is_negative = value.sign() == Sign::Negative;
        FormattedRelativeTime {
            options: &self.options,
            formatter: self,
            value: value.with_sign(Sign::None),
            is_negative,
        }
    }
}
