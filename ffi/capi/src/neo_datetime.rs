// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_calendar::Gregorian;
    use writeable::{TryWriteable, Writeable};

    use crate::{
        date::ffi::{Date, IsoDate},
        errors::ffi::{DateTimeMismatchedCalendarError, DateTimeWriteError},
        time::ffi::Time,
        timezone::ffi::TimeZoneInfo,
    };

    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{
        datetime_formatter::ffi::DateTimeLength,
        datetime_options::ffi::{DateTimeAlignment, TimePrecision, YearStyle},
        errors::ffi::DateTimeFormatterLoadError,
        locale_core::ffi::Locale,
        neo_datetime::impls::{formatter_with_zone, gregorian_formatter_with_zone, map_or_default},
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
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    #[diplomat::rust_link(
        icu::datetime::fieldsets::enums::DateAndTimeFieldSet::zone,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::datetime::fieldsets::DT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::DET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::ET::zone, FnInStruct, hidden)]
    #[diplomat::attr(demo_gen, disable)] // constructors are on a different type :(
    pub struct ZonedDateTimeFormatter(
        pub icu_datetime::DateTimeFormatter<icu_datetime::fieldsets::enums::CompositeFieldSet>,
    );

    impl ZonedDateTimeFormatter {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    names.as_mut().include_time_zone_generic_short_names()?;
                    names.as_mut().include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    names
                        .as_mut()
                        .load_time_zone_generic_short_names(&provider)?;
                    names.as_mut().load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    names.as_mut().include_time_zone_generic_long_names()?;
                    names.as_mut().include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    names
                        .as_mut()
                        .load_time_zone_generic_long_names(&provider)?;
                    names.as_mut().load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    names.as_mut().include_time_zone_specific_short_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    names
                        .as_mut()
                        .load_time_zone_specific_short_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    names.as_mut().include_time_zone_specific_long_names()?;
                    names.as_mut().include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    names
                        .as_mut()
                        .load_time_zone_specific_long_names(&provider)?;
                    names.as_mut().load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.as_mut().include_time_zone_essentials()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.as_mut().load_time_zone_essentials(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeWriteError> {
            let date = date.0.to_calendar(self.0.calendar());
            let mut input = icu_datetime::DateTimeInputUnchecked::default();
            input.set_date_fields(date);
            input.set_time_fields(time.0);
            input.set_time_zone_id(zone.time_zone_id);
            if let Some(offset) = zone.offset {
                input.set_time_zone_utc_offset(offset);
            }
            if let Some(local_time) = zone.local_time {
                input.set_time_zone_local_time(local_time);
            }
            if let Some(zone_variant) = zone.zone_variant {
                input.set_time_zone_variant(zone_variant);
            }
            let _infallible = self
                .0
                .format_unchecked(input)
                .try_write_to(write)
                .ok()
                .transpose()?;
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

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter, Struct)]
    #[diplomat::rust_link(
        icu::datetime::fieldsets::enums::DateAndTimeFieldSet::zone,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::datetime::fieldsets::DT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::DET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::ET::zone, FnInStruct, hidden)]
    #[diplomat::attr(demo_gen, disable)] // constructors are on a different type :(
    pub struct ZonedDateTimeFormatterGregorian(
        pub  icu_datetime::FixedCalendarDateTimeFormatter<
            Gregorian,
            icu_datetime::fieldsets::enums::CompositeFieldSet,
        >,
    );

    impl ZonedDateTimeFormatterGregorian {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    names.include_time_zone_generic_short_names()?;
                    names.include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    names.load_time_zone_generic_short_names(&provider)?;
                    names.load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    names.include_time_zone_generic_long_names()?;
                    names.include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    names.load_time_zone_generic_long_names(&provider)?;
                    names.load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    names.include_time_zone_specific_short_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    names.load_time_zone_specific_short_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    names.include_time_zone_specific_long_names()?;
                    names.include_time_zone_location_names()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    names.load_time_zone_specific_long_names(&provider)?;
                    names.load_time_zone_location_names(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    names.include_time_zone_essentials()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            gregorian_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    // NOTE: Keep this in sync with RawDateTimeNames::load_for_pattern
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_essentials(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeWriteError> {
            let date = date.0.to_calendar(Gregorian);
            let mut input = icu_datetime::DateTimeInputUnchecked::default();
            input.set_date_fields(date);
            input.set_time_fields(time.0);
            input.set_time_zone_id(zone.time_zone_id);
            if let Some(offset) = zone.offset {
                input.set_time_zone_utc_offset(offset);
            }
            if let Some(local_time) = zone.local_time {
                input.set_time_zone_local_time(local_time);
            }
            if let Some(zone_variant) = zone.zone_variant {
                input.set_time_zone_variant(zone_variant);
            }
            let _infallible = self
                .0
                .format_unchecked(input)
                .try_write_to(write)
                .ok()
                .transpose()?;
            Ok(())
        }
    }
}

mod impls {
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use alloc::boxed::Box;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use icu_calendar::Gregorian;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use icu_datetime::{
        fieldsets::enums::*, fieldsets::Combo, pattern::*, scaffold::*, DateTimeFormatter,
        DateTimeFormatterLoadError, FixedCalendarDateTimeFormatter,
    };

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    pub(super) fn map_or_default<Input, Output>(input: Option<Input>) -> Output
    where
        Output: From<Input> + Default,
    {
        input.map(Output::from).unwrap_or_default()
    }

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    pub(super) fn formatter_with_zone<Zone>(
        formatter: &DateTimeFormatter<CompositeDateTimeFieldSet>,
        locale: &crate::locale_core::ffi::Locale,
        zone: Zone,
        load: impl FnOnce(
            &mut DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
        ) -> Result<(), PatternLoadError>,
        to_formatter: impl FnOnce(
            DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
            Combo<DateAndTimeFieldSet, Zone>,
        ) -> Result<
            DateTimeFormatter<Combo<DateAndTimeFieldSet, Zone>>,
            (
                DateTimeFormatterLoadError,
                DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
            ),
        >,
    ) -> Result<
        Box<super::ffi::ZonedDateTimeFormatter>,
        crate::errors::ffi::DateTimeFormatterLoadError,
    >
    where
        Zone: DateTimeMarkers + ZoneMarkers,
        <Zone as DateTimeMarkers>::Z: ZoneMarkers,
        Combo<DateAndTimeFieldSet, Zone>: DateTimeNamesFrom<CompositeDateTimeFieldSet>,
        CompositeFieldSet: DateTimeNamesFrom<Combo<DateAndTimeFieldSet, Zone>>,
    {
        let prefs = (&locale.0).into();
        let mut names = DateTimeNames::from_formatter(prefs, formatter.clone())
            .cast_into_fset::<Combo<DateAndTimeFieldSet, Zone>>();
        load(&mut names)?;
        let field_set = formatter
            .to_field_set_builder()
            .build_date_and_time()
            .map_err(|e| {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::errors::ffi::DateTimeFormatterLoadError::Unknown
            })?
            .zone(zone);
        let formatter = to_formatter(names, field_set)
            // This can fail if the locale doesn't match and the fields conflict
            .map_err(|(e, _)| e)?
            .cast_into_fset();
        Ok(Box::new(super::ffi::ZonedDateTimeFormatter(formatter)))
    }

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    pub(super) fn gregorian_formatter_with_zone<Zone>(
        formatter: &FixedCalendarDateTimeFormatter<Gregorian, CompositeDateTimeFieldSet>,
        locale: &crate::locale_core::ffi::Locale,
        zone: Zone,
        load: impl FnOnce(
            &mut FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
        ) -> Result<(), PatternLoadError>,
        to_formatter: impl FnOnce(
            FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
            Combo<DateAndTimeFieldSet, Zone>,
        ) -> Result<
            FixedCalendarDateTimeFormatter<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
            (
                DateTimeFormatterLoadError,
                FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
            ),
        >,
    ) -> Result<
        Box<super::ffi::ZonedDateTimeFormatterGregorian>,
        crate::errors::ffi::DateTimeFormatterLoadError,
    >
    where
        Zone: DateTimeMarkers + ZoneMarkers,
        <Zone as DateTimeMarkers>::Z: ZoneMarkers,
        Combo<DateAndTimeFieldSet, Zone>: DateTimeNamesFrom<CompositeDateTimeFieldSet>,
        CompositeFieldSet: DateTimeNamesFrom<Combo<DateAndTimeFieldSet, Zone>>,
    {
        let prefs = (&locale.0).into();
        let mut names = FixedCalendarDateTimeNames::from_formatter(prefs, formatter.clone())
            .cast_into_fset::<Combo<DateAndTimeFieldSet, Zone>>();
        load(&mut names)?;
        let field_set = formatter
            .to_field_set_builder()
            .build_date_and_time()
            .map_err(|e| {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::errors::ffi::DateTimeFormatterLoadError::Unknown
            })?
            .zone(zone);
        let formatter = to_formatter(names, field_set)
            // This can fail if the locale doesn't match and the fields conflict
            .map_err(|(e, _)| e)?
            .cast_into_fset();
        Ok(Box::new(super::ffi::ZonedDateTimeFormatterGregorian(
            formatter,
        )))
    }
}
