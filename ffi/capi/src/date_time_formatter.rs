// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_calendar::Gregorian;
    use writeable::Writeable;

    use crate::{
        date::ffi::{Date, IsoDate},
        errors::ffi::DateTimeMismatchedCalendarError,
        time::ffi::Time,
    };

    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{
        datetime_helpers::map_or_default,
        datetime_options::ffi::{DateTimeAlignment, DateTimeLength, TimePrecision, YearStyle},
        errors::ffi::DateTimeFormatterLoadError,
        locale_core::ffi::Locale,
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Typedef)]
    pub struct DateTimeFormatter(
        pub  icu_datetime::DateTimeFormatter<
            icu_datetime::fieldsets::enums::CompositeDateTimeFieldSet,
        >,
    );

    impl DateTimeFormatter {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_dt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_dt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_mdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_year_style, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::hm, FnInStruct, hidden)]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_year_style, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_ymdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_det(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_det_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdet(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_mdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_year_style,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdet(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_year_style,
            FnInStruct,
            compact
        )]
        #[cfg(feature = "buffer_provider")]
        pub fn create_ymdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::ET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_et(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::ET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_et_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let value = icu_time::DateTime {
                date: date.0,
                time: time.0,
            };
            let _infallible = self.0.format(&value).write_to(write);
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format_same_calendar, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_same_calendar(
            &self,
            date: &Date,
            time: &Time,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeMismatchedCalendarError> {
            let value = icu_time::DateTime {
                date: date.0.wrap_calendar_in_ref(),
                time: time.0,
            };
            let _infallible = self.0.format_same_calendar(&value)?.write_to(write);
            Ok(())
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter, Typedef)]
    pub struct DateTimeFormatterGregorian(
        pub  icu_datetime::FixedCalendarDateTimeFormatter<
            Gregorian,
            icu_datetime::fieldsets::enums::CompositeDateTimeFieldSet,
        >,
    );

    impl DateTimeFormatterGregorian {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_dt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_dt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_mdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_year_style, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::hm, FnInStruct, hidden)]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdt(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDT::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::with_year_style, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_ymdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_det(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::DET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_det_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdet(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::MDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_mdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_year_style,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdet(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::YMDET::with_year_style,
            FnInStruct,
            compact
        )]
        #[cfg(feature = "buffer_provider")]
        pub fn create_ymdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
            year_style: Option<YearStyle>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision))
                .with_year_style(map_or_default(year_style));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::ET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_alignment, FnInStruct, compact)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::short, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::medium, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::long, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::hm, FnInStruct, hidden)]
        #[cfg(feature = "compiled_data")]
        pub fn create_et(
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?
                    .cast_into_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_length, FnInStruct, compact)]
        #[diplomat::rust_link(
            icu::datetime::fieldsets::ET::with_time_precision,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET::with_alignment, FnInStruct, compact)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_et_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: Option<DateTimeLength>,
            time_precision: Option<TimePrecision>,
            alignment: Option<DateTimeAlignment>,
        ) -> Result<Box<DateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(map_or_default(length))
                .with_alignment(map_or_default(alignment))
                .with_time_precision(map_or_default(time_precision));
            Ok(Box::new(DateTimeFormatterGregorian(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .cast_into_fset(),
            )))
        }

        #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let value = icu_time::DateTime {
                date: date.0.to_calendar(Gregorian),
                time: time.0,
            };
            let _infallible = self.0.format(&value).write_to(write);
        }
    }
}
