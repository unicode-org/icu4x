// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::fieldsets::enums::*;
use crate::scaffold::*;
use icu_calendar::types::DayOfYearInfo;
use icu_calendar::{AsCalendar, Calendar, Iso};
use icu_time::scaffold::IntoOption;
use icu_time::{zone::TimeZoneVariant, Hour, Minute, Nanosecond, Second};

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
    pub(crate) iso_weekday: Option<Weekday>,
    /// The day-of-year, required for field sets with weeks.
    pub(crate) day_of_year: Option<DayOfYearInfo>,
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
    pub(crate) time_zone_id: Option<TimeZone>,
    /// The time zone UTC offset, required for field sets with
    /// certain time zone styles.
    pub(crate) offset: Option<UtcOffset>,
    /// The time zone variant, required for field sets with
    /// certain time zone styles.
    pub(crate) zone_variant: Option<TimeZoneVariant>,
    /// The local ISO time, required for field sets with
    /// certain time zone styles.
    pub(crate) local_time: Option<(Date<Iso>, Time)>,
}

macro_rules! set_field {
    ($type:ident as $trait:ident, $receiver:expr, $assoc_type:ident, $input:expr) => {
        $receiver = GetField::<<$type as $trait>::$assoc_type>::get_field($input).into_option()
    };
    (@date, $receiver:expr, $assoc_type:ident, $input:expr) => {
        set_field!(
            DateFieldSet as DateInputMarkers,
            $receiver,
            $assoc_type,
            $input
        )
    };
    (@time, $receiver:expr, $assoc_type:ident, $input:expr) => {
        set_field!(TimeFieldSet as TimeMarkers, $receiver, $assoc_type, $input)
    };
    (@zone, $receiver:expr, $assoc_type:ident, $input:expr) => {
        set_field!(ZoneFieldSet as ZoneMarkers, $receiver, $assoc_type, $input)
    };
}

impl DateTimeInputUnchecked {
    /// Sets all fields from a [`Date`] input.
    pub fn set_date_fields<C: Calendar, A: AsCalendar<Calendar = C>>(&mut self, input: Date<A>) {
        set_field!(@date, self.year, YearInput, &input);
        set_field!(@date, self.month, MonthInput, &input);
        set_field!(@date, self.day_of_month, DayOfMonthInput, &input);
        set_field!(@date, self.iso_weekday, DayOfWeekInput, &input);
        set_field!(@date, self.day_of_year, DayOfYearInput, &input);
    }

    /// Sets all fields from a [`Time`] input.
    pub fn set_time_fields(&mut self, input: Time) {
        set_field!(@time, self.hour, HourInput, &input);
        set_field!(@time, self.minute, MinuteInput, &input);
        set_field!(@time, self.second, SecondInput, &input);
        set_field!(@time, self.subsecond, NanosecondInput, &input);
    }

    /// Sets the time zone UTC offset.
    pub fn set_time_zone_utc_offset(&mut self, offset: UtcOffset) {
        self.offset = Some(offset);
    }

    /// Sets the time zone ID.
    pub fn set_time_zone_id(&mut self, id: TimeZone) {
        self.time_zone_id = Some(id);
    }

    /// Sets the local time for time zone name resolution.
    pub fn set_time_zone_local_time(&mut self, local_time: (Date<Iso>, Time)) {
        self.local_time = Some(local_time);
    }

    /// Sets the time zone variant.
    pub fn set_time_zone_variant(&mut self, zone_variant: TimeZoneVariant) {
        self.zone_variant = Some(zone_variant);
    }

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
            + GetField<T::HourInput>
            + GetField<T::MinuteInput>
            + GetField<T::SecondInput>
            + GetField<T::NanosecondInput>
            + GetField<Z::TimeZoneIdInput>
            + GetField<Z::TimeZoneOffsetInput>
            + GetField<Z::TimeZoneVariantInput>
            + GetField<Z::TimeZoneLocalTimeInput>,
    {
        Self {
            year: GetField::<D::YearInput>::get_field(input).into_option(),
            month: GetField::<D::MonthInput>::get_field(input).into_option(),
            day_of_month: GetField::<D::DayOfMonthInput>::get_field(input).into_option(),
            iso_weekday: GetField::<D::DayOfWeekInput>::get_field(input).into_option(),
            day_of_year: GetField::<D::DayOfYearInput>::get_field(input).into_option(),
            hour: GetField::<T::HourInput>::get_field(input).into_option(),
            minute: GetField::<T::MinuteInput>::get_field(input).into_option(),
            second: GetField::<T::SecondInput>::get_field(input).into_option(),
            subsecond: GetField::<T::NanosecondInput>::get_field(input).into_option(),
            time_zone_id: GetField::<Z::TimeZoneIdInput>::get_field(input).into_option(),
            offset: GetField::<Z::TimeZoneOffsetInput>::get_field(input).into_option(),
            zone_variant: GetField::<Z::TimeZoneVariantInput>::get_field(input).into_option(),
            local_time: GetField::<Z::TimeZoneLocalTimeInput>::get_field(input).into_option(),
        }
    }
}
