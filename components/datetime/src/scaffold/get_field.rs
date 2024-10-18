// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz, *},
    scaffold::*,
};
use icu_calendar::{
    any_calendar::IntoAnyCalendar,
    types::{
        DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo,
        NanoSecond, YearInfo,
    },
    AnyCalendar, AnyCalendarKind, AsCalendar, Calendar, Date, DateTime, Iso, Ref, Time,
};
use icu_provider::{marker::NeverMarker, prelude::*};
use icu_timezone::scaffold::IntoOption;
use icu_timezone::{
    CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneInfo, TimeZoneModel, UtcOffset, ZoneVariant,
};

/// A type that can return a certain field `T`.
pub trait GetField<T> {
    /// Returns the value of this trait's field `T`.
    fn get_field(&self) -> T;
}

/// Generates an impl of [`GetField`]
macro_rules! impl_get_field {
    ($(< $($generics0:tt),+ >)? $type:ident $(< $($generics1:tt),+ >)?, never) => {
        impl $(<$($generics0),+>)? GetField<()> for $type $(<$($generics1),+>)? {
            fn get_field(&self) {}
        }
    };
    ($(< $($generics0:tt),+ >)? $type:ident $(< $($generics1:tt),+ >)?, length, yes) => {
        impl $(<$($generics0),+>)? GetField<NeoSkeletonLength> for $type $(<$($generics1),+>)? {
            fn get_field(&self) -> NeoSkeletonLength {
                self.length
            }
        }
    };
    ($(< $($generics0:tt),+ >)? $type:ident $(< $($generics1:tt),+ >)?, alignment, yes) => {
        impl $(<$($generics0),+>)? GetField<Option<Alignment>> for $type $(<$($generics1),+>)? {
            fn get_field(&self) -> Option<Alignment> {
                self.alignment
            }
        }
    };
    ($(< $($generics0:tt),+ >)? $type:ident $(< $($generics1:tt),+ >)?, year_style, yes) => {
        impl $(<$($generics0),+>)? GetField<Option<YearStyle>> for $type $(<$($generics1),+>)? {
            fn get_field(&self) -> Option<YearStyle> {
                self.year_style
            }
        }
    };
    ($(< $($generics0:tt),+ >)? $type:ident $(< $($generics1:tt),+ >)?, fractional_second_digits, yes) => {
        impl $(<$($generics0),+>)? GetField<Option<FractionalSecondDigits>> for $type $(<$($generics1),+>)? {
            fn get_field(&self) -> Option<FractionalSecondDigits> {
                self.fractional_second_digits
            }
        }
    };
}
pub(crate) use impl_get_field;

impl<T> GetField<T> for T
where
    T: Copy,
{
    #[inline]
    fn get_field(&self) -> T {
        *self
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<YearInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<MonthInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfMonth> for Date<A> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<IsoWeekday> for Date<A> {
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfYearInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> GetField<AnyCalendarKind> for Date<A> {
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.calendar().kind()
    }
}

impl GetField<IsoHour> for Time {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.hour
    }
}

impl GetField<IsoMinute> for Time {
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.minute
    }
}

impl GetField<IsoSecond> for Time {
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.second
    }
}

impl GetField<NanoSecond> for Time {
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.nanosecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<YearInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<MonthInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfMonth> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<IsoWeekday> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.date.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfYearInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.date.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> GetField<AnyCalendarKind> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.date.calendar().kind()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<IsoHour> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<IsoMinute> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<IsoSecond> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<NanoSecond> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.time.nanosecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<YearInfo> for CustomZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<MonthInfo>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<DayOfMonth>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<IsoWeekday>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.date.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<DayOfYearInfo>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.date.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>, Z> GetField<AnyCalendarKind>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.date.calendar().kind()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<IsoHour> for CustomZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<IsoMinute>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<IsoSecond>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<NanoSecond>
    for CustomZonedDateTime<A, Z>
{
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.time.nanosecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Option<UtcOffset>>
    for CustomZonedDateTime<A, UtcOffset>
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some(self.zone)
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, O> GetField<TimeZoneBcp47Id>
    for CustomZonedDateTime<A, TimeZoneInfo<O>>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> TimeZoneBcp47Id {
        self.zone.time_zone_id
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, O> GetField<Option<UtcOffset>>
    for CustomZonedDateTime<A, TimeZoneInfo<O>>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.zone.offset
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, O> GetField<ZoneVariant>
    for CustomZonedDateTime<A, TimeZoneInfo<O>>
where
    O: TimeZoneModel<ZoneVariant = ZoneVariant>,
{
    #[inline]
    fn get_field(&self) -> ZoneVariant {
        self.zone.zone_variant
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, O> GetField<(Date<Iso>, Time)>
    for CustomZonedDateTime<A, TimeZoneInfo<O>>
where
    O: TimeZoneModel<LocalTime = (Date<Iso>, Time)>,
{
    #[inline]
    fn get_field(&self) -> (Date<Iso>, Time) {
        self.zone.local_time
    }
}

impl GetField<Option<UtcOffset>> for UtcOffset {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some(*self)
    }
}

impl<O> GetField<TimeZoneBcp47Id> for TimeZoneInfo<O>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> TimeZoneBcp47Id {
        self.time_zone_id
    }
}

impl<O> GetField<Option<UtcOffset>> for TimeZoneInfo<O>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset
    }
}

impl<O> GetField<ZoneVariant> for TimeZoneInfo<O>
where
    O: TimeZoneModel<ZoneVariant = ZoneVariant>,
{
    #[inline]
    fn get_field(&self) -> ZoneVariant {
        self.zone_variant
    }
}

impl<O> GetField<(Date<Iso>, Time)> for TimeZoneInfo<O>
where
    O: TimeZoneModel<LocalTime = (Date<Iso>, Time)>,
{
    #[inline]
    fn get_field(&self) -> (Date<Iso>, Time) {
        self.local_time
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<()> for Date<A> {
    #[inline]
    fn get_field(&self) {}
}

impl GetField<()> for Time {
    #[inline]
    fn get_field(&self) {}
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<()> for DateTime<A> {
    #[inline]
    fn get_field(&self) {}
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<()> for CustomZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) {}
}

impl GetField<()> for UtcOffset {
    #[inline]
    fn get_field(&self) {}
}

impl<O: TimeZoneModel> GetField<()> for TimeZoneInfo<O> {
    #[inline]
    fn get_field(&self) {}
}
