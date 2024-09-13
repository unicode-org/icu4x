// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_datetime::{
        neo::NeoOptions, neo_marker::NeoYearMonthDayHourMinuteSecondTimeZoneGenericShortMarker,
        neo_skeleton::NeoSkeletonLength,
    };

    use crate::{
        datetime::ffi::DateTime, datetime::ffi::IsoDateTime,
        datetime_formatter::ffi::DateTimeLength, errors::ffi::Error, locale_core::ffi::Locale,
        provider::ffi::DataProvider, timezone::ffi::CustomTimeZone,
    };

    use writeable::TryWriteable;

    // TODO(https://github.com/rust-diplomat/diplomat/issues/248)
    #[allow(unused_imports)]
    use crate::{
        timezone_formatter::ffi::IsoTimeZoneFormat,
        timezone_formatter::ffi::IsoTimeZoneMinuteDisplay,
        timezone_formatter::ffi::IsoTimeZoneSecondDisplay,
    };

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct GregorianZonedDateTimeFormatter(
        pub  icu_datetime::neo::TypedNeoFormatter<
            icu_calendar::Gregorian,
            NeoYearMonthDayHourMinuteSecondTimeZoneGenericShortMarker,
        >,
    );

    impl GregorianZonedDateTimeFormatter {
        /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<GregorianZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(GregorianZonedDateTimeFormatter(
                call_constructor!(
                    icu_datetime::neo::TypedNeoFormatter::try_new,
                    icu_datetime::neo::TypedNeoFormatter::try_new_with_any_provider,
                    icu_datetime::neo::TypedNeoFormatter::try_new_with_buffer_provider,
                    provider,
                    &locale,
                    options
                )?,
            )))
        }

        /// Formats a [`IsoDateTime`] and [`CustomTimeZone`] to a string.
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(datetime.0, icu_calendar::Gregorian);
            let zdt = icu_timezone::CustomZonedDateTime {
                date: greg.date,
                time: greg.time,
                zone: time_zone.0,
            };
            let _infallible = self.0.format(&zdt).try_write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct ZonedDateTimeFormatter(
        pub  icu_datetime::neo::NeoFormatter<
            NeoYearMonthDayHourMinuteSecondTimeZoneGenericShortMarker,
        >,
    );

    impl ZonedDateTimeFormatter {
        /// Creates a new [`ZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<ZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();
            let options = NeoOptions::from(NeoSkeletonLength::from(length));

            Ok(Box::new(ZonedDateTimeFormatter(call_constructor!(
                icu_datetime::neo::NeoFormatter::try_new,
                icu_datetime::neo::NeoFormatter::try_new_with_any_provider,
                icu_datetime::neo::NeoFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options,
            )?)))
        }

        /// Formats a [`DateTime`] and [`CustomTimeZone`] to a string.
        pub fn format_datetime_with_custom_time_zone(
            &self,
            datetime: &DateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let zdt = icu_timezone::CustomZonedDateTime {
                date: datetime.0.date.clone(),
                time: datetime.0.time,
                zone: time_zone.0,
            };
            let _infallible = self.0.convert_and_format(&zdt).try_write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] and [`CustomTimeZone`] to a string.
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let zdt = icu_timezone::CustomZonedDateTime {
                date: datetime.0.date,
                time: datetime.0.time,
                zone: time_zone.0,
            };
            let _infallible = self.0.convert_and_format(&zdt).try_write_to(write);
            Ok(())
        }
    }
}
