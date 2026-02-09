// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{GetField, InFixedCalendar, UnstableSealed};
use chrono::{Datelike, Offset, Timelike};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{Date, Gregorian};
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
#[cfg(feature = "compiled_data")]
use icu_time::TimeZone;
use icu_time::{Hour, Minute, Nanosecond, Second, ZonedDateTime};

impl UnstableSealed for chrono::NaiveTime {}
impl InFixedCalendar<()> for chrono::NaiveTime {}
impl GetField<()> for chrono::NaiveTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for chrono::NaiveTime {
    fn get_field(&self) -> Hour {
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Hour::try_from(self.hour() as u8).unwrap()
    }
}

impl GetField<Minute> for chrono::NaiveTime {
    fn get_field(&self) -> Minute {
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Minute::try_from(self.minute() as u8).unwrap()
    }
}

impl GetField<Second> for chrono::NaiveTime {
    fn get_field(&self) -> Second {
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Second::try_from(self.second() as u8).unwrap()
    }
}

impl GetField<Nanosecond> for chrono::NaiveTime {
    fn get_field(&self) -> Nanosecond {
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Nanosecond::try_from(self.nanosecond()).unwrap()
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
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Date::try_new_gregorian(self.year(), self.month() as u8, self.day() as u8)
            .unwrap()
            .year()
    }
}

impl GetField<MonthInfo> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Date::try_new_gregorian(self.year(), self.month() as u8, self.day() as u8)
            .unwrap()
            .month()
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
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Date::try_new_gregorian(self.year(), self.month() as u8, self.day() as u8)
            .unwrap()
            .day_of_year()
    }
}

impl GetField<RataDie> for chrono::NaiveDate {
    #[inline]
    fn get_field(&self) -> RataDie {
        RataDie::new(self.num_days_from_ce() as i64)
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
        Some(UtcOffset::from_seconds_unchecked(self.local_minus_utc()))
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
        icu_time::zone::IanaParser::new().parse(self.timezone().name())
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

#[test]
fn chrono() {
    use icu::datetime::{fieldsets, FixedCalendarDateTimeFormatter, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Asia/Tokyo".parse().unwrap());

    let ymdt = FixedCalendarDateTimeFormatter::try_new(
        locale!("en-GB").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdt.format(&chrono),
        "11 Sept 2024, 08:37:20 Japan Standard Time"
    );

    let ymd =
        FixedCalendarDateTimeFormatter::try_new(locale!("en-GB").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&chrono.date_naive()), "11 Sept 2024");

    let t = NoCalendarFormatter::try_new(locale!("en-GB").into(), fieldsets::T::medium()).unwrap();
    assert_writeable_eq!(t.format(&chrono.time()), "08:37:20");

    let offset = NoCalendarFormatter::try_new(
        locale!("en-GB").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&chrono.offset().fix()), "GMT+09:00");
}
