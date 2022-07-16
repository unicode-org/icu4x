// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::Gregorian;
    use icu_datetime::{
        options::{length, preferences},
        DateFormatter, DateTimeFormatter, TimeFormatter,
    };

    use crate::{
        calendar::ffi::ICU4XGregorianDateTime, errors::ffi::ICU4XError, locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    /// An ICU4X TimeFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TimeFormatter, Struct)]
    // TODO(#2153) - Rename to ICU4XTimeFormatter when we remove the dependency on calendar
    // from TimeFormatter.
    pub struct ICU4XGregorianTimeFormatter(pub TimeFormatter<Gregorian>);

    pub enum ICU4XTimeLength {
        Full,
        Long,
        Medium,
        Short,
    }

    pub enum ICU4XHourCyclePreference {
        H24,
        H23,
        H12,
        H11,
        None,
    }

    impl ICU4XGregorianTimeFormatter {
        /// Creates a new [`ICU4XGregorianTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::DateFormatter::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            length: ICU4XTimeLength,
            preferences: ICU4XHourCyclePreference,
        ) -> DiplomatResult<Box<ICU4XGregorianTimeFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.0.as_ref().clone();
            let length = match length {
                ICU4XTimeLength::Full => length::Time::Full,
                ICU4XTimeLength::Long => length::Time::Long,
                ICU4XTimeLength::Medium => length::Time::Medium,
                ICU4XTimeLength::Short => length::Time::Short,
            };
            let preferences = match preferences {
                ICU4XHourCyclePreference::H24 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H24,
                )),
                ICU4XHourCyclePreference::H23 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H23,
                )),
                ICU4XHourCyclePreference::H12 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H12,
                )),
                ICU4XHourCyclePreference::H11 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H11,
                )),
                ICU4XHourCyclePreference::None => None,
            };

            TimeFormatter::try_new(locale, &provider, length, preferences)
                .map(|tf| Box::new(ICU4XGregorianTimeFormatter(tf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_write, FnInStruct)]
        pub fn format_datetime(
            &self,
            value: &ICU4XGregorianDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::DateFormatter, Struct)]
    pub struct ICU4XGregorianDateFormatter(pub DateFormatter<Gregorian>);

    pub enum ICU4XDateLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XGregorianDateFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::DateFormatter::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            length: ICU4XDateLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.0.as_ref().clone();
            let length = match length {
                ICU4XDateLength::Full => length::Date::Full,
                ICU4XDateLength::Long => length::Date::Long,
                ICU4XDateLength::Medium => length::Date::Medium,
                ICU4XDateLength::Short => length::Date::Short,
            };

            DateFormatter::try_new(locale, &provider, length)
                .map(|df| Box::new(ICU4XGregorianDateFormatter(df)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_write, FnInStruct)]
        pub fn format_datetime(
            &self,
            value: &ICU4XGregorianDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    pub struct ICU4XGregorianDateTimeFormatter(pub DateTimeFormatter<Gregorian>);

    impl ICU4XGregorianDateTimeFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
            time_preferences: ICU4XHourCyclePreference,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTimeFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.0.as_ref().clone();
            let date_length = match date_length {
                ICU4XDateLength::Full => length::Date::Full,
                ICU4XDateLength::Long => length::Date::Long,
                ICU4XDateLength::Medium => length::Date::Medium,
                ICU4XDateLength::Short => length::Date::Short,
            };
            let time_length = match time_length {
                ICU4XTimeLength::Full => length::Time::Full,
                ICU4XTimeLength::Long => length::Time::Long,
                ICU4XTimeLength::Medium => length::Time::Medium,
                ICU4XTimeLength::Short => length::Time::Short,
            };

            let mut options = length::Bag::from_date_time_style(date_length, time_length);
            options.preferences = match time_preferences {
                ICU4XHourCyclePreference::H24 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H24,
                )),
                ICU4XHourCyclePreference::H23 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H23,
                )),
                ICU4XHourCyclePreference::H12 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H12,
                )),
                ICU4XHourCyclePreference::H11 => Some(preferences::Bag::from_hour_cycle(
                    preferences::HourCycle::H11,
                )),
                ICU4XHourCyclePreference::None => None,
            };

            DateTimeFormatter::try_new(locale, &provider, &options.into())
                .map(|dtf| Box::new(ICU4XGregorianDateTimeFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format_to_write, FnInStruct)]
        pub fn format_datetime(
            &self,
            value: &ICU4XGregorianDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }
}
