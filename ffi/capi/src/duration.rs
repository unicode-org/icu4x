// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(feature = "unstable")]
    use crate::unstable::parts::ffi::Parts;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::unstable::locale_core::ffi::Locale;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use icu_experimental::duration::{
        DurationFormatterPreferences, ValidatedDurationFormatterOptions,
    };

    use writeable::Writeable;

    #[diplomat::rust_link(icu::experimental::duration::DurationSign, Enum)]
    #[diplomat::enum_convert(icu_experimental::duration::DurationSign)]
    #[non_exhaustive]
    pub enum DurationSign {
        #[diplomat::attr(auto, default)]
        Positive,
        Negative,
    }

    #[diplomat::rust_link(icu::experimental::duration::Duration, Struct)]
    #[diplomat::rust_link(icu::experimental::duration::Duration::new, FnInStruct, hidden)]
    pub struct Duration {
        pub sign: DurationSign,
        pub years: u64,
        pub months: u64,
        pub weeks: u64,
        pub days: u64,
        pub hours: u64,
        pub minutes: u64,
        pub seconds: u64,
        pub milliseconds: u64,
        pub microseconds: u64,
        pub nanoseconds: u64,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::BaseStyle, Enum)]
    #[diplomat::enum_convert(icu_experimental::duration::options::BaseStyle, needs_wildcard)]
    #[non_exhaustive]
    pub enum DurationBaseStyle {
        Narrow,
        #[diplomat::attr(auto, default)]
        Short,
        Long,
        Digital,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::YearStyle, Enum)]
    #[diplomat::rust_link(icu::experimental::duration::options::MonthStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::WeekStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::DayStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::HourStyle, Enum)]
    #[diplomat::rust_link(icu::experimental::duration::options::MinuteStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::SecondStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::MilliSecondStyle, Enum)]
    #[diplomat::rust_link(icu::experimental::duration::options::MicroSecondStyle, Enum, hidden)]
    #[diplomat::rust_link(icu::experimental::duration::options::NanoSecondStyle, Enum, hidden)]
    #[non_exhaustive]
    pub enum DurationUnitStyle {
        #[diplomat::attr(auto, default)]
        Default,
        Narrow,
        Short,
        Long,
        TwoDigit,
        Numeric,
        Fractional,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::FieldDisplay, Enum)]
    #[non_exhaustive]
    pub enum DurationFieldDisplay {
        #[diplomat::attr(auto, default)]
        Default,
        Auto,
        Always,
    }

    #[diplomat::rust_link(icu::experimental::duration::DurationFormatterOptionsError, Enum)]
    #[derive(PartialEq, Eq, Debug)]
    #[non_exhaustive]
    #[diplomat::attr(auto, error)]
    pub enum DurationFormatterError {
        FractionalDigitsOutOfRange,
        PreviousNumeric,
        DisplayAlwaysFractional,
        PreviousFractional,
        InvalidUnitStyle,
        DataError,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::DurationFormatterOptions, Struct)]
    #[diplomat::rust_link(icu::experimental::duration::options::FractionalDigits, Enum, hidden)]
    pub struct DurationFormatterOptions {
        pub base: DurationBaseStyle,
        pub year: DurationUnitStyle,
        pub year_visibility: DurationFieldDisplay,
        pub month: DurationUnitStyle,
        pub month_visibility: DurationFieldDisplay,
        pub week: DurationUnitStyle,
        pub week_visibility: DurationFieldDisplay,
        pub day: DurationUnitStyle,
        pub day_visibility: DurationFieldDisplay,
        pub hour: DurationUnitStyle,
        pub hour_visibility: DurationFieldDisplay,
        pub minute: DurationUnitStyle,
        pub minute_visibility: DurationFieldDisplay,
        pub second: DurationUnitStyle,
        pub second_visibility: DurationFieldDisplay,
        pub millisecond: DurationUnitStyle,
        pub millisecond_visibility: DurationFieldDisplay,
        pub microsecond: DurationUnitStyle,
        pub microsecond_visibility: DurationFieldDisplay,
        pub nanosecond: DurationUnitStyle,
        pub nanosecond_visibility: DurationFieldDisplay,
        /// Number of fractional digits to use when formatting sub-second units.
        /// Pass `-1` for `FractionalDigits::ShowAll`, or `0..=9` for `FractionalDigits::Fixed(n)`.
        pub fractional_digits: i8,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::experimental::duration::DurationFormatter, Struct)]
    #[diplomat::rust_link(icu::experimental::duration::FormattedDuration, Struct, hidden)]
    #[diplomat::rust_link(
        icu::experimental::duration::ValidatedDurationFormatterOptions,
        Struct,
        hidden
    )]
    pub struct DurationFormatter(pub icu_experimental::duration::DurationFormatter);

    impl DurationFormatter {
        /// Creates a new [`DurationFormatter`] from locale data and an options bag using compiled data.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::try_new, FnInStruct)]
        #[diplomat::rust_link(
            icu::experimental::duration::ValidatedDurationFormatterOptions::validate,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create(
            locale: &Locale,
            options: DurationFormatterOptions,
        ) -> Result<Box<DurationFormatter>, DurationFormatterError> {
            let prefs = DurationFormatterPreferences::from(&locale.0);
            let validated = ValidatedDurationFormatterOptions::try_from(options)?;
            icu_experimental::duration::DurationFormatter::try_new(prefs, validated)
                .map(|f| Box::new(DurationFormatter(f)))
                .map_err(|_| DurationFormatterError::DataError)
        }

        /// Creates a new [`DurationFormatter`] from locale data and an options bag using a particular data source.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::try_new, FnInStruct)]
        #[diplomat::rust_link(
            icu::experimental::duration::ValidatedDurationFormatterOptions::validate,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            options: DurationFormatterOptions,
        ) -> Result<Box<DurationFormatter>, DurationFormatterError> {
            let prefs = DurationFormatterPreferences::from(&locale.0);
            let validated = ValidatedDurationFormatterOptions::try_from(options)?;
            let provider = provider
                .get()
                .map_err(|_| DurationFormatterError::DataError)?;
            icu_experimental::duration::DurationFormatter::try_new_with_buffer_provider(
                provider, prefs, validated,
            )
            .map(|f| Box::new(DurationFormatter(f)))
            .map_err(|_| DurationFormatterError::DataError)
        }

        /// Formats a [`Duration`] to a string.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::experimental::duration::FormattedDuration, Struct, hidden)]
        #[diplomat::rust_link(
            icu::experimental::duration::FormattedDuration::to_string,
            FnInStruct,
            hidden
        )]
        pub fn format(&self, duration: Duration, write: &mut DiplomatWrite) {
            let rust_duration = icu_experimental::duration::Duration::from(duration);
            let _infallible = self.0.format(&rust_duration).write_to(write);
        }

        /// Formats a [`Duration`] to a UTF-8 string while recording part spans into `parts`.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::experimental::duration::FormattedDuration, Struct, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "format_to_parts")]
        #[cfg(feature = "unstable")]
        pub fn format_utf8_to_parts(
            &self,
            duration: Duration,
            parts: &mut Parts,
            write: &mut DiplomatWrite,
        ) {
            let rust_duration = icu_experimental::duration::Duration::from(duration);
            let _infallible = crate::unstable::parts::write_to_parts(
                &self.0.format(&rust_duration),
                parts,
                write,
            );
        }

        /// Formats a [`Duration`] to a UTF-16 string while recording UTF-16 code-unit part spans into `parts`.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::experimental::duration::FormattedDuration, Struct, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "format_to_parts")]
        #[diplomat::attr(supports = utf8_strings, rename = "format_to_parts16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        #[diplomat::attr(demo_gen, disable)]
        #[cfg(feature = "unstable")]
        pub fn format_utf16_to_parts(
            &self,
            duration: Duration,
            parts: &mut Parts,
            write: &mut DiplomatWrite16,
        ) {
            let rust_duration = icu_experimental::duration::Duration::from(duration);
            let _infallible = crate::unstable::parts::write_to_parts_utf16(
                &self.0.format(&rust_duration),
                parts,
                write,
            );
        }
    }
}

impl From<ffi::Duration> for icu_experimental::duration::Duration {
    fn from(other: ffi::Duration) -> Self {
        Self {
            sign: other.sign.into(),
            years: other.years,
            months: other.months,
            weeks: other.weeks,
            days: other.days,
            hours: other.hours,
            minutes: other.minutes,
            seconds: other.seconds,
            milliseconds: other.milliseconds,
            microseconds: other.microseconds,
            nanoseconds: other.nanoseconds,
        }
    }
}

impl From<ffi::DurationFieldDisplay>
    for Option<icu_experimental::duration::options::FieldDisplay>
{
    fn from(other: ffi::DurationFieldDisplay) -> Self {
        use icu_experimental::duration::options::FieldDisplay;
        match other {
            ffi::DurationFieldDisplay::Default => None,
            ffi::DurationFieldDisplay::Auto => Some(FieldDisplay::Auto),
            ffi::DurationFieldDisplay::Always => Some(FieldDisplay::Always),
        }
    }
}

impl TryFrom<ffi::DurationFormatterOptions>
    for icu_experimental::duration::ValidatedDurationFormatterOptions
{
    type Error = ffi::DurationFormatterError;

    fn try_from(options: ffi::DurationFormatterOptions) -> Result<Self, Self::Error> {
        use ffi::{DurationFormatterError, DurationUnitStyle};
        use icu_experimental::duration::{
            DurationFormatterOptionsError, ValidatedDurationFormatterOptions,
            options::{
                DayStyle, DurationFormatterOptions as RustOptions, FractionalDigits, HourStyle,
                MicroSecondStyle, MilliSecondStyle, MinuteStyle, MonthStyle, NanoSecondStyle,
                SecondStyle, WeekStyle, YearStyle,
            },
        };

        macro_rules! map_date_style {
            ($style:expr, $target_enum:ident) => {
                match $style {
                    DurationUnitStyle::Default => None,
                    DurationUnitStyle::Narrow => Some($target_enum::Narrow),
                    DurationUnitStyle::Short => Some($target_enum::Short),
                    DurationUnitStyle::Long => Some($target_enum::Long),
                    DurationUnitStyle::TwoDigit
                    | DurationUnitStyle::Numeric
                    | DurationUnitStyle::Fractional => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        macro_rules! map_time_style {
            ($style:expr, $target_enum:ident) => {
                match $style {
                    DurationUnitStyle::Default => None,
                    DurationUnitStyle::Narrow => Some($target_enum::Narrow),
                    DurationUnitStyle::Short => Some($target_enum::Short),
                    DurationUnitStyle::Long => Some($target_enum::Long),
                    DurationUnitStyle::TwoDigit => Some($target_enum::TwoDigit),
                    DurationUnitStyle::Numeric => Some($target_enum::Numeric),
                    DurationUnitStyle::Fractional => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        macro_rules! map_subsecond_style {
            ($style:expr, $target_enum:ident) => {
                match $style {
                    DurationUnitStyle::Default => None,
                    DurationUnitStyle::Narrow => Some($target_enum::Narrow),
                    DurationUnitStyle::Short => Some($target_enum::Short),
                    DurationUnitStyle::Long => Some($target_enum::Long),
                    DurationUnitStyle::Numeric | DurationUnitStyle::Fractional => {
                        Some($target_enum::Numeric)
                    }
                    DurationUnitStyle::TwoDigit => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        let fractional_digits = if options.fractional_digits == -1 {
            FractionalDigits::ShowAll
        } else if let Ok(digits) = u8::try_from(options.fractional_digits) {
            FractionalDigits::Fixed(digits)
        } else {
            return Err(DurationFormatterError::FractionalDigitsOutOfRange);
        };

        let mut rust_options = RustOptions::default();
        rust_options.base = options.base.into();
        rust_options.year = map_date_style!(options.year, YearStyle);
        rust_options.year_visibility = options.year_visibility.into();
        rust_options.month = map_date_style!(options.month, MonthStyle);
        rust_options.month_visibility = options.month_visibility.into();
        rust_options.week = map_date_style!(options.week, WeekStyle);
        rust_options.week_visibility = options.week_visibility.into();
        rust_options.day = map_date_style!(options.day, DayStyle);
        rust_options.day_visibility = options.day_visibility.into();
        rust_options.hour = map_time_style!(options.hour, HourStyle);
        rust_options.hour_visibility = options.hour_visibility.into();
        rust_options.minute = map_time_style!(options.minute, MinuteStyle);
        rust_options.minute_visibility = options.minute_visibility.into();
        rust_options.second = map_time_style!(options.second, SecondStyle);
        rust_options.second_visibility = options.second_visibility.into();
        rust_options.millisecond = map_subsecond_style!(options.millisecond, MilliSecondStyle);
        rust_options.millisecond_visibility = options.millisecond_visibility.into();
        rust_options.microsecond = map_subsecond_style!(options.microsecond, MicroSecondStyle);
        rust_options.microsecond_visibility = options.microsecond_visibility.into();
        rust_options.nanosecond = map_subsecond_style!(options.nanosecond, NanoSecondStyle);
        rust_options.nanosecond_visibility = options.nanosecond_visibility.into();
        rust_options.fractional_digits = fractional_digits;

        ValidatedDurationFormatterOptions::validate(rust_options).map_err(|e| match e {
            DurationFormatterOptionsError::FractionalDigitsOutOfRange => {
                DurationFormatterError::FractionalDigitsOutOfRange
            }
            DurationFormatterOptionsError::PreviousNumeric => {
                DurationFormatterError::PreviousNumeric
            }
            DurationFormatterOptionsError::DisplayAlwaysFractional => {
                DurationFormatterError::DisplayAlwaysFractional
            }
            DurationFormatterOptionsError::PreviousFractional => {
                DurationFormatterError::PreviousFractional
            }
            _ => DurationFormatterError::InvalidUnitStyle,
        })
    }
}

#[cfg(all(test, feature = "compiled_data", feature = "unstable"))]
mod tests {
    use super::ffi::*;
    use crate::unstable::locale_core::ffi::Locale;
    use crate::unstable::parts::ffi::{PartKind, PartSpan, Parts};
    use alloc::vec::Vec;
    use diplomat_runtime::rust_interop::{RustWriteVec, RustWriteVecGeneric};

    fn default_options(base: DurationBaseStyle) -> DurationFormatterOptions {
        DurationFormatterOptions {
            base,
            year: DurationUnitStyle::Default,
            year_visibility: DurationFieldDisplay::Default,
            month: DurationUnitStyle::Default,
            month_visibility: DurationFieldDisplay::Default,
            week: DurationUnitStyle::Default,
            week_visibility: DurationFieldDisplay::Default,
            day: DurationUnitStyle::Default,
            day_visibility: DurationFieldDisplay::Default,
            hour: DurationUnitStyle::Default,
            hour_visibility: DurationFieldDisplay::Default,
            minute: DurationUnitStyle::Default,
            minute_visibility: DurationFieldDisplay::Default,
            second: DurationUnitStyle::Default,
            second_visibility: DurationFieldDisplay::Default,
            millisecond: DurationUnitStyle::Default,
            millisecond_visibility: DurationFieldDisplay::Default,
            microsecond: DurationUnitStyle::Default,
            microsecond_visibility: DurationFieldDisplay::Default,
            nanosecond: DurationUnitStyle::Default,
            nanosecond_visibility: DurationFieldDisplay::Default,
            fractional_digits: -1,
        }
    }

    #[test]
    fn test_duration_formatter_to_parts() {
        let locale = Locale::from_string(b"en").unwrap();
        let mut options = default_options(DurationBaseStyle::Digital);
        options.year_visibility = DurationFieldDisplay::Always;

        let formatter = DurationFormatter::create(&locale, options).unwrap();
        let duration = Duration {
            sign: DurationSign::Negative,
            years: 0,
            months: 2,
            weeks: 3,
            days: 0,
            hours: 12,
            minutes: 1,
            seconds: 5,
            milliseconds: 130,
            microseconds: 140,
            nanoseconds: 0,
        };

        let mut parts = Parts::create();
        let mut write = RustWriteVec::with_capacity(0);
        formatter.format_utf8_to_parts(duration, &mut parts, unsafe { write.borrow_mut() });
        let out = core::str::from_utf8(write.borrow().as_bytes()).unwrap();
        assert_eq!(out, "-0 yrs, 2 mths, 3 wks, 12:01:05.13014");

        let expected = [
            PartSpan::new(0, 6, PartKind::ListElement),
            PartSpan::new(0, 6, PartKind::UnitYear),
            PartSpan::new(0, 1, PartKind::DecimalMinusSign),
            PartSpan::new(1, 2, PartKind::DecimalInteger),
            PartSpan::new(6, 8, PartKind::ListLiteral),
            PartSpan::new(8, 14, PartKind::ListElement),
            PartSpan::new(8, 14, PartKind::UnitMonth),
            PartSpan::new(8, 9, PartKind::DecimalInteger),
            PartSpan::new(14, 16, PartKind::ListLiteral),
            PartSpan::new(16, 21, PartKind::ListElement),
            PartSpan::new(16, 21, PartKind::UnitWeek),
            PartSpan::new(16, 17, PartKind::DecimalInteger),
            PartSpan::new(21, 23, PartKind::ListLiteral),
            PartSpan::new(23, 37, PartKind::ListElement),
            PartSpan::new(23, 25, PartKind::UnitHour),
            PartSpan::new(23, 25, PartKind::DecimalInteger),
            PartSpan::new(25, 26, PartKind::DurationLiteral),
            PartSpan::new(26, 28, PartKind::UnitMinute),
            PartSpan::new(26, 28, PartKind::DecimalInteger),
            PartSpan::new(28, 29, PartKind::DurationLiteral),
            PartSpan::new(29, 37, PartKind::UnitSecond),
            PartSpan::new(29, 31, PartKind::DecimalInteger),
            PartSpan::new(31, 32, PartKind::DecimalDecimal),
            PartSpan::new(32, 37, PartKind::DecimalFraction),
        ];
        assert_eq!(parts.len(), expected.len());
        for (i, expected_span) in expected.iter().copied().enumerate() {
            assert_eq!(parts.get(i), Some(expected_span));
        }

        let mut parts16 = Parts::create();
        let mut write16 = RustWriteVecGeneric::<u16>::with_capacity(0);
        let duration16 = Duration {
            sign: DurationSign::Negative,
            years: 0,
            months: 2,
            weeks: 3,
            days: 0,
            hours: 12,
            minutes: 1,
            seconds: 5,
            milliseconds: 130,
            microseconds: 140,
            nanoseconds: 0,
        };
        formatter.format_utf16_to_parts(duration16, &mut parts16, unsafe { write16.borrow_mut() });
        let expected_u16: Vec<u16> = "-0 yrs, 2 mths, 3 wks, 12:01:05.13014"
            .encode_utf16()
            .collect();
        assert_eq!(write16.as_slice(), expected_u16.as_slice());
        assert_eq!(parts16.len(), expected.len());
        for (i, expected_span) in expected.iter().copied().enumerate() {
            assert_eq!(parts16.get(i), Some(expected_span));
        }
    }
}
