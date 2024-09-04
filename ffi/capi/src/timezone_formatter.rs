// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! call_method {
    ($self:ident, $compiled:ident, $unstable:ident, $provider:expr) => {
        match &$provider.0 {
            $crate::provider::DataProviderInner::Destroyed => Err(
                icu_provider::DataError::custom("This provider has been destroyed"),
            )?,
            $crate::provider::DataProviderInner::Empty => $self
                .0
                .$unstable(&icu_provider_adapters::empty::EmptyDataProvider::new()),
            #[cfg(feature = "buffer_provider")]
            $crate::provider::DataProviderInner::Buffer(buffer_provider) => $self.0.$unstable(
                &icu_provider::buf::AsDeserializingBufferProvider::as_deserializing(
                    buffer_provider,
                ),
            ),
            #[cfg(feature = "compiled_data")]
            $crate::provider::DataProviderInner::Compiled => $self.0.$compiled(),
        }
    };
}

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::Error;
    use crate::locale_core::ffi::Locale;
    use crate::provider::ffi::DataProvider;
    use crate::timezone::ffi::CustomTimeZone;

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X TimeZoneFormatter object capable of formatting an [`CustomTimeZone`] type (and others) as a string
    #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedTimeZone, Struct, hidden)]
    pub struct TimeZoneFormatter(pub icu_datetime::time_zone::TimeZoneFormatter);

    #[diplomat::enum_convert(icu_datetime::time_zone::IsoFormat, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoFormat, Enum)]
    pub enum IsoTimeZoneFormat {
        Basic,
        Extended,
        UtcBasic,
        UtcExtended,
    }

    #[diplomat::enum_convert(icu_datetime::time_zone::IsoMinutes, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoMinutes, Enum)]
    pub enum IsoTimeZoneMinuteDisplay {
        Required,
        Optional,
    }

    #[diplomat::enum_convert(icu_datetime::time_zone::IsoSeconds, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::time_zone::IsoSeconds, Enum)]
    pub enum IsoTimeZoneSecondDisplay {
        Optional,
        Never,
    }

    pub struct IsoTimeZoneOptions {
        pub format: IsoTimeZoneFormat,
        pub minutes: IsoTimeZoneMinuteDisplay,
        pub seconds: IsoTimeZoneSecondDisplay,
    }

    impl TimeZoneFormatter {
        /// Creates a new [`TimeZoneFormatter`] from locale data.
        ///
        /// Uses localized GMT as the fallback format.
        #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatter::try_new, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::time_zone::FallbackFormat, Enum, compact)]
        #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatterOptions, Struct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_localized_gmt_fallback")]
        #[diplomat::demo(default_constructor)]
        pub fn create_with_localized_gmt_fallback(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<TimeZoneFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(TimeZoneFormatter(call_constructor!(
                icu_datetime::time_zone::TimeZoneFormatter::try_new,
                icu_datetime::time_zone::TimeZoneFormatter::try_new_with_any_provider,
                icu_datetime::time_zone::TimeZoneFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                icu_datetime::time_zone::FallbackFormat::LocalizedGmt.into(),
            )?)))
        }

        /// Creates a new [`TimeZoneFormatter`] from locale data.
        ///
        /// Uses ISO-8601 as the fallback format.
        #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatter::try_new, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::time_zone::FallbackFormat, Enum, compact)]
        #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatterOptions, Struct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_iso_8601_fallback")]
        pub fn create_with_iso_8601_fallback(
            provider: &DataProvider,
            locale: &Locale,
            options: IsoTimeZoneOptions,
        ) -> Result<Box<TimeZoneFormatter>, Error> {
            let locale = locale.to_datalocale();

            Ok(Box::new(TimeZoneFormatter(call_constructor!(
                icu_datetime::time_zone::TimeZoneFormatter::try_new,
                icu_datetime::time_zone::TimeZoneFormatter::try_new_with_any_provider,
                icu_datetime::time_zone::TimeZoneFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options.into(),
            )?)))
        }

        /// Loads generic non-location long format. Example: "Pacific Time"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_generic_non_location_long,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_generic_non_location_long,
            FnInStruct,
            hidden
        )]
        pub fn load_generic_non_location_long(
            &mut self,
            provider: &DataProvider,
        ) -> Result<(), Error> {
            call_method!(
                self,
                include_generic_non_location_long,
                load_generic_non_location_long,
                provider
            )?;
            Ok(())
        }

        /// Loads generic non-location short format. Example: "PT"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_generic_non_location_short,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_generic_non_location_short,
            FnInStruct,
            hidden
        )]
        pub fn load_generic_non_location_short(
            &mut self,
            provider: &DataProvider,
        ) -> Result<(), Error> {
            call_method!(
                self,
                include_generic_non_location_short,
                load_generic_non_location_short,
                provider
            )?;
            Ok(())
        }

        /// Loads specific non-location long format. Example: "Pacific Standard Time"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_specific_non_location_long,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_specific_non_location_long,
            FnInStruct,
            hidden
        )]
        pub fn load_specific_non_location_long(
            &mut self,
            provider: &DataProvider,
        ) -> Result<(), Error> {
            call_method!(
                self,
                include_specific_non_location_long,
                load_specific_non_location_long,
                provider
            )?;
            Ok(())
        }

        /// Loads specific non-location short format. Example: "PST"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_specific_non_location_short,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_specific_non_location_short,
            FnInStruct,
            hidden
        )]
        pub fn load_specific_non_location_short(
            &mut self,
            provider: &DataProvider,
        ) -> Result<(), Error> {
            call_method!(
                self,
                include_specific_non_location_short,
                load_specific_non_location_short,
                provider
            )?;
            Ok(())
        }

        /// Loads generic location format. Example: "Los Angeles Time"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_generic_location_format,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_generic_location_format,
            FnInStruct,
            hidden
        )]
        pub fn load_generic_location_format(
            &mut self,
            provider: &DataProvider,
        ) -> Result<(), Error> {
            call_method!(
                self,
                include_generic_location_format,
                load_generic_location_format,
                provider
            )?;
            Ok(())
        }

        /// Loads localized GMT format. Example: "GMT-07:00"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_localized_gmt_format,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_localized_gmt_format,
            FnInStruct,
            hidden
        )]
        pub fn include_localized_gmt_format(&mut self) -> Result<(), Error> {
            self.0.include_localized_gmt_format()?;
            Ok(())
        }

        /// Loads ISO-8601 format. Example: "-07:00"
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::include_iso_8601_format,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::load_iso_8601_format,
            FnInStruct,
            hidden
        )]
        pub fn load_iso_8601_format(&mut self, options: IsoTimeZoneOptions) -> Result<(), Error> {
            self.0.include_iso_8601_format(
                options.format.into(),
                options.minutes.into(),
                options.seconds.into(),
            )?;
            Ok(())
        }

        /// Formats a [`CustomTimeZone`] to a string.
        #[diplomat::rust_link(icu::datetime::time_zone::TimeZoneFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::datetime::time_zone::TimeZoneFormatter::format_to_string,
            FnInStruct
        )]
        #[diplomat::rust_link(icu::datetime::FormattedTimeZone::write_to, FnInStruct, hidden)]
        pub fn format_custom_time_zone(
            &self,
            value: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }

        /// Formats a [`CustomTimeZone`] to a string, performing no fallback
        #[diplomat::rust_link(icu::datetime::FormattedTimeZone::write_no_fallback, FnInStruct)]
        pub fn format_custom_time_zone_no_fallback(
            &self,
            value: &CustomTimeZone,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), Error> {
            match self.0.format(&value.0).write_no_fallback(write) {
                Ok(Ok(())) => Ok(()),
                // TODO: Use narrow error type here
                Ok(Err(_e)) => Err(Error::UnknownError),
                Err(core::fmt::Error) => {
                    debug_assert!(false, "unreachable");
                    Ok(())
                }
            }
        }
    }
}

impl From<ffi::IsoTimeZoneOptions> for icu_datetime::time_zone::TimeZoneFormatterOptions {
    fn from(other: ffi::IsoTimeZoneOptions) -> Self {
        icu_datetime::time_zone::FallbackFormat::Iso8601(
            other.format.into(),
            other.minutes.into(),
            other.seconds.into(),
        )
        .into()
    }
}
