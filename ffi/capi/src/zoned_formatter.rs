// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(*, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        datetime::ffi::DateTime, datetime::ffi::IsoDateTime, datetime_formatter::ffi::DateLength,
        datetime_formatter::ffi::TimeLength, errors::ffi::Error, locale_core::ffi::Locale,
        provider::ffi::DataProvider, timezone::ffi::CustomTimeZone,
        timezone_formatter::ffi::IsoTimeZoneOptions,
    };

    use writeable::Writeable;

    // TODO(https://github.com/rust-diplomat/diplomat/issues/248)
    #[allow(unused_imports)]
    use crate::{
        timezone_formatter::ffi::IsoTimeZoneFormat,
        timezone_formatter::ffi::IsoTimeZoneMinuteDisplay,
        timezone_formatter::ffi::IsoTimeZoneSecondDisplay,
    };

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter, Struct)]
    pub struct GregorianZonedDateTimeFormatter(
        pub icu_datetime::TypedZonedDateTimeFormatter<icu_calendar::Gregorian>,
    );

    impl GregorianZonedDateTimeFormatter {
        /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_lengths")]
        pub fn create_with_lengths(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
        ) -> Result<Box<GregorianZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(GregorianZonedDateTimeFormatter(
                call_constructor!(
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new,
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new_with_any_provider,
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new_with_buffer_provider,
                    provider,
                    &locale,
                    icu_datetime::options::length::Bag::from_date_time_style(date_length.into(), time_length.into())
                        .into(),
                    Default::default(),
                )?,
            )))
        }

        /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
        /// fallback for the time zone with the given configurations.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_lengths_and_iso_8601_time_zone_fallback")]
        pub fn create_with_lengths_and_iso_8601_time_zone_fallback(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
            zone_options: IsoTimeZoneOptions,
        ) -> Result<Box<GregorianZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(GregorianZonedDateTimeFormatter(
                call_constructor!(
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new,
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new_with_any_provider,
                    icu_datetime::TypedZonedDateTimeFormatter::<icu_calendar::Gregorian>::try_new_with_buffer_provider,
                    provider,
                    &locale,
                    icu_datetime::options::length::Bag::from_date_time_style(date_length.into(), time_length.into())
                        .into(),
                    zone_options.into(),
                )?,
            )))
        }

        /// Formats a [`IsoDateTime`] and [`CustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::TypedZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let greg = icu_calendar::DateTime::new_from_iso(datetime.0, icu_calendar::Gregorian);
            let _infallible = self.0.format(&greg, &time_zone.0).write_to(write);
        }
    }

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedZonedDateTime, Struct, hidden)]
    pub struct ZonedDateTimeFormatter(pub icu_datetime::ZonedDateTimeFormatter);

    impl ZonedDateTimeFormatter {
        /// Creates a new [`ZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_lengths")]
        pub fn create_with_lengths(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
        ) -> Result<Box<ZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(ZonedDateTimeFormatter(call_constructor!(
                icu_datetime::ZonedDateTimeFormatter::try_new,
                icu_datetime::ZonedDateTimeFormatter::try_new_with_any_provider,
                icu_datetime::ZonedDateTimeFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                icu_datetime::options::length::Bag::from_date_time_style(
                    date_length.into(),
                    time_length.into()
                )
                .into(),
                Default::default(),
            )?)))
        }

        /// Creates a new [`ZonedDateTimeFormatter`] from locale data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
        /// fallback for the time zone with the given configurations.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_lengths_and_iso_8601_time_zone_fallback")]
        pub fn create_with_lengths_and_iso_8601_time_zone_fallback(
            provider: &DataProvider,
            locale: &Locale,
            date_length: DateLength,
            time_length: TimeLength,
            zone_options: IsoTimeZoneOptions,
        ) -> Result<Box<ZonedDateTimeFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(ZonedDateTimeFormatter(call_constructor!(
                icu_datetime::ZonedDateTimeFormatter::try_new,
                icu_datetime::ZonedDateTimeFormatter::try_new_with_any_provider,
                icu_datetime::ZonedDateTimeFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                icu_datetime::options::length::Bag::from_date_time_style(
                    date_length.into(),
                    time_length.into()
                )
                .into(),
                zone_options.into(),
            )?)))
        }

        /// Formats a [`DateTime`] and [`CustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::ZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(icu::datetime::FormattedZonedDateTime::write_to, FnInStruct, hidden)]
        pub fn format_datetime_with_custom_time_zone(
            &self,
            datetime: &DateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self.0.format(&datetime.0, &time_zone.0)?.write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] and [`CustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::ZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            let _infallible = self
                .0
                .format(&datetime.0.to_any(), &time_zone.0)?
                .write_to(write);
            Ok(())
        }
    }
}
