// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo},
    AsCalendar, Calendar, Date,
};
use icu_time::{
    zone::{models::TimeZoneModel, UtcOffset, ZoneNameTimestamp},
    DateTime, Hour, Minute, Nanosecond, Second, Time, TimeZone, TimeZoneInfo, ZonedDateTime,
    ZonedTime,
};

use super::UnstableSealed;

/// A type that can return a certain field `T`.
///
/// This is used as a bound on various datetime functions.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break.
/// </div>
pub trait GetField<T>: UnstableSealed {
    /// Returns the value of this trait's field `T`.
    fn get_field(&self) -> T;
}

impl<T> GetField<T> for T
where
    T: Copy + UnstableSealed,
{
    #[inline]
    fn get_field(&self) -> T {
        *self
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> UnstableSealed for Date<A> {}

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

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Weekday> for Date<A> {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.weekday()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfYear> for Date<A> {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.day_of_year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<RataDie> for Date<A> {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.to_rata_die()
    }
}

impl UnstableSealed for Time {}

impl GetField<Hour> for Time {
    #[inline]
    fn get_field(&self) -> Hour {
        self.hour
    }
}

impl GetField<Minute> for Time {
    #[inline]
    fn get_field(&self) -> Minute {
        self.minute
    }
}

impl GetField<Second> for Time {
    #[inline]
    fn get_field(&self) -> Second {
        self.second
    }
}

impl GetField<Nanosecond> for Time {
    #[inline]
    fn get_field(&self) -> Nanosecond {
        self.subsecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> UnstableSealed for DateTime<A> {}

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

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Weekday> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date.weekday()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<DayOfYear> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date.day_of_year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<RataDie> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date.to_rata_die()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Hour> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> Hour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Minute> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> Minute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Second> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> Second {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> GetField<Nanosecond> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> Nanosecond {
        self.time.subsecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> UnstableSealed for ZonedDateTime<A, Z> {}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<YearInfo> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<MonthInfo> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<DayOfMonth> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Weekday> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date.weekday()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<DayOfYear> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date.day_of_year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<RataDie> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date.to_rata_die()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Hour> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> Hour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Minute> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> Minute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Second> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> Second {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Nanosecond> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) -> Nanosecond {
        self.time.subsecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<Option<UtcOffset>>
    for ZonedDateTime<A, Z>
where
    Z: GetField<Option<UtcOffset>>,
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.zone.get_field()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<TimeZone> for ZonedDateTime<A, Z>
where
    Z: GetField<TimeZone>,
{
    #[inline]
    fn get_field(&self) -> TimeZone {
        self.zone.get_field()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<ZoneNameTimestamp>
    for ZonedDateTime<A, Z>
where
    Z: GetField<ZoneNameTimestamp>,
{
    #[inline]
    fn get_field(&self) -> ZoneNameTimestamp {
        self.zone.get_field()
    }
}

impl<Z> UnstableSealed for ZonedTime<Z> {}

impl<Z> GetField<Hour> for ZonedTime<Z> {
    fn get_field(&self) -> Hour {
        self.time.hour
    }
}

impl<Z> GetField<Minute> for ZonedTime<Z> {
    fn get_field(&self) -> Minute {
        self.time.minute
    }
}

impl<Z> GetField<Second> for ZonedTime<Z> {
    fn get_field(&self) -> Second {
        self.time.second
    }
}

impl<Z> GetField<Nanosecond> for ZonedTime<Z> {
    fn get_field(&self) -> Nanosecond {
        self.time.subsecond
    }
}

impl<Z> GetField<Option<UtcOffset>> for ZonedTime<Z>
where
    Z: GetField<Option<UtcOffset>>,
{
    fn get_field(&self) -> Option<UtcOffset> {
        self.zone.get_field()
    }
}

impl<Z> GetField<TimeZone> for ZonedTime<Z>
where
    Z: GetField<TimeZone>,
{
    fn get_field(&self) -> TimeZone {
        self.zone.get_field()
    }
}

impl<Z> GetField<ZoneNameTimestamp> for ZonedTime<Z>
where
    Z: GetField<ZoneNameTimestamp>,
{
    fn get_field(&self) -> ZoneNameTimestamp {
        self.zone.get_field()
    }
}

impl UnstableSealed for UtcOffset {}

impl GetField<Option<UtcOffset>> for UtcOffset {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some(*self)
    }
}

impl<O: TimeZoneModel> UnstableSealed for TimeZoneInfo<O> {}

impl<O> GetField<TimeZone> for TimeZoneInfo<O>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> TimeZone {
        self.id()
    }
}

impl<O> GetField<Option<UtcOffset>> for TimeZoneInfo<O>
where
    O: TimeZoneModel,
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset()
    }
}

impl<O> GetField<ZoneNameTimestamp> for TimeZoneInfo<O>
where
    O: TimeZoneModel<ZoneNameTimestamp = ZoneNameTimestamp>,
{
    #[inline]
    fn get_field(&self) -> ZoneNameTimestamp {
        self.zone_name_timestamp()
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

impl<C: Calendar, A: AsCalendar<Calendar = C>, Z> GetField<()> for ZonedDateTime<A, Z> {
    #[inline]
    fn get_field(&self) {}
}

// Required for the `AllInputMarkers` trait bound
impl<Z> GetField<()> for ZonedTime<Z> {
    fn get_field(&self) -> () {}
}

impl GetField<()> for UtcOffset {
    #[inline]
    fn get_field(&self) {}
}

impl<O: TimeZoneModel> GetField<()> for TimeZoneInfo<O> {
    #[inline]
    fn get_field(&self) {}
}
