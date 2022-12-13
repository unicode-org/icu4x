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
/// let relative_time_formatter = RelativeTimeFormatter::try_new_short_day_unstable(
///     &icu_testdata::unstable(),
///     &locale!("es").into(),
///     RelativeTimeFormatterOptions { numeric: Numeric::Auto }
/// )
/// .expect("Data should load successfully.");
///
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(0u8)),
///         "hoy"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(-2i8)),
///         "anteayer"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(2u8)),
///         "pasado mañana"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(15i8)),
///         "dentro de 15 d"
/// );
///
/// ```
///
/// # Example
/// ```
/// use icu_relativetime::{RelativeTimeFormatter, RelativeTimeFormatterOptions};
/// use icu_relativetime::options::Numeric;
/// use icu_locid::locale;
/// use writeable::assert_writeable_eq;
/// use fixed_decimal::FixedDecimal;
///
/// let relative_time_formatter = RelativeTimeFormatter::try_new_narrow_year_unstable(
///     &icu_testdata::unstable(),
///     &locale!("bn").into(),
///     RelativeTimeFormatterOptions::default()
/// )
/// .expect("Data should load successfully.");
///
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(3u8)),
///         "৩ বছরে"
/// );
/// assert_writeable_eq!(
///         relative_time_formatter.format(FixedDecimal::from(-15i8)),
///         "১৫ বছর পূর্বে"
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
    constructor!(
        try_new_long_minute_unstable,
        LongMinuteRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_hour_unstable,
        LongHourRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_day_unstable,
        LongDayRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_week_unstable,
        LongWeekRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_month_unstable,
        LongMonthRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_quarter_unstable,
        LongQuarterRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_long_year_unstable,
        LongYearRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_second_unstable,
        ShortSecondRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_minute_unstable,
        ShortMinuteRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_hour_unstable,
        ShortHourRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_day_unstable,
        ShortDayRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_week_unstable,
        ShortWeekRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_month_unstable,
        ShortMonthRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_quarter_unstable,
        ShortQuarterRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_short_year_unstable,
        ShortYearRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_second_unstable,
        NarrowSecondRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_minute_unstable,
        NarrowMinuteRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_hour_unstable,
        NarrowHourRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_day_unstable,
        NarrowDayRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_week_unstable,
        NarrowWeekRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_month_unstable,
        NarrowMonthRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_quarter_unstable,
        NarrowQuarterRelativeTimeFormatDataV1Marker
    );
    constructor!(
        try_new_narrow_year_unstable,
        NarrowYearRelativeTimeFormatDataV1Marker
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
