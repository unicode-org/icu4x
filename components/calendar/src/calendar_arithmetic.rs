// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::range_check_with_overflow;
use crate::options::Overflow;
use crate::types::{DateFields, DayOfYear, MonthCode};
use crate::{types, Calendar, DateDuration, DateDurationUnit, DateError, RangeError};
use core::cmp::Ordering;
use core::convert::TryInto;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::num::NonZeroU8;
use tinystr::tinystr;

#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub(crate) struct ArithmeticDate<C: CalendarArithmetic> {
    pub year: C::YearInfo,
    /// 1-based month of year
    pub month: u8,
    /// 1-based day of month
    pub day: u8,
    marker: PhantomData<C>,
}

// Manual impls since the derive will introduce a C: Trait bound
// and only the year value should be compared
impl<C: CalendarArithmetic> Copy for ArithmeticDate<C> {}
impl<C: CalendarArithmetic> Clone for ArithmeticDate<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: CalendarArithmetic> PartialEq for ArithmeticDate<C> {
    fn eq(&self, other: &Self) -> bool {
        self.year.into() == other.year.into() && self.month == other.month && self.day == other.day
    }
}

impl<C: CalendarArithmetic> Eq for ArithmeticDate<C> {}

impl<C: CalendarArithmetic> Ord for ArithmeticDate<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .into()
            .cmp(&other.year.into())
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
    }
}

impl<C: CalendarArithmetic> PartialOrd for ArithmeticDate<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: CalendarArithmetic> Hash for ArithmeticDate<C> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.year.into().hash(state);
        self.month.hash(state);
        self.day.hash(state);
    }
}

/// Maximum number of iterations when iterating through the days of a month; can be increased if necessary
#[allow(dead_code)] // TODO: Remove dead code tag after use
pub(crate) const MAX_ITERS_FOR_DAYS_OF_MONTH: u8 = 33;

pub(crate) trait CalendarArithmetic: Calendar {
    /// This stores the year as either an i32, or a type containing more
    /// useful computational information.
    type YearInfo: Copy + Debug + Into<i32>;

    // TODO(#3933): potentially make these methods take &self instead, and absorb certain y/m parameters
    // based on usage patterns (e.g month_days is only ever called with self.year)
    fn days_in_provided_month(year: Self::YearInfo, month: u8) -> u8;
    fn months_in_provided_year(year: Self::YearInfo) -> u8;
    fn provided_year_is_leap(year: Self::YearInfo) -> bool;
    fn last_month_day_in_provided_year(year: Self::YearInfo) -> (u8, u8);

    fn day_of_provided_year(year: Self::YearInfo, month: u8, day: u8) -> u16 {
        let mut day_of_year = 0;
        for month in 1..month {
            day_of_year += Self::days_in_provided_month(year, month) as u16;
        }
        day_of_year + day as u16
    }

    /// Calculate the days in a given year
    /// Can be overridden with simpler implementations for solar calendars
    /// (typically, 366 in leap, 365 otherwise) Leave this as the default
    /// for lunar calendars
    ///
    /// The name has `provided` in it to avoid clashes with Calendar
    fn days_in_provided_year(year: Self::YearInfo) -> u16 {
        let months_in_year = Self::months_in_provided_year(year);
        let mut days: u16 = 0;
        for month in 1..=months_in_year {
            days += Self::days_in_provided_month(year, month) as u16;
        }
        days
    }

    fn date_from_provided_year_day(year: Self::YearInfo, year_day: u16) -> (u8, u8) {
        let mut month = 1;
        let mut day = year_day as i32;
        while month <= Self::months_in_provided_year(year) {
            let month_days = Self::days_in_provided_month(year, month) as i32;
            if day <= month_days {
                break;
            } else {
                day -= month_days;
                month += 1;
            }
        }

        debug_assert!(day <= Self::days_in_provided_month(year, month) as i32);

        (month, day.try_into().unwrap_or(1))
    }
}

// TODO: Rename this trait, because it should work with all calendars
// TODO: Consider putting these fns on CalendarArithmetic?
pub(crate) trait CalendarWithEras {
    fn era_year_to_monotonic(&self, era: &str, era_year: i32) -> Result<i32, DateError>;
}

// TODO: Consider merging this into CalendarLunisolar and having just one trait
// TODO: Consider putting these fns on CalendarArithmetic?
pub(crate) trait CalendarNonLunisolar {
    fn fixed_monotonic_reference_year(&self) -> i32;
}

// TODO: Plumb through the custom year type instead of monotonic year i32
pub(crate) trait CalendarLunisolar {
    fn variable_monotonic_reference_year(&self, month_code: MonthCode, day: u8) -> i32;
    fn variable_ordinal_month(
        &self,
        monotonic_year: i32,
        month_code: MonthCode,
    ) -> Result<NonZeroU8, DateError>;
}

pub(crate) trait PrecomputedDataSource<YearInfo> {
    /// Given a calendar year, load (or compute) the YearInfo for it
    ///
    /// In the future we may pass in an optional previous YearInfo alongside the year
    /// it matches to allow code to take shortcuts.
    fn load_or_compute_info(&self, year: i32) -> YearInfo;
}

impl PrecomputedDataSource<i32> for () {
    fn load_or_compute_info(&self, year: i32) -> i32 {
        year
    }
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    /// Create a new `ArithmeticDate` without checking that `month` and `day` are in bounds.
    #[inline]
    pub const fn new_unchecked(year: C::YearInfo, month: u8, day: u8) -> Self {
        ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn min_date() -> Self
    where
        C: CalendarArithmetic<YearInfo = i32>,
    {
        ArithmeticDate::new_unchecked(i32::MIN, 1, 1)
    }

    #[inline]
    pub fn max_date() -> Self
    where
        C: CalendarArithmetic<YearInfo = i32>,
    {
        let year = i32::MAX;
        let (month, day) = C::last_month_day_in_provided_year(year);
        ArithmeticDate::new_unchecked(year, month, day)
    }

    #[inline]
    fn offset_days(&mut self, mut day_offset: i32, data: &impl PrecomputedDataSource<C::YearInfo>) {
        while day_offset != 0 {
            let month_days = C::days_in_provided_month(self.year, self.month);
            if self.day as i32 + day_offset > month_days as i32 {
                self.offset_months(1, data);
                day_offset -= month_days as i32;
            } else if self.day as i32 + day_offset < 1 {
                self.offset_months(-1, data);
                day_offset += C::days_in_provided_month(self.year, self.month) as i32;
            } else {
                self.day = (self.day as i32 + day_offset) as u8;
                day_offset = 0;
            }
        }
    }

    #[inline]
    fn offset_months(
        &mut self,
        mut month_offset: i32,
        data: &impl PrecomputedDataSource<C::YearInfo>,
    ) {
        while month_offset != 0 {
            let year_months = C::months_in_provided_year(self.year);
            if self.month as i32 + month_offset > year_months as i32 {
                self.year = data.load_or_compute_info(self.year.into() + 1);
                month_offset -= year_months as i32;
            } else if self.month as i32 + month_offset < 1 {
                self.year = data.load_or_compute_info(self.year.into() - 1);
                month_offset += C::months_in_provided_year(self.year) as i32;
            } else {
                self.month = (self.month as i32 + month_offset) as u8;
                month_offset = 0
            }
        }
    }

    #[inline]
    pub fn offset_date(
        &mut self,
        offset: DateDuration<C>,
        data: &impl PrecomputedDataSource<C::YearInfo>,
    ) {
        if offset.years != 0 {
            // For offset_date to work with lunar calendars, need to handle an edge case where the original month is not valid in the future year.
            self.year = data.load_or_compute_info(self.year.into() + offset.years);
        }

        self.offset_months(offset.months, data);

        let day_offset = offset.days + offset.weeks * 7 + self.day as i32 - 1;
        self.day = 1;
        self.offset_days(day_offset, data);
    }

    #[inline]
    pub fn until(
        &self,
        date2: ArithmeticDate<C>,
        _largest_unit: DateDurationUnit,
        _smaller_unit: DateDurationUnit,
    ) -> DateDuration<C> {
        // This simple implementation does not need C::PrecomputedDataSource right now, but it
        // likely will once we've written a proper implementation
        DateDuration::new(
            self.year.into() - date2.year.into(),
            self.month as i32 - date2.month as i32,
            0,
            self.day as i32 - date2.day as i32,
        )
    }

    #[inline]
    pub fn days_in_year(&self) -> u16 {
        C::days_in_provided_year(self.year)
    }

    #[inline]
    pub fn months_in_year(&self) -> u8 {
        C::months_in_provided_year(self.year)
    }

    #[inline]
    pub fn days_in_month(&self) -> u8 {
        C::days_in_provided_month(self.year, self.month)
    }

    #[inline]
    pub fn date_from_year_day(year: C::YearInfo, year_day: u16) -> ArithmeticDate<C> {
        let (month, day) = C::date_from_provided_year_day(year, year_day);
        ArithmeticDate::new_unchecked(year, month, day)
    }

    #[inline]
    pub fn day_of_month(&self) -> types::DayOfMonth {
        types::DayOfMonth(self.day)
    }

    #[inline]
    pub fn day_of_year(&self) -> DayOfYear {
        DayOfYear(C::day_of_provided_year(self.year, self.month, self.day))
    }

    pub fn monotonic_year(&self) -> i32 {
        self.year.into()
    }

    /// The [`types::MonthInfo`] for the current month (with month code) for a solar calendar
    /// Lunar calendars should not use this method and instead manually implement a month code
    /// resolver.
    /// Originally "solar_month" but renamed because it can be used for some lunar calendars
    ///
    /// Returns "und" if run with months that are out of bounds for the current
    /// calendar.
    #[inline]
    pub fn month(&self) -> types::MonthInfo {
        let code = match self.month {
            a if a > C::months_in_provided_year(self.year) => tinystr!(4, "und"),
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
        types::MonthInfo {
            ordinal: self.month,
            standard_code: types::MonthCode(code),
            formatting_code: types::MonthCode(code),
        }
    }

    /// Construct a new arithmetic date from a year, month ordinal, and day, bounds checking
    /// the month and day according to the overflow parameter.
    pub(crate) fn new_from_ordinals(
        year: C::YearInfo,
        month: u8,
        day: u8,
        overflow: Overflow,
    ) -> Result<Self, RangeError> {
        Ok(Self::new_unchecked(
            year,
            range_check_with_overflow(
                month,
                "month",
                1..=C::months_in_provided_year(year),
                overflow,
            )?,
            range_check_with_overflow(
                day,
                "day",
                1..=C::days_in_provided_month(year, month),
                overflow,
            )?,
        ))
    }
}

// This impl block is in this file because it more closely relates to
// CalendarArithmetic than it does to DateFields.
impl<'a> DateFields<'a> {
    /// Returns:
    ///
    /// - Ok(Some) if there is a well-defined year
    /// - Ok(None) if there is no year
    /// - Err if the callback returns an error
    /// - Err if there is only one of era and era_year
    /// - Err if all three fields are set and they are inconsistent
    fn get_monotonic_year(
        self,
        era_year_to_monotonic: impl FnOnce(&str, i32) -> Result<i32, DateError>,
    ) -> Result<Option<i32>, DateError> {
        match (self.era, self.era_year) {
            (None, None) => Ok(self.monotonic_year),
            (Some(era), Some(era_year)) => {
                let computed_year = era_year_to_monotonic(era, era_year)?;
                if let Some(monotonic_year) = self.monotonic_year {
                    if computed_year != monotonic_year {
                        return Err(DateError::InconsistentYear);
                    }
                }
                Ok(Some(computed_year))
            }
            (Some(_), None) | (None, Some(_)) => Err(DateError::NotEnoughFields),
        }
    }

    fn get_non_lunisolar_ordinal_month(self) -> Result<Option<NonZeroU8>, DateError> {
        self.get_ordinal_month(|month_code| {
            match month_code.parsed_nonzero() {
                Some((month_number, false)) => Ok(month_number),
                _ => Err(DateError::UnknownMonthCode(month_code))
            }
        })
    }

    /// Returns:
    ///
    /// - Ok(Some) if there is a well-defined month
    /// - Ok(None) if there is no month
    /// - Err if there is no month
    /// - Err if there is a month code and it isn't a well-defined ordinal month
    /// - Err if both fields are set and they are inconstent
    fn get_ordinal_month(
        self,
        month_code_to_ordinal: impl FnOnce(MonthCode) -> Result<NonZeroU8, DateError>,
    ) -> Result<Option<NonZeroU8>, DateError> {
        match self.month_code {
            Some(month_code) => {
                let computed_month = month_code_to_ordinal(month_code)?;
                if let Some(ordinal_month) = self.ordinal_month {
                    if computed_month != ordinal_month {
                        return Err(DateError::InconsistentMonth);
                    }
                }
                Ok(Some(computed_month))
            },
            None => Ok(self.ordinal_month),
        }
    }

    /// Gets the monotonic year, ordinal month, and ordinal day for a
    /// non-lunisolar calendar (one without leap months).
    ///
    /// Returns an error when various conditions happen, in accordance with
    /// the ECMAScript Temporal specification.
    pub(crate) fn get_non_lunisolar_ordinals<C>(self, cal: &C) -> Result<(i32, u8, u8), DateError>
    where
        C: CalendarNonLunisolar + CalendarWithEras,
    {
        let maybe_year =
            self.get_monotonic_year(|era, era_year| cal.era_year_to_monotonic(era, era_year))?;
        let maybe_month = self.get_non_lunisolar_ordinal_month()?;
        let maybe_day = self.day;
        let year = maybe_year.unwrap_or(cal.fixed_monotonic_reference_year());
        let month = maybe_month.ok_or(DateError::NotEnoughFields)?.get();
        let day = if maybe_year.is_some() {
            maybe_day.map(|x| x.get()).unwrap_or(1)
        } else {
            return Err(DateError::NotEnoughFields);
        };
        Ok((year, month, day))
    }

    pub(crate) fn get_lunisolar_ordinals<C>(self, cal: &C) -> Result<(i32, u8, u8), DateError>
    where
        C: CalendarLunisolar + CalendarWithEras,
    {
        let maybe_year =
            self.get_monotonic_year(|era, era_year| cal.era_year_to_monotonic(era, era_year))?;
        let day = if maybe_year.is_some() {
            self.day.map(|x| x.get()).unwrap_or(1)
        } else {
            return Err(DateError::NotEnoughFields);
        };
        let year = match maybe_year {
            Some(year) => year,
            None => match self.month_code {
                Some(month_code) => cal.variable_monotonic_reference_year(month_code, day),
                None => return Err(DateError::NotEnoughFields),
            },
        };
        let month = self
            .get_ordinal_month(|month_code| {
                cal.variable_ordinal_month(year, month_code)
            })?
            .ok_or(DateError::NotEnoughFields)?
            .get();
        Ok((year, month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cal::Iso;

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

    #[test]
    pub fn zero() {
        use crate::Date;
        Date::try_new_iso(2024, 0, 1).unwrap_err();
        Date::try_new_iso(2024, 1, 0).unwrap_err();
        Date::try_new_iso(2024, 0, 0).unwrap_err();
    }
}
