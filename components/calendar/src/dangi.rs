// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: Docs

use crate::calendar_arithmetic::CalendarArithmetic;
use crate::{
    astronomy::Location,
    calendar_arithmetic::ArithmeticDate,
    chinese_based::{ChineseBased, ChineseBasedDateInner},
    helpers::div_rem_euclid,
    rata_die::RataDie,
    types::{self, Era, FormattableYear, MonthCode},
    AnyCalendarKind, Calendar, CalendarError, Date, DateTime, Iso,
};
use tinystr::tinystr;

// The first day in the Korean Dangi calendar (based on the founding of Gojoseon)
const KOREAN_EPOCH: RataDie = RataDie::new(-852065); // Lunar new year 2333 BCE (-2332 ISO)

// TODO: Docs
const UTC_OFFSET_ORIGINAL: f64 = (3809.0 / 450.0) / 24.0;
const UTC_OFFSET_1908: f64 = 8.5 / 24.0;
const UTC_OFFSET_1912: f64 = 9.0 / 24.0;
const UTC_OFFSET_1954: f64 = 8.5 / 24.0;
const UTC_OFFSET_1961: f64 = 9.0 / 24.0;

const FIXED_1908: RataDie = RataDie::new(696608); // Apr 1, 1908
const FIXED_1912: RataDie = RataDie::new(697978); // Jan 1, 1912
const FIXED_1954: RataDie = RataDie::new(713398); // Mar 21, 1954
const FIXED_1961: RataDie = RataDie::new(716097); // Aug 10, 1961

const KOREAN_LATITUDE: f64 = 37.0 + (34.0 / 60.0);
const KOREAN_LONGITUDE: f64 = 126.0 + (58.0 / 60.0);
const KOREAN_ELEVATION: f64 = 0.0;

const KOREAN_LOCATION_ORIGINAL: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_ORIGINAL,
);
const KOREAN_LOCATION_1908: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1908,
);
const KOREAN_LOCATION_1912: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1912,
);
const KOREAN_LOCATION_1954: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1954,
);
const KOREAN_LOCATION_1961: Location = Location::new_unchecked(
    KOREAN_LATITUDE,
    KOREAN_LONGITUDE,
    KOREAN_ELEVATION,
    UTC_OFFSET_1961,
);

/// TODO: Docs
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Dangi;

/// TODO: Docs
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct DangiDateInner(ChineseBasedDateInner<Dangi>);

type Inner = ChineseBasedDateInner<Dangi>;

impl Calendar for Dangi {
    type DateInner = DangiDateInner;

    fn date_from_codes(
        &self,
        era: crate::types::Era,
        year: i32,
        month_code: crate::types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, crate::Error> {
        let month = if let Some(ordinal) = Self::ordinal_lunar_month_from_code(year, month_code) {
            ordinal
        } else {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        };

        if month > Self::months_for_every_year(year) {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        }

        let max_day = Self::month_days(year, month);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        if era.0 != tinystr!(16, "dangi") {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        }

        Ok(ArithmeticDate::new_unchecked(year, month, day))
            .map(ChineseBasedDateInner)
            .map(DangiDateInner)
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        let fixed = Iso::fixed_from_iso(iso.inner);
        Inner::chinese_based_date_from_fixed(fixed).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        let fixed = Inner::fixed_from_chinese_based_date_inner(date.0);
        Iso::iso_from_fixed(fixed)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Self::months_for_every_year(date.0 .0.year)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Self::days_in_provided_year(date.0 .0.year)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::month_days(date.0 .0.year, date.0 .0.month)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: crate::DateDuration<Self>) {
        date.0 .0.offset_date(offset)
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        largest_unit: crate::DateDurationUnit,
        smallest_unit: crate::DateDurationUnit,
    ) -> crate::DateDuration<Self> {
        date1.0 .0.until(date2.0 .0, largest_unit, smallest_unit)
    }

    fn debug_name(&self) -> &'static str {
        "Dangi"
    }

    fn year(&self, date: &Self::DateInner) -> crate::types::FormattableYear {
        Self::format_dangi_year(date.0 .0.year)
    }

    fn month(&self, date: &Self::DateInner) -> crate::types::FormattableMonth {
        let ordinal = date.0 .0.month;
        let leap_month = if Self::is_leap_year(date.0 .0.year) {
            Inner::get_leap_month_in_year(Inner::fixed_mid_year_from_year(date.0 .0.year))
        } else {
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
        let code = types::MonthCode(code_inner);
        types::FormattableMonth {
            ordinal: ordinal as u32,
            code,
        }
    }

    fn day_of_month(&self, date: &Self::DateInner) -> crate::types::DayOfMonth {
        types::DayOfMonth(date.0 .0.day as u32)
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> crate::types::DayOfYearInfo {
        let prev_year = date.0 .0.year.saturating_sub(1);
        let next_year = date.0 .0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0 .0.day_of_year(),
            days_in_year: date.0 .0.days_in_year(),
            prev_year: Self::format_dangi_year(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::format_dangi_year(next_year),
        }
    }

    fn day_of_week(&self, date: &Self::DateInner) -> crate::types::IsoWeekday {
        self.date_to_iso(date).day_of_week()
    }

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(AnyCalendarKind::Dangi)
    }
}

impl Date<Dangi> {
    /// TODO: Docs
    pub fn try_new_dangi_date(year: i32, month: u8, day: u8) -> Result<Date<Dangi>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(ChineseBasedDateInner)
            .map(DangiDateInner)
            .map(|inner| Date::from_raw(inner, Dangi))
    }
}

impl DateTime<Dangi> {
    /// TODO: Docs
    pub fn try_new_dangi_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Dangi>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_dangi_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl ChineseBased for Dangi {
    fn location(fixed: RataDie) -> Location {
        if fixed < FIXED_1908 {
            KOREAN_LOCATION_ORIGINAL
        } else if fixed < FIXED_1912 {
            KOREAN_LOCATION_1908
        } else if fixed < FIXED_1954 {
            KOREAN_LOCATION_1912
        } else if fixed < FIXED_1961 {
            KOREAN_LOCATION_1954
        } else {
            KOREAN_LOCATION_1961
        }
    }

    const EPOCH: RataDie = KOREAN_EPOCH;

    #[allow(clippy::unwrap_used)]
    fn new_chinese_based_date(year: i32, month: u8, day: u8) -> Date<Self> {
        Date::try_new_dangi_date(year, month, day).unwrap()
    }
}

impl Dangi {
    /// TODO: Docs
    pub fn seollal_on_or_before_iso(iso: Date<Iso>) -> Date<Iso> {
        let iso_inner = iso.inner;
        let fixed = Iso::fixed_from_iso(iso_inner);
        let result_fixed = Inner::new_year_on_or_before_fixed_date(fixed);
        Iso::iso_from_fixed(result_fixed)
    }

    /// TODO: Docs
    fn format_dangi_year(year: i32) -> FormattableYear {
        let era = Era(tinystr!(16, "dangi"));
        let number = year;
        let cyclic = Some(div_rem_euclid(number - 1 + 364, 60).1 + 1);
        let mid_year = Inner::fixed_mid_year_from_year(number);
        let iso_formattable_year = Iso::iso_from_fixed(mid_year).year();
        let related_iso = Some(iso_formattable_year.number);
        types::FormattableYear {
            era,
            number,
            cyclic,
            related_iso,
        }
    }

    /// TODO: Docs
    fn ordinal_lunar_month_from_code(year: i32, code: MonthCode) -> Option<u8> {
        if code.0.len() < 3 {
            return None;
        }
        let mid_year = Inner::fixed_mid_year_from_year(year);
        let leap_month = if Self::is_leap_year(year) {
            Inner::get_leap_month_in_year(mid_year)
        } else {
            // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
            // it is impossible to actually have 14 months in a year.
            14
        };
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_seollal() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_year: i32,
            expected_month: u32,
            expected_day: u32,
        }

        let cases = [TestCase {
            iso_year: 2024,
            iso_month: 6,
            iso_day: 6,
            expected_year: 2024,
            expected_month: 2,
            expected_day: 10,
        }];

        for case in cases {
            let iso = Date::try_new_iso_date(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let seollal = Dangi::seollal_on_or_before_iso(iso);
            assert_eq!(
                seollal.year().number,
                case.expected_year,
                "Year check failed for case: {case:?}"
            );
            assert_eq!(
                seollal.month().ordinal,
                case.expected_month,
                "Month check failed for case: {case:?}"
            );
            assert_eq!(
                seollal.day_of_month().0,
                case.expected_day,
                "Day check failed for case: {case:?}"
            );
        }
    }
}
