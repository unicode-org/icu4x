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
    use crate::errors::ffi::{CalendarError, CalendarFromStrError};

    use tinystr::TinyAsciiStr;

    #[cfg(feature = "icu_calendar")]
    use crate::week::ffi::WeekCalculator;

    #[diplomat::enum_convert(icu_calendar::types::IsoWeekday)]
    pub enum IsoWeekday {
        Monday = 1,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    /// An ICU4X Date object capable of containing a ISO-8601 date
    #[diplomat::rust_link(icu::calendar::Date, Struct)]
    pub struct IsoDate(pub icu_calendar::Date<icu_calendar::Iso>);

    impl IsoDate {
        /// Creates a new [`IsoDate`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::Date::try_new_iso_date, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(year: i32, month: u8, day: u8) -> Result<Box<IsoDate>, CalendarError> {
            Ok(Box::new(IsoDate(icu_calendar::Date::try_new_iso_date(
                year, month, day,
            )?)))
        }

        /// Creates a new [`IsoDate`] from an IXDTF string.
        #[diplomat::rust_link(icu::calendar::Date::try_iso_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::Date::try_iso_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::calendar::Date::from_str, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor)]
        pub fn from_string(v: &DiplomatStr) -> Result<Box<IsoDate>, CalendarFromStrError> {
            Ok(Box::new(IsoDate(icu_calendar::Date::try_iso_from_utf8(v)?)))
        }

        /// Creates a new [`IsoDate`] representing January 1, 1970.
        #[diplomat::rust_link(icu::calendar::Date::unix_epoch, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor)]
        pub fn unix_epoch() -> Box<IsoDate> {
            Box::new(IsoDate(icu_calendar::Date::unix_epoch()))
        }

        /// Convert this date to one in a different calendar
        #[diplomat::rust_link(icu::calendar::Date::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &Calendar) -> Box<Date> {
            Box::new(Date(self.0.to_calendar(calendar.0.clone())))
        }

        #[diplomat::rust_link(icu::calendar::Date::to_any, FnInStruct)]
        pub fn to_any(&self) -> Box<Date> {
            Box::new(Date(self.0.to_any().wrap_calendar_in_arc()))
        }

        /// Returns the 1-indexed day in the year for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_year_info, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_year(&self) -> u16 {
            self.0.day_of_year_info().day_of_year
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_month(&self) -> u32 {
            self.0.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_week(&self) -> IsoWeekday {
            self.0.day_of_week().into()
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
            self.0.week_of_month(first_weekday.into()).0
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
            self.0.week_of_year(&calculator.0).into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn month(&self) -> u32 {
            self.0.month().ordinal
        }

        /// Returns the year number for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn year(&self) -> i32 {
            self.0.year().number
        }

        /// Returns if the year is a leap year for this date
        #[diplomat::rust_link(icu::calendar::Date::is_in_leap_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_in_leap_year(&self) -> bool {
            self.0.is_in_leap_year()
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn months_in_year(&self) -> u8 {
            self.0.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_month(&self) -> u8 {
            self.0.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_year(&self) -> u16 {
            self.0.days_in_year()
        }
    }

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    /// An ICU4X Date object capable of containing a date and time for any calendar.
    #[diplomat::rust_link(icu::calendar::Date, Struct)]
    pub struct Date(pub icu_calendar::Date<Arc<icu_calendar::AnyCalendar>>);

    impl Date {
        /// Creates a new [`Date`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::calendar::Date::new_from_iso, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor)]
        pub fn from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            calendar: &Calendar,
        ) -> Result<Box<Date>, CalendarError> {
            let cal = calendar.0.clone();
            Ok(Box::new(Date(
                icu_calendar::Date::try_new_iso_date(year, month, day)?.to_calendar(cal),
            )))
        }

        /// Creates a new [`Date`] from the given codes, which are interpreted in the given calendar system
        #[diplomat::rust_link(icu::calendar::Date::try_new_from_codes, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor)]
        pub fn from_codes_in_calendar(
            era_code: &DiplomatStr,
            year: i32,
            month_code: &DiplomatStr,
            day: u8,
            calendar: &Calendar,
        ) -> Result<Box<Date>, CalendarError> {
            let era = TinyAsciiStr::try_from_utf8(era_code)
                .map_err(|_| CalendarError::UnknownEra)?
                .into();
            let month = TinyAsciiStr::try_from_utf8(month_code)
                .map_err(|_| CalendarError::UnknownMonthCode)?
                .into();
            let cal = calendar.0.clone();
            Ok(Box::new(Date(icu_calendar::Date::try_new_from_codes(
                era, year, month, day, cal,
            )?)))
        }

        /// Creates a new [`Date`] from an IXDTF string.
        #[diplomat::rust_link(icu::calendar::Date::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::Date::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::calendar::Date::from_str, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn from_string(v: &DiplomatStr) -> Result<Box<Date>, CalendarFromStrError> {
            Ok(Box::new(Date(
                icu_calendar::Date::try_from_utf8(v)?.wrap_calendar_in_arc(),
            )))
        }

        /// Convert this date to one in a different calendar
        #[diplomat::rust_link(icu::calendar::Date::to_calendar, FnInStruct)]
        pub fn to_calendar(&self, calendar: &Calendar) -> Box<Date> {
            Box::new(Date(self.0.to_calendar(calendar.0.clone())))
        }

        /// Converts this date to ISO
        #[diplomat::rust_link(icu::calendar::Date::to_iso, FnInStruct)]
        pub fn to_iso(&self) -> Box<IsoDate> {
            Box::new(IsoDate(self.0.to_iso()))
        }

        /// Returns the 1-indexed day in the year for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_year_info, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_year(&self) -> u16 {
            self.0.day_of_year_info().day_of_year
        }

        /// Returns the 1-indexed day in the month for this date
        #[diplomat::rust_link(icu::calendar::Date::day_of_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_month(&self) -> u32 {
            self.0.day_of_month().0
        }

        /// Returns the day in the week for this day
        #[diplomat::rust_link(icu::calendar::Date::day_of_week, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn day_of_week(&self) -> IsoWeekday {
            self.0.day_of_week().into()
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
            self.0.week_of_month(first_weekday.into()).0
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
            self.0.week_of_year(&calculator.0).into()
        }

        /// Returns 1-indexed number of the month of this date in its year
        ///
        /// Note that for lunar calendars this may not lead to the same month
        /// having the same ordinal month across years; use month_code if you care
        /// about month identity.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn ordinal_month(&self) -> u32 {
            self.0.month().ordinal
        }

        /// Returns the month code for this date. Typically something
        /// like "M01", "M02", but can be more complicated for lunar calendars.
        #[diplomat::rust_link(icu::calendar::Date::month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn month_code(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let code = self.0.month().code;
            let _infallible = write.write_str(&code.0);
        }

        /// Returns the year number in the current era for this date
        #[diplomat::rust_link(icu::calendar::Date::year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn year_in_era(&self) -> i32 {
            self.0.year().number
        }

        /// Returns the era for this date,
        #[diplomat::rust_link(icu::Date::year, FnInStruct)]
        #[diplomat::rust_link(icu::types::Era, Struct, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn era(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let era = self.0.year().era;
            let _infallible = write.write_str(&era.0);
        }

        /// Returns the number of months in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::months_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn months_in_year(&self) -> u8 {
            self.0.months_in_year()
        }

        /// Returns the number of days in the month represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_month, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_month(&self) -> u8 {
            self.0.days_in_month()
        }

        /// Returns the number of days in the year represented by this date
        #[diplomat::rust_link(icu::calendar::Date::days_in_year, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn days_in_year(&self) -> u16 {
            self.0.days_in_year()
        }

        /// Returns the [`Calendar`] object backing this date
        #[diplomat::rust_link(icu::calendar::Date::calendar, FnInStruct)]
        #[diplomat::rust_link(icu::calendar::Date::calendar_wrapper, FnInStruct, hidden)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn calendar(&self) -> Box<Calendar> {
            Box::new(Calendar(self.0.calendar_wrapper().clone()))
        }
    }
}
