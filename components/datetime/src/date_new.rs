// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date::{self, MockDateTime};
use tinystr::tinystr8;
use tinystr::{TinyStr8, TinyStrAuto};

pub struct Era(pub TinyStrAuto);

pub struct Quarter(pub u8);

pub struct MonthCode(pub TinyStr8);

pub struct JulianDay(pub i64);

pub struct DayOfMonth(pub u32);

pub struct Weekday(pub u8);

pub struct WeekOfMonth(pub u32);




pub struct WeekOfYear {
    pub day_of_year: u32,
    pub days_in_year: u32,
    pub prev_year: Year,
    pub next_year: Year,
}

pub struct Year {
    pub era: Era,
    pub number: usize,   // FIXME: i64
    pub related_iso: usize, // FIXME: i64
}

pub struct Month {
    pub number: usize, // FIXME: i64
    pub code: MonthCode,
}

pub enum FractionalSecond {
    Millisecond(u16),
    Microsecond(u32),
    Nanosecond(u32),
}

pub trait NewDateType {
    fn year(&self) -> Option<Year>;
    fn quarter(&self) -> Option<Quarter>;
    fn month(&self) -> Option<Month>;
    fn day_of_month(&self) -> Option<DayOfMonth>;
    fn day_of_week(&self) -> Option<Weekday>;
    fn week_of_year(&self) -> Option<WeekOfYear>;
}

pub trait NewTimeType {
    fn hour(&self) -> Option<u8>;
    fn minute(&self) -> Option<u8>;
    fn second(&self) -> Option<u8>;
    fn fraction(&self) -> Option<FractionalSecond>;
}

pub trait NewDateTimeType : NewDateType + NewTimeType {
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
            length: 365, // TODO
        }
    }
    fn prev_year(&self) -> Year {
        Year {
            era: Era(tinystr8!("era001")),
            number: self.year - 1,
            extended: self.year - 1,
            length: 365, // TODO
        }
    }
    fn next_year(&self) -> Year {
        Year {
            era: Era(tinystr8!("era001")),
            number: self.year + 1,
            extended: self.year + 1,
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
