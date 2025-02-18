// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_calendar::Iso;

    use crate::calendar::ffi::Calendar;
    use crate::date::ffi::{Date, IsoDate};
    use crate::errors::ffi::CalendarParseError;
    use crate::iana_parser::ffi::IanaParser;
    use crate::time::ffi::Time;
    use crate::timezone::ffi::TimeZoneInfo;
    use crate::utc_offset::ffi::UtcOffsetCalculator;

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
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::ZonedDateTime::try_from_utf8, FnInStruct, hidden)]
        pub fn try_from_str(
            v: &DiplomatStr,
            iana_parser: &IanaParser,
            offset_calculator: &UtcOffsetCalculator,
        ) -> Result<ZonedIsoDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_from_utf8(
                    v,
                    Iso,
                    iana_parser.0.as_borrowed(),
                    &offset_calculator.0,
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
        #[diplomat::rust_link(icu::time::DateTime::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::DateTime::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::time::DateTime::from_str, FnInStruct, hidden)]
        pub fn try_from_str(
            v: &DiplomatStr,
            calendar: &Calendar,
            iana_parser: &IanaParser,
            offset_calculator: &UtcOffsetCalculator,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_time::ZonedDateTime { date, time, zone } =
                icu_time::ZonedDateTime::try_from_utf8(
                    v,
                    calendar.0.clone(),
                    iana_parser.0.as_borrowed(),
                    &offset_calculator.0,
                )?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }
    }
}
