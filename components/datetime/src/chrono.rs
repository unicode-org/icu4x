// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{ConvertCalendar, GetField, InFixedCalendar, InSameCalendar, UnstableSealed};
use crate::MismatchedCalendarError;
use chrono::{Datelike, Offset};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{AnyCalendar, AnyCalendarKind, Date, Gregorian};
use icu_time::zone::models::AtTime;
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
use icu_time::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};
use icu_time::{TimeZone, TimeZoneInfo};

impl UnstableSealed for chrono::NaiveTime {}
impl<C> InFixedCalendar<C> for chrono::NaiveTime {}
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
        self.weekday().get_field()
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
impl<C> InFixedCalendar<C> for chrono::FixedOffset {}
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

impl UnstableSealed for chrono::Weekday {}
impl<C> InFixedCalendar<C> for chrono::Weekday {}
impl GetField<()> for chrono::Weekday {
    fn get_field(&self) {}
}

impl GetField<Weekday> for chrono::Weekday {
    fn get_field(&self) -> Weekday {
        (*self).into()
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

impl<Tz: chrono::TimeZone> GetField<TimeZone> for chrono::DateTime<Tz>
where
    TimeZone: From<Tz>,
{
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

impl ConvertCalendar for chrono::Weekday {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl<Tz: chrono::TimeZone> ConvertCalendar for chrono::DateTime<Tz>
where
    for<'a> ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>>: From<&'a chrono::DateTime<Tz>>,
{
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

impl InSameCalendar for chrono::NaiveTime {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for chrono::Weekday {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for chrono::NaiveDate {
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        if any_calendar_kind == AnyCalendarKind::Gregorian {
            Ok(())
        } else {
            Err(MismatchedCalendarError {
                this_kind: any_calendar_kind,
                date_kind: Some(AnyCalendarKind::Gregorian),
            })
        }
    }
}

impl InSameCalendar for chrono::NaiveDateTime {
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date().check_any_calendar_kind(any_calendar_kind)
    }
}

impl<Tz: chrono::TimeZone> InSameCalendar for chrono::DateTime<Tz> {
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date_naive().check_any_calendar_kind(any_calendar_kind)
    }
}

impl InSameCalendar for chrono::FixedOffset {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

#[test]
fn chrono() {
    use icu::datetime::{fieldsets, DateTimeFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Asia/Tokyo".parse::<chrono_tz::Tz>().unwrap());

    let ymdto = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdto.format(&chrono.with_timezone(&chrono.offset().fix())),
        "令和6年9月11日 8:37:20 GMT+9"
    );

    let ymdtz = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(ymdtz.format(&chrono), "令和6年9月11日 8:37:20 日本標準時");

    let ymdt = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymdt.format(&chrono.naive_local()), "令和6年9月11日 8:37:20");

    let ymd =
        DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&chrono.date_naive()), "令和6年9月11日");

    let t = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::T::medium())
        .unwrap();
    assert_writeable_eq!(t.format(&chrono.time()), "8:37:20");

    let e = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::E::long())
        .unwrap();
    assert_writeable_eq!(e.format(&chrono.weekday()), "水曜日");

    let offset = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&chrono.offset().fix()), "GMT+09:00");
}

#[test]
fn chrono_fixed_calendar() {
    use icu::datetime::{fieldsets, FixedCalendarDateTimeFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Europe/Paris".parse::<chrono_tz::Tz>().unwrap());

    let ymdto = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdto.format(&chrono.with_timezone(&chrono.offset().fix())),
        "11 sept. 2024, 01:37:20 UTC+2"
    );

    let ymdtz = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdtz.format(&chrono),
        "11 sept. 2024, 01:37:20 heure d’été d’Europe centrale"
    );

    let ymdt = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdt.format(&chrono.naive_local()),
        "11 sept. 2024, 01:37:20"
    );

    let ymd = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMD::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymd.format(&chrono.date_naive()), "11 sept. 2024");

    let t = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::T::medium(),
    )
    .unwrap();
    assert_writeable_eq!(t.format(&chrono.time()), "01:37:20");

    let e = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::E::long(),
    )
    .unwrap();
    assert_writeable_eq!(e.format(&chrono.weekday()), "mercredi");

    let offset = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&chrono.offset().fix()), "UTC+02:00");
}

#[test]
fn chrono_no_calendar() {
    use icu::datetime::{fieldsets, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Europe/Zurich".parse::<chrono_tz::Tz>().unwrap());

    let t = NoCalendarFormatter::try_new(locale!("de-CH").into(), fieldsets::T::medium()).unwrap();
    assert_writeable_eq!(t.format(&chrono.time()), "01:37:20");

    let offset =
        NoCalendarFormatter::try_new(locale!("de").into(), fieldsets::zone::LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(offset.format(&chrono.offset().fix()), "GMT+02:00");
}
