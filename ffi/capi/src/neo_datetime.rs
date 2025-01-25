// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::options::FractionalSecondDigits;

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use icu_datetime::fieldsets::enums::CompositeDateTimeFieldSet;
    use writeable::Writeable;

    use crate::{date::ffi::IsoDate, time::ffi::Time};

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{errors::ffi::DateTimeFormatterLoadError, locale_core::ffi::Locale};

    #[diplomat::enum_convert(icu_datetime::options::Length, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::Length, Enum)]
    pub enum NeoDateTimeLength {
        Long,
        Medium,
        Short,
    }

    #[diplomat::enum_convert(icu_datetime::options::Alignment, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::Alignment, Enum)]
    pub enum DateTimeAlignment {
        Auto,
        Column,
    }

    #[diplomat::rust_link(icu::datetime::TimePrecision, Enum)]
    pub enum TimePrecision {
        Hour,
        Minute,
        MinuteOptional,
        Second,
        SecondF1,
        SecondF2,
        SecondF3,
        SecondF4,
        SecondF5,
        SecondF6,
        SecondF7,
        SecondF8,
        SecondF9,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Typedef)]
    pub struct NeoDateTimeFormatter(pub icu_datetime::DateTimeFormatter<CompositeDateTimeFieldSet>);

    impl NeoDateTimeFormatter {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_dt(
            locale: &Locale,
            length: NeoDateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<NeoDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(NeoDateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdt(
            locale: &Locale,
            length: NeoDateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<NeoDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(NeoDateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        pub fn format_iso(&self, date: &IsoDate, time: &Time, write: &mut diplomat_runtime::DiplomatWrite) {
            let value = icu_timezone::DateTime {
                date: date.0,
                time: time.0
            };
            let _infallible = self.0.format(&value).write_to(write);
        }
    }
}

impl From<ffi::TimePrecision> for icu_datetime::options::TimePrecision {
    fn from(time_precision: ffi::TimePrecision) -> Self {
        use icu_datetime::options::TimePrecision;
        match time_precision {
            ffi::TimePrecision::Hour => TimePrecision::Hour,
            ffi::TimePrecision::Minute => TimePrecision::Minute,
            ffi::TimePrecision::MinuteOptional => TimePrecision::MinuteOptional,
            ffi::TimePrecision::Second => TimePrecision::Second,
            ffi::TimePrecision::SecondF1 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F1)
            }
            ffi::TimePrecision::SecondF2 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F2)
            }
            ffi::TimePrecision::SecondF3 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F3)
            }
            ffi::TimePrecision::SecondF4 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F4)
            }
            ffi::TimePrecision::SecondF5 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F5)
            }
            ffi::TimePrecision::SecondF6 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F6)
            }
            ffi::TimePrecision::SecondF7 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F7)
            }
            ffi::TimePrecision::SecondF8 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F8)
            }
            ffi::TimePrecision::SecondF9 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F9)
            }
        }
    }
}
