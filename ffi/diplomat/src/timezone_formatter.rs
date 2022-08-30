// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;
    use crate::timezone::ffi::ICU4XCustomTimeZone;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_datetime::time_zone::FallbackFormat;
    use icu_datetime::time_zone::IsoFormat;
    use icu_datetime::time_zone::IsoMinutes;
    use icu_datetime::time_zone::IsoSeconds;
    use icu_datetime::TimeZoneFormatter;

    #[diplomat::opaque]
    /// An ICU4X TimeZoneFormatter object capable of formatting an [`ICU4XCustomTimeZone`] type (and others) as a string
    #[diplomat::rust_link(icu::datetime::TimeZoneFormatter, Struct)]
    pub struct ICU4XTimeZoneFormatter(pub TimeZoneFormatter);

    #[diplomat::enum_convert(IsoFormat, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoFormat, Enum)]
    pub enum ICU4XIsoTimeZoneFormat {
        Basic,
        Extended,
        UtcBasic,
        UtcExtended,
    }

    #[diplomat::enum_convert(IsoMinutes, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoMinutes, Enum)]
    pub enum ICU4XIsoTimeZoneMinuteDisplay {
        Required,
        Optional,
    }

    #[diplomat::enum_convert(IsoSeconds, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoSeconds, Enum)]
    pub enum ICU4XIsoTimeZoneSecondDisplay {
        Optional,
        Never,
    }

    impl ICU4XTimeZoneFormatter {
        /// Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
        ///
        /// Uses localized GMT as the fallback format.
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::try_new_unstable, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::time_zone::FallbackFormat, Enum, compact)]
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatterOptions, Struct, hidden)]
        pub fn try_new_with_localized_gmt_fallback(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XTimeZoneFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();

            TimeZoneFormatter::try_new_unstable(
                &provider,
                &locale,
                FallbackFormat::LocalizedGmt.into(),
            )
            .map(|tf| Box::new(ICU4XTimeZoneFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        /// Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
        ///
        /// Uses ISO-8601 as the fallback format.
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::try_new_unstable, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::time_zone::FallbackFormat, Enum, compact)]
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatterOptions, Struct, hidden)]
        pub fn try_new_with_iso_8601_fallback(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            format: ICU4XIsoTimeZoneFormat,
            minutes: ICU4XIsoTimeZoneMinuteDisplay,
            seconds: ICU4XIsoTimeZoneSecondDisplay,
        ) -> DiplomatResult<Box<ICU4XTimeZoneFormatter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();

            TimeZoneFormatter::try_new_unstable(
                &provider,
                &locale,
                FallbackFormat::Iso8601(format.into(), minutes.into(), seconds.into()).into(),
            )
            .map(|tf| Box::new(ICU4XTimeZoneFormatter(tf)))
            .map_err(Into::into)
            .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_generic_non_location_long,
            FnInStruct
        )]
        pub fn load_generic_non_location_long(
            &mut self,
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            self.0
                .load_generic_non_location_long(&provider)
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_generic_non_location_short,
            FnInStruct
        )]
        pub fn load_generic_non_location_short(
            &mut self,
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            self.0
                .load_generic_non_location_short(&provider)
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_specific_non_location_long,
            FnInStruct
        )]
        pub fn load_specific_non_location_long(
            &mut self,
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            self.0
                .load_specific_non_location_long(&provider)
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_specific_non_location_short,
            FnInStruct
        )]
        pub fn load_specific_non_location_short(
            &mut self,
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            self.0
                .load_specific_non_location_short(&provider)
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_generic_location_format,
            FnInStruct
        )]
        pub fn load_generic_location_format(
            &mut self,
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            self.0
                .load_generic_location_format(&provider)
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu::datetime::TimeZoneFormatter::load_localized_gmt_format,
            FnInStruct
        )]
        pub fn load_localized_gmt_format(&mut self) -> DiplomatResult<(), ICU4XError> {
            self.0
                .load_localized_gmt_format()
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::load_iso_8601_format, FnInStruct)]
        pub fn load_iso_8601_format(
            &mut self,
            format: ICU4XIsoTimeZoneFormat,
            minutes: ICU4XIsoTimeZoneMinuteDisplay,
            seconds: ICU4XIsoTimeZoneSecondDisplay,
        ) -> DiplomatResult<(), ICU4XError> {
            self.0
                .load_iso_8601_format(format.into(), minutes.into(), seconds.into())
                .map(|_| ())
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XCustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::format_to_string, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeZoneFormatter::format_to_write, FnInStruct)]
        pub fn format_custom_time_zone(
            &self,
            value: &ICU4XCustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
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
