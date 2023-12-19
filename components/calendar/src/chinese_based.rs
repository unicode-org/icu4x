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
    types::{FormattableMonth, MonthCode},
    Calendar, CalendarError, Iso,
};

use calendrical_calculations::chinese_based::{self, ChineseBased, YearBounds};
use calendrical_calculations::rata_die::RataDie;
use core::num::NonZeroU8;
use tinystr::tinystr;

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

/// Contains any loaded precomputed data. If constructed with Default, will
/// *not* contain any extra data and will always compute stuff from scratch
#[derive(Default)]
pub(crate) struct ChineseBasedPrecomputedData<CB: ChineseBased> {
    // TODO(#3933)
    // this should have the ability to be empty
    _cb: CB, // this is zero-sized
}

/// The first day of the ISO year on which Chinese New Year may occur
///
/// According to Reingold & Dershowitz, ch 19.6, Chinese New Year occurs on Jan 21 - Feb 21 inclusive.
///
/// Chinese New Year in the year 30 AD is January 20 (30-01-20)
const FIRST_NY: u8 = 20;

fn compute_cache<CB: ChineseBased>(extended_year: i32) -> ChineseBasedYearInfo {
    let mid_year = chinese_based::fixed_mid_year_from_year::<CB>(extended_year);
    let year_bounds = YearBounds::compute::<CB>(mid_year);
    let YearBounds {
        new_year,
        next_new_year,
        ..
    } = year_bounds;
    let (month_lengths, leap_month) =
        chinese_based::month_structure_for_year::<CB>(new_year, next_new_year);

    let related_iso = CB::iso_from_extended(extended_year);
    let iso_ny = calendrical_calculations::iso::fixed_from_iso(related_iso, 1, 1);

    // +1 because `new_year - iso_ny` is zero-indexed, but `FIRST_NY` is 1-indexed
    let ny_offset = new_year - iso_ny - i64::from(FIRST_NY) + 1;
    let ny_offset = if let Ok(ny_offset) = u8::try_from(ny_offset) {
        ny_offset
    } else {
        debug_assert!(false, "Expected small new years offset, got {ny_offset}");
        0
    };
    let days_in_prev_year = chinese_based::days_in_prev_year::<CB>(new_year);

    let packed_data = PackedChineseBasedYearInfo::new(month_lengths, leap_month, ny_offset);

    ChineseBasedYearInfo {
        days_in_prev_year,
        packed_data,
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
/// Bit structure (little endian: note that shifts go in the opposite direction!)
///
/// ```text
/// Bit:             0   1   2   3   4   5   6   7
/// Byte 0:          [  month lengths .............
/// Byte 1:         .. month lengths ] | [ leap month index ..
/// Byte 2:          ] | [   NY offset   ] | unused
/// ```
///
/// Where the New Year Offset is the offset from ISO Jan 21 of that year for Chinese New Year,
/// the month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
///
/// Should not
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PackedChineseBasedYearInfo(pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl PackedChineseBasedYearInfo {
    fn new(month_lengths: [bool; 13], leap_month_idx: Option<NonZeroU8>, ny_offset: u8) -> Self {
        debug_assert!(
            !month_lengths[12] || leap_month_idx.is_some(),
            "Last month length should not be set for non-leap years"
        );
        debug_assert!(ny_offset < 32, "Year offset too big to store");
        debug_assert!(
            leap_month_idx.map(|l| l.get() <= 13).unwrap_or(true),
            "Leap month indices must be 1 <= i <= 13"
        );
        let mut all = 0u32; // last byte unused

        for (month, length_30) in month_lengths.iter().enumerate() {
            #[allow(clippy::indexing_slicing)]
            if *length_30 {
                all |= 1 << month as u32;
            }
        }
        let leap_month_idx = leap_month_idx.map(|x| x.get()).unwrap_or(0);
        all |= (leap_month_idx as u32) << (8 + 5);
        all |= (ny_offset as u32) << (16 + 1);
        let le = all.to_le_bytes();
        Self(le[0], le[1], le[2])
    }

    // Get the new year offset from January 21
    fn ny_offset(self) -> u8 {
        self.2 >> 1
    }

    fn ny_rd(self, related_iso: i32) -> RataDie {
        let ny_offset = self.ny_offset();
        let iso_ny = calendrical_calculations::iso::fixed_from_iso(related_iso, 1, 1);
        // -1 because `iso_ny` is itself in the year, and `FIRST_NY` is 1-indexed
        iso_ny + i64::from(FIRST_NY) + i64::from(ny_offset) - 1
    }

    fn leap_month_idx(self) -> Option<NonZeroU8> {
        let low_bits = self.1 >> 5;
        let high_bits = (self.2 & 0b1) << 3;

        NonZeroU8::new(low_bits + high_bits)
    }

    // Whether a particular month has 30 days (month is 1-indexed)
    #[cfg(test)]
    fn month_has_30_days(self, month: u8) -> bool {
        let months = u16::from_le_bytes([self.0, self.1]);
        months & (1 << (month - 1) as u16) != 0
    }

    // Which day of year is the last day of a month (month is 1-indexed)
    fn last_day_of_month(self, month: u8) -> u16 {
        let months = u16::from_le_bytes([self.0, self.1]);
        // month is 1-indexed, so `29 * month` includes the current month
        let mut prev_month_lengths = 29 * month as u16;
        // month is 1-indexed, so `1 << month` is a mask with all zeroes except
        // for a 1 at the bit index at the next month. Subtracting 1 from it gets us
        // a bitmask for all months up to now
        let long_month_bits = months & ((1 << month as u16) - 1);
        prev_month_lengths += long_month_bits.count_ones().try_into().unwrap_or(0);
        prev_month_lengths
    }

    fn days_in_year(self) -> u16 {
        if self.leap_month_idx().is_some() {
            self.last_day_of_month(13)
        } else {
            self.last_day_of_month(12)
        }
    }
}

/// A data struct used to load and use information for a set of ChineseBasedDates
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// TODO(#3933): potentially make this smaller
pub(crate) struct ChineseBasedYearInfo {
    days_in_prev_year: u16,
    /// Contains:
    /// - length of each month in the year
    /// - whether or not there is a leap month, and which month it is
    /// - the date of Chinese New Year in the related ISO year
    packed_data: PackedChineseBasedYearInfo,
}

impl ChineseBasedYearInfo {
    /// Get the new year R.D. given the extended year that this yearinfo is for    
    pub(crate) fn new_year<CB: ChineseBased>(self, extended_year: i32) -> RataDie {
        self.packed_data.ny_rd(CB::iso_from_extended(extended_year))
    }

    /// Get the next new year R.D. given the extended year that this yearinfo is for
    /// (i.e, this year, not next year)
    fn next_new_year<CB: ChineseBased>(self, extended_year: i32) -> RataDie {
        self.new_year::<CB>(extended_year) + i64::from(self.packed_data.days_in_year())
    }

    /// Get which month is the leap month. This produces the month *number*
    /// that is the leap month (not the ordinal month). In other words, for
    /// a year with an M05L, this will return Some(5). Note that the regular month precedes
    /// the leap month.
    pub(crate) fn leap_month(self) -> Option<NonZeroU8> {
        self.packed_data.leap_month_idx()
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
        // Since `month` is 1-indexed, this needs to check if the month is 1 for the zero case
        if month == 1 {
            0
        } else {
            self.packed_data.last_day_of_month(month - 1)
        }
    }

    fn days_in_year(self) -> u16 {
        self.packed_data.days_in_year()
    }

    fn days_in_prev_year(self) -> u16 {
        self.days_in_prev_year
    }

    /// The last day of year in the current month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    fn last_day_of_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        self.packed_data.last_day_of_month(month)
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
        if date < year_info.new_year::<C::CB>(*getter_year) {
            *getter_year -= 1;
            data.load_or_compute_info(*getter_year)
        // FIXME (manishearth) try collapsing these new year calculations into one
        } else if date >= year_info.next_new_year::<C::CB>(*getter_year) {
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
            date < year_info.next_new_year::<C::CB>(extended_year),
            "Stored date {date:?} out of bounds!"
        );
        // 1-indexed day of year
        let day_of_year = u16::try_from(date - year_info.new_year::<C::CB>(extended_year) + 1);
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
            month < 13 || year_info.leap_month().is_some(),
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

    pub(crate) fn new_year(self) -> RataDie {
        self.0.year_info.new_year::<C::CB>(self.0.year)
    }

    /// Get a RataDie from a ChineseBasedDateInner
    ///
    /// This finds the RataDie of the new year of the year given, then finds the RataDie of the new moon
    /// (beginning of the month) of the month given, then adds the necessary number of days.
    pub(crate) fn fixed_from_chinese_based_date_inner(date: ChineseBasedDateInner<C>) -> RataDie {
        let first_day_of_year = date.new_year();
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
        if year_info.leap_month().is_some() {
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
        self.0.year_info.days_in_year()
    }
    /// Gets the days in the previous year
    pub(crate) fn days_in_prev_year(&self) -> u16 {
        self.0.year_info.days_in_prev_year()
    }

    /// Calculate the number of days in the year so far for a ChineseBasedDate;
    /// similar to `CalendarArithmetic::day_of_year`
    pub(crate) fn day_of_year(&self) -> u16 {
        self.0.year_info.last_day_of_previous_month(self.0.month) + u16::from(self.0.day)
    }

    /// The calendar-specific month code represented by `date`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal months 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    pub(crate) fn month(&self) -> FormattableMonth {
        let ordinal = self.0.month;
        let leap_month_option = self.0.year_info.leap_month();

        // 1 indexed leap month name. This is also the ordinal for the leap month
        // in the year (e.g. in `M01, M01L, M02, ..`, the leap month is for month 1, and it is also
        // ordinally `month 2`, zero-indexed)
        let leap_month = if let Some(leap) = leap_month_option {
            leap.get()
        } else {
            // sentinel value
            14
        };
        let code_inner = if leap_month == ordinal {
            // Month cannot be 1 because a year cannot have a leap month before the first actual month,
            // and the maximum num of months ina leap year is 13.
            debug_assert!((2..=13).contains(&ordinal));
            match ordinal {
                2 => tinystr!(4, "M01L"),
                3 => tinystr!(4, "M02L"),
                4 => tinystr!(4, "M03L"),
                5 => tinystr!(4, "M04L"),
                6 => tinystr!(4, "M05L"),
                7 => tinystr!(4, "M06L"),
                8 => tinystr!(4, "M07L"),
                9 => tinystr!(4, "M08L"),
                10 => tinystr!(4, "M09L"),
                11 => tinystr!(4, "M10L"),
                12 => tinystr!(4, "M11L"),
                13 => tinystr!(4, "M12L"),
                _ => tinystr!(4, "und"),
            }
        } else {
            let mut adjusted_ordinal = ordinal;
            if ordinal > leap_month {
                // Before adjusting for leap month, if ordinal > leap_month,
                // the month cannot be 1 because this implies the leap month is < 1, which is impossible;
                // cannot be 2 because that implies the leap month is = 1, which is impossible,
                // and cannot be more than 13 because max number of months in a year is 13.
                debug_assert!((2..=13).contains(&ordinal));
                adjusted_ordinal -= 1;
            }
            debug_assert!((1..=12).contains(&adjusted_ordinal));
            match adjusted_ordinal {
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
                _ => tinystr!(4, "und"),
            }
        };
        let code = MonthCode(code_inner);
        FormattableMonth {
            ordinal: ordinal as u32,
            code,
        }
    }
}

impl<C: ChineseBasedWithDataLoading> CalendarArithmetic for C {
    type YearInfo = ChineseBasedYearInfo;

    fn month_days(_year: i32, month: u8, year_info: ChineseBasedYearInfo) -> u8 {
        year_info.days_in_month(month)
    }

    /// Returns the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    fn months_for_every_year(_year: i32, year_info: ChineseBasedYearInfo) -> u8 {
        if year_info.leap_month().is_some() {
            13
        } else {
            12
        }
    }

    /// Returns true if the given year is a leap year, and false if not.
    fn is_leap_year(_year: i32, year_info: ChineseBasedYearInfo) -> bool {
        year_info.leap_month().is_some()
    }

    /// Returns the (month, day) of the last day in a Chinese year (the day before Chinese New Year).
    /// The last month in a year will always be 12 in a common year or 13 in a leap year. The day is
    /// determined by finding the day immediately before the next new year and calculating the number
    /// of days since the last new moon (beginning of the last month in the year).
    fn last_month_day_in_year(_year: i32, year_info: ChineseBasedYearInfo) -> (u8, u8) {
        if year_info.leap_month().is_some() {
            (13, year_info.days_in_month(13))
        } else {
            (12, year_info.days_in_month(12))
        }
    }

    fn days_in_provided_year(_year: i32, year_info: ChineseBasedYearInfo) -> u16 {
        year_info.last_day_of_month(13)
    }
}

/// Get the ordinal lunar month from a code for chinese-based calendars.
pub(crate) fn chinese_based_ordinal_lunar_month_from_code(
    code: MonthCode,
    year_info: ChineseBasedYearInfo,
) -> Option<u8> {
    let leap_month = if let Some(leap) = year_info.leap_month() {
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
    // Unadjusted is zero-indexed month index, must add one to it to use
    let mut unadjusted = 0;
    if bytes[1] == b'0' {
        if bytes[2] >= b'1' && bytes[2] <= b'9' {
            unadjusted = bytes[2] - b'0';
        }
    } else if bytes[1] == b'1' && bytes[2] >= b'0' && bytes[2] <= b'2' {
        unadjusted = 10 + bytes[2] - b'0';
    }
    if bytes[3] == b'L' {
        // Asked for a leap month that doesn't exist
        if unadjusted + 1 != leap_month {
            return None;
        } else {
            // The leap month occurs after the regular month of the same name
            return Some(unadjusted + 1);
        }
    }
    if unadjusted != 0 {
        // If the month has an index greater than that of the leap month,
        // bump it up by one
        if unadjusted + 1 > leap_month {
            return Some(unadjusted + 1);
        } else {
            return Some(unadjusted);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn packed_roundtrip_single(
        mut month_lengths: [bool; 13],
        leap_month_idx: Option<NonZeroU8>,
        ny_offset: u8,
    ) {
        if leap_month_idx.is_none() {
            // Avoid bad invariants
            month_lengths[12] = false;
        }
        let packed = PackedChineseBasedYearInfo::new(month_lengths, leap_month_idx, ny_offset);

        assert_eq!(
            ny_offset,
            packed.ny_offset(),
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
        assert_eq!(
            leap_month_idx,
            packed.leap_month_idx(),
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
        let mut month_lengths_roundtrip = [false; 13];
        for (i, len) in month_lengths_roundtrip.iter_mut().enumerate() {
            *len = packed.month_has_30_days(i as u8 + 1);
        }
        assert_eq!(
            month_lengths, month_lengths_roundtrip,
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
    }

    #[test]
    fn test_roundtrip_packed() {
        const SHORT: [bool; 13] = [false; 13];
        const LONG: [bool; 13] = [true; 13];
        const ALTERNATING1: [bool; 13] = [
            false, true, false, true, false, true, false, true, false, true, false, true, false,
        ];
        const ALTERNATING2: [bool; 13] = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
        ];
        const RANDOM1: [bool; 13] = [
            true, true, false, false, true, true, false, true, true, true, true, false, true,
        ];
        const RANDOM2: [bool; 13] = [
            false, true, true, true, true, false, true, true, true, false, false, true, false,
        ];
        packed_roundtrip_single(SHORT, None, 5);
        packed_roundtrip_single(SHORT, None, 10);
        packed_roundtrip_single(SHORT, NonZeroU8::new(11), 15);
        packed_roundtrip_single(LONG, NonZeroU8::new(12), 15);
        packed_roundtrip_single(ALTERNATING1, None, 2);
        packed_roundtrip_single(ALTERNATING1, NonZeroU8::new(3), 5);
        packed_roundtrip_single(ALTERNATING2, None, 9);
        packed_roundtrip_single(ALTERNATING2, NonZeroU8::new(7), 26);
        packed_roundtrip_single(RANDOM1, None, 29);
        packed_roundtrip_single(RANDOM1, NonZeroU8::new(12), 29);
        packed_roundtrip_single(RANDOM1, NonZeroU8::new(2), 21);
        packed_roundtrip_single(RANDOM2, None, 25);
        packed_roundtrip_single(RANDOM2, NonZeroU8::new(2), 19);
        packed_roundtrip_single(RANDOM2, NonZeroU8::new(5), 2);
        packed_roundtrip_single(RANDOM2, NonZeroU8::new(12), 5);
    }
}
