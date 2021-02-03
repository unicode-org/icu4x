// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date::{
    MockDateTime,
    WeekDay,
    Day as DayOfMonth,
    Hour,
    Minute,
    Second,
};
use tinystr::tinystr8;
use tinystr::{TinyStr8};

#[derive(Clone, Debug, PartialEq)]
pub struct Era(pub TinyStr8);

#[derive(Clone, Debug, PartialEq)]
pub struct MonthCode(pub TinyStr8);

#[derive(Clone, Debug, PartialEq)]
pub struct WeekOfMonth(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct WeekOfYear(pub u32);




#[derive(Clone, Debug, PartialEq)]
pub struct DayOfYearInfo {
    pub day_of_year: u32,
    pub days_in_year: u32,
    pub prev_year: Year,
    pub next_year: Year,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Year {
    pub era: Era,
    pub number: i32,
    pub related_iso: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Month {
    pub number: u32,
    pub code: MonthCode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FractionalSecond {
    Millisecond(u16),
    Microsecond(u32),
    Nanosecond(u32),
}

pub trait DateInput {
    fn year(&self) -> Option<Year>;
    fn month(&self) -> Option<Month>;
    fn day_of_month(&self) -> Option<DayOfMonth>;
    fn day_of_week(&self) -> Option<WeekDay>;
    fn day_of_year_info(&self) -> Option<DayOfYearInfo>;
}

pub trait TimeInput {
    fn hour(&self) -> Option<Hour>;
    fn minute(&self) -> Option<Minute>;
    fn second(&self) -> Option<Second>;
    fn fraction(&self) -> Option<FractionalSecond>;
}

pub trait DateTimeInput : DateInput + TimeInput {
}

impl<T> DateTimeInput for T where T: DateInput + TimeInput {}

pub trait LocalizedDateTimeInput: DateTimeInput {
    fn year_week(&self) -> Year;
    fn week_of_month(&self) -> WeekOfMonth;
    fn week_of_year(&self) -> WeekOfYear;
    fn flexible_day_period(&self) -> ();
}






pub(crate) struct DateTimeInputWithLocale<'s, T: DateTimeInput> {
    pub data: &'s T,
}

fn iso_year_to_gregorian(iso_year: i32) -> Year {
    if iso_year > 0 {
        Year {
            era: Era(tinystr8!("ce")),
            number: iso_year,
            related_iso: iso_year,
        }
    } else {
        Year {
            era: Era(tinystr8!("bce")),
            number: -iso_year + 1,
            related_iso: iso_year,
        }
    }
}

#[test]
fn test_iso_year_to_gregorian() {
    assert_eq!(iso_year_to_gregorian(2020), Year {
        era: Era(tinystr8!("ce")),
        number: 2020,
        related_iso: 2020,
    });
    assert_eq!(iso_year_to_gregorian(1), Year {
        era: Era(tinystr8!("ce")),
        number: 1,
        related_iso: 1,
    });
    assert_eq!(iso_year_to_gregorian(0), Year {
        era: Era(tinystr8!("bce")),
        number: 1,
        related_iso: 0,
    });
    assert_eq!(iso_year_to_gregorian(-1), Year {
        era: Era(tinystr8!("bce")),
        number: 2,
        related_iso: -1,
    });
}

impl DateInput for MockDateTime {
    fn year(&self) -> Option<Year> {
        Some(iso_year_to_gregorian(self.year))
    }

    fn month(&self) -> Option<Month> {
        Some(Month {
            number: u8::from(self.month) as u32,
            // TODO: Implement month codes
            code: MonthCode(tinystr8!("TODO")),
        })
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(self.day)
    }

    fn day_of_week(&self) -> Option<WeekDay> {
        unimplemented!()
    }

    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        unimplemented!()
    }
}

impl TimeInput for MockDateTime {
    fn hour(&self) -> Option<Hour> {
        Some(self.hour)
    }

    fn minute(&self) -> Option<Minute> {
        Some(self.minute)
    }

    fn second(&self) -> Option<Second> {
        Some(self.second)
    }

    fn fraction(&self) -> Option<FractionalSecond> {
        None
    }
}
