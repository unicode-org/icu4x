// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

#[cfg(feature = "experimental")]
use crate::neo_marker::{DateInputMarkers, NeoGetField, TimeMarkers, ZoneMarkers};
use crate::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::Calendar;
use icu_calendar::{AsCalendar, Date, DateTime, Iso};
use icu_timezone::{CustomTimeZone, UtcOffset, ZoneVariant};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{
    DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond,
    Time, YearInfo,
};

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
    fn year(&self) -> Option<YearInfo>;

    /// Gets the month input.
    fn month(&self) -> Option<MonthInfo>;

    /// Gets the day input.
    fn day_of_month(&self) -> Option<DayOfMonth>;

    /// Gets the weekday input.
    fn iso_weekday(&self) -> Option<IsoWeekday>;

    /// Gets information on the position of the day within the year.
    fn day_of_year_info(&self) -> Option<DayOfYearInfo>;

    /// Gets the kind of calendar this date is for, if associated with [`AnyCalendar`]
    /// In most cases you'll probably want to return [`AnyCalendarKind::Iso`].
    ///
    /// [`AnyCalendar`]: icu_calendar::any_calendar::AnyCalendar
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
/// Only the [`UtcOffset`] is required, since it is the final format fallback.
///
/// All data represented in [`TimeZoneInput`] should be locale-agnostic.
pub trait TimeZoneInput {
    /// The UTC offset.
    fn offset(&self) -> Option<UtcOffset>;

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

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct ExtractedInput {
    pub(crate) year: Option<YearInfo>,
    pub(crate) month: Option<MonthInfo>,
    pub(crate) day_of_month: Option<DayOfMonth>,
    pub(crate) iso_weekday: Option<IsoWeekday>,
    pub(crate) day_of_year_info: Option<DayOfYearInfo>,
    pub(crate) any_calendar_kind: Option<AnyCalendarKind>,
    pub(crate) hour: Option<IsoHour>,
    pub(crate) minute: Option<IsoMinute>,
    pub(crate) second: Option<IsoSecond>,
    pub(crate) nanosecond: Option<NanoSecond>,
    pub(crate) offset: Option<UtcOffset>,
    pub(crate) time_zone_id: Option<TimeZoneBcp47Id>,
    pub(crate) metazone_id: Option<MetazoneId>,
    pub(crate) zone_variant: Option<ZoneVariant>,
}

impl ExtractedInput {
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
            ..Default::default()
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
    /// Construct given neo date input instances.
    #[cfg(feature = "experimental")]
    pub(crate) fn extract_from_neo_input<D, T, Z, I>(input: &I) -> Self
    where
        D: DateInputMarkers,
        T: TimeMarkers,
        Z: ZoneMarkers,
        I: ?Sized
            + NeoGetField<D::YearInput>
            + NeoGetField<D::MonthInput>
            + NeoGetField<D::DayOfMonthInput>
            + NeoGetField<D::DayOfWeekInput>
            + NeoGetField<D::DayOfYearInput>
            + NeoGetField<D::AnyCalendarKindInput>
            + NeoGetField<T::HourInput>
            + NeoGetField<T::MinuteInput>
            + NeoGetField<T::SecondInput>
            + NeoGetField<T::NanoSecondInput>
            + NeoGetField<Z::TimeZoneOffsetInput>
            + NeoGetField<Z::TimeZoneIdInput>
            + NeoGetField<Z::TimeZoneMetazoneInput>
            + NeoGetField<Z::TimeZoneVariantInput>,
    {
        Self {
            year: NeoGetField::<D::YearInput>::get_field(input).into(),
            month: NeoGetField::<D::MonthInput>::get_field(input).into(),
            day_of_month: NeoGetField::<D::DayOfMonthInput>::get_field(input).into(),
            iso_weekday: NeoGetField::<D::DayOfWeekInput>::get_field(input).into(),
            day_of_year_info: NeoGetField::<D::DayOfYearInput>::get_field(input).into(),
            any_calendar_kind: NeoGetField::<D::AnyCalendarKindInput>::get_field(input).into(),
            hour: NeoGetField::<T::HourInput>::get_field(input).into(),
            minute: NeoGetField::<T::MinuteInput>::get_field(input).into(),
            second: NeoGetField::<T::SecondInput>::get_field(input).into(),
            nanosecond: NeoGetField::<T::NanoSecondInput>::get_field(input).into(),
            offset: NeoGetField::<Z::TimeZoneOffsetInput>::get_field(input).into(),
            time_zone_id: NeoGetField::<Z::TimeZoneIdInput>::get_field(input).into(),
            metazone_id: NeoGetField::<Z::TimeZoneMetazoneInput>::get_field(input).into(),
            zone_variant: NeoGetField::<Z::TimeZoneVariantInput>::get_field(input).into(),
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> DateInput for Date<A> {
    type Calendar = C;
    /// Gets the era and year input.
    fn year(&self) -> Option<YearInfo> {
        Some(self.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<MonthInfo> {
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
    fn year(&self) -> Option<YearInfo> {
        Some(self.date.year())
    }

    /// Gets the month input.
    fn month(&self) -> Option<MonthInfo> {
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
    fn offset(&self) -> Option<UtcOffset> {
        self.offset
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
