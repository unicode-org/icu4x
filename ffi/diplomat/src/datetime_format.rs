// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::Gregorian;
    use icu_datetime::{
        options::length, DateTimeFormatter, TimeFormatter, TypedDateFormatter,
        TypedDateTimeFormatter,
    };

    use crate::{
        datetime::ffi::ICU4XDateTime, datetime::ffi::ICU4XGregorianDateTime,
        errors::ffi::ICU4XError, locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    /// An ICU4X TimeFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string
    #[diplomat::rust_link(icu::datetime::TimeFormatter, Struct)]
    // TODO(#2153) - Rename to ICU4XTimeFormatter when we remove the dependency on calendar
    // from TimeFormatter.
    pub struct ICU4XTimeFormatter(pub TimeFormatter);

    pub enum ICU4XTimeLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XTimeFormatter {
        /// Creates a new [`ICU4XTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::TypedDateFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XTimeFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();
            let length = match length {
                ICU4XTimeLength::Full => length::Time::Full,
                ICU4XTimeLength::Long => length::Time::Long,
                ICU4XTimeLength::Medium => length::Time::Medium,
                ICU4XTimeLength::Short => length::Time::Short,
            };

            TimeFormatter::try_new_unstable(&provider, &locale, length)
                .map(|tf| Box::new(ICU4XTimeFormatter(tf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_write, FnInStruct)]
        pub fn format_gregorian_datetime(
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
    /// An ICU4X TypedDateFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateFormatter, Struct)]
    pub struct ICU4XGregorianDateFormatter(pub TypedDateFormatter<Gregorian>);

    pub enum ICU4XDateLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XGregorianDateFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::TypedDateFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ICU4XDateLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();
            let length = match length {
                ICU4XDateLength::Full => length::Date::Full,
                ICU4XDateLength::Long => length::Date::Long,
                ICU4XDateLength::Medium => length::Date::Medium,
                ICU4XDateLength::Short => length::Date::Short,
            };

            TypedDateFormatter::try_new_unstable(&provider, &locale, length)
                .map(|df| Box::new(ICU4XGregorianDateFormatter(df)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format_to_write, FnInStruct)]
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
    /// An ICU4X TypedDateFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter, Struct)]
    pub struct ICU4XGregorianDateTimeFormatter(pub TypedDateTimeFormatter<Gregorian>);

    impl ICU4XGregorianDateTimeFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTimeFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();
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

            let options = length::Bag::from_date_time_style(date_length, time_length);

            TypedDateTimeFormatter::try_new_unstable(&provider, &locale, options.into())
                .map(|dtf| Box::new(ICU4XGregorianDateTimeFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XGregorianDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::format_to_write, FnInStruct)]
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
    /// An ICU4X DateFormatter object capable of formatting a [`ICU4XDateTime`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    pub struct ICU4XDateTimeFormatter(pub DateTimeFormatter);

    impl ICU4XDateTimeFormatter {
        /// Creates a new [`ICU4XDateTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XDateTimeFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();
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

            let options = length::Bag::from_date_time_style(date_length, time_length);

            DateTimeFormatter::try_new_unstable(&provider, &locale, options.into())
                .map(|dtf| Box::new(ICU4XDateTimeFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format_to_write, FnInStruct)]
        pub fn format_datetime(
            &self,
            value: &ICU4XDateTime,
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
