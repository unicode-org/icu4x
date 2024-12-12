// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_datetime::{fieldsets::YMDTV, options::Length};
    use icu_timezone::ZoneVariant;

    use crate::{
        datetime::ffi::{DateTime, IsoDateTime},
        datetime_formatter::ffi::DateTimeLength,
        errors::ffi::{DateTimeFormatError, DateTimeFormatterLoadError},
        locale_core::ffi::Locale,
        provider::ffi::DataProvider,
        timezone::ffi::TimeZoneInfo,
    };

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct GregorianZonedDateTimeFormatter(
        pub icu_datetime::FixedCalendarDateTimeFormatter<icu_calendar::Gregorian, YMDTV>,
    );

    impl GregorianZonedDateTimeFormatter {
        /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data using compiled data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_with_length(
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<GregorianZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = YMDTV::with_length(Length::from(length));

            Ok(Box::new(GregorianZonedDateTimeFormatter(
                icu_datetime::FixedCalendarDateTimeFormatter::try_new(prefs, options)?,
            )))
        }
        /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data using a particular data source.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length_and_provider")]
        pub fn create_with_length_and_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<GregorianZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = YMDTV::with_length(Length::from(length));

            Ok(Box::new(GregorianZonedDateTimeFormatter(
                call_constructor!(
                    icu_datetime::FixedCalendarDateTimeFormatter::try_new_with_buffer_provider,
                    provider,
                    prefs,
                    options
                )?,
            )))
        }
        /// Formats a [`IsoDateTime`] and [`TimeZoneInfo`] to a string.
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeFormatError> {
            let greg = icu_calendar::DateTime::new_from_iso(datetime.0, icu_calendar::Gregorian);
            let zdt = icu_timezone::CustomZonedDateTime {
                date: greg.date,
                time: greg.time,
                zone: time_zone
                    .time_zone_id
                    .with_offset(time_zone.offset)
                    .at_time((datetime.0.date, datetime.0.time))
                    .with_zone_variant(time_zone.zone_variant.unwrap_or(ZoneVariant::Standard)),
            };
            let _infallible = self.0.format(&zdt).write_to(write);
            Ok(())
        }
    }

    #[diplomat::opaque]
    /// An object capable of formatting a date time with time zone to a string.
    #[diplomat::rust_link(icu::datetime, Mod)]
    pub struct ZonedDateTimeFormatter(pub icu_datetime::DateTimeFormatter<YMDTV>);

    impl ZonedDateTimeFormatter {
        /// Creates a new [`ZonedDateTimeFormatter`] from locale data using compiled data.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length")]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_with_length(
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = YMDTV::with_length(Length::from(length));

            Ok(Box::new(ZonedDateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?,
            )))
        }
        /// Creates a new [`ZonedDateTimeFormatter`] from locale data using a particular data source.
        ///
        /// This function has `date_length` and `time_length` arguments and uses default options
        /// for the time zone.
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_length_and_provider")]
        pub fn create_with_length_and_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = YMDTV::with_length(Length::from(length));

            Ok(Box::new(ZonedDateTimeFormatter(call_constructor!(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider,
                provider,
                prefs,
                options,
            )?)))
        }
        /// Formats a [`DateTime`] and [`TimeZoneInfo`] to a string.
        pub fn format_datetime_with_custom_time_zone(
            &self,
            datetime: &DateTime,
            time_zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeFormatError> {
            let zdt = icu_timezone::CustomZonedDateTime {
                date: datetime.0.date.clone(),
                time: datetime.0.time,
                zone: time_zone
                    .time_zone_id
                    .with_offset(time_zone.offset)
                    .at_time((datetime.0.date.to_iso(), datetime.0.time))
                    .with_zone_variant(
                        time_zone
                            .zone_variant
                            .ok_or(DateTimeFormatError::ZoneInfoMissingFields)?,
                    ),
            };
            let _infallible = self.0.format_any_calendar(&zdt).write_to(write);
            Ok(())
        }

        /// Formats a [`IsoDateTime`] and [`TimeZoneInfo`] to a string.
        pub fn format_iso_datetime_with_custom_time_zone(
            &self,
            datetime: &IsoDateTime,
            time_zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeFormatError> {
            let zdt = icu_timezone::CustomZonedDateTime {
                date: datetime.0.date,
                time: datetime.0.time,
                zone: time_zone
                    .time_zone_id
                    .with_offset(time_zone.offset)
                    .at_time((datetime.0.date, datetime.0.time))
                    .with_zone_variant(time_zone.zone_variant.unwrap_or(ZoneVariant::Standard)),
            };
            let _infallible = self.0.format_any_calendar(&zdt).write_to(write);
            Ok(())
        }
    }
}
