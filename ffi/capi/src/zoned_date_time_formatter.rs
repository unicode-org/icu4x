// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_calendar::Gregorian;
    use writeable::TryWriteable;

    use crate::{
        date::ffi::IsoDate, errors::ffi::DateTimeWriteError, time::ffi::Time,
        timezone::ffi::TimeZoneInfo,
    };

    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{
        date_time_formatter::ffi::{DateTimeFormatter, DateTimeFormatterGregorian},
        datetime_helpers::{datetime_formatter_gregorian_with_zone, datetime_formatter_with_zone},
        errors::ffi::DateTimeFormatterLoadError,
        locale_core::ffi::Locale,
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    #[diplomat::rust_link(
        icu::datetime::fieldsets::enums::DateAndTimeFieldSet::zone,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::datetime::fieldsets::DT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::DET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::ET::zone, FnInStruct, hidden)]
    #[diplomat::attr(demo_gen, disable)] // constructors are on a different type :(
    pub struct ZonedDateTimeFormatter(
        pub icu_datetime::DateTimeFormatter<icu_datetime::fieldsets::enums::CompositeFieldSet>,
    );

    impl ZonedDateTimeFormatter {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_generic_short_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_generic_short_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_generic_long_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_generic_long_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_specific_short_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_specific_short_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_specific_long_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_specific_long_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_short(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_localized_offset_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_localized_offset_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_long(
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    names
                        .as_mut()
                        .include_time_zone_localized_offset_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatter,
        ) -> Result<Box<ZonedDateTimeFormatter>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names
                        .as_mut()
                        .load_time_zone_localized_offset_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeWriteError> {
            let date = date.0.to_calendar(self.0.calendar());
            let mut input = icu_datetime::DateTimeInputUnchecked::default();
            input.set_date_fields(date);
            input.set_time_fields(time.0);
            input.set_time_zone_id(zone.time_zone_id);
            if let Some(offset) = zone.offset {
                input.set_time_zone_utc_offset(offset);
            }
            if let Some(local_time) = zone.local_time {
                input.set_time_zone_local_time(local_time);
            }
            if let Some(zone_variant) = zone.zone_variant {
                input.set_time_zone_variant(zone_variant);
            }
            let _infallible = self
                .0
                .format_unchecked(input)
                .try_write_to(write)
                .ok()
                .transpose()?;
            Ok(())
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter, Struct)]
    #[diplomat::rust_link(
        icu::datetime::fieldsets::enums::DateAndTimeFieldSet::zone,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::datetime::fieldsets::DT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDT::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::DET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::MDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::YMDET::zone, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::datetime::fieldsets::ET::zone, FnInStruct, hidden)]
    #[diplomat::attr(demo_gen, disable)] // constructors are on a different type :(
    pub struct ZonedDateTimeFormatterGregorian(
        pub  icu_datetime::FixedCalendarDateTimeFormatter<
            Gregorian,
            icu_datetime::fieldsets::enums::CompositeFieldSet,
        >,
    );

    impl ZonedDateTimeFormatterGregorian {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    names.include_time_zone_generic_short_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_generic_short_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_generic_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    names.include_time_zone_generic_long_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "generic_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::GenericLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_generic_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::GenericLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_generic_long_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    names.include_time_zone_specific_short_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_specific_short_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_specific_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    names.include_time_zone_specific_long_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "specific_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::SpecificLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_specific_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::SpecificLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_specific_long_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_short(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    names.include_time_zone_localized_offset_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_short_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetShort, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_short_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetShort,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_localized_offset_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_localized_offset_long(
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    names.include_time_zone_localized_offset_names_with_fallback()?;
                    Ok(())
                },
                |names, field_set| names.try_into_formatter(field_set),
            )
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "localized_offset_long_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::zone::LocalizedOffsetLong, Struct)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_localized_offset_long_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            formatter: &DateTimeFormatterGregorian,
        ) -> Result<Box<ZonedDateTimeFormatterGregorian>, DateTimeFormatterLoadError> {
            let provider = provider.get()?;
            datetime_formatter_gregorian_with_zone(
                &formatter.0,
                locale,
                icu_datetime::fieldsets::zone::LocalizedOffsetLong,
                |names| {
                    use icu_provider::buf::AsDeserializingBufferProvider;
                    let provider = provider.as_deserializing();
                    names.load_time_zone_localized_offset_names_with_fallback(&provider)?;
                    Ok(())
                },
                |names, field_set| {
                    names.try_into_formatter_with_buffer_provider(provider, field_set)
                },
            )
        }

        #[diplomat::rust_link(icu::datetime::FixedCalendarDateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime::to_string, FnInStruct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            zone: &TimeZoneInfo,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), DateTimeWriteError> {
            let date = date.0.to_calendar(Gregorian);
            let mut input = icu_datetime::DateTimeInputUnchecked::default();
            input.set_date_fields(date);
            input.set_time_fields(time.0);
            input.set_time_zone_id(zone.time_zone_id);
            if let Some(offset) = zone.offset {
                input.set_time_zone_utc_offset(offset);
            }
            if let Some(local_time) = zone.local_time {
                input.set_time_zone_local_time(local_time);
            }
            if let Some(zone_variant) = zone.zone_variant {
                input.set_time_zone_variant(zone_variant);
            }
            let _infallible = self
                .0
                .format_unchecked(input)
                .try_write_to(write)
                .ok()
                .transpose()?;
            Ok(())
        }
    }
}
