// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use calendrical_calculations::rata_die::RataDie;

use crate::duration::{DateDuration, DateDurationUnit};
use crate::error::{
    range_check, range_check_with_overflow, DateFromFieldsError, EcmaReferenceYearError,
    MonthCodeError, MonthCodeParseError, UnknownEraError,
};
use crate::options::{DateAddOptions, DateDifferenceOptions};
use crate::options::{DateFromFieldsOptions, MissingFieldsStrategy, Overflow};
use crate::types::{DateFields, Month};
use crate::{types, Calendar, DateError, RangeError};
use core::cmp::Ordering;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::ops::RangeInclusive;

/// This is checked by constructors. Internally we don't care about this invariant.
pub const VALID_YEAR_RANGE: RangeInclusive<i32> = -1_000_000..=1_000_000;

/// This is a fundamental invariant of `ArithmeticDate` and by extension all our
/// date types. Because this range slightly exceeds the [`VALID_YEAR_RANGE`], only
/// the valid year range is checked in constructors. Only the `Date::from_rata_die`
/// constructor actually uses this, but for clamping instead of erroring.
// This range is the tightest possible range that includes all valid years for all
// calendars, this is asserted in [`test_validity_ranges`].
pub const VALID_RD_RANGE: RangeInclusive<RataDie> =
    RataDie::new(-367256444)..=RataDie::new(365940477);

// Invariant: VALID_RD_RANGE contains the date
#[derive(Debug)]
pub(crate) struct ArithmeticDate<C: DateFieldsResolver>(<C::YearInfo as PackWithMD>::Packed);

// Manual impls since the derive will introduce a C: Trait bound
// and only the year value should be compared
impl<C: DateFieldsResolver> Copy for ArithmeticDate<C> {}
impl<C: DateFieldsResolver> Clone for ArithmeticDate<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: DateFieldsResolver> PartialEq for ArithmeticDate<C> {
    fn eq(&self, other: &Self) -> bool {
        self.year().to_extended_year() == other.year().to_extended_year()
            && self.month() == other.month()
            && self.day() == other.day()
    }
}

impl<C: DateFieldsResolver> Eq for ArithmeticDate<C> {}

impl<C: DateFieldsResolver> Ord for ArithmeticDate<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year()
            .to_extended_year()
            .cmp(&other.year().to_extended_year())
            .then(self.month().cmp(&other.month()))
            .then(self.day().cmp(&other.day()))
    }
}

impl<C: DateFieldsResolver> PartialOrd for ArithmeticDate<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: DateFieldsResolver> Hash for ArithmeticDate<C> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.year().to_extended_year().hash(state);
        self.month().hash(state);
        self.day().hash(state);
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

pub(crate) trait PackWithMD: Copy {
    type Packed: Copy + Debug;

    fn pack(self, month: u8, day: u8) -> Self::Packed;
    fn unpack_year(packed: Self::Packed) -> Self;
    fn unpack_month(packed: Self::Packed) -> u8;
    fn unpack_day(packed: Self::Packed) -> u8;
}

impl PackWithMD for i32 {
    /// 2 bits unused, 21 bits year (test_validity_ranges),
    /// 4 bits month (1..13), 5 bits day (1..31)
    type Packed = [u8; 4];

    fn pack(self, month: u8, day: u8) -> Self::Packed {
        (self << 9 | (month as i32) << 5 | day as i32).to_le_bytes()
    }

    fn unpack_year(packed: Self::Packed) -> Self {
        let packed = i32::from_le_bytes(packed);
        packed >> 9
    }

    fn unpack_month(packed: Self::Packed) -> u8 {
        let packed = i32::from_le_bytes(packed);
        (packed >> 5 & 0b1111) as u8
    }

    fn unpack_day(packed: Self::Packed) -> u8 {
        let packed = i32::from_le_bytes(packed);
        (packed & 0b11111) as u8
    }
}

/// Trait for converting from era codes, month codes, and other fields to year/month/day ordinals.
pub(crate) trait DateFieldsResolver: Calendar {
    /// This stores the year as either an i32, or a type containing more
    /// useful computational information.
    type YearInfo: Copy + Debug + PartialEq + ToExtendedYear + PackWithMD;

    fn days_in_provided_month(year: Self::YearInfo, month: u8) -> u8;

    /// Converts the era and era year to a YearInfo. If the calendar does not have eras,
    /// this should always return an Err result.
    fn year_info_from_era(
        &self,
        era: &[u8],
        era_year: i32,
    ) -> Result<Self::YearInfo, UnknownEraError>;

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
        month: Month,
        day: u8,
    ) -> Result<Self::YearInfo, EcmaReferenceYearError>;

    /// The number of months for the given year.
    ///
    /// The default impl is for non-lunisolar calendars with 12 months!
    fn months_in_provided_year(_year: Self::YearInfo) -> u8 {
        12
    }

    /// Calculates the ordinal month for the given year and month code.
    ///
    /// The default impl is for non-lunisolar calendars!
    #[inline]
    fn ordinal_from_month(
        &self,
        year: Self::YearInfo,
        month: Month,
        _options: DateFromFieldsOptions,
    ) -> Result<u8, MonthCodeError> {
        match (month.number(), month.is_leap()) {
            (month_number, false)
                if (1..=Self::months_in_provided_year(year)).contains(&month_number) =>
            {
                Ok(month_number)
            }
            _ => Err(MonthCodeError::NotInCalendar),
        }
    }

    /// Calculates the month code from the given ordinal month and year.
    ///
    /// The caller must ensure that the ordinal is in range.
    ///
    /// The default impl is for non-lunisolar calendars!
    #[inline]
    fn month_from_ordinal(&self, _year: Self::YearInfo, ordinal_month: u8) -> Month {
        Month::new_unchecked(ordinal_month, false)
    }
}

impl<C: DateFieldsResolver> ArithmeticDate<C> {
    pub(crate) fn year(self) -> C::YearInfo {
        C::YearInfo::unpack_year(self.0)
    }

    pub(crate) fn month(self) -> u8 {
        C::YearInfo::unpack_month(self.0)
    }

    pub(crate) fn day(self) -> u8 {
        C::YearInfo::unpack_day(self.0)
    }

    // Precondition: the date is in the VALID_RD_RANGE
    #[inline]
    pub(crate) fn new_unchecked(year: C::YearInfo, month: u8, day: u8) -> Self {
        ArithmeticDate(C::YearInfo::pack(year, month, day))
    }

    pub(crate) fn cast<C2: DateFieldsResolver<YearInfo = C::YearInfo>>(self) -> ArithmeticDate<C2> {
        ArithmeticDate::new_unchecked(self.year(), self.month(), self.day())
    }

    pub(crate) fn from_codes(
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
        calendar: &C,
    ) -> Result<Self, DateError> {
        let year = range_check(year, "year", VALID_YEAR_RANGE)?;
        let year = if let Some(era) = era {
            calendar.year_info_from_era(era.as_bytes(), year)?
        } else {
            calendar.year_info_from_extended(year)
        };
        let validated = Month::try_from_utf8(month_code.0.as_bytes()).map_err(|e| match e {
            MonthCodeParseError::InvalidSyntax => DateError::UnknownMonthCode(month_code),
        })?;
        let month = calendar
            .ordinal_from_month(year, validated, Default::default())
            .map_err(|e| match e {
                MonthCodeError::NotInCalendar | MonthCodeError::NotInYear => {
                    DateError::UnknownMonthCode(month_code)
                }
            })?;

        let day = range_check(day, "day", 1..=C::days_in_provided_month(year, month))?;

        // date is in the valid year range, and therefore in the valid RD range
        Ok(ArithmeticDate::new_unchecked(year, month, day))
    }

    pub(crate) fn from_fields(
        fields: DateFields,
        options: DateFromFieldsOptions,
        calendar: &C,
    ) -> Result<Self, DateFromFieldsError> {
        let missing_fields_strategy = options.missing_fields_strategy.unwrap_or_default();

        let day = match fields.day {
            Some(day) => day,
            None => match missing_fields_strategy {
                MissingFieldsStrategy::Reject => return Err(DateFromFieldsError::NotEnoughFields),
                MissingFieldsStrategy::Ecma => {
                    if fields.extended_year.is_some() || fields.era_year.is_some() {
                        // The ECMAScript strategy is to pick day 1, always, regardless of whether
                        // that day exists for the month/year combo
                        1
                    } else {
                        return Err(DateFromFieldsError::NotEnoughFields);
                    }
                }
            },
        };

        if fields.month_code.is_none() && fields.ordinal_month.is_none() {
            // We're returning this error early so that we return structural type
            // errors before range errors, see comment in the year code below.
            return Err(DateFromFieldsError::NotEnoughFields);
        }

        let mut valid_month_code = None;

        // NOTE: The year/extendedyear range check is important to avoid arithmetic
        // overflow in `year_info_from_era` and `year_info_from_extended`. It
        // must happen before they are called.
        //
        // To better match the Temporal specification's order of operations, we try
        // to return structural type errors (`NotEnoughFields`) before checking for range errors.
        // This isn't behavior we *must* have, but it is not much additional work to maintain
        // so we make an attempt.
        let year = match (fields.era, fields.era_year) {
            (None, None) => match fields.extended_year {
                Some(extended_year) => calendar.year_info_from_extended(range_check(
                    extended_year,
                    "year",
                    VALID_YEAR_RANGE,
                )?),
                None => match missing_fields_strategy {
                    MissingFieldsStrategy::Reject => {
                        return Err(DateFromFieldsError::NotEnoughFields)
                    }
                    MissingFieldsStrategy::Ecma => {
                        match (fields.month_code, fields.ordinal_month) {
                            (Some(month_code), None) => {
                                let validated = Month::try_from_utf8(month_code)?;
                                valid_month_code = Some(validated);
                                calendar.reference_year_from_month_day(validated, day)?
                            }
                            _ => return Err(DateFromFieldsError::NotEnoughFields),
                        }
                    }
                },
            },
            (Some(era), Some(era_year)) => {
                let era_year_as_year_info = calendar
                    .year_info_from_era(era, range_check(era_year, "year", VALID_YEAR_RANGE)?)?;
                if let Some(extended_year) = fields.extended_year {
                    if era_year_as_year_info
                        != calendar.year_info_from_extended(range_check(
                            extended_year,
                            "year",
                            VALID_YEAR_RANGE,
                        )?)
                    {
                        return Err(DateFromFieldsError::InconsistentYear);
                    }
                }
                era_year_as_year_info
            }
            // Era and Era Year must be both or neither
            (Some(_), None) | (None, Some(_)) => return Err(DateFromFieldsError::NotEnoughFields),
        };

        let month = match fields.month_code {
            Some(month_code) => {
                let validated = match valid_month_code {
                    Some(validated) => validated,
                    None => Month::try_from_utf8(month_code)?,
                };
                let computed_month = calendar.ordinal_from_month(year, validated, options)?;
                if let Some(ordinal_month) = fields.ordinal_month {
                    if computed_month != ordinal_month {
                        return Err(DateFromFieldsError::InconsistentMonth);
                    }
                }
                computed_month
            }
            None => match fields.ordinal_month {
                Some(month) => month,
                None => {
                    debug_assert!(false, "Already checked above");
                    return Err(DateFromFieldsError::NotEnoughFields);
                }
            },
        };

        let constrained_month = range_check_with_overflow(
            month,
            "month",
            1..=C::months_in_provided_year(year),
            options.overflow.unwrap_or_default(),
        )?;

        let day = range_check_with_overflow(
            day,
            "day",
            1..=C::days_in_provided_month(year, constrained_month),
            options.overflow.unwrap_or_default(),
        )?;
        // date is in the valid year range, and therefore in the valid RD range
        Ok(Self::new_unchecked(year, constrained_month, day))
    }

    pub(crate) fn try_from_ymd(year: C::YearInfo, month: u8, day: u8) -> Result<Self, RangeError> {
        range_check(year.to_extended_year(), "year", VALID_YEAR_RANGE)?;
        range_check(month, "month", 1..=C::months_in_provided_year(year))?;
        range_check(day, "day", 1..=C::days_in_provided_month(year, month))?;
        // date is in the valid year range, and therefore in the valid RD range
        Ok(ArithmeticDate::new_unchecked(year, month, day))
    }

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
        // TODO: does not obey precondition
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
        let y0 = cal.year_info_from_extended(duration.add_years_to(self.year().to_extended_year()));
        // 1. Let _m0_ be MonthCodeToOrdinal(_calendar_, _y0_, ! ConstrainMonthCode(_calendar_, _y0_, _parts_.[[MonthCode]], ~constrain~)).
        let base_month = cal.month_from_ordinal(self.year(), self.month());
        let constrain = DateFromFieldsOptions {
            overflow: Some(Overflow::Constrain),
            ..Default::default()
        };
        let m0_result = cal.ordinal_from_month(y0, base_month, constrain);
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
        let base_day = self.day();
        let y1;
        let m1;
        let d1;
        // 1. If _weeks_ is not 0 or _days_ is not 0, then
        if duration.weeks != 0 || duration.days != 0 {
            //   1. If _baseDay_ &lt; _endOfMonth_.[[Day]], then
            //     1. Let _regulatedDay_ be _baseDay_.
            //   1. Else,
            //     1. Let _regulatedDay_ be _endOfMonth_.[[Day]].
            let regulated_day = if base_day < end_of_month.day() {
                base_day
            } else {
                end_of_month.day()
            };
            //   1. Let _balancedDate_ be BalanceNonISODate(_calendar_, _endOfMonth_.[[Year]], _endOfMonth_.[[Month]], _regulatedDay_ + 7 * _weeks_ + _days_).
            //   1. Let _y1_ be _balancedDate_.[[Year]].
            //   1. Let _m1_ be _balancedDate_.[[Month]].
            //   1. Let _d1_ be _balancedDate_.[[Day]].
            let balanced_date = Self::new_balanced(
                end_of_month.year(),
                i64::from(end_of_month.month()),
                duration.add_weeks_and_days_to(regulated_day),
                cal,
            );
            y1 = balanced_date.year();
            m1 = balanced_date.month();
            d1 = balanced_date.day();
        } else {
            // 1. Else,
            //   1. Let _y1_ be _endOfMonth_.[[Year]].
            //   1. Let _m1_ be _endOfMonth_.[[Month]].
            //   1. Let _d1_ be _baseDay_.
            y1 = end_of_month.year();
            m1 = end_of_month.month();
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
        if y1 != other.year() {
            if sign
                * (i64::from(y1.to_extended_year()) - i64::from(other.year().to_extended_year()))
                > 0
            {
                return true;
            }
        } else if m1 != other.month() {
            if sign * (i64::from(m1) - i64::from(other.month())) > 0 {
                return true;
            }
        } else if d1 != other.day() {
            if sign * (i64::from(d1) - i64::from(other.day())) > 0 {
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
        let y0 = cal.year_info_from_extended(duration.add_years_to(self.year().to_extended_year()));
        // 1. Let _m0_ be MonthCodeToOrdinal(_calendar_, _y0_, ! ConstrainMonthCode(_calendar_, _y0_, _parts_.[[MonthCode]], _overflow_)).
        let base_month = cal.month_from_ordinal(self.year(), self.month());
        let m0 = cal
            .ordinal_from_month(
                y0,
                base_month,
                DateFromFieldsOptions::from_add_options(options),
            )
            .map_err(|e| {
                // TODO: Use a narrower error type here. For now, convert into DateError.
                match e {
                    MonthCodeError::NotInCalendar => DateError::UnknownMonthCode(base_month.code()),
                    MonthCodeError::NotInYear => DateError::UnknownMonthCode(base_month.code()),
                }
            })?;
        // 1. Let _endOfMonth_ be BalanceNonISODate(_calendar_, _y0_, _m0_ + _duration_.[[Months]] + 1, 0).
        let end_of_month = Self::new_balanced(y0, duration.add_months_to(m0) + 1, 0, cal);
        // 1. Let _baseDay_ be _parts_.[[Day]].
        let base_day = self.day();
        // 1. If _baseDay_ &lt; _endOfMonth_.[[Day]], then
        //   1. Let _regulatedDay_ be _baseDay_.
        let regulated_day = if base_day < end_of_month.day() {
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
                    max: i32::from(end_of_month.day()),
                });
            }
            end_of_month.day()
        };
        // 1. Let _balancedDate_ be BalanceNonISODate(_calendar_, _endOfMonth_.[[Year]], _endOfMonth_.[[Month]], _regulatedDay_ + 7 * _duration_.[[Weeks]] + _duration_.[[Days]]).
        // 1. Let _result_ be ? CalendarIntegersToISO(_calendar_, _balancedDate_.[[Year]], _balancedDate_.[[Month]], _balancedDate_.[[Day]]).
        // 1. Return _result_.
        Ok(Self::new_balanced(
            end_of_month.year(),
            i64::from(end_of_month.month()),
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

    #[test]
    fn test_validity_ranges() {
        use crate::{cal::*, Date};

        #[rustfmt::skip]
        let lowest_years = [
            Date::from_rata_die(*VALID_RD_RANGE.start(), Buddhist).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), ChineseTraditional::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Coptic).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Gregorian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Hebrew).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Hijri::new_umm_al_qura()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Indian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Iso).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Japanese::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Julian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), KoreanTraditional::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Persian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.start(), Roc).year().extended_year(),
        ];

        #[rustfmt::skip]
        let highest_years = [
            Date::from_rata_die(*VALID_RD_RANGE.end(), Buddhist).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), ChineseTraditional::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Coptic).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Gregorian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Hebrew).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Hijri::new_umm_al_qura()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday)).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Indian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Iso).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Japanese::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Julian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), KoreanTraditional::new()).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Persian).year().extended_year(),
            Date::from_rata_die(*VALID_RD_RANGE.end(), Roc).year().extended_year(),
        ];

        #[rustfmt::skip]
        let lowest_rds = [
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Buddhist).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, ChineseTraditional::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Coptic).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Gregorian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Hebrew).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Hijri::new_umm_al_qura()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Indian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Iso).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Japanese::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Julian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, KoreanTraditional::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Persian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.start(), Month::new(1).code(), 1, Roc).unwrap().to_rata_die(),
        ];

        #[rustfmt::skip]
        let highest_rds = [
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Buddhist).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, ChineseTraditional::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(13).code(), 5, Coptic).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(13).code(), 5, Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(13).code(), 5, Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Gregorian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 29, Hebrew).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, Hijri::new_umm_al_qura()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday)).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, Indian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Iso).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Japanese::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Julian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, KoreanTraditional::new()).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 30, Persian).unwrap().to_rata_die(),
            Date::try_new_from_codes(None, *VALID_YEAR_RANGE.end(), Month::new(12).code(), 31, Roc).unwrap().to_rata_die(),
        ];

        // RD range is tight
        assert_eq!(
            lowest_rds.iter().copied().min().unwrap(),
            *VALID_RD_RANGE.start()
        );
        assert_eq!(
            highest_rds.iter().copied().max().unwrap(),
            *VALID_RD_RANGE.end()
        );

        // Valid RDs can represent all valid years
        assert!(lowest_years.iter().all(|y| y <= VALID_YEAR_RANGE.start()));
        assert!(highest_years.iter().all(|y| y >= VALID_YEAR_RANGE.end()));

        // All years are 21-bits
        assert!(-lowest_years.iter().copied().min().unwrap() < 1 << 20);
        assert!(highest_years.iter().copied().max().unwrap() < 1 << 20);
    }
}
