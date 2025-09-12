// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and traits for use in the Chinese traditional lunar calendar,
//! as well as in related and derived calendars such as the Korean and Vietnamese lunar calendars.

use crate::{
    error::DateError,
    provider::chinese_based::PackedChineseBasedYearInfo,
    types::{MonthCode, MonthInfo},
};

use calendrical_calculations::chinese_based::{
    self, ChineseBased, YearBounds, WELL_BEHAVED_ASTRONOMICAL_RANGE,
};
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// A data struct used to load and use information for a set of ChineseBasedDates
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// TODO(#3933): potentially make this smaller
pub(crate) struct ChineseBasedYearInfo {
    /// Contains:
    /// - length of each month in the year
    /// - whether or not there is a leap month, and which month it is
    /// - the date of Chinese New Year in the related ISO year
    pub(crate) packed_data: PackedChineseBasedYearInfo,
    pub(crate) related_iso: i32,
}

impl From<ChineseBasedYearInfo> for i32 {
    fn from(value: ChineseBasedYearInfo) -> Self {
        value.related_iso
    }
}

/// Compute ChineseBasedYearInfo for a given extended year
pub(crate) fn compute<CB: ChineseBased>(related_iso: i32) -> (RataDie, Option<u8>, [bool; 13]) {
    let mid_year = calendrical_calculations::iso::fixed_from_iso(related_iso, 7, 1);
    let year_bounds = YearBounds::compute::<CB>(mid_year);

    let YearBounds {
        new_year,
        next_new_year,
        ..
    } = year_bounds;
    let (month_lengths, leap_month) =
        chinese_based::month_structure_for_year::<CB>(new_year, next_new_year);

    (new_year, leap_month, month_lengths)
}

impl ChineseBasedYearInfo {
    /// Get the new year R.D.    
    pub(crate) fn new_year(self) -> RataDie {
        self.packed_data.new_year(self.related_iso)
    }

    /// Get the next new year R.D.
    pub(crate) fn next_new_year(self) -> RataDie {
        self.new_year() + i64::from(self.days_in_year())
    }

    /// Get which month is the leap month. This produces the month *number*
    /// that is the leap month (not the ordinal month). In other words, for
    /// a year with an M05L, this will return Some(5). Note that the regular month precedes
    /// the leap month.
    pub(crate) fn leap_month(self) -> Option<u8> {
        self.packed_data.leap_month()
    }

    /// The last day of year in the previous month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    pub(crate) fn last_day_of_previous_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        // Get the last day of the previous month.
        // Since `month` is 1-indexed, this needs to check if the month is 1 for the zero case
        if month == 1 {
            0
        } else {
            self.packed_data.last_day_of_month(month - 1)
        }
    }

    pub(crate) fn days_in_year(self) -> u16 {
        self.last_day_of_month(self.months_in_year())
    }

    /// Return the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    pub(crate) fn months_in_year(self) -> u8 {
        if self.leap_month().is_some() {
            13
        } else {
            12
        }
    }

    /// The last day of year in the current month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    pub(crate) fn last_day_of_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        self.packed_data.last_day_of_month(month)
    }

    pub(crate) fn days_in_month(self, month: u8) -> u8 {
        if self.packed_data.month_has_30_days(month) {
            30
        } else {
            29
        }
    }

    pub(crate) fn md_from_rd(self, rd: RataDie) -> (u8, u8) {
        debug_assert!(
            rd < self.next_new_year() || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Stored date {rd:?} out of bounds!"
        );
        // 1-indexed day of year
        let day_of_year = u16::try_from(rd - self.new_year() + 1);
        debug_assert!(
            day_of_year.is_ok() || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Somehow got a very large year in data"
        );
        let day_of_year = day_of_year.unwrap_or(1);
        let mut month = 1;
        // TODO(#3933) perhaps use a binary search
        for iter_month in 1..=13 {
            month = iter_month;
            if self.last_day_of_month(iter_month) >= day_of_year {
                break;
            }
        }

        debug_assert!((1..=13).contains(&month), "Month out of bounds!");

        debug_assert!(
            month < 13
                || self.leap_month().is_some()
                || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Cannot have 13 months in a non-leap year!"
        );
        let day_before_month_start = self.last_day_of_previous_month(month);
        let day_of_month = day_of_year - day_before_month_start;
        let day_of_month = u8::try_from(day_of_month);
        debug_assert!(day_of_month.is_ok(), "Month too big!");
        let day_of_month = day_of_month.unwrap_or(1);

        (month, day_of_month)
    }

    pub(crate) fn rd_from_md(self, month: u8, day: u8) -> RataDie {
        self.new_year() + self.day_of_year(month, day) as i64 - 1
    }

    /// Calculate the number of days in the year so far for a ChineseBasedDate;
    /// similar to `CalendarArithmetic::day_of_year`
    pub(crate) fn day_of_year(self, month: u8, day: u8) -> u16 {
        self.last_day_of_previous_month(month) + day as u16
    }

    /// The calendar-specific month code represented by `month`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal months 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    pub(crate) fn month(self, month: u8) -> MonthInfo {
        // 1 indexed leap month name. This is also the ordinal for the leap month
        // in the year (e.g. in `M01, M01L, M02, ..`, the leap month is for month 1, and it is also
        // ordinally `month 2`, zero-indexed)
        // 14 is a sentinel value
        let leap_month = self.leap_month().unwrap_or(14);
        let code_inner = if leap_month == month {
            // Month cannot be 1 because a year cannot have a leap month before the first actual month,
            // and the maximum num of months ina leap year is 13.
            debug_assert!((2..=13).contains(&month));
            match month {
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
            let mut adjusted_ordinal = month;
            if month > leap_month {
                // Before adjusting for leap month, if ordinal > leap_month,
                // the month cannot be 1 because this implies the leap month is < 1, which is impossible;
                // cannot be 2 because that implies the leap month is = 1, which is impossible,
                // and cannot be more than 13 because max number of months in a year is 13.
                debug_assert!((2..=13).contains(&month));
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
        MonthInfo {
            ordinal: month,
            standard_code: code,
            formatting_code: code,
        }
    }

    /// Create a new arithmetic date from a year, month ordinal, and day with bounds checking; returns the
    /// result of creating this arithmetic date, as well as a ChineseBasedYearInfo - either the one passed in
    /// optionally as an argument, or a new ChineseBasedYearInfo for the given year, month, and day args.
    pub(crate) fn validate_md(self, month: u8, day: u8) -> Result<(), DateError> {
        let max_month = self.months_in_year();
        if month == 0 || !(1..=max_month).contains(&month) {
            return Err(DateError::Range {
                field: "month",
                value: month as i32,
                min: 1,
                max: max_month as i32,
            });
        }

        let max_day = self.days_in_month(month);
        if day == 0 || day > max_day {
            return Err(DateError::Range {
                field: "day",
                value: day as i32,
                min: 1,
                max: max_day as i32,
            });
        }
        Ok(())
    }

    /// Get the ordinal lunar month from a code for chinese-based calendars.
    pub(crate) fn parse_month_code(self, code: MonthCode) -> Option<u8> {
        // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
        // it is impossible to actually have 14 months in a year.
        let leap_month = self.leap_month().unwrap_or(14);

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
}
