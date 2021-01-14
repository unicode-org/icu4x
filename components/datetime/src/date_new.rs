// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date::{self, MockDateTime};
use tinystr::tinystr8;
use tinystr::TinyStr8;

pub struct Era(pub TinyStr8);

pub struct CyclicYear(pub TinyStr8);

pub struct Quarter(pub u8);

pub struct MonthCode(pub TinyStr8);

pub struct JulianDay(pub i64);

pub struct DayOfMonth(pub u32);

pub struct Weekday(pub u8);

pub struct DayOfYear(pub u32);

pub struct WeekOfMonth(pub u32);

pub struct WeekOfYear(pub u32);





pub struct Year {
    pub era: Era,
    pub number: usize,   // FIXME: i64
    pub extended: usize, // FIXME: i64
    pub cyclic: CyclicYear,
    pub length: usize, // FIXME: u32
}

pub struct Month {
    pub number: usize, // FIXME: i64
    pub code: MonthCode,
}

pub enum FractionalSecond {
    Whole,
    Millisecond(u16),
    Microsecond(u32),
    Nanosecond(u32),
}

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub fractional: FractionalSecond,
}

pub trait NewDateTimeType {
    fn year(&self) -> Year;
    fn prev_year(&self) -> Year;
    fn next_year(&self) -> Year;
    fn quarter(&self) -> Quarter;
    fn month(&self) -> Month;
    fn day_of_year(&self) -> DayOfYear;
    fn day_of_month(&self) -> DayOfMonth;
    fn weekday(&self) -> Weekday;
    fn time(&self) -> Time;
}

pub struct DateTimeLocalizer<'s, T: NewDateTimeType> {
    pub data: &'s T,
}

pub trait FullDateTime: NewDateTimeType {
    fn year_week(&self) -> Year;
    fn week_of_month(&self) -> WeekOfMonth;
    fn week_of_year(&self) -> WeekOfYear;
    fn flexible_day_period(&self) -> ();
}

fn month_day_of_year(month: date::Month) -> u32 {
    // FIXME: This doesn't do leap years!!!
    match u8::from(month) {
        1 => 0,
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 253,
        10 => 283,
        11 => 314,
        12 => 334,
        _ => unreachable!(),
    }
}

impl NewDateTimeType for MockDateTime {
    fn year(&self) -> Year {
        Year {
            era: Era(tinystr8!("era001")),
            number: self.year,
            extended: self.year,
            cyclic: CyclicYear(tinystr8!("TODO")),
            length: 365, // TODO
        }
    }
    fn prev_year(&self) -> Year {
        Year {
            era: Era(tinystr8!("era001")),
            number: self.year - 1,
            extended: self.year - 1,
            cyclic: CyclicYear(tinystr8!("TODO")),
            length: 365, // TODO
        }
    }
    fn next_year(&self) -> Year {
        Year {
            era: Era(tinystr8!("era001")),
            number: self.year + 1,
            extended: self.year + 1,
            cyclic: CyclicYear(tinystr8!("TODO")),
            length: 365, // TODO
        }
    }
    fn quarter(&self) -> Quarter {
        match u8::from(self.month) {
            1 | 2 | 3 => Quarter(1),
            4 | 5 | 6 => Quarter(2),
            7 | 8 | 9 => Quarter(3),
            10 | 11 | 12 => Quarter(4),
            _ => unreachable!(),
        }
    }
    fn month(&self) -> Month {
        Month {
            number: u8::from(self.month) as usize,
            code: MonthCode(tinystr8!("TODO")),
        }
    }
    fn day_of_year(&self) -> DayOfYear {
        DayOfYear(month_day_of_year(self.month) + u8::from(self.day) as u32)
    }
    fn day_of_month(&self) -> DayOfMonth {
        DayOfMonth(u8::from(self.day) as u32)
    }
    fn weekday(&self) -> Weekday {
        unimplemented!()
    }
    fn time(&self) -> Time {
        Time {
            hour: u8::from(self.hour),
            minute: u8::from(self.minute),
            second: u8::from(self.second),
            fractional: FractionalSecond::Whole,
        }
    }
}
