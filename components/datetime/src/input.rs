// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::neo_marker::{DateInputMarkers, NeoGetField, TimeMarkers, ZoneMarkers};
use crate::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_timezone::{UtcOffset, ZoneVariant};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{
    DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond,
    YearInfo,
};

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
    /// Construct given neo date input instances.
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
