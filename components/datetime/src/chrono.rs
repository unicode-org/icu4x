// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{ConvertCalendar, GetField, InFixedCalendar, UnstableSealed};
use chrono::{Datelike, Offset};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{AnyCalendar, Date, Gregorian};
#[cfg(feature = "compiled_data")]
use icu_time::zone::models::AtTime;
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
use icu_time::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};
#[cfg(feature = "compiled_data")]
use icu_time::{TimeZone, TimeZoneInfo};

impl UnstableSealed for chrono::NaiveTime {}
impl InFixedCalendar<()> for chrono::NaiveTime {}
impl GetField<()> for chrono::NaiveTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for chrono::NaiveTime {
    fn get_field(&self) -> Hour {
        Time::from(*self).hour
    }
}

impl GetField<Minute> for chrono::NaiveTime {
    fn get_field(&self) -> Minute {
        Time::from(*self).minute
    }
}

impl GetField<Second> for chrono::NaiveTime {
    fn get_field(&self) -> Second {
        Time::from(*self).second
    }
}

impl GetField<Nanosecond> for chrono::NaiveTime {
    fn get_field(&self) -> Nanosecond {
        Time::from(*self).subsecond
    }
}

impl UnstableSealed for chrono::NaiveDate {}
impl InFixedCalendar<Gregorian> for chrono::NaiveDate {}
impl GetField<()> for chrono::NaiveDate {
    fn get_field(&self) {}
}

impl GetField<YearInfo> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> YearInfo {
        Date::from(*self).year()
    }
}

impl GetField<MonthInfo> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        Date::from(*self).month()
    }
}

impl GetField<DayOfMonth> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        DayOfMonth(self.day() as u8)
    }
}

impl GetField<Weekday> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> Weekday {
        match self.weekday() {
            chrono::Weekday::Mon => Weekday::Monday,
            chrono::Weekday::Tue => Weekday::Tuesday,
            chrono::Weekday::Wed => Weekday::Wednesday,
            chrono::Weekday::Thu => Weekday::Thursday,
            chrono::Weekday::Fri => Weekday::Friday,
            chrono::Weekday::Sat => Weekday::Saturday,
            chrono::Weekday::Sun => Weekday::Sunday,
        }
    }
}

impl GetField<DayOfYear> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        DayOfYear(self.ordinal() as u16)
    }
}

impl GetField<RataDie> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> RataDie {
        RataDie::new(0) + self.num_days_from_ce() as i64
    }
}

impl UnstableSealed for chrono::FixedOffset {}
impl InFixedCalendar<()> for chrono::FixedOffset {}
impl GetField<()> for chrono::FixedOffset {
    fn get_field(&self) {}
}

impl GetField<Option<UtcOffset>> for chrono::FixedOffset {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some((*self).into())
    }
}

impl UnstableSealed for chrono::NaiveDateTime {}
impl InFixedCalendar<Gregorian> for chrono::NaiveDateTime {}
impl GetField<()> for chrono::NaiveDateTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for chrono::NaiveDateTime {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl GetField<Minute> for chrono::NaiveDateTime {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl GetField<Second> for chrono::NaiveDateTime {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl GetField<Nanosecond> for chrono::NaiveDateTime {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl GetField<YearInfo> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date().get_field()
    }
}

impl GetField<MonthInfo> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date().get_field()
    }
}

impl GetField<DayOfMonth> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date().get_field()
    }
}

impl GetField<Weekday> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date().get_field()
    }
}

impl GetField<DayOfYear> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date().get_field()
    }
}

impl GetField<RataDie> for chrono::NaiveDateTime {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date().get_field()
    }
}

impl<Tz: chrono::TimeZone> UnstableSealed for chrono::DateTime<Tz> {}
impl<Tz: chrono::TimeZone> InFixedCalendar<Gregorian> for chrono::DateTime<Tz> {}
impl<Tz: chrono::TimeZone> GetField<()> for chrono::DateTime<Tz> {
    fn get_field(&self) {}
}

impl<Tz: chrono::TimeZone> GetField<Hour> for chrono::DateTime<Tz> {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<Minute> for chrono::DateTime<Tz> {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<Second> for chrono::DateTime<Tz> {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<Nanosecond> for chrono::DateTime<Tz> {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<YearInfo> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.naive_local().date().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<MonthInfo> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.naive_local().date().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<DayOfMonth> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.naive_local().date().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<Weekday> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.naive_local().date().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<DayOfYear> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.naive_local().date().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<RataDie> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.naive_local().date().get_field()
    }
}

#[cfg(feature = "compiled_data")]
impl GetField<TimeZone> for chrono::DateTime<chrono_tz::Tz> {
    #[inline]
    fn get_field(&self) -> TimeZone {
        TimeZone::from(self.timezone())
    }
}

impl<Tz: chrono::TimeZone> GetField<Option<UtcOffset>> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset().fix().get_field()
    }
}

impl<Tz: chrono::TimeZone> GetField<ZoneNameTimestamp> for chrono::DateTime<Tz> {
    #[inline]
    fn get_field(&self) -> ZoneNameTimestamp {
        ZoneNameTimestamp::from_zoned_date_time_iso(
            ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                self.timestamp() * 1000,
                UtcOffset::zero(),
            ),
        )
    }
}

impl ConvertCalendar for chrono::NaiveTime {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl ConvertCalendar for chrono::NaiveDate {
    type Converted<'a> = <Date<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&Date::from(*self), calendar)
    }
}

impl ConvertCalendar for chrono::NaiveDateTime {
    type Converted<'a> = <DateTime<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&DateTime::from(*self), calendar)
    }
}

#[cfg(feature = "compiled_data")]
impl ConvertCalendar for chrono::DateTime<chrono_tz::Tz> {
    type Converted<'a> =
        <ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&ZonedDateTime::from(self), calendar)
    }
}

impl ConvertCalendar for chrono::FixedOffset {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

#[test]
fn chrono() {
    use icu::datetime::{fieldsets, DateTimeFormatter, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Asia/Tokyo".parse().unwrap());

    let ymdt = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(ymdt.format(&chrono), "令和6年9月11日 8:37:20 日本標準時");

    let ymd =
        DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&chrono.date_naive()), "令和6年9月11日");

    let t =
        NoCalendarFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::T::medium())
            .unwrap();
    assert_writeable_eq!(t.format(&chrono.time()), "8:37:20");

    let offset = NoCalendarFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&chrono.offset().fix()), "GMT+09:00");
}
