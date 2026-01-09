// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::scaffold::*;
use icu_calendar::types::{DayOfYear, RataDie};
use icu_calendar::{AsCalendar, Calendar};
use icu_time::scaffold::IntoOption;
use icu_time::zone::ZoneNameTimestamp;
use icu_time::{Hour, Minute, Nanosecond, Second};

use icu_calendar::Date;
use icu_time::{zone::UtcOffset, Time, TimeZone};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{DayOfMonth, MonthInfo, Weekday, YearInfo};

#[cfg(doc)]
use crate::input::*;

/// An input bag with all possible datetime input fields.
///
/// Each input field may or may not be required, depending on the field set
/// and the options.
#[derive(Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub struct DateTimeInputUnchecked {
    /// The year, required for field sets with years (`Y`).
    pub(crate) year: Option<YearInfo>,
    /// The month, required for field sets with months (`M`)
    pub(crate) month: Option<MonthInfo>,
    /// The day-of-month, required for field sets with days (`D`).
    pub(crate) day_of_month: Option<DayOfMonth>,
    /// The weekday, required for field sets with weekdays (`E`).
    pub(crate) weekday: Option<Weekday>,
    /// The day-of-year, required for field sets with weeks.
    pub(crate) day_of_year: Option<DayOfYear>,
    /// The RataDie of the day
    pub(crate) rata_die: Option<RataDie>,
    /// The hour, required for field sets with times (`T`).
    pub(crate) hour: Option<Hour>,
    /// The minute, required for field sets with times (`T`).
    pub(crate) minute: Option<Minute>,
    /// The second, required for field sets with times (`T`).
    pub(crate) second: Option<Second>,
    /// The subsecond, required for field sets with times (`T`).
    pub(crate) subsecond: Option<Nanosecond>,
    /// The time zone ID, required for field sets with
    /// certain time zone styles.
    pub(crate) zone_id: Option<TimeZone>,
    /// The time zone UTC offset, required for field sets with
    /// certain time zone styles.
    pub(crate) zone_offset: Option<UtcOffset>,
    /// The local ISO time, required for field sets with
    /// certain time zone styles.
    pub(crate) zone_name_timestamp: Option<ZoneNameTimestamp>,
}

impl DateTimeInputUnchecked {
    /// Sets all fields from a [`Date`] input.
    ///
    /// This method does not check the calendar of the date! The caller is
    /// responsible for making sure the calendar matches the formatter.
    pub fn set_date_fields_unchecked<C: Calendar, A: AsCalendar<Calendar = C>>(
        &mut self,
        date_in_calendar: Date<A>,
    ) {
        self.year = Some(date_in_calendar.year());
        self.month = Some(date_in_calendar.month());
        self.day_of_month = Some(date_in_calendar.day_of_month());
        self.weekday = Some(date_in_calendar.weekday());
        self.day_of_year = Some(date_in_calendar.day_of_year());
    }

    /// Sets all fields from a [`Time`] input.
    pub fn set_time_fields(&mut self, time: Time) {
        self.hour = Some(time.hour);
        self.minute = Some(time.minute);
        self.second = Some(time.second);
        self.subsecond = Some(time.subsecond);
    }

    /// Sets the time zone UTC offset.
    pub fn set_time_zone_utc_offset(&mut self, utc_offset: UtcOffset) {
        self.zone_offset = Some(utc_offset);
    }

    /// Sets the time zone ID.
    pub fn set_time_zone_id(&mut self, id: TimeZone) {
        self.zone_id = Some(id);
    }

    /// Sets the local time for time zone name resolution.
    pub fn set_time_zone_name_timestamp(&mut self, timestamp: ZoneNameTimestamp) {
        self.zone_name_timestamp = Some(timestamp);
    }

    /// No-op
    #[deprecated]
    pub fn set_time_zone_variant(&mut self, _zone_variant: icu_time::zone::TimeZoneVariant) {}

    /// Construct given neo date input instances.
    pub(crate) fn extract_from_neo_input<D, T, Z, I>(input: &I) -> Self
    where
        D: DateInputMarkers,
        T: TimeMarkers,
        Z: ZoneMarkers,
        I: ?Sized
            + GetField<D::YearInput>
            + GetField<D::MonthInput>
            + GetField<D::DayOfMonthInput>
            + GetField<D::DayOfWeekInput>
            + GetField<D::DayOfYearInput>
            + GetField<D::RataDieInput>
            + GetField<T::HourInput>
            + GetField<T::MinuteInput>
            + GetField<T::SecondInput>
            + GetField<T::NanosecondInput>
            + GetField<Z::TimeZoneIdInput>
            + GetField<Z::TimeZoneOffsetInput>
            + GetField<Z::TimeZoneNameTimestampInput>,
    {
        Self {
            year: GetField::<D::YearInput>::get_field(input).into_option(),
            month: GetField::<D::MonthInput>::get_field(input).into_option(),
            day_of_month: GetField::<D::DayOfMonthInput>::get_field(input).into_option(),
            weekday: GetField::<D::DayOfWeekInput>::get_field(input).into_option(),
            day_of_year: GetField::<D::DayOfYearInput>::get_field(input).into_option(),
            rata_die: GetField::<D::RataDieInput>::get_field(input).into_option(),
            hour: GetField::<T::HourInput>::get_field(input).into_option(),
            minute: GetField::<T::MinuteInput>::get_field(input).into_option(),
            second: GetField::<T::SecondInput>::get_field(input).into_option(),
            subsecond: GetField::<T::NanosecondInput>::get_field(input).into_option(),
            zone_id: GetField::<Z::TimeZoneIdInput>::get_field(input).into_option(),
            zone_offset: GetField::<Z::TimeZoneOffsetInput>::get_field(input).into_option(),
            zone_name_timestamp: GetField::<Z::TimeZoneNameTimestampInput>::get_field(input)
                .into_option(),
        }
    }
}
