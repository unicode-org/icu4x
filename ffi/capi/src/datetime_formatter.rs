// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        date::ffi::{Date, IsoDate},
        datetime::ffi::{DateTime, IsoDateTime},
        errors::ffi::Error,
        locale_core::ffi::Locale,
        provider::ffi::DataProvider,
        time::ffi::Time,
    };

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X TimeFormatter object capable of formatting an [`Time`] type (and others) as a string
    #[diplomat::rust_link(icu::datetime::TimeFormatter, Struct)]
    pub struct TimeFormatter(pub icu_datetime::TimeFormatter);

    #[diplomat::enum_convert(icu_datetime::options::length::Time, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::options::length::Time, Enum)]
    pub enum TimeLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl TimeFormatter {
        /// Creates a new [`TimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::try_new_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_length")]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: TimeLength,
        ) -> Result<Box<TimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(TimeFormatter(call_constructor!(
                icu_datetime::TimeFormatter::try_new_with_length,
                icu_datetime::TimeFormatter::try_new_with_length_with_any_provider,
                icu_datetime::TimeFormatter::try_new_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }

        /// Formats a [`Time`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_time(&self, value: &Time, write: &mut diplomat_runtime::DiplomatWrite) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }

        /// Formats a [`DateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }

        /// Formats a [`IsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateFormatter object capable of formatting a [`IsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateFormatter, Struct)]
    pub struct GregorianDateFormatter(
        pub icu_datetime::TypedDateFormatter<icu_calendar::Gregorian>,
    );

    #[diplomat::enum_convert(icu_datetime::options::length::Date, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::options::length::Date, Enum)]
    pub enum DateLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl GregorianDateFormatter {
        /// Creates a new [`GregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::try_new_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_length")]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateLength,
        ) -> Result<Box<GregorianDateFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(GregorianDateFormatter(call_constructor!(
                icu_datetime::TypedDateFormatter::try_new_with_length,
                icu_datetime::TypedDateFormatter::try_new_with_length_with_any_provider,
                icu_datetime::TypedDateFormatter::try_new_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }

        /// Formats a [`IsoDate`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_date(
            &self,
            value: &IsoDate,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::Date::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).write_to(write);
        }
        /// Formats a [`IsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateTimeFormatter object capable of formatting a [`IsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter, Struct)]
    pub struct GregorianDateTimeFormatter(
        pub icu_datetime::TypedDateTimeFormatter<icu_calendar::Gregorian>,
    );

    impl GregorianDateTimeFormatter {
        /// Creates a new [`GregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_lengths")]
        pub fn create_with_lengths(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
        ) -> Result<Box<GregorianDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            let options = icu_datetime::options::length::Bag::from_date_time_style(
                date_length.into(),
                time_length.into(),
            );

            Ok(Box::new(GregorianDateTimeFormatter(call_constructor!(
                icu_datetime::TypedDateTimeFormatter::try_new,
                icu_datetime::TypedDateTimeFormatter::try_new_with_any_provider,
                icu_datetime::TypedDateTimeFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options.into()
            )?)))
        }

        /// Formats a [`IsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime::DateFormatter, Struct)]
    pub struct DateFormatter(pub icu_datetime::DateFormatter);

    impl DateFormatter {
        /// Creates a new [`DateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateFormatter::try_new_with_length, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_length")]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
        ) -> Result<Box<DateFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(DateFormatter(call_constructor!(
                icu_datetime::DateFormatter::try_new_with_length,
                icu_datetime::DateFormatter::try_new_with_length_with_any_provider,
                icu_datetime::DateFormatter::try_new_with_length_with_buffer_provider,
                provider,
                &locale,
                date_length.into()
            )?)))
        }

        /// Formats a [`Date`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_date(
            &self,
            value: &Date,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.format(&value.0)?.write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDate`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_date(
            &self,
            value: &IsoDate,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.format(&any)?.write_to(write);
            Ok(())
        }

        /// Formats a [`DateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.format(&value.0)?.write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.format(&any)?.write_to(write);
            Ok(())
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`DateTime`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
    pub struct DateTimeFormatter(pub icu_datetime::DateTimeFormatter);

    impl DateTimeFormatter {
        /// Creates a new [`DateTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_lengths")]
        pub fn create_with_lengths(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
        ) -> Result<Box<DateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = icu_datetime::options::length::Bag::from_date_time_style(
                date_length.into(),
                time_length.into(),
            );

            Ok(Box::new(DateTimeFormatter(call_constructor!(
                icu_datetime::DateTimeFormatter::try_new,
                icu_datetime::DateTimeFormatter::try_new_with_any_provider,
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options.into()
            )?)))
        }

        /// Formats a [`DateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::DateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::write_to, FnInStruct, hidden)]
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.format(&value.0)?.write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::DateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.format(&any)?.write_to(write);
            Ok(())
        }
    }
}
