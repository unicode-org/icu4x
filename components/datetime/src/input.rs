// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::week::{RelativeUnit, WeekCalculator};
use icu_calendar::Calendar;
use icu_calendar::{AsCalendar, Date, DateTime, Iso};
use icu_timezone::{CustomTimeZone, GmtOffset, ZoneVariant};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{
    DayOfMonth, DayOfWeekInMonth, DayOfYearInfo, FormattableMonth, FormattableYear, IsoHour,
    IsoMinute, IsoSecond, IsoWeekday, NanoSecond, Time, WeekOfMonth, WeekOfYear,
};
pub(crate) use icu_calendar::CalendarError;

/// Representation of a formattable calendar date. Supports dates in any calendar system that uses
/// solar days indexed by an era, year, month, and day.
///
/// All fields are optional. If a field is not present but is required when formatting, an error
/// result will be returned from the formatter.
///
/// All data represented in [`DateInput`] should be locale-agnostic.
pub trait DateInput {
    /// The calendar this date relates to
    type Calendar: Calendar;
    /// Gets the era and year input.
    fn year(&self) -> Option<FormattableYear>;

    /// Gets the month input.
    fn month(&self) -> Option<FormattableMonth>;

    /// Gets the day input.
    fn day_of_month(&self) -> Option<DayOfMonth>;

    /// Gets the weekday input.
    fn iso_weekday(&self) -> Option<IsoWeekday>;

    /// Gets information on the position of the day within the year.
    fn day_of_year_info(&self) -> Option<DayOfYearInfo>;

    /// Gets the kind of calendar this date is for, if associated with AnyCalendar
    /// In most cases you'll probably want to return AnyCalendarKind::Iso
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind>;

    /// Converts date to ISO
    fn to_iso(&self) -> Date<Iso>;
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

    /// Gets the nanosecond input.
    fn nanosecond(&self) -> Option<NanoSecond>;
}

/// Representation of a formattable time zone.
///
/// Only the [`GmtOffset`] is required, since it is the final format fallback.
///
/// All data represented in [`TimeZoneInput`] should be locale-agnostic.
pub trait TimeZoneInput {
    /// The GMT offset in Nanoseconds.
    fn gmt_offset(&self) -> Option<GmtOffset>;

    /// The IANA time-zone identifier.
    fn time_zone_id(&self) -> Option<TimeZoneBcp47Id>;

    /// The metazone identifier.
    fn metazone_id(&self) -> Option<MetazoneId>;

    /// The time variant (e.g. "daylight", "standard")
    fn zone_variant(&self) -> Option<ZoneVariant>;
}

/// A combination of a formattable calendar date and ISO time.
pub trait DateTimeInput: DateInput + IsoTimeInput {}

impl<T> DateTimeInput for T where T: DateInput + IsoTimeInput {}

/// A formattable calendar date and ISO time that takes the locale into account.
pub trait LocalizedDateTimeInput<T: DateTimeInput> {
    /// A reference to this instance's [`DateTimeInput`].
    fn datetime(&self) -> &T;

    /// The week of the month.
    ///
    /// For example, January 1, 2021 is part of the first week of January.
    fn week_of_month(&self) -> Result<WeekOfMonth, CalendarError>;

    /// The week number of the year and the corresponding year.
    ///
    /// For example, December 31, 2020 is part of the first week of 2021.
    fn week_of_year(&self) -> Result<(FormattableYear, WeekOfYear), CalendarError>;

    /// The day of week in this month.
    ///
    /// For example, July 8, 2020 is the 2nd Wednesday of July.
    fn day_of_week_in_month(&self) -> Result<DayOfWeekInMonth, CalendarError>;

    /// TODO(#487): Implement flexible day periods.
    fn flexible_day_period(&self);
}

pub(crate) struct DateTimeInputWithWeekConfig<'data, T: DateTimeInput> {
    data: &'data T,
    calendar: Option<WeekCalculator>,
}

/// A [`DateTimeInput`] type with all of the fields pre-extracted
///
/// See [`DateTimeInput`] for documentation on individual fields
#[derive(Default, Debug)]
pub(crate) struct ExtractedDateTimeInput {
    year: Option<FormattableYear>,
    month: Option<FormattableMonth>,
    day_of_month: Option<DayOfMonth>,
    iso_weekday: Option<IsoWeekday>,
    day_of_year_info: Option<DayOfYearInfo>,
    any_calendar_kind: Option<AnyCalendarKind>,
    hour: Option<IsoHour>,
    minute: Option<IsoMinute>,
    second: Option<IsoSecond>,
    nanosecond: Option<NanoSecond>,
}

/// A [`TimeZoneInput`] type with all of the fields pre-extracted
///
/// See [`TimeZoneInput`] for documentation on individual fields
#[derive(Debug)]
pub(crate) struct ExtractedTimeZoneInput {
    gmt_offset: Option<GmtOffset>,
    time_zone_id: Option<TimeZoneBcp47Id>,
    metazone_id: Option<MetazoneId>,
    zone_variant: Option<ZoneVariant>,
}

impl ExtractedDateTimeInput {
    /// Construct given an instance of a [`DateTimeInput`].
    pub(crate) fn extract_from<T: DateTimeInput>(input: &T) -> Self {
        Self {
            year: input.year(),
            month: input.month(),
            day_of_month: input.day_of_month(),
            iso_weekday: input.iso_weekday(),
            day_of_year_info: input.day_of_year_info(),
            any_calendar_kind: input.any_calendar_kind(),
            hour: input.hour(),
            minute: input.minute(),
            second: input.second(),
            nanosecond: input.nanosecond(),
        }
    }
    /// Construct given an instance of a [`DateTimeInput`].
    pub(crate) fn extract_from_date<T: DateInput>(input: &T) -> Self {
        Self {
            year: input.year(),
            month: input.month(),
            day_of_month: input.day_of_month(),
            iso_weekday: input.iso_weekday(),
            day_of_year_info: input.day_of_year_info(),
            any_calendar_kind: input.any_calendar_kind(),
            ..Default::default()
        }
    }
    /// Construct given an instance of a [`DateTimeInput`].
    pub(crate) fn extract_from_time<T: IsoTimeInput>(input: &T) -> Self {
        Self {
            hour: input.hour(),
            minute: input.minute(),
            second: input.second(),
            nanosecond: input.nanosecond(),
            ..Default::default()
        }
    }
}

impl ExtractedTimeZoneInput {
    /// Construct given an instance of a [`ZonedDateTimeInput`].
    pub(crate) fn extract_from<T: TimeZoneInput>(input: &T) -> Self {
        Self {
            gmt_offset: input.gmt_offset(),
            time_zone_id: input.time_zone_id(),
            metazone_id: input.metazone_id(),
            zone_variant: input.zone_variant(),
        }
    }
}

impl DateInput for ExtractedDateTimeInput {
    /// This actually doesn't matter, by the time we use this
    /// it's purely internal raw code where calendars are irrelevant
    type Calendar = icu_calendar::any_calendar::AnyCalendar;
    fn year(&self) -> Option<FormattableYear> {
        self.year
    }
    fn month(&self) -> Option<FormattableMonth> {
        self.month
    }
    fn day_of_month(&self) -> Option<DayOfMonth> {
        self.day_of_month
    }
    fn iso_weekday(&self) -> Option<IsoWeekday> {
        self.iso_weekday
    }
    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        self.day_of_year_info
    }
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        self.any_calendar_kind
    }
    fn to_iso(&self) -> Date<Iso> {
        unreachable!("ExtractedDateTimeInput should never be directly passed to DateTimeFormatter")
    }
}

impl IsoTimeInput for ExtractedDateTimeInput {
    fn hour(&self) -> Option<IsoHour> {
        self.hour
    }
    fn minute(&self) -> Option<IsoMinute> {
        self.minute
    }
    fn second(&self) -> Option<IsoSecond> {
        self.second
    }
    fn nanosecond(&self) -> Option<NanoSecond> {
        self.nanosecond
    }
}

impl TimeZoneInput for ExtractedTimeZoneInput {
    fn gmt_offset(&self) -> Option<GmtOffset> {
        self.gmt_offset
    }
    fn time_zone_id(&self) -> Option<TimeZoneBcp47Id> {
        self.time_zone_id
    }
    fn metazone_id(&self) -> Option<MetazoneId> {
        self.metazone_id
    }
    fn zone_variant(&self) -> Option<ZoneVariant> {
        self.zone_variant
    }
}

impl<'data, T: DateTimeInput> DateTimeInputWithWeekConfig<'data, T> {
    pub(crate) fn new(data: &'data T, calendar: Option<WeekCalculator>) -> Self {
        Self { data, calendar }
    }
}

impl<'data, T: DateTimeInput> LocalizedDateTimeInput<T> for DateTimeInputWithWeekConfig<'data, T> {
    fn datetime(&self) -> &T {
        self.data
    }

    fn week_of_month(&self) -> Result<WeekOfMonth, CalendarError> {
        let config = self.calendar.ok_or(CalendarError::MissingCalendar)?;
        let day_of_month = self
            .data
            .day_of_month()
            .ok_or(CalendarError::MissingInput("DateTimeInput::day_of_month"))?;
        let iso_weekday = self
            .data
            .iso_weekday()
            .ok_or(CalendarError::MissingInput("DateTimeInput::iso_weekday"))?;
        Ok(config.week_of_month(day_of_month, iso_weekday))
    }

    fn week_of_year(&self) -> Result<(FormattableYear, WeekOfYear), CalendarError> {
        let config = self.calendar.ok_or(CalendarError::MissingCalendar)?;
        let day_of_year_info = self
            .data
            .day_of_year_info()
            .ok_or(CalendarError::MissingInput(
                "DateTimeInput::day_of_year_info",
            ))?;
        let iso_weekday = self
            .data
            .iso_weekday()
            .ok_or(CalendarError::MissingInput("DateTimeInput::iso_weekday"))?;
        let week_of = config.week_of_year(day_of_year_info, iso_weekday)?;
        let year = match week_of.unit {
            RelativeUnit::Previous => day_of_year_info.prev_year,
            RelativeUnit::Current => self
                .data
                .year()
                .ok_or(CalendarError::MissingInput("DateTimeInput::year"))?,
            RelativeUnit::Next => day_of_year_info.next_year,
        };
        Ok((year, WeekOfYear(week_of.week as u32)))
    }

    fn day_of_week_in_month(&self) -> Result<DayOfWeekInMonth, CalendarError> {
        let day_of_month = self
            .data
            .day_of_month()
            .ok_or(CalendarError::MissingInput("DateTimeInput::day_of_month"))?;
        Ok(day_of_month.into())
    }

    fn flexible_day_period(&self) {
        todo!("#487")
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> DateInput for Date<A> {
    type Calendar = C;
    /// Gets the era and year input.
    fn year(&self) -> Option<FormattableYear> {
        Some(self.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<FormattableMonth> {
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

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        self.calendar().any_calendar_kind()
    }

    fn to_iso(&self) -> Date<Iso> {
        Date::to_iso(self)
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> DateInput for DateTime<A> {
    type Calendar = C;
    /// Gets the era and year input.
    fn year(&self) -> Option<FormattableYear> {
        Some(self.date.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<FormattableMonth> {
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

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        self.date.calendar().any_calendar_kind()
    }
    fn to_iso(&self) -> Date<Iso> {
        Date::to_iso(&self.date)
    }
}

impl<A: AsCalendar> IsoTimeInput for DateTime<A> {
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
    fn nanosecond(&self) -> Option<NanoSecond> {
        Some(self.time.nanosecond)
    }
}

impl TimeZoneInput for CustomTimeZone {
    fn gmt_offset(&self) -> Option<GmtOffset> {
        self.gmt_offset
    }

    fn time_zone_id(&self) -> Option<TimeZoneBcp47Id> {
        self.time_zone_id
    }

    fn metazone_id(&self) -> Option<MetazoneId> {
        self.metazone_id
    }

    fn zone_variant(&self) -> Option<ZoneVariant> {
        self.zone_variant
    }
}

impl IsoTimeInput for Time {
    fn hour(&self) -> Option<IsoHour> {
        Some(self.hour)
    }
    fn minute(&self) -> Option<IsoMinute> {
        Some(self.minute)
    }
    fn second(&self) -> Option<IsoSecond> {
        Some(self.second)
    }
    fn nanosecond(&self) -> Option<NanoSecond> {
        Some(self.nanosecond)
    }
}
