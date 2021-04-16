// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::arithmetic;
use crate::date::*;
use std::convert::TryInto;
use std::str::FromStr;
use tinystr::tinystr8;

/// A temporary struct that implements [`DateTimeInput`]
/// and is used in tests, benchmarks and examples of this component.
///
/// All fields in [`MockDateTime`] are 0-based. For example, January is represented as 0, not 1.
///
/// *Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
/// [`MockDateTime`] as an example of the data necessary for ICU [`DateTimeFormat`] to work, and
/// [we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md)
/// to develop core date and time APIs that will work as an input for this component.
///
/// # Examples
///
/// ```
/// use icu::datetime::mock::datetime::MockDateTime;
///
/// let dt1 = MockDateTime::try_new(2020, 9, 24, 13, 21, 0)
///     .expect("Failed to construct DateTime.");
///
/// let dt2: MockDateTime = "2020-10-14T13:21:00".parse()
///     .expect("Failed to parse a datetime.");
/// ```
/// [`DateTimeFormat`]: crate::datetime::DateTimeFormat
#[derive(Debug, Default)]
pub struct MockDateTime {
    /// ISO-8601 year (proleptic Gregorian).
    pub year: i32,

    /// 0-based month index.
    pub month: u32,

    /// 0-based day index.
    pub day: u32,

    /// 0-based hour.
    pub hour: IsoHour,

    /// 0-based minute.
    pub minute: IsoMinute,

    /// 0-based second.
    pub second: IsoSecond,
}

impl MockDateTime {
    /// Creates a new [`MockDateTime`] from a list of already validated date/time parameters.
    pub const fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: IsoHour,
        minute: IsoMinute,
        second: IsoSecond,
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

    /// Constructor for [`MockDateTime`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::mock::datetime::MockDateTime;
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
            day: day as u32,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
        })
    }
}

impl FromStr for MockDateTime {
    type Err = DateTimeError;

    /// Parse a [`MockDateTime`] from a string.
    ///
    /// This utility is for easily creating dates, not a complete robust solution. The
    /// string must take a specific form of the ISO-8601 format: `YYYY-MM-DDThh:mm:ss`.
    ///
    /// ```
    /// use icu::datetime::mock::datetime::MockDateTime;
    ///
    /// let date: MockDateTime = "2020-10-14T13:21:00".parse()
    ///     .expect("Failed to parse a datetime.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let year: i32 = input[0..4].parse()?;
        let month: u32 = input[5..7].parse()?;
        let day: u32 = input[8..10].parse()?;
        let hour: IsoHour = input[11..13].parse()?;
        let minute: IsoMinute = input[14..16].parse()?;
        let second: IsoSecond = input[17..19].parse()?;
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
            // TODO(#486): Implement month codes
            code: MonthCode(tinystr8!("TODO")),
        })
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(DayOfMonth(self.day + 1))
    }

    fn iso_weekday(&self) -> Option<IsoWeekday> {
        Some(arithmetic::iso_date_to_weekday(
            self.year,
            self.month as usize,
            self.day as usize,
        ))
    }

    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        unimplemented!()
    }
}

impl IsoTimeInput for MockDateTime {
    fn hour(&self) -> Option<IsoHour> {
        Some(self.hour)
    }

    fn minute(&self) -> Option<IsoMinute> {
        Some(self.minute)
    }

    fn second(&self) -> Option<IsoSecond> {
        Some(self.second)
    }

    fn fraction(&self) -> Option<FractionalSecond> {
        None
    }
}
