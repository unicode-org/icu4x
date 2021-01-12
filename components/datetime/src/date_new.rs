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

pub struct Year {
    pub start: JulianDay,

    pub era: Era,
    pub number: usize,   // FIXME: i64
    pub extended: usize, // FIXME: i64
    pub cyclic: CyclicYear,
}

pub struct Month {
    pub start: JulianDay,

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
    fn julian_day(&self) -> JulianDay;
    fn year(&self) -> Year;
    fn year_week(&self) -> Year;
    fn quarter(&self) -> Quarter;
    fn month(&self) -> Month;
    fn time(&self) -> Time;
}

fn julian_year(year: usize) -> i64 {
    // FIXME: Implement this correctly!
    // This is a NOT a real implementation!!!
    ((year as i64) - 2000) * 365
}

fn julian_month(month: date::Month) -> i64 {
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
    fn julian_day(&self) -> JulianDay {
        JulianDay(julian_year(self.year) + julian_month(self.month) + u8::from(self.day) as i64)
    }
    fn year(&self) -> Year {
        Year {
            start: JulianDay(julian_year(self.year)),
            era: Era(tinystr8!("era001")),
            number: self.year,
            extended: self.year,
            cyclic: CyclicYear(tinystr8!("TODO")),
        }
    }
    fn year_week(&self) -> Year {
        // FIXME
        self.year()
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
            start: JulianDay(julian_year(self.year) + julian_month(self.month)),
            number: u8::from(self.month) as usize,
            code: MonthCode(tinystr8!("TODO")),
        }
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
