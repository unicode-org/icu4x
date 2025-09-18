// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::range_check_with_overflow;
use crate::options::{DateFromFieldsOptions, MissingFieldsStrategy, Overflow};
use crate::types::{DateFields, DayOfYear, MonthCode};
use crate::{types, Calendar, DateDuration, DateDurationUnit, DateError, RangeError};
use core::cmp::Ordering;
use core::convert::TryInto;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
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

/// Trait for converting from era codes, month codes, and other fields to year/month/day ordinals.
pub(crate) trait DateFieldsResolver: Calendar {
    type YearInfo: PartialEq;

    /// Converts the era and era year to a YearInfo. If the calendar does not have eras,
    /// this should always return an Err result.
    fn year_info_from_era(&self, era: &str, era_year: i32) -> Result<Self::YearInfo, DateError>;

    /// Converts an extended year to a YearInfo.
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo;

    /// Calculates the ECMA reference year for the month code and day, or an error
    /// if the month code and day are invalid.
    fn reference_year_from_month_day(
        &self,
        month_code: MonthCode,
        day: u8,
    ) -> Result<Self::YearInfo, DateError>;

    /// Calculates the ordinal month for the given year and month code.
    ///
    /// The default impl is for non-lunisolar calendars!
    #[inline]
    fn ordinal_month_from_code(
        &self,
        _year: &Self::YearInfo,
        month_code: MonthCode,
        _options: DateFromFieldsOptions,
    ) -> Result<u8, DateError> {
        match month_code.parsed() {
            Some((month_number, false)) => Ok(month_number),
            _ => Err(DateError::UnknownMonthCode(month_code)),
        }
    }
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
    pub const fn new_unchecked(builder: ArithmeticDateBuilder<C::YearInfo>) -> Self {
        let ArithmeticDateBuilder { year, month, day } = builder;
        Self::new_unchecked_ymd(year, month, day)
    }

    #[inline]
    pub(crate) const fn new_unchecked_ymd(year: C::YearInfo, month: u8, day: u8) -> Self {
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
        ArithmeticDate::new_unchecked_ymd(i32::MIN, 1, 1)
    }

    #[inline]
    pub fn max_date() -> Self
    where
        C: CalendarArithmetic<YearInfo = i32>,
    {
        let year = i32::MAX;
        let (month, day) = C::last_month_day_in_provided_year(year);
        ArithmeticDate::new_unchecked_ymd(year, month, day)
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
        ArithmeticDate::new_unchecked_ymd(year, month, day)
    }

    #[inline]
    pub fn day_of_month(&self) -> types::DayOfMonth {
        types::DayOfMonth(self.day)
    }

    #[inline]
    pub fn day_of_year(&self) -> DayOfYear {
        DayOfYear(C::day_of_provided_year(self.year, self.month, self.day))
    }

    pub fn extended_year(&self) -> i32 {
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
    pub(crate) fn try_from_builder(
        builder: ArithmeticDateBuilder<C::YearInfo>,
        options: DateFromFieldsOptions,
    ) -> Result<Self, RangeError> {
        let ArithmeticDateBuilder { year, month, day } = builder;
        Ok(Self::new_unchecked_ymd(
            year,
            range_check_with_overflow(
                month,
                "month",
                1..=C::months_in_provided_year(year),
                options.overflow.unwrap_or_default(),
            )?,
            range_check_with_overflow(
                day,
                "day",
                1..=C::days_in_provided_month(year, month),
                options.overflow.unwrap_or_default(),
            )?,
        ))
    }

    pub(crate) fn try_from_ymd(year: C::YearInfo, month: u8, day: u8) -> Result<Self, RangeError> {
        let builder = ArithmeticDateBuilder { year, month, day };
        Self::try_from_builder(
            builder,
            DateFromFieldsOptions {
                overflow: Some(Overflow::Reject),
                ..Default::default()
            },
        )
    }
}

pub(crate) struct ArithmeticDateBuilder<YearInfo> {
    pub(crate) year: YearInfo,
    pub(crate) month: u8,
    pub(crate) day: u8,
}

impl<YearInfo> ArithmeticDateBuilder<YearInfo>
where
    YearInfo: PartialEq,
{
    pub(crate) fn try_from_fields<C>(
        fields: DateFields,
        cal: &C,
        options: DateFromFieldsOptions,
    ) -> Result<Self, DateError>
    where
        C: DateFieldsResolver<YearInfo = YearInfo>,
    {
        let missing_fields_strategy = options.missing_fields_strategy.unwrap_or_default();
        let maybe_year = {
            let extended_year_as_year_info = fields
                .extended_year
                .map(|extended_year| cal.year_info_from_extended(extended_year));
            match (fields.era, fields.era_year) {
                (None, None) => extended_year_as_year_info,
                (Some(era), Some(era_year)) => {
                    let era_year_as_year_info = cal.year_info_from_era(era, era_year)?;
                    if let Some(other) = extended_year_as_year_info {
                        if era_year_as_year_info != other {
                            return Err(DateError::InconsistentYear);
                        }
                    }
                    Some(era_year_as_year_info)
                }
                // Era and Era Year must be both or neither
                (Some(_), None) | (None, Some(_)) => return Err(DateError::NotEnoughFields),
            }
        };
        let day = match fields.day {
            Some(day) => day.get(),
            None => match missing_fields_strategy {
                MissingFieldsStrategy::Reject => return Err(DateError::NotEnoughFields),
                MissingFieldsStrategy::Ecma => match maybe_year {
                    // The ECMAScript strategy is to pick day 1, always, regardless of whether
                    // that day exists for the month/year combo
                    Some(_) => 1,
                    None => return Err(DateError::NotEnoughFields),
                },
            },
        };
        let year = match maybe_year {
            Some(year) => year,
            None => match missing_fields_strategy {
                MissingFieldsStrategy::Reject => return Err(DateError::NotEnoughFields),
                MissingFieldsStrategy::Ecma => match (fields.month_code, fields.ordinal_month) {
                    (Some(month_code), None) => {
                        cal.reference_year_from_month_day(month_code, day)?
                    }
                    _ => return Err(DateError::NotEnoughFields),
                },
            },
        };
        let month = {
            let ordinal_month_as_u8 = fields.ordinal_month.map(|x| x.get());
            match fields.month_code {
                Some(month_code) => {
                    let computed_month = cal.ordinal_month_from_code(&year, month_code, options)?;
                    if let Some(ordinal_month) = ordinal_month_as_u8 {
                        if computed_month != ordinal_month {
                            return Err(DateError::InconsistentMonth);
                        }
                    }
                    computed_month
                }
                None => match ordinal_month_as_u8 {
                    Some(month) => month,
                    None => return Err(DateError::NotEnoughFields),
                },
            }
        };
        Ok(Self { year, month, day })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cal::{abstract_gregorian::AbstractGregorian, iso::IsoEra};

    #[test]
    fn test_ord() {
        let dates_in_order = [
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-10, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-10, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-10, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-1, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-1, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(-1, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(0, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(0, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(0, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(1, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(1, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(1, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(10, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(10, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked_ymd(10, 2, 1),
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
