// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::neo_marker::{DateInputMarkers, GetField, TimeMarkers, ZoneMarkers};
use crate::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_timezone::{UtcOffset, ZoneVariant};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{
    DayOfMonth, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond, YearInfo,
};

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct ExtractedInput {
    pub(crate) year: Option<YearInfo>,
    pub(crate) month: Option<MonthInfo>,
    pub(crate) day_of_month: Option<DayOfMonth>,
    pub(crate) iso_weekday: Option<IsoWeekday>,
    pub(crate) any_calendar_kind: Option<AnyCalendarKind>,
    pub(crate) hour: Option<IsoHour>,
    pub(crate) minute: Option<IsoMinute>,
    pub(crate) second: Option<IsoSecond>,
    pub(crate) nanosecond: Option<NanoSecond>,
    pub(crate) offset: Option<UtcOffset>,
    pub(crate) time_zone_id: Option<TimeZoneBcp47Id>,
    pub(crate) metazone_id: Option<Option<MetazoneId>>,
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
            + GetField<D::YearInput>
            + GetField<D::MonthInput>
            + GetField<D::DayOfMonthInput>
            + GetField<D::DayOfWeekInput>
            + GetField<D::AnyCalendarKindInput>
            + GetField<T::HourInput>
            + GetField<T::MinuteInput>
            + GetField<T::SecondInput>
            + GetField<T::NanoSecondInput>
            + GetField<Z::TimeZoneOffsetInput>
            + GetField<Z::TimeZoneIdInput>
            + GetField<Z::TimeZoneMetazoneInput>
            + GetField<Z::TimeZoneVariantInput>,
    {
        Self {
            year: GetField::<D::YearInput>::get_field(input).into(),
            month: GetField::<D::MonthInput>::get_field(input).into(),
            day_of_month: GetField::<D::DayOfMonthInput>::get_field(input).into(),
            iso_weekday: GetField::<D::DayOfWeekInput>::get_field(input).into(),
            any_calendar_kind: GetField::<D::AnyCalendarKindInput>::get_field(input).into(),
            hour: GetField::<T::HourInput>::get_field(input).into(),
            minute: GetField::<T::MinuteInput>::get_field(input).into(),
            second: GetField::<T::SecondInput>::get_field(input).into(),
            nanosecond: GetField::<T::NanoSecondInput>::get_field(input).into(),
            offset: GetField::<Z::TimeZoneOffsetInput>::get_field(input).into(),
            time_zone_id: GetField::<Z::TimeZoneIdInput>::get_field(input).into(),
            metazone_id: GetField::<Z::TimeZoneMetazoneInput>::get_field(input).into(),
            zone_variant: GetField::<Z::TimeZoneVariantInput>::get_field(input).into(),
        }
    }
}
