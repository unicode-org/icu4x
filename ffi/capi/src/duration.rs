// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::unstable::locale_core::ffi::Locale;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;
    use diplomat_runtime::DiplomatOption;
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
        // TODO(#8687): Support u128 ranges for microseconds and nanoseconds in FFI.
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
        Narrow,
        #[diplomat::attr(auto, default)]
        Short,
        Long,
        TwoDigit,
        Numeric,
        Fractional,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::FieldDisplay, Enum)]
    #[diplomat::enum_convert(icu_experimental::duration::options::FieldDisplay, needs_wildcard)]
    #[non_exhaustive]
    pub enum DurationFieldDisplay {
        #[diplomat::attr(auto, default)]
        Auto,
        Always,
    }

    #[diplomat::rust_link(icu::experimental::duration::DurationFormatterOptionsError, Enum)]
    #[diplomat::rust_link(icu_provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu_provider::DataErrorKind, Enum, compact)]
    #[derive(PartialEq, Eq, Debug)]
    #[repr(C)]
    #[non_exhaustive]
    #[diplomat::attr(auto, error)]
    pub enum DurationFormatterError {
        Unknown = 0x00,

        FractionalDigitsOutOfRange = 0x8_01,
        PreviousNumeric = 0x8_02,
        DisplayAlwaysFractional = 0x8_03,
        PreviousFractional = 0x8_04,
        InvalidUnitStyle = 0x8_05,

        DataMarkerNotFound = 0x01,
        DataIdentifierNotFound = 0x02,
        DataInvalidRequest = 0x03,
        DataInconsistentData = 0x04,
        DataDowncast = 0x05,
        DataDeserialize = 0x06,
        DataCustom = 0x07,
        DataIo = 0x08,
    }

    #[diplomat::rust_link(icu::experimental::duration::options::DurationFormatterOptions, Struct)]
    #[diplomat::rust_link(
        icu::experimental::duration::options::DurationFormatterOptions::default,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::experimental::duration::options::FractionalDigits, Enum, hidden)]
    #[diplomat::attr(supports = non_exhaustive_structs, rename = "DurationFormatterOptions")]
    pub struct DurationFormatterOptionsV1 {
        pub base: DurationBaseStyle,
        pub year: DiplomatOption<DurationUnitStyle>,
        pub year_visibility: DiplomatOption<DurationFieldDisplay>,
        pub month: DiplomatOption<DurationUnitStyle>,
        pub month_visibility: DiplomatOption<DurationFieldDisplay>,
        pub week: DiplomatOption<DurationUnitStyle>,
        pub week_visibility: DiplomatOption<DurationFieldDisplay>,
        pub day: DiplomatOption<DurationUnitStyle>,
        pub day_visibility: DiplomatOption<DurationFieldDisplay>,
        pub hour: DiplomatOption<DurationUnitStyle>,
        pub hour_visibility: DiplomatOption<DurationFieldDisplay>,
        pub minute: DiplomatOption<DurationUnitStyle>,
        pub minute_visibility: DiplomatOption<DurationFieldDisplay>,
        pub second: DiplomatOption<DurationUnitStyle>,
        pub second_visibility: DiplomatOption<DurationFieldDisplay>,
        pub millisecond: DiplomatOption<DurationUnitStyle>,
        pub millisecond_visibility: DiplomatOption<DurationFieldDisplay>,
        pub microsecond: DiplomatOption<DurationUnitStyle>,
        pub microsecond_visibility: DiplomatOption<DurationFieldDisplay>,
        pub nanosecond: DiplomatOption<DurationUnitStyle>,
        pub nanosecond_visibility: DiplomatOption<DurationFieldDisplay>,
        pub fractional_digits: DiplomatOption<u8>,
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
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "create")]
        #[cfg(feature = "compiled_data")]
        pub fn create_v1(
            locale: &Locale,
            options: DurationFormatterOptionsV1,
        ) -> Result<Box<DurationFormatter>, DurationFormatterError> {
            let prefs = DurationFormatterPreferences::from(&locale.0);
            let validated = ValidatedDurationFormatterOptions::try_from(options)?;
            Ok(Box::new(DurationFormatter(
                icu_experimental::duration::DurationFormatter::try_new(prefs, validated)?,
            )))
        }

        /// Creates a new [`DurationFormatter`] from locale data and an options bag using a particular data source.
        #[diplomat::rust_link(icu::experimental::duration::DurationFormatter::try_new, FnInStruct)]
        #[diplomat::rust_link(
            icu::experimental::duration::ValidatedDurationFormatterOptions::validate,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "create_with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_v1_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            options: DurationFormatterOptionsV1,
        ) -> Result<Box<DurationFormatter>, DurationFormatterError> {
            let prefs = DurationFormatterPreferences::from(&locale.0);
            let validated = ValidatedDurationFormatterOptions::try_from(options)?;
            Ok(Box::new(DurationFormatter(
                icu_experimental::duration::DurationFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    validated,
                )?,
            )))
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
    }
}

impl From<icu_provider::DataError> for ffi::DurationFormatterError {
    fn from(e: icu_provider::DataError) -> Self {
        match e.kind {
            icu_provider::DataErrorKind::MarkerNotFound => Self::DataMarkerNotFound,
            icu_provider::DataErrorKind::IdentifierNotFound => Self::DataIdentifierNotFound,
            icu_provider::DataErrorKind::InvalidRequest => Self::DataInvalidRequest,
            icu_provider::DataErrorKind::InconsistentData(..) => Self::DataInconsistentData,
            icu_provider::DataErrorKind::Downcast(..) => Self::DataDowncast,
            icu_provider::DataErrorKind::Deserialize => Self::DataDeserialize,
            icu_provider::DataErrorKind::Custom => Self::DataCustom,
            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            icu_provider::DataErrorKind::Io(..) => Self::DataIo,
            _ => Self::Unknown,
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
            microseconds: other.microseconds.into(),
            nanoseconds: other.nanoseconds.into(),
        }
    }
}

impl TryFrom<ffi::DurationFormatterOptionsV1>
    for icu_experimental::duration::ValidatedDurationFormatterOptions
{
    type Error = ffi::DurationFormatterError;

    fn try_from(options: ffi::DurationFormatterOptionsV1) -> Result<Self, Self::Error> {
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
                match $style.into_option() {
                    None => None,
                    Some(DurationUnitStyle::Narrow) => Some($target_enum::Narrow),
                    Some(DurationUnitStyle::Short) => Some($target_enum::Short),
                    Some(DurationUnitStyle::Long) => Some($target_enum::Long),
                    Some(
                        DurationUnitStyle::TwoDigit
                        | DurationUnitStyle::Numeric
                        | DurationUnitStyle::Fractional,
                    ) => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        macro_rules! map_time_style {
            ($style:expr, $target_enum:ident) => {
                match $style.into_option() {
                    None => None,
                    Some(DurationUnitStyle::Narrow) => Some($target_enum::Narrow),
                    Some(DurationUnitStyle::Short) => Some($target_enum::Short),
                    Some(DurationUnitStyle::Long) => Some($target_enum::Long),
                    Some(DurationUnitStyle::TwoDigit) => Some($target_enum::TwoDigit),
                    Some(DurationUnitStyle::Numeric) => Some($target_enum::Numeric),
                    Some(DurationUnitStyle::Fractional) => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        macro_rules! map_subsecond_style {
            ($style:expr, $target_enum:ident) => {
                match $style.into_option() {
                    None => None,
                    Some(DurationUnitStyle::Narrow) => Some($target_enum::Narrow),
                    Some(DurationUnitStyle::Short) => Some($target_enum::Short),
                    Some(DurationUnitStyle::Long) => Some($target_enum::Long),
                    Some(DurationUnitStyle::Numeric | DurationUnitStyle::Fractional) => {
                        Some($target_enum::Numeric)
                    }
                    Some(DurationUnitStyle::TwoDigit) => {
                        return Err(DurationFormatterError::InvalidUnitStyle);
                    }
                }
            };
        }

        let fractional_digits = match options.fractional_digits.into_option() {
            None => FractionalDigits::ShowAll,
            Some(digits) => FractionalDigits::Fixed(digits),
        };

        let mut rust_options = RustOptions::default();
        rust_options.base = options.base.into();
        rust_options.year = map_date_style!(options.year, YearStyle);
        rust_options.year_visibility = options.year_visibility.into_converted_option();
        rust_options.month = map_date_style!(options.month, MonthStyle);
        rust_options.month_visibility = options.month_visibility.into_converted_option();
        rust_options.week = map_date_style!(options.week, WeekStyle);
        rust_options.week_visibility = options.week_visibility.into_converted_option();
        rust_options.day = map_date_style!(options.day, DayStyle);
        rust_options.day_visibility = options.day_visibility.into_converted_option();
        rust_options.hour = map_time_style!(options.hour, HourStyle);
        rust_options.hour_visibility = options.hour_visibility.into_converted_option();
        rust_options.minute = map_time_style!(options.minute, MinuteStyle);
        rust_options.minute_visibility = options.minute_visibility.into_converted_option();
        rust_options.second = map_time_style!(options.second, SecondStyle);
        rust_options.second_visibility = options.second_visibility.into_converted_option();
        rust_options.millisecond = map_subsecond_style!(options.millisecond, MilliSecondStyle);
        rust_options.millisecond_visibility =
            options.millisecond_visibility.into_converted_option();
        rust_options.microsecond = map_subsecond_style!(options.microsecond, MicroSecondStyle);
        rust_options.microsecond_visibility =
            options.microsecond_visibility.into_converted_option();
        rust_options.nanosecond = map_subsecond_style!(options.nanosecond, NanoSecondStyle);
        rust_options.nanosecond_visibility = options.nanosecond_visibility.into_converted_option();
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
