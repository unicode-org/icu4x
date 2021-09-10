// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

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

pub(crate) struct DateTimeInputWithLocale<'data, T: DateTimeInput> {
    data: &'data T,
    _first_weekday: u8,
    _anchor_weekday: u8,
}

impl<'data, T: DateTimeInput> DateTimeInputWithLocale<'data, T> {
    pub fn new(data: &'data T, _locale: &Locale) -> Self {
        Self {
            data,
            // TODO(#488): Implement week calculations.
            _first_weekday: 1,
            _anchor_weekday: 4,
        }
    }
}

pub(crate) struct ZonedDateTimeInputWithLocale<'data, T: ZonedDateTimeInput> {
    data: &'data T,
    _first_weekday: u8,
    _anchor_weekday: u8,
}

impl<'data, T: ZonedDateTimeInput> ZonedDateTimeInputWithLocale<'data, T> {
    pub fn new(data: &'data T, _locale: &Locale) -> Self {
        Self {
            data,
            // TODO(#488): Implement week calculations.
            _first_weekday: 1,
            _anchor_weekday: 4,
        }
    }
}

impl<'data, T: DateTimeInput> LocalizedDateTimeInput<T> for DateTimeInputWithLocale<'data, T> {
    fn datetime(&self) -> &T {
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

impl<'data, T: ZonedDateTimeInput> LocalizedDateTimeInput<T>
    for ZonedDateTimeInputWithLocale<'data, T>
{
    fn datetime(&self) -> &T {
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
