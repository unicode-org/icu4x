// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{types, Calendar, CalendarError, DateDuration, DateDurationUnit};
use core::convert::TryInto;
use core::marker::PhantomData;
use tinystr::tinystr;

// Note: The Ord/PartialOrd impls can be derived because the fields are in the correct order.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ArithmeticDate<C> {
    pub year: i32,
    /// 1-based month of year
    pub month: u8,
    /// 1-based day of month
    pub day: u8,
    marker: PhantomData<C>,
}

impl<C> Copy for ArithmeticDate<C> {}
impl<C> Clone for ArithmeticDate<C> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Maximum number of iterations when iterating through the days of a month; can be increased if necessary
#[allow(dead_code)] // TODO: Remove dead code tag after use
pub(crate) const MAX_ITERS_FOR_DAYS_OF_MONTH: u8 = 33;

pub trait CalendarArithmetic: Calendar {
    type PrecomputedData;
    fn month_days(year: i32, month: u8, data: &Self::PrecomputedData) -> u8;
    fn months_for_every_year(year: i32, data: &Self::PrecomputedData) -> u8;
    fn is_leap_year(year: i32, data: &Self::PrecomputedData) -> bool;
    fn last_month_day_in_year(year: i32, data: &Self::PrecomputedData) -> (u8, u8);

    /// Calculate the days in a given year
    /// Can be overridden with simpler implementations for solar calendars
    /// (typically, 366 in leap, 365 otherwise) Leave this as the default
    /// for lunar calendars
    ///
    /// The name has `provided` in it to avoid clashes with Calendar
    fn days_in_provided_year(year: i32, data: &Self::PrecomputedData) -> u16 {
        let months_in_year = Self::months_for_every_year(year, data);
        let mut days: u16 = 0;
        for month in 1..=months_in_year {
            days += Self::month_days(year, month, data) as u16;
        }
        days
    }
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    /// Create a new `ArithmeticDate` without checking that `month` and `day` are in bounds.
    #[inline]
    pub const fn new_unchecked(year: i32, month: u8, day: u8) -> Self {
        ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn min_date() -> Self {
        ArithmeticDate {
            year: i32::MIN,
            month: 1,
            day: 1,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn max_date_with_data(data: &C::PrecomputedData) -> Self {
        let year = i32::MAX;
        let (month, day) = C::last_month_day_in_year(year, data);
        ArithmeticDate {
            year: i32::MAX,
            month,
            day,
            marker: PhantomData,
        }
    }

    #[inline]
    fn offset_days(&mut self, mut day_offset: i32, data: &C::PrecomputedData) {
        while day_offset != 0 {
            let month_days = C::month_days(self.year, self.month, data);
            if self.day as i32 + day_offset > month_days as i32 {
                self.offset_months(1, data);
                day_offset -= month_days as i32;
            } else if self.day as i32 + day_offset < 1 {
                self.offset_months(-1, data);
                day_offset += C::month_days(self.year, self.month, data) as i32;
            } else {
                self.day = (self.day as i32 + day_offset) as u8;
                day_offset = 0;
            }
        }
    }

    #[inline]
    fn offset_months(&mut self, mut month_offset: i32, data: &C::PrecomputedData) {
        while month_offset != 0 {
            let year_months = C::months_for_every_year(self.year, data);
            if self.month as i32 + month_offset > year_months as i32 {
                self.year += 1;
                month_offset -= year_months as i32;
            } else if self.month as i32 + month_offset < 1 {
                self.year -= 1;
                month_offset += C::months_for_every_year(self.year, data) as i32;
            } else {
                self.month = (self.month as i32 + month_offset) as u8;
                month_offset = 0
            }
        }
    }

    #[inline]
    pub fn offset_date_with_data(&mut self, offset: DateDuration<C>, data: &C::PrecomputedData) {
        // For offset_date to work with lunar calendars, need to handle an edge case where the original month is not valid in the future year.
        self.year += offset.years;

        self.offset_months(offset.months, data);

        let day_offset = offset.days + offset.weeks * 7 + self.day as i32 - 1;
        self.day = 1;
        self.offset_days(day_offset, data);
    }

    #[inline]
    pub fn until_with_data(
        &self,
        date2: ArithmeticDate<C>,
        _largest_unit: DateDurationUnit,
        _smaller_unit: DateDurationUnit,
        _data: &C::PrecomputedData,
    ) -> DateDuration<C> {
        DateDuration::new(
            self.year - date2.year,
            self.month as i32 - date2.month as i32,
            0,
            self.day as i32 - date2.day as i32,
        )
    }

    #[inline]
    pub fn days_in_year_with_data(&self, data: &C::PrecomputedData) -> u16 {
        C::days_in_provided_year(self.year, data)
    }

    #[inline]
    pub fn months_in_year_with_data(&self, data: &C::PrecomputedData) -> u8 {
        C::months_for_every_year(self.year, data)
    }

    #[inline]
    pub fn days_in_month_with_data(&self, data: &C::PrecomputedData) -> u8 {
        C::month_days(self.year, self.month, data)
    }

    #[inline]
    pub fn day_of_year_with_data(&self, data: &C::PrecomputedData) -> u16 {
        let mut day_of_year = 0;
        for month in 1..self.month {
            day_of_year += C::month_days(self.year, month, data) as u16;
        }
        day_of_year + (self.day as u16)
    }

    #[inline]
    pub fn date_from_year_day_with_data(
        year: i32,
        year_day: u32,
        data: &C::PrecomputedData,
    ) -> ArithmeticDate<C> {
        let mut month = 1;
        let mut day = year_day as i32;
        while month <= C::months_for_every_year(year, data) {
            let month_days = C::month_days(year, month, data) as i32;
            if day <= month_days {
                break;
            } else {
                day -= month_days;
                month += 1;
            }
        }

        debug_assert!(day <= C::month_days(year, month, data) as i32);
        #[allow(clippy::unwrap_used)]
        // The day is expected to be within the range of month_days of C
        ArithmeticDate {
            year,
            month,
            day: day.try_into().unwrap_or(0),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn day_of_month(&self) -> types::DayOfMonth {
        types::DayOfMonth(self.day.into())
    }

    /// The [`types::FormattableMonth`] for the current month (with month code) for a solar calendar
    /// Lunar calendars should not use this method and instead manually implement a month code
    /// resolver.
    /// Originally "solar_month" but renamed because it can be used for some lunar calendars
    ///
    /// Returns "und" if run with months that are out of bounds for the current
    /// calendar.
    #[inline]
    pub fn month_with_data(&self, data: &C::PrecomputedData) -> types::FormattableMonth {
        let code = match self.month {
            // todo: check if we're actually calling this in a situation where this matters
            a if a > C::months_for_every_year(self.year, data) => tinystr!(4, "und"),
            1 => tinystr!(4, "M01"),
            2 => tinystr!(4, "M02"),
            3 => tinystr!(4, "M03"),
            4 => tinystr!(4, "M04"),
            5 => tinystr!(4, "M05"),
            6 => tinystr!(4, "M06"),
            7 => tinystr!(4, "M07"),
            8 => tinystr!(4, "M08"),
            9 => tinystr!(4, "M09"),
            10 => tinystr!(4, "M10"),
            11 => tinystr!(4, "M11"),
            12 => tinystr!(4, "M12"),
            13 => tinystr!(4, "M13"),
            _ => tinystr!(4, "und"),
        };
        types::FormattableMonth {
            ordinal: self.month as u32,
            code: types::MonthCode(code),
        }
    }

    /// Construct a new arithmetic date from a year, month code, and day, bounds checking
    /// the month and day
    /// Originally (new_from_solar_codes) but renamed because it works for some lunar calendars
    pub fn new_from_codes_with_data<C2: Calendar>(
        // Separate type since the debug_name() impl may differ when DateInner types
        // are nested (e.g. in GregorianDateInner)
        cal: &C2,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
        data: &C::PrecomputedData,
    ) -> Result<Self, CalendarError> {
        let month = if let Some((ordinal, false)) = month_code.parsed() {
            ordinal
        } else {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                cal.debug_name(),
            ));
        };

        if month > C::months_for_every_year(year, data) {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                cal.debug_name(),
            ));
        }

        let max_day = C::month_days(year, month, data);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        Ok(Self::new_unchecked(year, month, day))
    }

    /// Construct a new arithmetic date from a year, month ordinal, and day, bounds checking
    /// the month and day
    /// Originally (new_from_solar_ordinals) but renamed because it works for some lunar calendars
    pub fn new_from_ordinals_with_data(
        year: i32,
        month: u8,
        day: u8,
        data: &C::PrecomputedData,
    ) -> Result<Self, CalendarError> {
        let max_month = C::months_for_every_year(year, data);
        if month > max_month {
            return Err(CalendarError::Overflow {
                field: "month",
                max: max_month as usize,
            });
        }
        let max_day = C::month_days(year, month, data);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        Ok(Self::new_unchecked(year, month, day))
    }

    /// This fn currently just calls [`new_from_ordinals`], but exists separately for
    /// lunar calendars in case different logic needs to be implemented later.
    pub fn new_from_lunar_ordinals_with_data(
        year: i32,
        month: u8,
        day: u8,
        data: &C::PrecomputedData,
    ) -> Result<Self, CalendarError> {
        Self::new_from_ordinals_with_data(year, month, day, data)
    }
}

/// Convenience methods for dataless calendars (the majority)
///
/// May be removed in the long run
impl<C: CalendarArithmetic<PrecomputedData = ()>> ArithmeticDate<C> {
    #[inline]
    pub fn max_date() -> Self {
        Self::max_date_with_data(&())
    }

    #[inline]
    pub fn offset_date(&mut self, offset: DateDuration<C>) {
        self.offset_date_with_data(offset, &())
    }

    #[inline]
    pub fn until(
        &self,
        date2: ArithmeticDate<C>,
        largest_unit: DateDurationUnit,
        smaller_unit: DateDurationUnit,
    ) -> DateDuration<C> {
        self.until_with_data(date2, largest_unit, smaller_unit, &())
    }

    #[inline]
    pub fn days_in_year(&self) -> u16 {
        self.days_in_year_with_data(&())
    }

    #[inline]
    pub fn months_in_year(&self) -> u8 {
        self.months_in_year_with_data(&())
    }

    #[inline]
    pub fn days_in_month(&self) -> u8 {
        self.days_in_month_with_data(&())
    }

    #[inline]
    pub fn day_of_year(&self) -> u16 {
        self.day_of_year_with_data(&())
    }

    #[inline]
    pub fn date_from_year_day(year: i32, year_day: u32) -> ArithmeticDate<C> {
        Self::date_from_year_day_with_data(year, year_day, &())
    }

    /// The [`types::FormattableMonth`] for the current month (with month code) for a solar calendar
    /// Lunar calendars should not use this method and instead manually implement a month code
    /// resolver.
    /// Originally "solar_month" but renamed because it can be used for some lunar calendars
    ///
    /// Returns "und" if run with months that are out of bounds for the current
    /// calendar.
    #[inline]
    pub fn month(&self) -> types::FormattableMonth {
        self.month_with_data(&())
    }

    /// Construct a new arithmetic date from a year, month code, and day, bounds checking
    /// the month and day
    /// Originally (new_from_solar_codes) but renamed because it works for some lunar calendars
    pub fn new_from_codes<C2: Calendar>(
        cal: &C2,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self, CalendarError> {
        Self::new_from_codes_with_data(cal, year, month_code, day, &())
    }

    /// Construct a new arithmetic date from a year, month ordinal, and day, bounds checking
    /// the month and day
    /// Originally (new_from_solar_ordinals) but renamed because it works for some lunar calendars
    pub fn new_from_ordinals(year: i32, month: u8, day: u8) -> Result<Self, CalendarError> {
        Self::new_from_ordinals_with_data(year, month, day, &())
    }

    /// This fn currently just calls [`new_from_ordinals`], but exists separately for
    /// lunar calendars in case different logic needs to be implemented later.
    pub fn new_from_lunar_ordinals(year: i32, month: u8, day: u8) -> Result<Self, CalendarError> {
        Self::new_from_lunar_ordinals_with_data(year, month, day, &())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Iso;

    #[test]
    fn test_ord() {
        let dates_in_order = [
            ArithmeticDate::<Iso>::new_unchecked(-10, 1, 1),
            ArithmeticDate::<Iso>::new_unchecked(-10, 1, 2),
            ArithmeticDate::<Iso>::new_unchecked(-10, 2, 1),
            ArithmeticDate::<Iso>::new_unchecked(-1, 1, 1),
            ArithmeticDate::<Iso>::new_unchecked(-1, 1, 2),
            ArithmeticDate::<Iso>::new_unchecked(-1, 2, 1),
            ArithmeticDate::<Iso>::new_unchecked(0, 1, 1),
            ArithmeticDate::<Iso>::new_unchecked(0, 1, 2),
            ArithmeticDate::<Iso>::new_unchecked(0, 2, 1),
            ArithmeticDate::<Iso>::new_unchecked(1, 1, 1),
            ArithmeticDate::<Iso>::new_unchecked(1, 1, 2),
            ArithmeticDate::<Iso>::new_unchecked(1, 2, 1),
            ArithmeticDate::<Iso>::new_unchecked(10, 1, 1),
            ArithmeticDate::<Iso>::new_unchecked(10, 1, 2),
            ArithmeticDate::<Iso>::new_unchecked(10, 2, 1),
        ];
        for (i, i_date) in dates_in_order.iter().enumerate() {
            for (j, j_date) in dates_in_order.iter().enumerate() {
                let result1 = i_date.cmp(j_date);
                let result2 = j_date.cmp(i_date);
                assert_eq!(result1.reverse(), result2);
                assert_eq!(i.cmp(&j), i_date.cmp(j_date));
            }
        }
    }
}
