// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date::*;
use std::str::FromStr;
use std::convert::TryInto;
use tinystr::tinystr8;
use crate::arithmetic;

/// Temporary implementation of [`DateTimeInput`],
/// which is used in tests, benchmarks and examples of this component.
///
/// Month and day are zero-based.
///
/// *Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
/// `MockDateTime` as an example of the data necessary for ICU [`DateTimeFormat`] to work, and
/// [we hope to work with the community](https://github.com/unicode-org/icu4x/blob/master/docs/research/date_time.md)
/// to develop core date and time APIs that will work as an input for this component.
///
/// # Examples
///
/// ```
/// use icu_datetime::mock::MockDateTime;
///
/// let dt1 = MockDateTime::try_new(2020, 9, 24, 13, 21, 0)
///     .expect("Failed to construct DateTime.");
///
/// let dt2: MockDateTime = "2020-10-14T13:21:00".parse()
///     .expect("Failed to parse a date time.");
/// ```
/// [`DateTimeFormat`]: super::DateTimeFormat
#[derive(Debug, Default)]
pub struct MockDateTime {
    pub year: i32,
    pub month: u32,
    pub day: DayOfMonth,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
}

impl MockDateTime {
    /// Creates a new `MockDateTime` from a list of already validated date/time parameters.
    pub const fn new(
        year: i32,
        month: u32,
        day: DayOfMonth,
        hour: Hour,
        minute: Minute,
        second: Second,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Constructor for the `MockDateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::mock::MockDateTime;
    ///
    /// let dt = MockDateTime::try_new(2020, 9, 24, 13, 21, 0)
    ///     .expect("Failed to construct a DateTime");
    /// ```
    pub fn try_new(
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            year: year.try_into().map_err(|_| DateTimeError::Overflow {
                field: "Year",
                max: i32::MAX as usize,
            })?,
            month: month as u32,
            day: day.try_into()?,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
        })
    }
}

impl FromStr for MockDateTime {
    type Err = DateTimeError;

    /// Parse a `MockDateTime` from a string.
    ///
    /// This utility is for easily creating dates, not a complete robust solution. The
    /// string must take a specific form of the ISO 8601 format: `YYYY-MM-DDThh:mm:ss`.
    ///
    /// ```
    /// use icu_datetime::mock::MockDateTime;
    ///
    /// let date: MockDateTime = "2020-10-14T13:21:00".parse()
    ///     .expect("Failed to parse a date time.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let year: i32 = input[0..4].parse()?;
        let month: u32 = input[5..7].parse()?;
        let day: DayOfMonth = input[8..10].parse()?;
        let hour: Hour = input[11..13].parse()?;
        let minute: Minute = input[14..16].parse()?;
        let second: Second = input[17..19].parse()?;
        Ok(Self {
            year,
            month: month - 1,
            day: day - 1,
            hour,
            minute,
            second,
        })
    }
}

impl DateInput for MockDateTime {
    fn year(&self) -> Option<Year> {
        Some(arithmetic::iso_year_to_gregorian(self.year))
    }

    fn month(&self) -> Option<Month> {
        Some(Month {
            number: self.month + 1,
            // TODO: Implement month codes
            code: MonthCode(tinystr8!("TODO")),
        })
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(self.day + 1)
    }

    fn day_of_week(&self) -> Option<WeekDay> {
        Some(arithmetic::iso_date_to_weekday(
            self.year,
            self.month as usize,
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
