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
    #[cfg(feature = "buffer_provider")]
    use crate::errors::ffi::DataError;
    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    use crate::time::ffi::Time;
    use crate::timezone::ffi::TimeZoneInfo;

    #[diplomat::rust_link(icu::timezone::ZonedDateTimeParser, Struct)]
    #[diplomat::opaque]
    pub struct ZonedDateTimeParser(
        icu_timezone::IanaParser,
        icu_timezone::ZoneOffsetCalculator,
    );

    impl ZonedDateTimeParser {
        /// Construct a new [`ZonedDateTimeParser`] instance using compiled data.
        #[diplomat::rust_link(icu::timezone::ZonedDateTimeParser::new, FnInStruct)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<ZonedDateTimeParser> {
            Box::new(ZonedDateTimeParser(
                icu_timezone::IanaParser::new().static_to_owned(),
                icu_timezone::ZoneOffsetCalculator::new(),
            ))
        }
        /// Construct a new [`ZonedDateTimeParser`] instance using a particular data source.
        #[diplomat::rust_link(icu::timezone::ZonedDateTimeParser::new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<ZonedDateTimeParser>, DataError> {
            Ok(Box::new(ZonedDateTimeParser(
                icu_timezone::IanaParser::try_new_with_buffer_provider(provider.get()?)?,
                icu_timezone::ZoneOffsetCalculator::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }
    }

    /// An ICU4X ZonedDateTime object capable of containing a ISO-8601 date, time, and zone.
    #[diplomat::rust_link(icu::timezone::ZonedDateTime, Struct)]
    #[diplomat::out]
    pub struct ZonedIsoDateTime {
        pub date: Box<IsoDate>,
        pub time: Box<Time>,
        pub zone: Box<TimeZoneInfo>,
    }

    impl ZonedDateTimeParser {
        /// Creates a new [`ZonedIsoDateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu::timezone::ZonedDateTime::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::ZonedDateTime::try_from_utf8, FnInStruct, hidden)]
        pub fn try_iso_from_str(
            &self,
            v: &DiplomatStr,
        ) -> Result<ZonedIsoDateTime, CalendarParseError> {
            let icu_timezone::ZonedDateTime { date, time, zone } =
                icu_timezone::ZonedDateTime::try_from_utf8(v, Iso, self.0.as_borrowed(), &self.1)?;
            Ok(ZonedIsoDateTime {
                date: Box::new(IsoDate(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }
    }

    /// An ICU4X DateTime object capable of containing a date, time, and zone for any calendar.
    #[diplomat::rust_link(icu::timezone::ZonedDateTime, Struct)]
    #[diplomat::out]
    pub struct ZonedDateTime {
        pub date: Box<Date>,
        pub time: Box<Time>,
        pub zone: Box<TimeZoneInfo>,
    }

    impl ZonedDateTimeParser {
        /// Creates a new [`ZonedDateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu::timezone::DateTime::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::DateTime::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::DateTime::from_str, FnInStruct, hidden)]
        pub fn try_from_str(
            &self,
            v: &DiplomatStr,
            calendar: &Calendar,
        ) -> Result<ZonedDateTime, CalendarParseError> {
            let icu_timezone::ZonedDateTime { date, time, zone } =
                icu_timezone::ZonedDateTime::try_from_utf8(
                    v,
                    calendar.0.clone(),
                    self.0.as_borrowed(),
                    &self.1,
                )?;
            Ok(ZonedDateTime {
                date: Box::new(Date(date)),
                time: Box::new(Time(time)),
                zone: Box::new(TimeZoneInfo::from(zone)),
            })
        }
    }
}
