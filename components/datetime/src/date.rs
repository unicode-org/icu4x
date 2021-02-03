// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::Locale;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use tinystr::TinyStr8;

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

/// Temporary trait used to represent the input data for [`DateTimeFormat`].
///
/// This type represents all data that the formatted needs in order to produced formatted string.
///
/// *Note*: At the moment we support only `gregorian` calendar, and plan to extend support to
/// other calendars in the upcoming releases. See <https://github.com/unicode-org/icu4x/issues/355>
///
/// [`DateTimeFormat`]: super::DateTimeFormat
pub trait DateTimeInput: DateInput + TimeInput {}

impl<T> DateTimeInput for T where T: DateInput + TimeInput {}

pub trait LocalizedDateTimeInput<T: DateTimeInput> {
    fn date_time(&self) -> &T;
    fn year_week(&self) -> Year;
    fn week_of_month(&self) -> WeekOfMonth;
    fn week_of_year(&self) -> WeekOfYear;
    fn flexible_day_period(&self) -> ();
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

#[derive(Debug)]
pub enum DateTimeError {
    Parse(std::num::ParseIntError),
    Overflow { field: &'static str, max: usize },
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{}", err),
            Self::Overflow { field, max } => write!(f, "{} must be between 0-{}", field, max),
        }
    }
}

impl From<std::num::ParseIntError> for DateTimeError {
    fn from(input: std::num::ParseIntError) -> Self {
        Self::Parse(input)
    }
}

/// This macro defines a struct for each type of unit to be used in a DateTime. Each
/// unit is bounded by a range. The traits implemented here will return a Result on
/// whether or not the unit is in range from the given input.
macro_rules! dt_unit {
    ($name:ident, $value:expr) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
        pub struct $name(u8);

        impl $name {
            pub const fn new_unchecked(input: u8) -> Self {
                Self(input)
            }
        }

        impl FromStr for $name {
            type Err = DateTimeError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                let val: u8 = input.parse()?;
                if val > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(val))
                }
            }
        }

        impl TryFrom<u8> for $name {
            type Error = DateTimeError;

            fn try_from(input: u8) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input))
                }
            }
        }

        impl TryFrom<usize> for $name {
            type Error = DateTimeError;

            fn try_from(input: usize) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input as u8))
                }
            }
        }

        impl From<$name> for u8 {
            fn from(input: $name) -> Self {
                input.0
            }
        }

        impl From<$name> for usize {
            fn from(input: $name) -> Self {
                input.0 as Self
            }
        }

        impl Add<u8> for $name {
            type Output = Self;

            fn add(self, other: u8) -> Self {
                Self(self.0 + other)
            }
        }

        impl Sub<u8> for $name {
            type Output = Self;

            fn sub(self, other: u8) -> Self {
                Self(self.0 - other)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct Era(pub TinyStr8);

#[derive(Clone, Debug, PartialEq)]
pub struct Year {
    pub era: Era,
    pub number: i32,
    pub related_iso: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MonthCode(pub TinyStr8);

#[derive(Clone, Debug, PartialEq)]
pub struct Month {
    pub number: u32,
    pub code: MonthCode,
}

// TODO: There shouldn't be a bound on DayOfMonth
dt_unit!(DayOfMonth, 32);

#[derive(Clone, Debug, PartialEq)]
pub struct DayOfYearInfo {
    pub day_of_year: u32,
    pub days_in_year: u32,
    pub prev_year: Year,
    pub next_year: Year,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeekOfMonth(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct WeekOfYear(pub u32);

dt_unit!(WeekDay, 7);

dt_unit!(Hour, 24);

dt_unit!(Minute, 60);

dt_unit!(Second, 60);

#[derive(Clone, Debug, PartialEq)]
pub enum FractionalSecond {
    Millisecond(u16),
    Microsecond(u32),
    Nanosecond(u32),
}

pub mod weekdays {
    // TODO: Change this ISO numbering (Sunday = 7)
    use super::WeekDay;
    pub const SUN: WeekDay = WeekDay(0);
    pub const MON: WeekDay = WeekDay(1);
    pub const TUE: WeekDay = WeekDay(2);
    pub const WED: WeekDay = WeekDay(3);
    pub const THU: WeekDay = WeekDay(4);
    pub const FRI: WeekDay = WeekDay(5);
    pub const SAT: WeekDay = WeekDay(6);
}
