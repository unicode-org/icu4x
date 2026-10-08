// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::{Decimal, Sign};
use icu_decimal::{
    DecimalFormatter, DecimalFormatterPreferences, options::DecimalFormatterOptions,
    provider::DecimalDigitsV1, provider::DecimalSymbolsV1,
};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRulesPreferences;
use icu_plurals::{PluralRules, provider::PluralsCardinalV1};
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;

use crate::relativetime::format::FormattedRelativeTime;
use crate::relativetime::options::{RelativeTimeFormatterOptions, Width};
use crate::relativetime::provider::*;

define_preferences!(
    /// The preferences for relative time formatting.
    [Copy]
    RelativeTimeFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        ///
        /// To get the resolved numbering system, you can inspect the data provider.
        /// See the [`provider`] module for an example.
        numbering_system: preferences::NumberingSystem
    }
);
prefs_convert!(
    RelativeTimeFormatterPreferences,
    DecimalFormatterPreferences,
    { numbering_system }
);
prefs_convert!(RelativeTimeFormatterPreferences, PluralRulesPreferences);

/// Locale preferences used by this crate
pub mod preferences {
    /// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
    #[doc = "\n"] // prevent autoformatting
    pub use icu_locale_core::preferences::extensions::unicode::keywords::NumberingSystem;
}

/// A formatter to render locale-sensitive relative time.
///
/// # Example
///
/// ```
/// use fixed_decimal::Decimal;
/// use icu::experimental::relativetime::{
///     RelativeTimeFormatter, RelativeTimeFormatterOptions,
/// };
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let relative_time_formatter = RelativeTimeFormatter::try_new_second(
///     locale!("en").into(),
///     RelativeTimeFormatterOptions::default(),
/// )
/// .expect("locale should be present");
///
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(5i8)),
///     "in 5 seconds"
/// );
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(-10i8)),
///     "10 seconds ago"
/// );
/// ```
///
/// # Example
///
/// ```
/// use fixed_decimal::Decimal;
/// use icu::experimental::relativetime::options::{Numeric, Width};
/// use icu::experimental::relativetime::{
///     RelativeTimeFormatter, RelativeTimeFormatterOptions,
/// };
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let options = RelativeTimeFormatterOptions::default()
///     .with_width(Width::Short)
///     .with_numeric(Numeric::Auto);
///
/// let relative_time_formatter =
///     RelativeTimeFormatter::try_new_day(locale!("es").into(), options)
///         .expect("locale should be present");
///
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(0u8)),
///     "hoy"
/// );
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(-2i8)),
///     "anteayer"
/// );
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(2u8)),
///     "pasado mañana"
/// );
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(15i8)),
///     "dentro de 15 d"
/// );
/// ```
///
/// # Example
/// ```
/// use fixed_decimal::Decimal;
/// use icu::experimental::relativetime::options::Width;
/// use icu::experimental::relativetime::{
///     RelativeTimeFormatter, RelativeTimeFormatterOptions,
/// };
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let relative_time_formatter = RelativeTimeFormatter::try_new_year(
///     locale!("bn").into(),
///     RelativeTimeFormatterOptions::default().with_width(Width::Narrow),
/// )
/// .expect("locale should be present");
///
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(3u8)),
///     "৩ বছরে"
/// );
/// assert_writeable_eq!(
///     relative_time_formatter.format(Decimal::from(-15i8)),
///     "১৫ বছর আগে"
/// );
/// ```
#[derive(Debug)]
pub struct RelativeTimeFormatter {
    pub(crate) plural_rules: PluralRules,
    pub(crate) rt: DataPayload<ErasedMarker<RelativeTimePatternData<'static>>>,
    pub(crate) options: RelativeTimeFormatterOptions,
    pub(crate) decimal_formatter: DecimalFormatter,
}

macro_rules! constructor {
    ($unstable: ident, $baked: ident, $buffer: ident, $marker: ty) => {

        /// Create a new [`RelativeTimeFormatter`] from compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        #[cfg(feature = "compiled_data")]
        pub fn $baked(
            prefs: RelativeTimeFormatterPreferences,
            options: RelativeTimeFormatterOptions,
        ) -> Result<Self, DataError> {
            let width_attr = match options.width {
                Width::Long => RelativeTimePatternData::LONG,
                Width::Short => RelativeTimePatternData::SHORT,
                Width::Narrow => RelativeTimePatternData::NARROW,
            };
            let locale = <$marker>::make_locale(prefs.locale_preferences);
            let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;
            // Initialize DecimalFormatter with default options
            let decimal_formatter = DecimalFormatter::try_new(
                (&prefs).into(),
                DecimalFormatterOptions::default(),
            )?;
            let rt: DataResponse<$marker> = crate::provider::Baked
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        width_attr,
                        &locale,
                    ),
                    ..Default::default()
                })?;
            let rt = rt.payload.cast();
            Ok(RelativeTimeFormatter {
                plural_rules,
                options,
                rt,
                decimal_formatter,
            })
        }

        icu_provider::gen_buffer_data_constructors!(
            (prefs: RelativeTimeFormatterPreferences, options: RelativeTimeFormatterOptions) -> error: DataError,
            functions: [
                $baked: skip,
                $buffer,
                $unstable,
                Self,
            ]
        );


        #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$baked)]
        pub fn $unstable<D>(
            provider: &D,
            prefs: RelativeTimeFormatterPreferences,
            options: RelativeTimeFormatterOptions,
        ) -> Result<Self, DataError>
        where
            D: DataProvider<PluralsCardinalV1>
                + DataProvider<$marker>
                + DataProvider<DecimalSymbolsV1> + DataProvider<DecimalDigitsV1>
                + ?Sized,
        {
            let width_attr = match options.width {
                Width::Long => RelativeTimePatternData::LONG,
                Width::Short => RelativeTimePatternData::SHORT,
                Width::Narrow => RelativeTimePatternData::NARROW,
            };
            let locale = <$marker>::make_locale(prefs.locale_preferences);
            let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;
            // Initialize DecimalFormatter with default options
            let decimal_formatter = DecimalFormatter::try_new_unstable(
                provider,
                (&prefs).into(),
                DecimalFormatterOptions::default(),
            )?;
            let rt: DataResponse<$marker> = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        width_attr,
                        &locale,
                    ),
                    ..Default::default()
                })?;
            let rt = rt.payload.cast();
            Ok(RelativeTimeFormatter {
                plural_rules,
                options,
                rt,
                decimal_formatter,
            })
        }
    };
}

impl RelativeTimeFormatter {
    constructor!(
        try_new_second_unstable,
        try_new_second,
        try_new_second_with_buffer_provider,
        DatetimeRelativeSecondV1
    );
    constructor!(
        try_new_minute_unstable,
        try_new_minute,
        try_new_minute_with_buffer_provider,
        DatetimeRelativeMinuteV1
    );
    constructor!(
        try_new_hour_unstable,
        try_new_hour,
        try_new_hour_with_buffer_provider,
        DatetimeRelativeHourV1
    );
    constructor!(
        try_new_day_unstable,
        try_new_day,
        try_new_day_with_buffer_provider,
        DatetimeRelativeDayV1
    );
    constructor!(
        try_new_week_unstable,
        try_new_week,
        try_new_week_with_buffer_provider,
        DatetimeRelativeWeekV1
    );
    constructor!(
        try_new_month_unstable,
        try_new_month,
        try_new_month_with_buffer_provider,
        DatetimeRelativeMonthV1
    );
    constructor!(
        try_new_quarter_unstable,
        try_new_quarter,
        try_new_quarter_with_buffer_provider,
        DatetimeRelativeQuarterV1
    );
    constructor!(
        try_new_year_unstable,
        try_new_year,
        try_new_year_with_buffer_provider,
        DatetimeRelativeYearV1
    );

    /// Format a `value` according to the locale and formatting options of
    /// [`RelativeTimeFormatter`].
    pub fn format(&self, value: Decimal) -> FormattedRelativeTime<'_> {
        let is_negative = value.sign() == Sign::Negative;
        FormattedRelativeTime {
            options: &self.options,
            formatter: self,
            value: value.with_sign(Sign::None),
            is_negative,
        }
    }
}
