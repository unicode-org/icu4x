// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and traits for use in the Chinese traditional lunar calendar,
//! as well as in related and derived calendars such as the Korean and Vietnamese lunar calendars.
//!
//! ```rust
//! use icu::calendar::{chinese::Chinese, Date, Iso};
//!
//! let iso_date = Date::try_new_iso_date(2023, 6, 23).unwrap();
//! let chinese_date =
//!     Date::new_from_iso(iso_date, Chinese::new_always_calculating());
//!
//! assert_eq!(chinese_date.year().number, 4660);
//! assert_eq!(chinese_date.year().related_iso, Some(2023));
//! assert_eq!(chinese_date.year().cyclic.unwrap().get(), 40);
//! assert_eq!(chinese_date.month().ordinal, 6);
//! assert_eq!(chinese_date.day_of_month().0, 6);
//! ```

use crate::{
    calendar_arithmetic::{ArithmeticDate, CalendarArithmetic, PrecomputedDataSource},
    types::MonthCode,
    Calendar, CalendarError, Iso,
};

use calendrical_calculations::chinese_based::{self, ChineseBased, YearBounds};
use calendrical_calculations::rata_die::RataDie;
use core::num::NonZeroU8;

/// The trait ChineseBased is used by Chinese-based calendars to perform computations shared by such calendar.
///
/// For an example of how to use this trait, see `impl ChineseBasedWithDataLoading for Chinese` in [`Chinese`].
pub(crate) trait ChineseBasedWithDataLoading: Calendar {
    type CB: ChineseBased;
    /// Get the compiled const data for a ChineseBased calendar; can return `None` if the given year
    /// does not correspond to any compiled data.
    fn get_precomputed_data(&self) -> ChineseBasedPrecomputedData<Self::CB>;
}

/// Chinese-based calendars define DateInner as a calendar-specific struct wrapping ChineseBasedDateInner.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChineseBasedDateInner<C: CalendarArithmetic>(pub(crate) ArithmeticDate<C>);

// we want these impls without the `C: Copy/Clone` bounds
impl<C: CalendarArithmetic> Copy for ChineseBasedDateInner<C> {}
impl<C: CalendarArithmetic> Clone for ChineseBasedDateInner<C> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Default)]
pub(crate) struct ChineseBasedPrecomputedData<CB: ChineseBased> {
    // TODO(#3933)
    // this should have the ability to be empty
    _cb: CB, // this is zero-sized
}

fn compute_cache<CB: ChineseBased>(extended_year: i32) -> ChineseBasedYearInfo {
    let mid_year = chinese_based::fixed_mid_year_from_year::<CB>(extended_year);
    let year_bounds = YearBounds::compute::<CB>(mid_year);
    let YearBounds {
        new_year,
        next_new_year,
        ..
    } = year_bounds;
    let (last_day_of_month, leap_month) =
        chinese_based::month_structure_for_year::<CB>(new_year, next_new_year);

    ChineseBasedYearInfo {
        new_year,
        // TODO(#3933): switch ChineseBasedYearInfo to packed info so we don't need to store as bloaty u16s
        last_day_of_month: last_day_of_month
            .map(|rd| (rd.to_i64_date() - new_year.to_i64_date()) as u16),
        leap_month,
    }
}

impl<CB: ChineseBased> PrecomputedDataSource<ChineseBasedYearInfo>
    for ChineseBasedPrecomputedData<CB>
{
    fn load_or_compute_info(&self, extended_year: i32) -> ChineseBasedYearInfo {
        // TODO(#3933): load based on year

        compute_cache::<CB>(extended_year)
    }
}

/// The struct containing compiled ChineseData
///
/// Bit structure:
///
/// ```text
/// Bit:             7   6   5   4   3   2   1   0
/// Byte 0:          [new year offset] | [  month lengths ..
/// Byte 1:          ....... month lengths .......
/// Byte 2:          ... ] | [ leap month index  ]
/// ```
///
/// Where the New Year Offset is the offset from ISO Jan 21 of that year for Chinese New Year,
/// the month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
#[derive(Debug, Copy, Clone)]
pub(crate) struct PackedChineseBasedYearInfo(pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl PackedChineseBasedYearInfo {
    #[allow(unused)] // TODO(#3933)
    pub(crate) fn unpack(self, related_iso: i32) -> ChineseBasedYearInfo {
        fn month_length(is_long: bool) -> u16 {
            if is_long {
                30
            } else {
                29
            }
        }

        let new_year_offset = ((self.0 & 0b11111000) >> 3) as u16;
        let new_year =
            Iso::fixed_from_iso(Iso::iso_from_year_day(related_iso, 21 + new_year_offset).inner);

        let mut last_day_of_month: [u16; 13] = [0; 13];
        let mut months_total = 0;

        months_total += month_length(self.0 & 0b100 != 0);
        last_day_of_month[0] = months_total;
        months_total += month_length(self.0 & 0b010 != 0);
        last_day_of_month[1] = months_total;
        months_total += month_length(self.0 & 0b001 != 0);
        last_day_of_month[2] = months_total;
        months_total += month_length(self.1 & 0b10000000 != 0);
        last_day_of_month[3] = months_total;
        months_total += month_length(self.1 & 0b01000000 != 0);
        last_day_of_month[4] = months_total;
        months_total += month_length(self.1 & 0b00100000 != 0);
        last_day_of_month[5] = months_total;
        months_total += month_length(self.1 & 0b00010000 != 0);
        last_day_of_month[6] = months_total;
        months_total += month_length(self.1 & 0b00001000 != 0);
        last_day_of_month[7] = months_total;
        months_total += month_length(self.1 & 0b00000100 != 0);
        last_day_of_month[8] = months_total;
        months_total += month_length(self.1 & 0b00000010 != 0);
        last_day_of_month[9] = months_total;
        months_total += month_length(self.1 & 0b00000001 != 0);
        last_day_of_month[10] = months_total;
        months_total += month_length(self.2 & 0b10000000 != 0);
        last_day_of_month[11] = months_total;

        let leap_month_bits = self.2 & 0b00111111;
        // Leap month is if the sentinel bit is set
        if leap_month_bits != 0 {
            months_total += month_length(self.2 & 0b01000000 != 0);
        }
        // In non-leap months, `last_day_of_month` will have identical entries at 12 and 11
        last_day_of_month[12] = months_total;

        // Will automatically set to None when the leap month bits are zero
        let leap_month = NonZeroU8::new(leap_month_bits);

        ChineseBasedYearInfo {
            new_year,
            last_day_of_month,
            leap_month,
        }
    }
}
/// A data struct used to load and use information for a set of ChineseBasedDates
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// TODO(#3933): potentially make this smaller
pub(crate) struct ChineseBasedYearInfo {
    pub(crate) new_year: RataDie,
    /// last_day_of_month[12] = last_day_of_month[11] in non-leap years
    /// These days are 1-indexed: so the last day of month for a 30-day 一月 is 30
    /// The array itself is zero-indexed, be careful passing it self.0.month!
    last_day_of_month: [u16; 13],
    ///
    pub(crate) leap_month: Option<NonZeroU8>,
}

impl ChineseBasedYearInfo {
    fn next_new_year(self) -> RataDie {
        self.new_year + i64::from(self.last_day_of_month[12])
    }

    /// The last day of year in the previous month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    fn last_day_of_previous_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        // Get the last day of the previous month.
        // Since `month` is 1-indexed, this needs to subtract *two* to get to the right index of the array
        if month < 2 {
            0
        } else {
            self.last_day_of_month
                .get(usize::from(month - 2))
                .copied()
                .unwrap_or(0)
        }
    }

    /// The last day of year in the current month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    fn last_day_of_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        // Get the last day of the previous month.
        // Since `month` is 1-indexed, this needs to subtract one
        self.last_day_of_month
            .get(usize::from(month - 1))
            .copied()
            .unwrap_or(0)
    }

    fn days_in_month(self, month: u8) -> u8 {
        let ret =
            u8::try_from(self.last_day_of_month(month) - self.last_day_of_previous_month(month));
        debug_assert!(ret.is_ok(), "Month too big!");
        ret.unwrap_or(30)
    }
}

impl<C: ChineseBasedWithDataLoading + CalendarArithmetic<YearInfo = ChineseBasedYearInfo>>
    ChineseBasedDateInner<C>
{
    /// Given a 1-indexed chinese extended year, fetch its data from the cache.
    ///
    /// If the actual year data that was fetched is for a different year, update the getter year
    fn get_precomputed_data_for_year_helper(
        cal: &C,
        date: RataDie,
        getter_year: &mut i32,
    ) -> ChineseBasedYearInfo {
        let data = cal.get_precomputed_data();
        let year_info = data.load_or_compute_info(*getter_year);
        if date < year_info.new_year {
            *getter_year -= 1;
            data.load_or_compute_info(*getter_year)
        } else if date >= year_info.next_new_year() {
            *getter_year += 1;
            data.load_or_compute_info(*getter_year)
        } else {
            year_info
        }
    }

    /// Get a ChineseBasedDateInner from a fixed date and the cache/extended year associated with it
    fn chinese_based_date_from_info(
        date: RataDie,
        year_info: ChineseBasedYearInfo,
        extended_year: i32,
    ) -> ChineseBasedDateInner<C> {
        debug_assert!(
            date < year_info.next_new_year(),
            "Stored date {date:?} out of bounds!"
        );
        // 1-indexed day of year
        let day_of_year = u16::try_from(date - year_info.new_year + 1);
        debug_assert!(day_of_year.is_ok(), "Somehow got a very large year in data");
        let day_of_year = day_of_year.unwrap_or(1);
        let mut month = 1;
        // TODO(#3933) perhaps use a binary search
        for iter_month in 1..=13 {
            month = iter_month;
            if year_info.last_day_of_month(iter_month) >= day_of_year {
                break;
            }
        }

        debug_assert!((1..=13).contains(&month), "Month out of bounds!");

        debug_assert!(
            month < 13 || year_info.leap_month.is_some(),
            "Cannot have 13 months in a non-leap year!"
        );
        let day_before_month_start = year_info.last_day_of_previous_month(month);
        let day_of_month = day_of_year - day_before_month_start;
        let day_of_month = u8::try_from(day_of_month);
        debug_assert!(day_of_month.is_ok(), "Month too big!");
        let day_of_month = day_of_month.unwrap_or(1);

        // This can use `new_unchecked` because this function is only ever called from functions which
        // generate the year, month, and day; therefore, there should never be a situation where
        // creating this ArithmeticDate would fail, since the same algorithms used to generate the ymd
        // are also used to check for valid ymd.
        ChineseBasedDateInner(ArithmeticDate::new_unchecked_with_info(
            extended_year,
            month,
            day_of_month,
            year_info,
        ))
    }

    /// Get a ChineseBasedDateInner from a fixed date, with the related ISO year
    pub(crate) fn chinese_based_date_from_fixed(
        cal: &C,
        date: RataDie,
        iso_year: i32,
    ) -> ChineseBasedDateInner<C> {
        // Get the 1-indexed Chinese extended year, used for fetching data from the cache
        let epoch_as_iso = Iso::iso_from_fixed(C::CB::EPOCH);
        let mut getter_year = iso_year - epoch_as_iso.year().number + 1;

        let year_info = Self::get_precomputed_data_for_year_helper(cal, date, &mut getter_year);
        Self::chinese_based_date_from_info(date, year_info, getter_year)
    }

    /// Get a RataDie from a ChineseBasedDateInner
    ///
    /// This finds the RataDie of the new year of the year given, then finds the RataDie of the new moon
    /// (beginning of the month) of the month given, then adds the necessary number of days.
    pub(crate) fn fixed_from_chinese_based_date_inner(date: ChineseBasedDateInner<C>) -> RataDie {
        let first_day_of_year = date.0.year_info.new_year;
        let day_of_year = date.day_of_year(); // 1 indexed
        first_day_of_year + i64::from(day_of_year) - 1
    }

    /// Create a new arithmetic date from a year, month ordinal, and day with bounds checking; returns the
    /// result of creating this arithmetic date, as well as a ChineseBasedYearInfo - either the one passed in
    /// optionally as an argument, or a new ChineseBasedYearInfo for the given year, month, and day args.
    pub(crate) fn new_from_ordinals(
        year: i32,
        month: u8,
        day: u8,
        year_info: ChineseBasedYearInfo,
    ) -> Result<ArithmeticDate<C>, CalendarError> {
        let max_month = Self::months_in_year_with_info(year_info);
        if !(1..=max_month).contains(&month) {
            return Err(CalendarError::Overflow {
                field: "month",
                max: max_month as usize,
            });
        }

        let max_day = year_info.days_in_month(month);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        // Unchecked can be used because month and day are already checked in this fn
        Ok(ArithmeticDate::<C>::new_unchecked_with_info(
            year, month, day, year_info,
        ))
    }

    /// Call `months_in_year_with_info` on a `ChineseBasedDateInner`
    pub(crate) fn months_in_year_inner(&self) -> u8 {
        Self::months_in_year_with_info(self.0.year_info)
    }

    /// Return the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    /// Also takes a `ChineseBasedYearInfo` argument.
    fn months_in_year_with_info(year_info: ChineseBasedYearInfo) -> u8 {
        if year_info.leap_month.is_some() {
            13
        } else {
            12
        }
    }

    /// Calls `days_in_month` on an instance of ChineseBasedDateInner
    pub(crate) fn days_in_month_inner(&self) -> u8 {
        self.0.year_info.days_in_month(self.0.month)
    }

    pub(crate) fn fixed_mid_year_from_year(year: i32) -> RataDie {
        chinese_based::fixed_mid_year_from_year::<C::CB>(year)
    }

    /// Calls days_in_year on an instance of ChineseBasedDateInner
    pub(crate) fn days_in_year_inner(&self) -> u16 {
        let next_new_year = self.0.year_info.next_new_year();
        let new_year = self.0.year_info.new_year;
        YearBounds {
            new_year,
            next_new_year,
        }
        .count_days()
    }

    /// Calculate the number of days in the year so far for a ChineseBasedDate;
    /// similar to `CalendarArithmetic::day_of_year`
    pub(crate) fn day_of_year(&self) -> u16 {
        self.0.year_info.last_day_of_previous_month(self.0.month)
    }
}

// TODO(#3933): pass around YearInfo in CalendarArithmetic (oops)
impl<C: ChineseBasedWithDataLoading> CalendarArithmetic for C {
    type YearInfo = ChineseBasedYearInfo;

    fn month_days(year: i32, month: u8, year_info: ChineseBasedYearInfo) -> u8 {
        chinese_based::month_days::<C::CB>(year, month)
    }

    /// Returns the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    fn months_for_every_year(year: i32, year_info: ChineseBasedYearInfo) -> u8 {
        if Self::is_leap_year(year, year_info) {
            13
        } else {
            12
        }
    }

    /// Returns true if the given year is a leap year, and false if not.
    fn is_leap_year(year: i32, year_info: ChineseBasedYearInfo) -> bool {
        chinese_based::is_leap_year::<C::CB>(year)
    }

    /// Returns the (month, day) of the last day in a Chinese year (the day before Chinese New Year).
    /// The last month in a year will always be 12 in a common year or 13 in a leap year. The day is
    /// determined by finding the day immediately before the next new year and calculating the number
    /// of days since the last new moon (beginning of the last month in the year).
    fn last_month_day_in_year(year: i32, year_info: ChineseBasedYearInfo) -> (u8, u8) {
        chinese_based::last_month_day_in_year::<C::CB>(year)
    }

    fn days_in_provided_year(year: i32, year_info: ChineseBasedYearInfo) -> u16 {
        chinese_based::days_in_provided_year::<C::CB>(year)
    }
}

/// Get the ordinal lunar month from a code for chinese-based calendars.
pub(crate) fn chinese_based_ordinal_lunar_month_from_code(
    code: MonthCode,
    year_info: ChineseBasedYearInfo,
) -> Option<u8> {
    let leap_month = if let Some(leap) = year_info.leap_month {
        leap.get()
    } else {
        // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
        // it is impossible to actually have 14 months in a year.
        14
    };

    if code.0.len() < 3 {
        return None;
    }
    let bytes = code.0.all_bytes();
    if bytes[0] != b'M' {
        return None;
    }
    if code.0.len() == 4 && bytes[3] != b'L' {
        return None;
    }
    let mut unadjusted = 0;
    if bytes[1] == b'0' {
        if bytes[2] >= b'1' && bytes[2] <= b'9' {
            unadjusted = bytes[2] - b'0';
        }
    } else if bytes[1] == b'1' && bytes[2] >= b'0' && bytes[2] <= b'2' {
        unadjusted = 10 + bytes[2] - b'0';
    }
    if bytes[3] == b'L' {
        if unadjusted + 1 != leap_month {
            return None;
        } else {
            return Some(unadjusted + 1);
        }
    }
    if unadjusted != 0 {
        if unadjusted + 1 > leap_month {
            return Some(unadjusted + 1);
        } else {
            return Some(unadjusted);
        }
    }
    None
}
