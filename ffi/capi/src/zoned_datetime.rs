// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_calendar::Iso;

    use crate::unstable::calendar::ffi::Calendar;
    use crate::unstable::date::ffi::{Date, IsoDate};
    use crate::unstable::errors::ffi::CalendarParseError;
    use crate::unstable::iana_parser::ffi::IanaParser;
    use crate::unstable::time::ffi::Time;
    use crate::unstable::timezone::ffi::TimeZoneInfo;
    use crate::unstable::variant_offset::ffi::VariantOffsetsCalculator;

    /// An ICU4X ZonedDateTime object capable of containing a ISO-8601 date, time, and zone.
    #[diplomat::rust_link(icu::time::ZonedDateTime, Struct)]
    #[diplomat::out]
    pub struct ZonedIsoDateTime {
        pub date: Box<IsoDate>,
        pub time: Box<Time>,
        pub zone: Box<TimeZoneInfo>,
    }

    impl ZonedIsoDateTime {
        /// Creates a new [`ZonedIsoDateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_full_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = named_constructors, supports = fallible_constructors), named_constructor = "full_from_string")]
        pub fn full_from_string(
            v: &DiplomatStr,
            iana_parser: &IanaParser,
            offset_calculator: &VariantOffsetsCalculator,
        ) -> Result<ZonedIsoDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_full_from_utf8(
                    v,
                    Iso,
                    iana_parser.0.as_borrowed(),
                    offset_calculator.0.as_borrowed(),
                )?;
            Ok(ZonedIsoDateTime {
                date: Box::new(IsoDate(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }
    }

    /// An ICU4X DateTime object capable of containing a date, time, and zone for any calendar.
    #[diplomat::rust_link(icu::time::ZonedDateTime, Struct)]
    #[diplomat::out]
    pub struct ZonedDateTime {
        pub date: Box<Date>,
        pub time: Box<Time>,
        pub zone: Box<TimeZoneInfo>,
    }

    impl ZonedDateTime {
        /// Creates a new [`ZonedDateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_full_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = named_constructors, supports = fallible_constructors), named_constructor = "full_from_string")]
        pub fn full_from_string(
            v: &DiplomatStr,
            calendar: &Calendar,
            iana_parser: &IanaParser,
            offset_calculator: &VariantOffsetsCalculator,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_full_from_utf8(
                    v,
                    calendar.0.clone(),
                    iana_parser.0.as_borrowed(),
                    offset_calculator.0.as_borrowed(),
                )?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }

        /// Creates a new [`ZonedDateTime`] from a location-only IXDTF string.
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_location_only_from_str, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::ZonedDateTime::try_location_only_from_utf8,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(all(supports = named_constructors, supports = fallible_constructors), named_constructor = "location_only_from_string")]
        pub fn location_only_from_string(
            v: &DiplomatStr,
            calendar: &Calendar,
            iana_parser: &IanaParser,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_location_only_from_utf8(
                    v,
                    calendar.0.clone(),
                    iana_parser.0.as_borrowed(),
                )?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }

        /// Creates a new [`ZonedDateTime`] from an offset-only IXDTF string.
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_offset_only_from_str, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::ZonedDateTime::try_offset_only_from_utf8,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(all(supports = named_constructors, supports = fallible_constructors), named_constructor = "offset_only_from_string")]
        pub fn offset_only_from_string(
            v: &DiplomatStr,
            calendar: &Calendar,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_offset_only_from_utf8(v, calendar.0.clone())?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }

        /// Creates a new [`ZonedDateTime`] from an IXDTF string, without requiring the offset or calculating the zone variant.
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_lenient_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_lenient_from_utf8, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = named_constructors, supports = fallible_constructors), named_constructor = "lenient_from_string")]
        pub fn lenient_from_string(
            v: &DiplomatStr,
            calendar: &Calendar,
            iana_parser: &IanaParser,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_lenient_from_utf8(
                    v,
                    calendar.0.clone(),
                    iana_parser.0.as_borrowed(),
                )?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }
    }
}
