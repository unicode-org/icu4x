// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{ConvertCalendar, GetField, InFixedCalendar, InSameCalendar, UnstableSealed};
use crate::MismatchedCalendarError;
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{AnyCalendar, AnyCalendarKind, Date, Gregorian};
#[cfg(feature = "compiled_data")]
use icu_time::zone::models::AtTime;
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
use icu_time::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};
#[cfg(feature = "compiled_data")]
use icu_time::{TimeZone, TimeZoneInfo};

impl UnstableSealed for jiff::civil::Time {}
impl<C> InFixedCalendar<C> for jiff::civil::Time {}
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
        self.weekday().get_field()
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
impl<C> InFixedCalendar<C> for jiff::tz::Offset {}
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

impl UnstableSealed for jiff::civil::Weekday {}
impl<C> InFixedCalendar<C> for jiff::civil::Weekday {}
impl GetField<()> for jiff::civil::Weekday {
    fn get_field(&self) {}
}

impl GetField<Weekday> for jiff::civil::Weekday {
    fn get_field(&self) -> Weekday {
        (*self).into()
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

impl ConvertCalendar for jiff::civil::Weekday {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
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

impl InSameCalendar for jiff::civil::Time {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for jiff::civil::Date {
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

impl InSameCalendar for jiff::civil::DateTime {
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date().check_any_calendar_kind(any_calendar_kind)
    }
}

impl InSameCalendar for jiff::civil::Weekday {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for jiff::Zoned {
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date().check_any_calendar_kind(any_calendar_kind)
    }
}

impl InSameCalendar for jiff::tz::Offset {
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

#[test]
fn jiff() {
    use icu::datetime::{fieldsets, DateTimeFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Asia/Tokyo").unwrap());

    let ymdto = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(ymdto.format(&jiff), "令和6年9月11日 8:37:20 GMT+9");

    let ymdtz = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(ymdtz.format(&jiff), "令和6年9月11日 8:37:20 日本標準時");

    let ymdt = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymdt.format(&jiff.datetime()), "令和6年9月11日 8:37:20");

    let ymd =
        DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&jiff.date()), "令和6年9月11日");

    let t = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::T::medium())
        .unwrap();
    assert_writeable_eq!(t.format(&jiff.time()), "8:37:20");

    let e = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::E::long())
        .unwrap();
    assert_writeable_eq!(e.format(&jiff.weekday()), "水曜日");

    let offset = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&jiff.offset()), "GMT+09:00");
}

#[test]
fn jiff_fixed_calendar() {
    use icu::datetime::{fieldsets, FixedCalendarDateTimeFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Europe/Paris").unwrap());

    let ymdto = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(ymdto.format(&jiff), "11 sept. 2024, 01:37:20 UTC+2");

    let ymdtz = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdtz.format(&jiff),
        "11 sept. 2024, 01:37:20 heure d’été d’Europe centrale"
    );

    let ymdt = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymdt.format(&jiff.datetime()), "11 sept. 2024, 01:37:20");

    let ymd = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMD::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymd.format(&jiff.date()), "11 sept. 2024");

    let t = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::T::medium(),
    )
    .unwrap();
    assert_writeable_eq!(t.format(&jiff.time()), "01:37:20");

    let e = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::E::long(),
    )
    .unwrap();
    assert_writeable_eq!(e.format(&jiff.weekday()), "mercredi");

    let offset = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&jiff.offset()), "UTC+02:00");
}

#[test]
fn jiff_no_calendar() {
    use icu::datetime::{fieldsets, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Europe/Zurich").unwrap());

    let t = NoCalendarFormatter::try_new(locale!("de-CH").into(), fieldsets::T::medium()).unwrap();
    assert_writeable_eq!(t.format(&jiff.time()), "01:37:20");

    let offset =
        NoCalendarFormatter::try_new(locale!("de").into(), fieldsets::zone::LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(offset.format(&jiff.offset()), "GMT+02:00");
}
