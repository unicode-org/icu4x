// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use icu_calendar::{arithmetic::week_of, AsCalendar, Date, DateTime, Gregorian};
use icu_locid::Locale;
use tinystr::TinyStr8;

// TODO (Manishearth) fix up imports to directly import from icu_calendar
pub use icu_calendar::types::*;
pub use icu_calendar::DateTimeError;

/// Representation of a formattable calendar date. Supports dates in any calendar system that uses
/// solar days indexed by an era, year, month, and day.
///
/// All fields are optional. If a field is not present but is required when formatting, an error
/// result will be returned from the formatter.
///
/// All data represented in [`DateInput`] should be locale-agnostic.
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
/// All data represented in [`IsoTimeInput`] should be locale-agnostic.
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

/// Representation of a formattable time zone.
///
/// Only the [`GmtOffset`] is required, since it is the final format fallback.
///
/// All data represented in [`TimeZoneInput`] should be locale-agnostic.
pub trait TimeZoneInput {
    /// The GMT offset in Nanoseconds.
    fn gmt_offset(&self) -> GmtOffset;

    /// The IANA time-zone identifier.
    /// TODO(#606) switch this to BCP-47 identifier.
    fn time_zone_id(&self) -> Option<&str>;

    /// The metazone identifier.
    /// TODO(#528) switch to a compact, stable ID.
    fn metazone_id(&self) -> Option<&str>;

    /// The time variant (e.g. "daylight", "standard")
    /// TODO(#619) use TinyStr for time variants.
    fn time_variant(&self) -> Option<&TinyStr8>;
}

/// A combination of a formattable calendar date and ISO time.
pub trait DateTimeInput: DateInput + IsoTimeInput {}

/// A combination of a formattable calendar date, ISO time, and time zone.
pub trait ZonedDateTimeInput: TimeZoneInput + DateTimeInput {}

impl<T> DateTimeInput for T where T: DateInput + IsoTimeInput {}
impl<T> ZonedDateTimeInput for T where T: TimeZoneInput + DateTimeInput {}

/// A formattable calendar date and ISO time that takes the locale into account.
pub trait LocalizedDateTimeInput<T: DateTimeInput> {
    /// A reference to this instance's [`DateTimeInput`].
    fn datetime(&self) -> &T;

    /// The year number according to week numbering.
    ///
    /// For example, December 31, 2020 is part of the first week of 2021.
    fn year_week(&self) -> Result<Year, DateTimeError>;

    /// The week of the month according to UTS 35.
    fn week_of_month(&self) -> WeekOfMonth;

    /// The week number of the year.
    ///
    /// For example, December 31, 2020 is part of the first week of 2021.
    fn week_of_year(&self) -> Result<WeekOfYear, DateTimeError>;

    /// TODO(#487): Implement flexible day periods.
    fn flexible_day_period(&self);
}

pub(crate) struct DateTimeInputWithLocale<'data, T: DateTimeInput> {
    data: &'data T,
    calendar: week_of::CalendarInfo,
}

fn compute_week_of_year<T: DateInput>(
    datetime: &T,
    calendar: &week_of::CalendarInfo,
) -> Result<(DayOfYearInfo, week_of::WeekOf), DateTimeError> {
    let doy_info = datetime
        .day_of_year_info()
        .ok_or(DateTimeError::MissingInput(
            "DateTimeInput::day_of_year_info",
        ))?;
    let week = week_of::week_of(
        calendar,
        doy_info.days_in_prev_year as u16,
        doy_info.days_in_year as u16,
        doy_info.day_of_year as u16,
        datetime
            .iso_weekday()
            .ok_or(DateTimeError::MissingInput("DateTimeInput::iso_weekday"))?,
    )?;
    Ok((doy_info, week))
}

fn year_week<T: DateInput>(
    datetime: &T,
    calendar: &week_of::CalendarInfo,
) -> Result<Year, DateTimeError> {
    let (doy_info, week) = compute_week_of_year(datetime, calendar)?;
    Ok(match week.unit {
        week_of::RelativeUnit::Previous => doy_info.prev_year,
        week_of::RelativeUnit::Current => datetime
            .year()
            .ok_or(DateTimeError::MissingInput("DateTimeInput::year"))?,
        week_of::RelativeUnit::Next => doy_info.next_year,
    })
}

fn week_of_year<T: DateInput>(
    datetime: &T,
    calendar: &week_of::CalendarInfo,
) -> Result<WeekOfYear, DateTimeError> {
    let (_, week) = compute_week_of_year(datetime, calendar)?;
    Ok(WeekOfYear(u32::from(week.week)))
}

impl<'data, T: DateTimeInput> DateTimeInputWithLocale<'data, T> {
    pub fn new(data: &'data T, _locale: &Locale) -> Self {
        Self {
            data,
            // TODO(#488): Implement week calculations.
            calendar: week_of::CalendarInfo {
                first_weekday: IsoWeekday::Monday,
                min_week_days: 4,
            },
        }
    }
}

pub(crate) struct ZonedDateTimeInputWithLocale<'data, T: ZonedDateTimeInput> {
    data: &'data T,
    calendar: week_of::CalendarInfo,
}

impl<'data, T: ZonedDateTimeInput> ZonedDateTimeInputWithLocale<'data, T> {
    pub fn new(data: &'data T, _locale: &Locale) -> Self {
        Self {
            data,
            // TODO(#488): Implement week calculations.
            calendar: week_of::CalendarInfo {
                first_weekday: IsoWeekday::Monday,
                min_week_days: 4,
            },
        }
    }
}

impl<'data, T: DateTimeInput> LocalizedDateTimeInput<T> for DateTimeInputWithLocale<'data, T> {
    fn datetime(&self) -> &T {
        self.data
    }

    fn year_week(&self) -> Result<Year, DateTimeError> {
        year_week(self.data, &self.calendar)
    }

    fn week_of_month(&self) -> WeekOfMonth {
        todo!("#488")
    }

    fn week_of_year(&self) -> Result<WeekOfYear, DateTimeError> {
        week_of_year(self.data, &self.calendar)
    }

    fn flexible_day_period(&self) {
        todo!("#487")
    }
}

impl<'data, T: ZonedDateTimeInput> LocalizedDateTimeInput<T>
    for ZonedDateTimeInputWithLocale<'data, T>
{
    fn datetime(&self) -> &T {
        self.data
    }

    fn year_week(&self) -> Result<Year, DateTimeError> {
        year_week(self.data, &self.calendar)
    }

    fn week_of_month(&self) -> WeekOfMonth {
        todo!("#488")
    }

    fn week_of_year(&self) -> Result<WeekOfYear, DateTimeError> {
        week_of_year(self.data, &self.calendar)
    }

    fn flexible_day_period(&self) {
        todo!("#487")
    }
}

impl<A: AsCalendar<Calendar = Gregorian>> DateInput for Date<A> {
    /// Gets the era and year input.
    fn year(&self) -> Option<Year> {
        Some(self.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<Month> {
        Some(self.month())
    }

    /// Gets the day input.
    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(self.day_of_month())
    }

    /// Gets the weekday input.
    fn iso_weekday(&self) -> Option<IsoWeekday> {
        Some(self.day_of_week())
    }

    /// Gets information on the position of the day within the year.
    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        Some(self.day_of_year_info())
    }
}

impl<A: AsCalendar<Calendar = Gregorian>> DateInput for DateTime<A> {
    /// Gets the era and year input.
    fn year(&self) -> Option<Year> {
        Some(self.date.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<Month> {
        Some(self.date.month())
    }

    /// Gets the day input.
    fn day_of_month(&self) -> Option<DayOfMonth> {
        Some(self.date.day_of_month())
    }

    /// Gets the weekday input.
    fn iso_weekday(&self) -> Option<IsoWeekday> {
        Some(self.date.day_of_week())
    }

    /// Gets information on the position of the day within the year.
    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        Some(self.date.day_of_year_info())
    }
}

impl<A: AsCalendar<Calendar = Gregorian>> IsoTimeInput for DateTime<A> {
    /// Gets the hour input.
    fn hour(&self) -> Option<IsoHour> {
        Some(self.time.hour)
    }

    /// Gets the minute input.
    fn minute(&self) -> Option<IsoMinute> {
        Some(self.time.minute)
    }

    /// Gets the second input.
    fn second(&self) -> Option<IsoSecond> {
        Some(self.time.second)
    }

    /// Gets the fractional second input.
    fn fraction(&self) -> Option<FractionalSecond> {
        None
    }
}
