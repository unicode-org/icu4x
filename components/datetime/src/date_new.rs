// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::arithmetic;
use crate::date::{Day as DayOfMonth, Hour, Minute, MockDateTime, Second, WeekDay};
use icu_locid::Locale;
use tinystr::tinystr8;
use tinystr::TinyStr8;

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

pub trait DateTimeInput: DateInput + TimeInput {}

impl<T> DateTimeInput for T where T: DateInput + TimeInput {}

pub trait LocalizedDateTimeInput<T: DateTimeInput> {
    fn date_time(&self) -> &T;
    fn year_week(&self) -> Year;
    fn week_of_month(&self) -> WeekOfMonth;
    fn week_of_year(&self) -> WeekOfYear;
    fn flexible_day_period(&self) -> ();
}

impl DateInput for MockDateTime {
    fn year(&self) -> Option<Year> {
        Some(arithmetic::iso_year_to_gregorian(self.year))
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
        Some(arithmetic::iso_date_to_weekday(
            self.year,
            usize::from(self.month),
            usize::from(self.day),
        ))
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

pub(crate) struct DateTimeInputWithLocale<'s, T: DateTimeInput> {
    data: &'s T,
    _first_weekday: u8,
    _anchor_weekday: u8,
}

impl<'s, T: DateTimeInput> DateTimeInputWithLocale<'s, T> {
    pub fn new(data: &'s T, _locale: &Locale) -> Self {
        Self {
            data,
            // TODO
            _first_weekday: 1,
            _anchor_weekday: 4,
        }
    }
}

impl<'s, T: DateTimeInput> LocalizedDateTimeInput<T> for DateTimeInputWithLocale<'s, T> {
    fn date_time(&self) -> &T {
        self.data
    }

    fn year_week(&self) -> Year {
        unimplemented!()
    }

    fn week_of_month(&self) -> WeekOfMonth {
        unimplemented!()
    }

    fn week_of_year(&self) -> WeekOfYear {
        unimplemented!()
    }

    fn flexible_day_period(&self) -> () {
        unimplemented!()
    }
}
