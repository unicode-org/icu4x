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
        DateFormat, DateTimeFormat, TimeFormat,
    };

    use crate::{
        calendar::ffi::ICU4XGregorianDateTime, errors::ffi::ICU4XError, locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    /// An ICU4X TimeFormat object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TimeFormat, Struct)]
    pub struct ICU4XGregorianTimeFormat(pub TimeFormat<Gregorian>);

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

    impl ICU4XGregorianTimeFormat {
        /// Creates a new [`ICU4XGregorianTimeFormat`] from locale data.
        #[diplomat::rust_link(icu::decimal::DateFormat::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            length: ICU4XTimeLength,
            preferences: ICU4XHourCyclePreference,
        ) -> DiplomatResult<Box<ICU4XGregorianTimeFormat>, ICU4XError> {
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

            TimeFormat::try_new(locale, &provider, length, preferences)
                .map(|tf| Box::new(ICU4XGregorianTimeFormat(tf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormat::format_to_write, FnInStruct)]
        pub fn format_to_write(
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
    /// An ICU4X DateFormat object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::DateFormat, Struct)]
    pub struct ICU4XGregorianDateFormat(pub DateFormat<Gregorian>);

    pub enum ICU4XDateLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XGregorianDateFormat {
        /// Creates a new [`ICU4XGregorianDateFormat`] from locale data.
        #[diplomat::rust_link(icu::decimal::DateFormat::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            length: ICU4XDateLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateFormat>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.0.as_ref().clone();
            let length = match length {
                ICU4XDateLength::Full => length::Date::Full,
                ICU4XDateLength::Long => length::Date::Long,
                ICU4XDateLength::Medium => length::Date::Medium,
                ICU4XDateLength::Short => length::Date::Short,
            };

            DateFormat::try_new(locale, &provider, length)
                .map(|df| Box::new(ICU4XGregorianDateFormat(df)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormat::format_to_write, FnInStruct)]
        pub fn format_to_write(
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
    /// An ICU4X DateFormat object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::DateTimeFormat, Struct)]
    pub struct ICU4XGregorianDateTimeFormat(pub DateTimeFormat<Gregorian>);

    impl ICU4XGregorianDateTimeFormat {
        /// Creates a new [`ICU4XGregorianDateFormat`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateTimeFormat::try_new, FnInStruct)]
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
            time_preferences: ICU4XHourCyclePreference,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTimeFormat>, ICU4XError> {
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

            DateTimeFormat::try_new(locale, &provider, &options.into())
                .map(|dtf| Box::new(ICU4XGregorianDateTimeFormat(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateTimeFormat::format_to_write, FnInStruct)]
        pub fn format_to_write(
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
