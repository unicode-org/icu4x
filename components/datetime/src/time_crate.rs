// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{ConvertCalendar, GetField, InFixedCalendar, InSameCalendar, UnstableSealed};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{AnyCalendar, Date, Gregorian};
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
use icu_time::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};

impl UnstableSealed for time::Time {}
impl<C> InFixedCalendar<C> for time::Time {}
impl GetField<()> for time::Time {
    fn get_field(&self) {}
}

impl GetField<Hour> for time::Time {
    fn get_field(&self) -> Hour {
        Time::from(*self).hour
    }
}

impl GetField<Minute> for time::Time {
    fn get_field(&self) -> Minute {
        Time::from(*self).minute
    }
}

impl GetField<Second> for time::Time {
    fn get_field(&self) -> Second {
        Time::from(*self).second
    }
}

impl GetField<Nanosecond> for time::Time {
    fn get_field(&self) -> Nanosecond {
        Time::from(*self).subsecond
    }
}

impl UnstableSealed for time::Date {}
impl InFixedCalendar<Gregorian> for time::Date {}
impl GetField<()> for time::Date {
    fn get_field(&self) {}
}

impl GetField<YearInfo> for time::Date {
    #[inline]
    fn get_field(&self) -> YearInfo {
        Date::from(*self).year()
    }
}

impl GetField<MonthInfo> for time::Date {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        Date::from(*self).month()
    }
}

impl GetField<DayOfMonth> for time::Date {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        DayOfMonth(self.day())
    }
}

impl GetField<Weekday> for time::Date {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.weekday().get_field()
    }
}

impl GetField<DayOfYear> for time::Date {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        DayOfYear(self.ordinal())
    }
}

impl GetField<RataDie> for time::Date {
    #[inline]
    fn get_field(&self) -> RataDie {
        Date::from(*self).to_rata_die()
    }
}

impl UnstableSealed for time::UtcOffset {}
impl<C> InFixedCalendar<C> for time::UtcOffset {}
impl GetField<()> for time::UtcOffset {
    fn get_field(&self) {}
}

impl GetField<Option<UtcOffset>> for time::UtcOffset {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        Some((*self).into())
    }
}

impl UnstableSealed for time::PrimitiveDateTime {}
impl InFixedCalendar<Gregorian> for time::PrimitiveDateTime {}
impl GetField<()> for time::PrimitiveDateTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for time::PrimitiveDateTime {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl GetField<Minute> for time::PrimitiveDateTime {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl GetField<Second> for time::PrimitiveDateTime {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl GetField<Nanosecond> for time::PrimitiveDateTime {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl GetField<YearInfo> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date().get_field()
    }
}

impl GetField<MonthInfo> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date().get_field()
    }
}

impl GetField<DayOfMonth> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date().get_field()
    }
}

impl GetField<Weekday> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date().get_field()
    }
}

impl GetField<DayOfYear> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date().get_field()
    }
}

impl GetField<RataDie> for time::PrimitiveDateTime {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date().get_field()
    }
}

impl UnstableSealed for time::Weekday {}
impl<C> InFixedCalendar<C> for time::Weekday {}
impl GetField<()> for time::Weekday {
    fn get_field(&self) {}
}

impl GetField<Weekday> for time::Weekday {
    fn get_field(&self) -> Weekday {
        (*self).into()
    }
}

impl UnstableSealed for time::OffsetDateTime {}
impl InFixedCalendar<Gregorian> for time::OffsetDateTime {}
impl GetField<()> for time::OffsetDateTime {
    fn get_field(&self) {}
}

impl GetField<Hour> for time::OffsetDateTime {
    fn get_field(&self) -> Hour {
        self.time().get_field()
    }
}

impl GetField<Minute> for time::OffsetDateTime {
    fn get_field(&self) -> Minute {
        self.time().get_field()
    }
}

impl GetField<Second> for time::OffsetDateTime {
    fn get_field(&self) -> Second {
        self.time().get_field()
    }
}

impl GetField<Nanosecond> for time::OffsetDateTime {
    fn get_field(&self) -> Nanosecond {
        self.time().get_field()
    }
}

impl GetField<YearInfo> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date().get_field()
    }
}

impl GetField<MonthInfo> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date().get_field()
    }
}

impl GetField<DayOfMonth> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date().get_field()
    }
}

impl GetField<Weekday> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> Weekday {
        self.date().get_field()
    }
}

impl GetField<DayOfYear> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> DayOfYear {
        self.date().get_field()
    }
}

impl GetField<RataDie> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> RataDie {
        self.date().get_field()
    }
}

impl GetField<Option<UtcOffset>> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset().get_field()
    }
}

impl GetField<ZoneNameTimestamp> for time::OffsetDateTime {
    #[inline]
    fn get_field(&self) -> ZoneNameTimestamp {
        ZoneNameTimestamp::from_zoned_date_time_iso(
            ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                self.unix_timestamp(),
                UtcOffset::zero(),
            ),
        )
    }
}

impl ConvertCalendar for time::Time {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl ConvertCalendar for time::Weekday {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl ConvertCalendar for time::Date {
    type Converted<'a> = <Date<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&Date::from(*self), calendar)
    }
}

impl ConvertCalendar for time::PrimitiveDateTime {
    type Converted<'a> = <DateTime<Gregorian> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&DateTime::from(*self), calendar)
    }
}

impl ConvertCalendar for time::OffsetDateTime {
    type Converted<'a> = <ZonedDateTime<Gregorian, UtcOffset> as ConvertCalendar>::Converted<'a>;

    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ConvertCalendar::to_calendar(&ZonedDateTime::from(self), calendar)
    }
}

impl ConvertCalendar for time::UtcOffset {
    type Converted<'a> = Self;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl InSameCalendar for time::Time {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for time::Date {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for time::PrimitiveDateTime {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for time::Weekday {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for time::OffsetDateTime {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

impl InSameCalendar for time::UtcOffset {
    fn check_any_calendar_kind(
        &self,
        _: icu_calendar::AnyCalendarKind,
    ) -> Result<(), crate::MismatchedCalendarError> {
        Ok(())
    }
}

#[test]
fn time() {
    use crate::{fieldsets, DateTimeFormatter};
    use icu_locale::locale;
    use writeable::assert_writeable_eq;

    let time = time::OffsetDateTime::from_unix_timestamp_nanos(1726011440123456789)
        .unwrap()
        .to_offset(time::UtcOffset::from_hms(9, 0, 0).unwrap());

    let ymdto = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(ymdto.format(&time), "令和6年9月11日 8:37:20 GMT+9");

    let ymdt = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdt.format(&time::PrimitiveDateTime::new(time.date(), time.time())),
        "令和6年9月11日 8:37:20"
    );

    let ymd =
        DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&time.date()), "令和6年9月11日");

    let t = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::T::medium())
        .unwrap();
    assert_writeable_eq!(t.format(&time.time()), "8:37:20");

    let e = DateTimeFormatter::try_new(locale!("ja-u-ca-japanese").into(), fieldsets::E::long())
        .unwrap();
    assert_writeable_eq!(e.format(&time.weekday()), "水曜日");

    let offset = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&time.offset()), "GMT+09:00");
}

#[test]
fn time_fixed_calendar() {
    use crate::{fieldsets, FixedCalendarDateTimeFormatter};
    use icu_locale::locale;
    use writeable::assert_writeable_eq;

    let time = time::OffsetDateTime::from_unix_timestamp_nanos(1726011440123456789)
        .unwrap()
        .to_offset(time::UtcOffset::from_hms(2, 0, 0).unwrap());

    let ymdto = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
    )
    .unwrap();
    assert_writeable_eq!(ymdto.format(&time), "11 sept. 2024, 01:37:20 UTC+2");

    let ymdt = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium(),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdt.format(&time::PrimitiveDateTime::new(time.date(), time.time())),
        "11 sept. 2024, 01:37:20"
    );

    let ymd = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::YMD::medium(),
    )
    .unwrap();
    assert_writeable_eq!(ymd.format(&time.date()), "11 sept. 2024");

    let t = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::T::medium(),
    )
    .unwrap();
    assert_writeable_eq!(t.format(&time.time()), "01:37:20");

    let e = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::E::long(),
    )
    .unwrap();
    assert_writeable_eq!(e.format(&time.weekday()), "mercredi");

    let offset = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
        locale!("fr").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&time.offset()), "UTC+02:00");
}

#[test]
fn time_no_calendar() {
    use crate::{fieldsets, NoCalendarFormatter};
    use icu_locale::locale;
    use writeable::assert_writeable_eq;

    let time = time::OffsetDateTime::from_unix_timestamp_nanos(1726011440123456789)
        .unwrap()
        .to_offset(time::UtcOffset::from_hms(2, 0, 0).unwrap());

    let t = NoCalendarFormatter::try_new(locale!("de-CH").into(), fieldsets::T::medium()).unwrap();
    assert_writeable_eq!(t.format(&time.time()), "01:37:20");

    let offset =
        NoCalendarFormatter::try_new(locale!("de").into(), fieldsets::zone::LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(offset.format(&time.offset()), "GMT+02:00");
}
