// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use tinystr::TinyStr8;

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

/// Representation of a formattable calendar date. Supports dates in any calendar system that uses
/// solar days indexed by an era, year, month, and day.
///
/// All fields are optional. If a field is not present but is required when formatting, an error
/// result will be returned from the formatter.
///
/// All data represented in DateInput should be locale-agnostic.
pub trait DateInput {
    /// Gets the era and year input.
    fn year(&self) -> Option<Year>;

    /// Gets the month input.
    fn month(&self) -> Option<Month>;

    /// Gets the day input.
    fn day_of_month(&self) -> Option<DayOfMonth>;

    /// Gets the weekday input.
    fn iso_weekday(&self) -> Option<IsoWeekday>;

    /// Gets information on the position of the day within the year.
    fn day_of_year_info(&self) -> Option<DayOfYearInfo>;
}

/// Representation of a time of day according to ISO-8601 conventions. Always indexed from
/// midnight, regardless of calendar system.
///
/// All fields are optional. If a field is not present but is required when formatting, an error
/// result will be returned from the formatter.
///
/// All data represented in IsoTimeInput should be locale-agnostic.
pub trait IsoTimeInput {
    /// Gets the hour input.
    fn hour(&self) -> Option<IsoHour>;

    /// Gets the minute input.
    fn minute(&self) -> Option<IsoMinute>;

    /// Gets the second input.
    fn second(&self) -> Option<IsoSecond>;

    /// Gets the fractional second input.
    fn fraction(&self) -> Option<FractionalSecond>;
}

/// A combination of a formattable calendar date and ISO time.
pub trait DateTimeInput: DateInput + IsoTimeInput {}

impl<T> DateTimeInput for T where T: DateInput + IsoTimeInput {}

/// A formattable calendar date and ISO time that takes the locale into account.
pub trait LocalizedDateTimeInput<T: DateTimeInput> {
    /// A reference to this instance's DateTimeInput.
    fn date_time(&self) -> &T;

    /// The year number according to week numbering.
    ///
    /// For example, December 31, 2020 is part of the first week of 2021.
    fn year_week(&self) -> Year;

    /// The week of the month according to UTS 35.
    fn week_of_month(&self) -> WeekOfMonth;

    /// The week number of the year.
    ///
    /// For example, December 31, 2020 is part of the first week of 2021.
    fn week_of_year(&self) -> WeekOfYear;

    /// TODO(#487): Implement flexible day periods.
    fn flexible_day_period(&self);
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
            // TODO(#488): Implement week calculations.
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
        todo!("#488")
    }

    fn week_of_month(&self) -> WeekOfMonth {
        todo!("#488")
    }

    fn week_of_year(&self) -> WeekOfYear {
        todo!("#488")
    }

    fn flexible_day_period(&self) {
        todo!("#487")
    }
}

/// TODO(#486): Implement era codes.
#[derive(Clone, Debug, PartialEq)]
pub struct Era(pub TinyStr8);

/// Representation of a formattable year.
#[derive(Clone, Debug, PartialEq)]
pub struct Year {
    /// The era containing the year.
    pub era: Era,

    /// Year number in the current era (usually 1-based).
    pub number: i32,

    /// Related ISO year. This is normally the ISO (proleptic Gregorian) year having the greatest
    /// overlap with the calendar year. It is used in certain date formatting patterns.
    pub related_iso: i32,
}

/// TODO(#486): Implement month codes.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthCode(pub TinyStr8);

/// Representation of a formattable month.
#[derive(Clone, Debug, PartialEq)]
pub struct Month {
    /// A month number in a year. In normal years, this is usually the 1-based month index. In leap
    /// years, this is what the month number would have been in a non-leap year.
    ///
    /// For example:
    ///
    /// - January = 1
    /// - December = 12
    /// - Adar, Adar I, and Adar II = 6
    ///
    /// The `code` property is used to distinguish between unique months in leap years.
    pub number: u32,

    /// The month code, used to distinguish months during leap years.
    pub code: MonthCode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DayOfYearInfo {
    pub day_of_year: u32,
    pub days_in_year: u32,
    pub prev_year: Year,
    pub next_year: Year,
}

/// A weekday in a 7-day week, according to ISO-8601.
///
/// The discriminant values correspond to ISO-8601 weekday numbers (Monday = 1, Sunday = 7).
///
/// # Example
///
/// ```
/// use icu_datetime::date::IsoWeekday;
///
/// assert_eq!(1, IsoWeekday::MONDAY as usize);
/// assert_eq!(7, IsoWeekday::SUNDAY as usize);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum IsoWeekday {
    MONDAY = 1,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

impl From<usize> for IsoWeekday {
    /// Convert from an ISO-8601 weekday number to an IsoWeekday enum. 0 is automatically converted
    /// to 7 (Sunday). If the number is out of range, it is interpreted modulo 7.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_datetime::date::IsoWeekday;
    ///
    /// assert_eq!(IsoWeekday::SUNDAY, IsoWeekday::from(0));
    /// assert_eq!(IsoWeekday::MONDAY, IsoWeekday::from(1));
    /// assert_eq!(IsoWeekday::SUNDAY, IsoWeekday::from(7));
    /// assert_eq!(IsoWeekday::MONDAY, IsoWeekday::from(8));
    /// ```
    fn from(input: usize) -> IsoWeekday {
        let mut ordinal = (input % 7) as i8;
        if ordinal == 0 {
            ordinal = 7;
        }
        unsafe { std::mem::transmute(ordinal) }
    }
}

/// A day number in a month. Usually 1-based.
pub struct DayOfMonth(pub u32);

/// A week number in a month. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekOfMonth(pub u32);

/// A week number in a year. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekOfYear(pub u32);

/// This macro defines a struct for 0-based date fields: hours, minutes, and seconds. Each
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

dt_unit!(IsoHour, 24);

dt_unit!(IsoMinute, 60);

dt_unit!(IsoSecond, 61);

// TODO(#485): Improve FractionalSecond.
#[derive(Clone, Debug, PartialEq)]
pub enum FractionalSecond {
    Millisecond(u16),
    Microsecond(u32),
    Nanosecond(u32),
}
