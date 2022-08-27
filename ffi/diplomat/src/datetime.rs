// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::convert::TryInto;

    use diplomat_runtime::DiplomatResult;
    use icu_calendar::types::Time;
    use icu_calendar::AnyCalendar;
    use icu_calendar::{DateTime, Gregorian, Iso};

    use crate::calendar::ffi::ICU4XCalendar;
    use crate::date::ffi::{ICU4XDate, ICU4XIsoDate};
    use crate::errors::ffi::ICU4XError;
    use crate::time::ffi::ICU4XTime;

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a Gregorian date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XGregorianDateTime(pub DateTime<Gregorian>);

    impl ICU4XGregorianDateTime {
        /// Creates a new [`ICU4XGregorianDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::DateTime::new_gregorian_datetime, FnInStruct)]
        pub fn try_new(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTime>, ICU4XError> {
            let nanosecond = try_icu4x!(nanosecond.try_into());
            DateTime::new_gregorian_datetime(year, month, day, hour, minute, second)
                .map(|mut dt| {
                    dt.time.nanosecond = nanosecond;
                    Box::new(ICU4XGregorianDateTime(dt))
                })
                .map_err(Into::into)
                .into()
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        pub fn time(&self) -> Box<ICU4XTime> {
            Box::new(ICU4XTime(self.0.time))
        }

        /// Converts this to an [`ICU4XDateTime`] capable of being mixed with dates of
        /// other calendars
        #[diplomat::rust_link(icu::calendar::DateTime::to_any, FnInStruct)]
        pub fn to_any(&self) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_any().wrap_calendar_in_arc()))
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a ISO-8601 date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XIsoDateTime(pub DateTime<Iso>);

    impl ICU4XIsoDateTime {
        /// Creates a new [`ICU4XIsoDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::DateTime::new_gregorian_datetime, FnInStruct)]
        pub fn try_new(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
        ) -> DiplomatResult<Box<ICU4XIsoDateTime>, ICU4XError> {
            let nanosecond = try_icu4x!(nanosecond.try_into());
            DateTime::new_iso_datetime(year, month, day, hour, minute, second)
                .map(|mut dt| {
                    dt.time.nanosecond = nanosecond;
                    Box::new(ICU4XIsoDateTime(dt))
                })
                .map_err(Into::into)
                .into()
        }

        /// Creates a new [`ICU4XIsoDateTime`] from an [`ICU4XIsoDate`] and [`ICU4XTime`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        pub fn new_from_date_and_time(
            date: &ICU4XIsoDate,
            time: &ICU4XTime,
        ) -> Box<ICU4XIsoDateTime> {
            let dt = DateTime::new(date.0.clone(), time.0);
            Box::new(ICU4XIsoDateTime(dt))
        }

        /// Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(
            icu::calendar::DateTime::from_minutes_since_local_unix_epoch,
            FnInStruct
        )]
        pub fn from_minutes_since_local_unix_epoch(
            minutes: i32,
        ) -> DiplomatResult<Box<ICU4XIsoDateTime>, ICU4XError> {
            DateTime::from_minutes_since_local_unix_epoch(minutes)
                .map(|dt| Box::new(ICU4XIsoDateTime(dt)))
                .map_err(Into::into)
                .into()
        }

        /// Gets the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        pub fn date(&self) -> Box<ICU4XIsoDate> {
            Box::new(ICU4XIsoDate(self.0.date.clone()))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        pub fn time(&self) -> Box<ICU4XTime> {
            Box::new(ICU4XTime(self.0.time))
        }

        /// Converts this to an [`ICU4XDateTime`] capable of being mixed with dates of
        /// other calendars
        #[diplomat::rust_link(icu::calendar::DateTime::to_any, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::DateTime::new_from_iso, FnInStruct, hidden)]
        pub fn to_any(&self) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_any().wrap_calendar_in_arc()))
        }

        /// Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(icu::calendar::DateTime::minutes_since_local_unix_epoch, FnInStruct)]
        pub fn minutes_since_local_unix_epoch(&self) -> i32 {
            self.0.minutes_since_local_unix_epoch()
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a date and time for any calendar.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XDateTime(pub DateTime<Arc<AnyCalendar>>);

    impl ICU4XDateTime {
        /// Creates a new [`ICU4XDateTime`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::calendar::DateTime::new_iso_datetime, FnInStruct)]
        #[allow(clippy::too_many_arguments)]
        pub fn try_new_from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDateTime>, ICU4XError> {
            let cal = calendar.0.clone();
            let nanosecond = try_icu4x!(nanosecond.try_into());
            DateTime::new_iso_datetime(year, month, day, hour, minute, second)
                .map(|mut dt| {
                    dt.time.nanosecond = nanosecond;
                    Box::new(ICU4XDateTime(dt.to_calendar(cal)))
                })
                .map_err(Into::into)
                .into()
        }
        /// Creates a new [`ICU4XDateTime`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::calendar::DateTime::new_from_codes, FnInStruct)]
        #[allow(clippy::too_many_arguments)]
        pub fn try_new_from_codes_in_calendar(
            era_code: &str,
            year: i32,
            month_code: &str,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDateTime>, ICU4XError> {
            let era = try_icu4x!(era_code.parse());
            let month = try_icu4x!(month_code.parse());
            let cal = calendar.0.clone();
            let hour = try_icu4x!(hour.try_into());
            let minute = try_icu4x!(minute.try_into());
            let second = try_icu4x!(second.try_into());
            let nanosecond = try_icu4x!(nanosecond.try_into());
            let time = Time {
                hour,
                minute,
                second,
                nanosecond,
            };
            DateTime::new_from_codes(era, year, month, day, time, cal)
                .map(|dt| Box::new(ICU4XDateTime(dt)))
                .map_err(Into::into)
                .into()
        }
        /// Creates a new [`ICU4XDateTime`] from an [`ICU4XDate`] and [`ICU4XTime`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        pub fn new_from_date_and_time(date: &ICU4XDate, time: &ICU4XTime) -> Box<ICU4XDateTime> {
            let dt = DateTime::new(date.0.clone(), time.0);
            Box::new(ICU4XDateTime(dt))
        }

        /// Gets a copy of the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        pub fn date(&self) -> Box<ICU4XDate> {
            Box::new(ICU4XDate(self.0.date.clone()))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        pub fn time(&self) -> Box<ICU4XTime> {
            Box::new(ICU4XTime(self.0.time))
        }

        /// Converts this date to ISO
        #[diplomat::rust_link(icu::calendar::DateTime::to_iso, FnInStruct)]
        pub fn to_iso(&self) -> Box<ICU4XIsoDateTime> {
            Box::new(ICU4XIsoDateTime(self.0.to_iso()))
        }

        /// Convert this datetime to one in a different calendar
        #[diplomat::rust_link(icu::calendar::DateTime::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &ICU4XCalendar) -> Box<ICU4XDateTime> {
            Box::new(ICU4XDateTime(self.0.to_calendar(calendar.0.clone())))
        }
    }
}
