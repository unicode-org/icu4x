// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::{DateTime, Gregorian};
    use icu_datetime::{options::length, TypedZonedDateTimeFormatter, ZonedDateTimeFormatter};

    use crate::{
        datetime::ffi::ICU4XDateTime, datetime::ffi::ICU4XIsoDateTime,
        datetime_formatter::ffi::ICU4XDateLength, datetime_formatter::ffi::ICU4XTimeLength,
        errors::ffi::ICU4XError, locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider,
        timezone::ffi::ICU4XCustomTimeZone, timezone_formatter::ffi::ICU4XIsoTimeZoneOptions,
    };

    // TODO(https://github.com/rust-diplomat/diplomat/issues/248)
    #[allow(unused_imports)]
    use crate::{
        timezone_formatter::ffi::ICU4XIsoTimeZoneFormat,
        timezone_formatter::ffi::ICU4XIsoTimeZoneMinuteDisplay,
        timezone_formatter::ffi::ICU4XIsoTimeZoneSecondDisplay,
    };

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter, Struct)]
    pub struct ICU4XGregorianZonedDateTimeFormatter(pub TypedZonedDateTimeFormatter<Gregorian>);

    impl ICU4XGregorianZonedDateTimeFormatter {
        /// Creates a new [`ICU4XGregorianZonedDateTimeFormatter`] from locale data.
        /// 
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> {
            
            let locale = locale.to_datalocale();

            TypedZonedDateTimeFormatter::<Gregorian>::try_new_unstable(
                &provider,
                &locale,
                length::Bag::from_date_time_style(date_length.into(), time_length.into()).into(),
                Default::default(),
            )
            .map(|tf| Box::new(ICU4XGregorianZonedDateTimeFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        /// Creates a new [`ICU4XGregorianZonedDateTimeFormatter`] from locale data.
        /// 
        /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
        /// fallback for the time zone with the given configurations.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new_with_iso_8601_time_zone_fallback(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
            zone_options: ICU4XIsoTimeZoneOptions,
        ) -> DiplomatResult<Box<ICU4XGregorianZonedDateTimeFormatter>, ICU4XError> {
            
            let locale = locale.to_datalocale();

            TypedZonedDateTimeFormatter::<Gregorian>::try_new_unstable(
                &provider,
                &locale,
                length::Bag::from_date_time_style(date_length.into(), time_length.into()).into(),
                zone_options.into(),
            )
            .map(|tf| Box::new(ICU4XGregorianZonedDateTimeFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        /// Formats a [`ICU4XIsoDateTime`] and [`ICU4XCustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TypedZonedDateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::ZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &ICU4XIsoDateTime,
            time_zone: &ICU4XCustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let greg = DateTime::new_from_iso(datetime.0, Gregorian);
            let result = self
                .0
                .format_to_write(write, &greg, &time_zone.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter, Struct)]
    pub struct ICU4XZonedDateTimeFormatter(pub ZonedDateTimeFormatter);

    impl ICU4XZonedDateTimeFormatter {
        /// Creates a new [`ICU4XZonedDateTimeFormatter`] from locale data.
        /// 
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XZonedDateTimeFormatter>, ICU4XError> {
            
            let locale = locale.to_datalocale();

            ZonedDateTimeFormatter::try_new_unstable(
                &provider,
                &locale,
                length::Bag::from_date_time_style(date_length.into(), time_length.into()).into(),
                Default::default(),
            )
            .map(|tf| Box::new(ICU4XZonedDateTimeFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        /// Creates a new [`ICU4XZonedDateTimeFormatter`] from locale data.
        /// 
        /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
        /// fallback for the time zone with the given configurations.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new_with_iso_8601_time_zone_fallback(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
            zone_options: ICU4XIsoTimeZoneOptions,
        ) -> DiplomatResult<Box<ICU4XZonedDateTimeFormatter>, ICU4XError> {
            
            let locale = locale.to_datalocale();

            ZonedDateTimeFormatter::try_new_unstable(
                &provider,
                &locale,
                length::Bag::from_date_time_style(date_length.into(), time_length.into()).into(),
                zone_options.into(),
            )
            .map(|tf| Box::new(ICU4XZonedDateTimeFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        /// Formats a [`ICU4XDateTime`] and [`ICU4XCustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::ZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_datetime_with_custom_time_zone(
            &self,
            datetime: &ICU4XDateTime,
            time_zone: &ICU4XCustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &datetime.0, &time_zone.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }

        /// Formats a [`ICU4XIsoDateTime`] and [`ICU4XCustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::ZonedDateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::ZonedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &ICU4XIsoDateTime,
            time_zone: &ICU4XCustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &datetime.0.to_any(), &time_zone.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }
}
