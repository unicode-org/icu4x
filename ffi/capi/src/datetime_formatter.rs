// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_datetime::{
        neo::NeoOptions,
        neo_marker::{NeoHourMinuteMarker, NeoYearMonthDayHourMinuteMarker, NeoYearMonthDayMarker},
        neo_skeleton::NeoSkeletonLength,
        NeverCalendar,
    };

    use crate::{
        date::ffi::{Date, IsoDate},
        datetime::ffi::{DateTime, IsoDateTime},
        errors::ffi::Error,
        locale_core::ffi::Locale,
        provider::ffi::DataProvider,
        time::ffi::Time,
    };

    use writeable::TryWriteable;

    #[diplomat::opaque]
    /// An ICU4X TimeFormatter object capable of formatting an [`Time`] type (and others) as a string
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct TimeFormatter(
        pub icu_datetime::neo::TypedNeoFormatter<NeverCalendar, NeoHourMinuteMarker>,
    );

    #[diplomat::enum_convert(icu_datetime::neo_skeleton::NeoSkeletonLength, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::neo_skeleton::NeoSkeletonLength, Enum)]
    pub enum DateTimeLength {
        Long,
        Medium,
        Short,
    }

    impl TimeFormatter {
        /// Creates a new [`TimeFormatter`] from locale data.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<TimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(TimeFormatter(call_constructor!(
                icu_datetime::neo::TypedNeoFormatter::try_new,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options
            )?)))
        }

        /// Formats a [`Time`] to a string.
        pub fn format_time(&self, value: &Time, write: &mut diplomat_runtime::DiplomatWrite) {
            let _infallible = self.0.format(&value.0).try_write_to(write);
        }

        /// Formats a [`DateTime`] to a string.
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0.time).try_write_to(write);
        }

        /// Formats a [`IsoDateTime`] to a string.
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0.time).try_write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateFormatter object capable of formatting a [`IsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct GregorianDateFormatter(
        pub icu_datetime::neo::TypedNeoFormatter<icu_calendar::Gregorian, NeoYearMonthDayMarker>,
    );

    impl GregorianDateFormatter {
        /// Creates a new [`GregorianDateFormatter`] from locale data.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<GregorianDateFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(GregorianDateFormatter(call_constructor!(
                icu_datetime::neo::TypedNeoFormatter::try_new,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options
            )?)))
        }

        /// Formats a [`IsoDate`] to a string.
        pub fn format_iso_date(
            &self,
            value: &IsoDate,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::Date::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).try_write_to(write);
        }
        /// Formats a [`IsoDateTime`] to a string.
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).try_write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateTimeFormatter object capable of formatting a [`IsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct GregorianDateTimeFormatter(
        pub  icu_datetime::neo::TypedNeoFormatter<
            icu_calendar::Gregorian,
            NeoYearMonthDayHourMinuteMarker,
        >,
    );

    impl GregorianDateTimeFormatter {
        /// Creates a new [`GregorianDateFormatter`] from locale data.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<GregorianDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(GregorianDateTimeFormatter(call_constructor!(
                icu_datetime::neo::TypedNeoFormatter::try_new,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::TypedNeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options
            )?)))
        }

        /// Formats a [`IsoDateTime`] to a string.
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(value.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg).try_write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct DateFormatter(pub icu_datetime::neo::NeoFormatter<NeoYearMonthDayMarker>);

    impl DateFormatter {
        /// Creates a new [`DateFormatter`] from locale data.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<DateFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(DateFormatter(call_constructor!(
                icu_datetime::neo::NeoFormatter::try_new,
                icu_datetime::neo::NeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::NeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options
            )?)))
        }

        /// Formats a [`Date`] to a string.
        pub fn format_date(
            &self,
            value: &Date,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.convert_and_format(&value.0).try_write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDate`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        pub fn format_iso_date(
            &self,
            value: &IsoDate,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.convert_and_format(&any).try_write_to(write);
            Ok(())
        }

        /// Formats a [`DateTime`] to a string.
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.convert_and_format(&value.0).try_write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.convert_and_format(&any).try_write_to(write);
            Ok(())
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`DateTime`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct DateTimeFormatter(
        pub icu_datetime::neo::NeoFormatter<NeoYearMonthDayHourMinuteMarker>,
    );

    impl DateTimeFormatter {
        /// Creates a new [`DateTimeFormatter`] from locale data.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<DateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(DateTimeFormatter(call_constructor!(
                icu_datetime::neo::NeoFormatter::try_new,
                icu_datetime::neo::NeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::NeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options,
            )?)))
        }

        /// Formats a [`DateTime`] to a string.
        pub fn format_datetime(
            &self,
            value: &DateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.convert_and_format(&value.0).try_write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        pub fn format_iso_datetime(
            &self,
            value: &IsoDateTime,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let any = value.0.to_any();
            let _infallible = self.0.convert_and_format(&any).try_write_to(write);
            Ok(())
        }
    }
}
