// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{ConvertCalendar, GetField, InFixedCalendar, UnstableSealed};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{AnyCalendar, Date, Gregorian};
#[cfg(feature = "compiled_data")]
use icu_time::zone::models::AtTime;
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
use icu_time::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};
#[cfg(feature = "compiled_data")]
use icu_time::{TimeZone, TimeZoneInfo};

impl UnstableSealed for jiff::civil::Time {}
impl InFixedCalendar<()> for jiff::civil::Time {}
impl GetField<()> for jiff::civil::Time {
    fn get_field(&self) {}
}

impl GetField<Hour> for jiff::civil::Time {
    fn get_field(&self) -> Hour {
        Time::from(*self).hour
    }
}

impl GetField<Minute> for jiff::civil::Time {
    fn get_field(&self) -> Minute {
        Time::from(*self).minute
    }
}

impl GetField<Second> for jiff::civil::Time {
    fn get_field(&self) -> Second {
        Time::from(*self).second
    }
}

impl GetField<Nanosecond> for jiff::civil::Time {
    fn get_field(&self) -> Nanosecond {
        Time::from(*self).subsecond
    }
}

impl UnstableSealed for jiff::civil::Date {}
impl InFixedCalendar<Gregorian> for jiff::civil::Date {}
impl GetField<()> for jiff::civil::Date {
    fn get_field(&self) {}
}

impl GetField<YearInfo> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> YearInfo {
        Date::from(*self).year()
    }
}

impl GetField<MonthInfo> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        Date::from(*self).month()
    }
}

impl GetField<DayOfMonth> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        DayOfMonth(self.day() as u8)
    }
}

impl GetField<Weekday> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> Weekday {
        match self.weekday() {
            jiff::civil::Weekday::Monday => Weekday::Monday,
            jiff::civil::Weekday::Tuesday => Weekday::Tuesday,
            jiff::civil::Weekday::Wednesday => Weekday::Wednesday,
            jiff::civil::Weekday::Thursday => Weekday::Thursday,
            jiff::civil::Weekday::Friday => Weekday::Friday,
            jiff::civil::Weekday::Saturday => Weekday::Saturday,
            jiff::civil::Weekday::Sunday => Weekday::Sunday,
        }
    }
}

impl GetField<DayOfYear> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        DayOfYear(self.day_of_year() as u16)
    }
}

impl GetField<RataDie> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> RataDie {
        Date::from(*self).to_rata_die()
    }
}

impl UnstableSealed for jiff::tz::Offset {}
impl InFixedCalendar<()> for jiff::tz::Offset {}
impl GetField<()> for jiff::tz::Offset {
    fn get_field(&self) {}
}

impl GetField<Option<UtcOffset>> for jiff::tz::Offset {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some((*self).into())
    }
}

impl UnstableSealed for jiff::civil::DateTime {}
impl InFixedCalendar<Gregorian> for jiff::civil::DateTime {}
impl GetField<()> for jiff::civil::DateTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for jiff::civil::DateTime {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl GetField<Minute> for jiff::civil::DateTime {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl GetField<Second> for jiff::civil::DateTime {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl GetField<Nanosecond> for jiff::civil::DateTime {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl GetField<YearInfo> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date().get_field()
    }
}

impl GetField<MonthInfo> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date().get_field()
    }
}

impl GetField<DayOfMonth> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date().get_field()
    }
}

impl GetField<Weekday> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date().get_field()
    }
}

impl GetField<DayOfYear> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date().get_field()
    }
}

impl GetField<RataDie> for jiff::civil::DateTime {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date().get_field()
    }
}

impl UnstableSealed for jiff::Zoned {}
impl InFixedCalendar<Gregorian> for jiff::Zoned {}
impl GetField<()> for jiff::Zoned {
    fn get_field(&self) {}
}

impl GetField<Hour> for jiff::Zoned {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl GetField<Minute> for jiff::Zoned {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl GetField<Second> for jiff::Zoned {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl GetField<Nanosecond> for jiff::Zoned {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl GetField<YearInfo> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date().get_field()
    }
}

impl GetField<MonthInfo> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date().get_field()
    }
}

impl GetField<DayOfMonth> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date().get_field()
    }
}

impl GetField<Weekday> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date().get_field()
    }
}

impl GetField<DayOfYear> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date().get_field()
    }
}

impl GetField<RataDie> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date().get_field()
    }
}

#[cfg(feature = "compiled_data")]
impl GetField<TimeZone> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> TimeZone {
        TimeZone::from(self.time_zone())
    }
}

impl GetField<Option<UtcOffset>> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset().get_field()
    }
}

impl GetField<ZoneNameTimestamp> for jiff::Zoned {
    #[inline]
    fn get_field(&self) -> ZoneNameTimestamp {
        ZoneNameTimestamp::from_zoned_date_time_iso(
            ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                self.timestamp().as_millisecond(),
                UtcOffset::zero(),
            ),
        )
    }
}

impl ConvertCalendar for jiff::civil::Time {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl ConvertCalendar for jiff::civil::Date {
    type Converted<'a> = <Date<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&Date::from(*self), calendar)
    }
}

impl ConvertCalendar for jiff::civil::DateTime {
    type Converted<'a> = <DateTime<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&DateTime::from(*self), calendar)
    }
}

#[cfg(feature = "compiled_data")]
impl ConvertCalendar for jiff::Zoned {
    type Converted<'a> =
        <ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&ZonedDateTime::from(self), calendar)
    }
}

impl ConvertCalendar for jiff::tz::Offset {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

#[test]
fn jiff() {
    use icu::datetime::{fieldsets, DateTimeFormatter, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Asia/Tokyo").unwrap());

    let ymdt = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(ymdt.format(&jiff), "令和6年9月11日 8:37:20 日本標準時");

    let ymd =
        DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&jiff.date()), "令和6年9月11日");

    let t =
        NoCalendarFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::T::medium())
            .unwrap();
    assert_writeable_eq!(t.format(&jiff.time()), "8:37:20");

    let offset = NoCalendarFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&jiff.offset()), "GMT+09:00");
}
