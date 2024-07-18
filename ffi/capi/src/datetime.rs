// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::fmt::Write;

    use crate::calendar::ffi::Calendar;
    use crate::date::ffi::{Date, IsoDate, IsoWeekday};
    use crate::errors::ffi::{CalendarError, FromIxdtfError};
    use crate::time::ffi::Time;

    use tinystr::TinyAsciiStr;

    #[cfg(feature = "icu_calendar")]
    use crate::week::ffi::WeekCalculator;

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a ISO-8601 date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct IsoDateTime(pub icu_calendar::DateTime<icu_calendar::Iso>);

    impl IsoDateTime {
        /// Creates a new [`IsoDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::DateTime::try_new_iso_datetime, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
        ) -> Result<Box<IsoDateTime>, CalendarError> {
            let mut dt = icu_calendar::DateTime::try_new_iso_datetime(
                year, month, day, hour, minute, second,
            )?;
            dt.time.nanosecond = nanosecond.try_into()?;
            Ok(Box::new(IsoDateTime(dt)))
        }

        /// Creates a new [`IsoDateTime`] from an [`IsoDate`] and [`Time`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_date_and_time")]
        pub fn crate_from_date_and_time(date: &IsoDate, time: &Time) -> Box<IsoDateTime> {
            let dt = icu_calendar::DateTime::new(date.0, time.0);
            Box::new(IsoDateTime(dt))
        }

        /// Creates a new [`IsoDateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu_calendar::DateTime::try_iso_from_str, FnInStruct)]
        #[diplomat::rust_link(icu_calendar::DateTime::try_iso_from_utf8, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_string")]
        pub fn create_from_string(v: &DiplomatStr) -> Result<Box<IsoDateTime>, FromIxdtfError> {
            Ok(Box::new(IsoDateTime(
                icu_calendar::DateTime::try_iso_from_utf8(v)?,
            )))
        }

        /// Creates a new [`IsoDateTime`] of midnight on January 1, 1970
        #[diplomat::rust_link(icu::calendar::DateTime::local_unix_epoch, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "local_unix_epoch")]
        pub fn local_unix_epoch() -> Box<IsoDateTime> {
            let dt = icu_calendar::DateTime::local_unix_epoch();
            Box::new(IsoDateTime(dt))
        }

        /// Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(
            icu::calendar::DateTime::from_minutes_since_local_unix_epoch,
            FnInStruct
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_minutes_since_local_unix_epoch")]
        pub fn create_from_minutes_since_local_unix_epoch(minutes: i32) -> Box<IsoDateTime> {
            Box::new(IsoDateTime(
                icu_calendar::DateTime::from_minutes_since_local_unix_epoch(minutes),
            ))
        }

        /// Gets the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn date(&self) -> Box<IsoDate> {
            Box::new(IsoDate(self.0.date))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn time(&self) -> Box<Time> {
            Box::new(Time(self.0.time))
        }

        /// Converts this to an [`DateTime`] capable of being mixed with dates of
        /// other calendars
        #[diplomat::rust_link(icu::calendar::DateTime::to_any, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::DateTime::new_from_iso, FnInStruct, hidden)]
        pub fn to_any(&self) -> Box<DateTime> {
            Box::new(DateTime(self.0.to_any().wrap_calendar_in_arc()))
        }

        /// Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
        #[diplomat::rust_link(icu::calendar::DateTime::minutes_since_local_unix_epoch, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn minutes_since_local_unix_epoch(&self) -> i32 {
            self.0.minutes_since_local_unix_epoch()
        }

        /// Convert this datetime to one in a different calendar
        #[diplomat::rust_link(icu::calendar::DateTime::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &Calendar) -> Box<DateTime> {
            Box::new(DateTime(self.0.to_calendar(calendar.0.clone())))
        }

        /// Returns the hour in this time
        #[diplomat::rust_link(icu::calendar::Time::hour, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn hour(&self) -> u8 {
            self.0.time.hour.into()
        }
        /// Returns the minute in this time
        #[diplomat::rust_link(icu::calendar::Time::minute, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn minute(&self) -> u8 {
            self.0.time.minute.into()
        }
        /// Returns the second in this time
        #[diplomat::rust_link(icu::calendar::Time::second, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn second(&self) -> u8 {
            self.0.time.second.into()
        }
        /// Returns the nanosecond in this time
        #[diplomat::rust_link(icu::calendar::Time::nanosecond, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn nanosecond(&self) -> u32 {
            self.0.time.nanosecond.into()
        }

        /// Returns the 1-indexed day in the year for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_year_info, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_year(&self) -> u16 {
            self.0.date.day_of_year_info().day_of_year
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_month(&self) -> u32 {
            self.0.date.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_week(&self) -> IsoWeekday {
            self.0.date.day_of_week().into()
        }

        /// Returns the week number in this month, 1-indexed, based on what
        /// is considered the first day of the week (often a locale preference).
        ///
        /// `first_weekday` can be obtained via `first_weekday()` on [`WeekCalculator`]
        #[diplomat::rust_link(icu::calendar::Date::week_of_month, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_month,
            FnInStruct,
            hidden
        )]
        pub fn week_of_month(&self, first_weekday: IsoWeekday) -> u32 {
            self.0.date.week_of_month(first_weekday.into()).0
        }

        /// Returns the week number in this year, using week data
        #[diplomat::rust_link(icu::calendar::Date::week_of_year, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_year,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "icu_calendar")]
        pub fn week_of_year(&self, calculator: &WeekCalculator) -> crate::week::ffi::WeekOf {
            self.0.date.week_of_year(&calculator.0).into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn month(&self) -> u32 {
            self.0.date.month().ordinal
        }

        /// Returns the year number for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn year(&self) -> i32 {
            self.0.date.year().number
        }

        /// Returns whether this date is in a leap year
        #[diplomat::rust_link(icu::calendar::Date::is_in_leap_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_in_leap_year(&self) -> bool {
            self.0.date.is_in_leap_year()
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn months_in_year(&self) -> u8 {
            self.0.date.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_month(&self) -> u8 {
            self.0.date.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_year(&self) -> u16 {
            self.0.date.days_in_year()
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a date and time for any calendar.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct DateTime(pub icu_calendar::DateTime<Arc<icu_calendar::AnyCalendar>>);

    impl DateTime {
        /// Creates a new [`DateTime`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::DateTime::new_from_iso, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_iso_in_calendar")]
        #[allow(clippy::too_many_arguments)]
        pub fn create_from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &Calendar,
        ) -> Result<Box<DateTime>, CalendarError> {
            let cal = calendar.0.clone();
            let mut dt = icu_calendar::DateTime::try_new_iso_datetime(
                year, month, day, hour, minute, second,
            )?;
            dt.time.nanosecond = nanosecond.try_into()?;
            Ok(Box::new(DateTime(dt.to_calendar(cal))))
        }
        /// Creates a new [`DateTime`] from the given codes, which are interpreted in the given calendar system
        #[diplomat::rust_link(icu::calendar::DateTime::try_new_from_codes, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_codes_in_calendar")]
        #[allow(clippy::too_many_arguments)]
        pub fn create_from_codes_in_calendar(
            era_code: &DiplomatStr,
            year: i32,
            month_code: &DiplomatStr,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            nanosecond: u32,
            calendar: &Calendar,
        ) -> Result<Box<DateTime>, CalendarError> {
            let era = TinyAsciiStr::try_from_utf8(era_code)
                .map_err(|_| CalendarError::UnknownEra)?
                .into();
            let month = TinyAsciiStr::try_from_utf8(month_code)
                .map_err(|_| CalendarError::UnknownMonthCode)?
                .into();
            let cal = calendar.0.clone();
            let hour = hour.try_into()?;
            let minute = minute.try_into()?;
            let second = second.try_into()?;
            let nanosecond = nanosecond.try_into()?;
            let time = icu_calendar::Time {
                hour,
                minute,
                second,
                nanosecond,
            };
            Ok(Box::new(DateTime(
                icu_calendar::DateTime::try_new_from_codes(era, year, month, day, time, cal)?,
            )))
        }
        /// Creates a new [`DateTime`] from an [`Date`] and [`Time`] object
        #[diplomat::rust_link(icu::calendar::DateTime::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_date_and_time")]
        pub fn create_from_date_and_time(date: &Date, time: &Time) -> Box<DateTime> {
            let dt = icu_calendar::DateTime::new(date.0.clone(), time.0);
            Box::new(DateTime(dt))
        }

        /// Creates a new [`DateTime`] from an IXDTF string.
        #[diplomat::rust_link(icu_calendar::DateTime::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu_calendar::DateTime::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_string")]
        #[cfg(feature = "compiled_data")]
        pub fn create_from_string(v: &DiplomatStr) -> Result<Box<DateTime>, FromIxdtfError> {
            Ok(Box::new(DateTime(
                icu_calendar::DateTime::try_from_utf8(v)?.wrap_calendar_in_arc(),
            )))
        }

        /// Gets a copy of the date contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::date, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn date(&self) -> Box<Date> {
            Box::new(Date(self.0.date.clone()))
        }

        /// Gets the time contained in this object
        #[diplomat::rust_link(icu::calendar::DateTime::time, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn time(&self) -> Box<Time> {
            Box::new(Time(self.0.time))
        }

        /// Converts this date to ISO
        #[diplomat::rust_link(icu::calendar::DateTime::to_iso, FnInStruct)]
        pub fn to_iso(&self) -> Box<IsoDateTime> {
            Box::new(IsoDateTime(self.0.to_iso()))
        }

        /// Convert this datetime to one in a different calendar
        #[diplomat::rust_link(icu::calendar::DateTime::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &Calendar) -> Box<DateTime> {
            Box::new(DateTime(self.0.to_calendar(calendar.0.clone())))
        }

        /// Returns the hour in this time
        #[diplomat::rust_link(icu::calendar::Time::hour, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn hour(&self) -> u8 {
            self.0.time.hour.into()
        }
        /// Returns the minute in this time
        #[diplomat::rust_link(icu::calendar::Time::minute, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn minute(&self) -> u8 {
            self.0.time.minute.into()
        }
        /// Returns the second in this time
        #[diplomat::rust_link(icu::calendar::Time::second, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn second(&self) -> u8 {
            self.0.time.second.into()
        }
        /// Returns the nanosecond in this time
        #[diplomat::rust_link(icu::calendar::Time::nanosecond, StructField)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn nanosecond(&self) -> u32 {
            self.0.time.nanosecond.into()
        }

        /// Returns the 1-indexed day in the year for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_year_info, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_year(&self) -> u16 {
            self.0.date.day_of_year_info().day_of_year
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_month(&self) -> u32 {
            self.0.date.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_week(&self) -> IsoWeekday {
            self.0.date.day_of_week().into()
        }

        /// Returns the week number in this month, 1-indexed, based on what
        /// is considered the first day of the week (often a locale preference).
        ///
        /// `first_weekday` can be obtained via `first_weekday()` on [`WeekCalculator`]
        #[diplomat::rust_link(icu::calendar::Date::week_of_month, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_month,
            FnInStruct,
            hidden
        )]
        pub fn week_of_month(&self, first_weekday: IsoWeekday) -> u32 {
            self.0.date.week_of_month(first_weekday.into()).0
        }

        /// Returns the week number in this year, using week data
        #[diplomat::rust_link(icu::calendar::Date::week_of_year, FnInStruct)]
        #[diplomat::rust_link(
            icu::calendar::week::WeekCalculator::week_of_year,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "icu_calendar")]
        pub fn week_of_year(&self, calculator: &WeekCalculator) -> crate::week::ffi::WeekOf {
            self.0.date.week_of_year(&calculator.0).into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        ///
        /// Note that for lunar calendars this may not lead to the same month
        /// having the same ordinal month across years; use month_code if you care
        /// about month identity.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn ordinal_month(&self) -> u32 {
            self.0.date.month().ordinal
        }

        /// Returns the month code for this date. Typically something
        /// like "M01", "M02", but can be more complicated for lunar calendars.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn month_code(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let code = self.0.date.month().code;
            let _infallible = write.write_str(&code.0);
        }

        /// Returns the year number in the current era for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn year_in_era(&self) -> i32 {
            self.0.date.year().number
        }

        /// Returns the era for this date,
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn era(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let era = self.0.date.year().era;
            let _infallible = write.write_str(&era.0);
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn months_in_year(&self) -> u8 {
            self.0.date.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_month(&self) -> u8 {
            self.0.date.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_year(&self) -> u16 {
            self.0.date.days_in_year()
        }

        /// Returns the [`Calendar`] object backing this date
        #[diplomat::rust_link(icu::calendar::Date::calendar, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::Date::calendar_wrapper, FnInStruct, hidden)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn calendar(&self) -> Box<Calendar> {
            Box::new(Calendar(self.0.date.calendar_wrapper().clone()))
        }
    }
}
