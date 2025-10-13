// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::range_check_with_overflow;
use crate::options::{DateAddOptions, DateDifferenceOptions};
use crate::options::{DateFromFieldsOptions, MissingFieldsStrategy, Overflow};
use crate::types::{DateDuration, DateDurationUnit, DateFields, DayOfYear, MonthCode};
use crate::{types, Calendar, DateError, RangeError};
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
        self.year.to_extended_year() == other.year.to_extended_year()
            && self.month == other.month
            && self.day == other.day
    }
}

impl<C: CalendarArithmetic> Eq for ArithmeticDate<C> {}

impl<C: CalendarArithmetic> Ord for ArithmeticDate<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .to_extended_year()
            .cmp(&other.year.to_extended_year())
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
        self.year.to_extended_year().hash(state);
        self.month.hash(state);
        self.day.hash(state);
    }
}

/// Maximum number of iterations when iterating through the days of a month; can be increased if necessary
#[allow(dead_code)] // TODO: Remove dead code tag after use
pub(crate) const MAX_ITERS_FOR_DAYS_OF_MONTH: u8 = 33;

pub(crate) trait ToExtendedYear {
    fn to_extended_year(&self) -> i32;
}

impl ToExtendedYear for i32 {
    fn to_extended_year(&self) -> i32 {
        *self
    }
}

pub(crate) trait CalendarArithmetic: Calendar + DateFieldsResolver {
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
    /// This stores the year as either an i32, or a type containing more
    /// useful computational information.
    type YearInfo: Copy + Debug + PartialEq + ToExtendedYear;

    /// Converts the era and era year to a YearInfo. If the calendar does not have eras,
    /// this should always return an Err result.
    fn year_info_from_era(&self, era: &str, era_year: i32) -> Result<Self::YearInfo, DateError>;

    /// Converts an extended year to a YearInfo.
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo;

    /// Calculates the ECMA reference year for the month code and day, or an error
    /// if the month code and day are invalid.
    ///
    /// Note that this is called before any potential Overflow::Constrain application,
    /// so this should accept out-of-range day values as if they are the highest possible
    /// day for the given month.
    fn reference_year_from_month_day(
        &self,
        month_code: MonthCode,
        day: u8,
    ) -> Result<Self::YearInfo, DateError>;

    /// Calculates the ordinal month for the given year and month code.
    ///
    /// The default impl is for non-lunisolar calendars with 12 months!
    #[inline]
    fn ordinal_month_from_code(
        &self,
        _year: &Self::YearInfo,
        month_code: MonthCode,
        _options: DateFromFieldsOptions,
    ) -> Result<u8, DateError> {
        match month_code.parsed() {
            Some((month_number @ 1..=12, false)) => Ok(month_number),
            _ => Err(DateError::UnknownMonthCode(month_code)),
        }
    }

    /// Calculates the month code from the given ordinal month and year.
    ///
    /// The caller must ensure that the ordinal is in range.
    ///
    /// The default impl is for non-lunisolar calendars!
    #[inline]
    fn month_code_from_ordinal(
        &self,
        _year: &Self::YearInfo,
        ordinal_month: u8,
    ) -> types::MonthInfo {
        let code = match MonthCode::new_normal(ordinal_month) {
            Some(code) => code,
            None => {
                debug_assert!(false, "ordinal month out of range!");
                MonthCode(tinystr!(4, "und"))
            }
        };
        types::MonthInfo {
            ordinal: ordinal_month,
            standard_code: code,
            formatting_code: code,
        }
    }
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    #[inline]
    pub(crate) const fn new_unchecked(year: C::YearInfo, month: u8, day: u8) -> Self {
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

    pub fn extended_year(&self) -> i32 {
        self.year.to_extended_year()
    }

    /// Construct a new arithmetic date from a year, month ordinal, and day, bounds checking
    /// the month and day according to the overflow parameter.
    pub(crate) fn try_from_builder(
        builder: ArithmeticDateBuilder<C::YearInfo>,
        options: DateFromFieldsOptions,
    ) -> Result<Self, RangeError> {
        let ArithmeticDateBuilder { year, month, day } = builder;
        let constrained_month = range_check_with_overflow(
            month,
            "month",
            1..=C::months_in_provided_year(year),
            options.overflow.unwrap_or_default(),
        )?;
        Ok(Self::new_unchecked(
            year,
            constrained_month,
            range_check_with_overflow(
                day,
                "day",
                1..=C::days_in_provided_month(year, constrained_month),
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

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    /// Implements the Temporal abstract operation BalanceNonISODate.
    ///
    /// This takes a year, month, and day, where the month and day might be out of range, then
    /// balances excess months into the year field and excess days into the month field.
    pub(crate) fn new_balanced(year: C::YearInfo, ordinal_month: i64, day: i64, cal: &C) -> Self {
        // 1. Let _resolvedYear_ be _arithmeticYear_.
        // 1. Let _resolvedMonth_ be _ordinalMonth_.
        let mut resolved_year = year;
        let mut resolved_month = ordinal_month;
        // 1. Let _monthsInYear_ be CalendarMonthsInYear(_calendar_, _resolvedYear_).
        let mut months_in_year = C::months_in_provided_year(resolved_year);
        // 1. Repeat, while _resolvedMonth_ &le; 0,
        //   1. Set _resolvedYear_ to _resolvedYear_ - 1.
        //   1. Set _monthsInYear_ to CalendarMonthsInYear(_calendar_, _resolvedYear_).
        //   1. Set _resolvedMonth_ to _resolvedMonth_ + _monthsInYear_.
        while resolved_month <= 0 {
            resolved_year = cal.year_info_from_extended(resolved_year.to_extended_year() - 1);
            months_in_year = C::months_in_provided_year(resolved_year);
            resolved_month += i64::from(months_in_year);
        }
        // 1. Repeat, while _resolvedMonth_ &gt; _monthsInYear_,
        //   1. Set _resolvedMonth_ to _resolvedMonth_ - _monthsInYear_.
        //   1. Set _resolvedYear_ to _resolvedYear_ + 1.
        //   1. Set _monthsInYear_ to CalendarMonthsInYear(_calendar_, _resolvedYear_).
        while resolved_month > i64::from(months_in_year) {
            resolved_month -= i64::from(months_in_year);
            resolved_year = cal.year_info_from_extended(resolved_year.to_extended_year() + 1);
            months_in_year = C::months_in_provided_year(resolved_year);
        }
        debug_assert!(u8::try_from(resolved_month).is_ok());
        let mut resolved_month = resolved_month as u8;
        // 1. Let _resolvedDay_ be _day_.
        let mut resolved_day = day;
        // 1. Let _daysInMonth_ be CalendarDaysInMonth(_calendar_, _resolvedYear_, _resolvedMonth_).
        let mut days_in_month = C::days_in_provided_month(resolved_year, resolved_month);
        // 1. Repeat, while _resolvedDay_ &le; 0,
        while resolved_day <= 0 {
            //   1. Set _resolvedMonth_ to _resolvedMonth_ - 1.
            //   1. If _resolvedMonth_ is 0, then
            resolved_month -= 1;
            if resolved_month == 0 {
                //     1. Set _resolvedYear_ to _resolvedYear_ - 1.
                //     1. Set _monthsInYear_ to CalendarMonthsInYear(_calendar_, _resolvedYear_).
                //     1. Set _resolvedMonth_ to _monthsInYear_.
                resolved_year = cal.year_info_from_extended(resolved_year.to_extended_year() - 1);
                months_in_year = C::months_in_provided_year(resolved_year);
                resolved_month = months_in_year;
            }
            //   1. Set _daysInMonth_ to CalendarDaysInMonth(_calendar_, _resolvedYear_, _resolvedMonth_).
            //   1. Set _resolvedDay_ to _resolvedDay_ + _daysInMonth_.
            days_in_month = C::days_in_provided_month(resolved_year, resolved_month);
            resolved_day += i64::from(days_in_month);
        }
        // 1. Repeat, while _resolvedDay_ &gt; _daysInMonth_,
        while resolved_day > i64::from(days_in_month) {
            //   1. Set _resolvedDay_ to _resolvedDay_ - _daysInMonth_.
            //   1. Set _resolvedMonth_ to _resolvedMonth_ + 1.
            //   1. If _resolvedMonth_ &gt; _monthsInYear_, then
            resolved_day -= i64::from(days_in_month);
            resolved_month += 1;
            if resolved_month > months_in_year {
                //     1. Set _resolvedYear_ to _resolvedYear_ + 1.
                //     1. Set _monthsInYear_ to CalendarMonthsInYear(_calendar_, _resolvedYear_).
                //     1. Set _resolvedMonth_ to 1.
                resolved_year = cal.year_info_from_extended(resolved_year.to_extended_year() + 1);
                months_in_year = C::months_in_provided_year(resolved_year);
                resolved_month = 1;
            }
            //   1. Set _daysInMonth_ to CalendarDaysInMonth(_calendar_, _resolvedYear_, _resolvedMonth_).
            days_in_month = C::days_in_provided_month(resolved_year, resolved_month);
        }
        debug_assert!(u8::try_from(resolved_day).is_ok());
        let resolved_day = resolved_day as u8;
        // 1. Return the Record { [[Year]]: _resolvedYear_, [[Month]]: _resolvedMonth_, [[Day]]: _resolvedDay_ }.
        Self::new_unchecked(resolved_year, resolved_month, resolved_day)
    }

    /// Implements the Temporal abstract operation NonISODateSurpasses.
    ///
    /// This takes two dates (`self` and `other`), `duration`, and `sign` (either -1 or 1), then
    /// returns whether adding the duration to `self` results in a year/month/day that exceeds
    /// `other` in the direction indicated by `sign`, constraining the month but not the day.
    pub(crate) fn surpasses(
        &self,
        other: &Self,
        duration: DateDuration,
        sign: i64,
        cal: &C,
    ) -> bool {
        // 1. Let _parts_ be CalendarISOToDate(_calendar_, _fromIsoDate_).
        // 1. Let _y0_ be _parts_.[[Year]] + _years_.
        let y0 = cal.year_info_from_extended(duration.add_years_to(self.year.to_extended_year()));
        // 1. Let _m0_ be MonthCodeToOrdinal(_calendar_, _y0_, ! ConstrainMonthCode(_calendar_, _y0_, _parts_.[[MonthCode]], ~constrain~)).
        let base_month_code = cal
            .month_code_from_ordinal(&self.year, self.month)
            .standard_code;
        let constrain = DateFromFieldsOptions {
            overflow: Some(Overflow::Constrain),
            ..Default::default()
        };
        let m0_result = cal.ordinal_month_from_code(&y0, base_month_code, constrain);
        let m0 = match m0_result {
            Ok(m0) => m0,
            Err(_) => {
                debug_assert!(
                    false,
                    "valid month code for calendar, and constrained to the year"
                );
                1
            }
        };
        // 1. Let _endOfMonth_ be BalanceNonISODate(_calendar_, _y0_, _m0_ + _months_ + 1, 0).
        let end_of_month = Self::new_balanced(y0, duration.add_months_to(m0) + 1, 0, cal);
        // 1. Let _baseDay_ be _parts_.[[Day]].
        let base_day = self.day;
        let y1;
        let m1;
        let d1;
        // 1. If _weeks_ is not 0 or _days_ is not 0, then
        if duration.weeks != 0 || duration.days != 0 {
            //   1. If _baseDay_ &lt; _endOfMonth_.[[Day]], then
            //     1. Let _regulatedDay_ be _baseDay_.
            //   1. Else,
            //     1. Let _regulatedDay_ be _endOfMonth_.[[Day]].
            let regulated_day = if base_day < end_of_month.day {
                base_day
            } else {
                end_of_month.day
            };
            //   1. Let _balancedDate_ be BalanceNonISODate(_calendar_, _endOfMonth_.[[Year]], _endOfMonth_.[[Month]], _regulatedDay_ + 7 * _weeks_ + _days_).
            //   1. Let _y1_ be _balancedDate_.[[Year]].
            //   1. Let _m1_ be _balancedDate_.[[Month]].
            //   1. Let _d1_ be _balancedDate_.[[Day]].
            let balanced_date = Self::new_balanced(
                end_of_month.year,
                i64::from(end_of_month.month),
                duration.add_weeks_and_days_to(regulated_day),
                cal,
            );
            y1 = balanced_date.year;
            m1 = balanced_date.month;
            d1 = balanced_date.day;
        } else {
            // 1. Else,
            //   1. Let _y1_ be _endOfMonth_.[[Year]].
            //   1. Let _m1_ be _endOfMonth_.[[Month]].
            //   1. Let _d1_ be _baseDay_.
            y1 = end_of_month.year;
            m1 = end_of_month.month;
            d1 = base_day;
        }
        // 1. Let _calDate2_ be CalendarISOToDate(_calendar_, _toIsoDate_).
        // 1. If _y1_ ≠ _calDate2_.[[Year]], then
        //   1. If _sign_ × (_y1_ - _calDate2_.[[Year]]) > 0, return *true*.
        // 1. Else if _m1_ ≠ _calDate2_.[[Month]], then
        //   1. If _sign_ × (_m1_ - _calDate2_.[[Month]]) > 0, return *true*.
        // 1. Else if _d1_ ≠ _calDate2_.[[Day]], then
        //   1. If _sign_ × (_d1_ - _calDate2_.[[Day]]) > 0, return *true*.
        #[allow(clippy::collapsible_if)] // to align with the spec
        if y1 != other.year {
            if sign * (i64::from(y1.to_extended_year()) - i64::from(other.year.to_extended_year()))
                > 0
            {
                return true;
            }
        } else if m1 != other.month {
            if sign * (i64::from(m1) - i64::from(other.month)) > 0 {
                return true;
            }
        } else if d1 != other.day {
            if sign * (i64::from(d1) - i64::from(other.day)) > 0 {
                return true;
            }
        }
        // 1. Return *false*.
        false
    }

    /// Implements the Temporal abstract operation NonISODateAdd.
    ///
    /// This takes a date (`self`) and `duration`, then returns a new date resulting from
    /// adding `duration` to `self`, constrained according to `options`.
    pub(crate) fn added(
        &self,
        duration: DateDuration,
        cal: &C,
        options: DateAddOptions,
    ) -> Result<Self, DateError> {
        // 1. Let _parts_ be CalendarISOToDate(_calendar_, _isoDate_).
        // 1. Let _y0_ be _parts_.[[Year]] + _duration_.[[Years]].
        let y0 = cal.year_info_from_extended(duration.add_years_to(self.year.to_extended_year()));
        // 1. Let _m0_ be MonthCodeToOrdinal(_calendar_, _y0_, ! ConstrainMonthCode(_calendar_, _y0_, _parts_.[[MonthCode]], _overflow_)).
        let base_month_code = cal
            .month_code_from_ordinal(&self.year, self.month)
            .standard_code;
        let m0 = cal.ordinal_month_from_code(
            &y0,
            base_month_code,
            DateFromFieldsOptions::from_add_options(options),
        )?;
        // 1. Let _endOfMonth_ be BalanceNonISODate(_calendar_, _y0_, _m0_ + _duration_.[[Months]] + 1, 0).
        let end_of_month = Self::new_balanced(y0, duration.add_months_to(m0) + 1, 0, cal);
        // 1. Let _baseDay_ be _parts_.[[Day]].
        let base_day = self.day;
        // 1. If _baseDay_ &lt; _endOfMonth_.[[Day]], then
        //   1. Let _regulatedDay_ be _baseDay_.
        let regulated_day = if base_day < end_of_month.day {
            base_day
        } else {
            // 1. Else,
            //   1. If _overflow_ is ~reject~, throw a *RangeError* exception.
            // Note: ICU4X default is constrain here
            if matches!(options.overflow, Some(Overflow::Reject)) {
                return Err(DateError::Range {
                    field: "day",
                    value: i32::from(base_day),
                    min: 1,
                    max: i32::from(end_of_month.day),
                });
            }
            end_of_month.day
        };
        // 1. Let _balancedDate_ be BalanceNonISODate(_calendar_, _endOfMonth_.[[Year]], _endOfMonth_.[[Month]], _regulatedDay_ + 7 * _duration_.[[Weeks]] + _duration_.[[Days]]).
        // 1. Let _result_ be ? CalendarIntegersToISO(_calendar_, _balancedDate_.[[Year]], _balancedDate_.[[Month]], _balancedDate_.[[Day]]).
        // 1. Return _result_.
        Ok(Self::new_balanced(
            end_of_month.year,
            i64::from(end_of_month.month),
            duration.add_weeks_and_days_to(regulated_day),
            cal,
        ))
    }

    /// Implements the Temporal abstract operation NonISODateUntil.
    ///
    /// This takes a duration (`self`) and a date (`other`), then returns a duration that, when
    /// added to `self`, results in `other`, with largest unit according to `options`.
    pub(crate) fn until(
        &self,
        other: &Self,
        cal: &C,
        options: DateDifferenceOptions,
    ) -> DateDuration {
        // 1. Let _sign_ be -1 × CompareISODate(_one_, _two_).
        // 1. If _sign_ = 0, return ZeroDateDuration().
        let sign = match other.cmp(self) {
            Ordering::Greater => 1i64,
            Ordering::Equal => return DateDuration::default(),
            Ordering::Less => -1i64,
        };
        // 1. Let _years_ be 0.
        // 1. If _largestUnit_ is ~year~, then
        //   1. Let _candidateYears_ be _sign_.
        //   1. Repeat, while NonISODateSurpasses(_calendar_, _sign_, _one_, _two_, _candidateYears_, 0, 0, 0) is *false*,
        //     1. Set _years_ to _candidateYears_.
        //     1. Set _candidateYears_ to _candidateYears_ + _sign_.
        let mut years = 0;
        if matches!(options.largest_unit, Some(DateDurationUnit::Years)) {
            let mut candidate_years = sign;
            while !self.surpasses(
                other,
                DateDuration::from_signed_ymwd(candidate_years, 0, 0, 0),
                sign,
                cal,
            ) {
                years = candidate_years;
                candidate_years += sign;
            }
        }
        // 1. Let _months_ be 0.
        // 1. If _largestUnit_ is ~year~ or _largestUnit_ is ~month~, then
        //   1. Let _candidateMonths_ be _sign_.
        //   1. Repeat, while NonISODateSurpasses(_calendar_, _sign_, _one_, _two_, _years_, _candidateMonths_, 0, 0) is *false*,
        //     1. Set _months_ to _candidateMonths_.
        //     1. Set _candidateMonths_ to _candidateMonths_ + _sign_.
        let mut months = 0;
        if matches!(
            options.largest_unit,
            Some(DateDurationUnit::Years) | Some(DateDurationUnit::Months)
        ) {
            let mut candidate_months = sign;
            while !self.surpasses(
                other,
                DateDuration::from_signed_ymwd(years, candidate_months, 0, 0),
                sign,
                cal,
            ) {
                months = candidate_months;
                candidate_months += sign;
            }
        }
        // 1. Let _weeks_ be 0.
        // 1. If _largestUnit_ is ~week~, then
        //   1. Let _candidateWeeks_ be _sign_.
        //   1. Repeat, while NonISODateSurpasses(_calendar_, _sign_, _one_, _two_, _years_, _months_, _candidateWeeks_, 0) is *false*,
        //     1. Set _weeks_ to _candidateWeeks_.
        //     1. Set _candidateWeeks_ to _candidateWeeks_ + sign.
        let mut weeks = 0;
        if matches!(options.largest_unit, Some(DateDurationUnit::Weeks)) {
            let mut candidate_weeks = sign;
            while !self.surpasses(
                other,
                DateDuration::from_signed_ymwd(years, months, candidate_weeks, 0),
                sign,
                cal,
            ) {
                weeks = candidate_weeks;
                candidate_weeks += sign;
            }
        }
        // 1. Let _days_ be 0.
        // 1. Let _candidateDays_ be _sign_.
        // 1. Repeat, while NonISODateSurpasses(_calendar_, _sign_, _one_, _two_, _years_, _months_, _weeks_, _candidateDays_) is *false*,
        //   1. Set _days_ to _candidateDays_.
        //   1. Set _candidateDays_ to _candidateDays_ + _sign_.
        let mut days = 0;
        let mut candidate_days = sign;
        while !self.surpasses(
            other,
            DateDuration::from_signed_ymwd(years, months, weeks, candidate_days),
            sign,
            cal,
        ) {
            days = candidate_days;
            candidate_days += sign;
        }
        // 1. Return ! CreateDateDurationRecord(_years_, _months_, _weeks_, _days_).
        DateDuration::from_signed_ymwd(years, months, weeks, days)
    }
}

pub(crate) struct ArithmeticDateBuilder<YearInfo> {
    pub(crate) year: YearInfo,
    pub(crate) month: u8,
    pub(crate) day: u8,
}

impl<YearInfo> ArithmeticDateBuilder<YearInfo>
where
    YearInfo: PartialEq + Debug,
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
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-10, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-10, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-10, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-1, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-1, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(-1, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(0, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(0, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(0, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(1, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(1, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(1, 2, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(10, 1, 1),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(10, 1, 2),
            ArithmeticDate::<AbstractGregorian<IsoEra>>::new_unchecked(10, 2, 1),
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
