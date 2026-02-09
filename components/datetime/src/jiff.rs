// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::scaffold::{GetField, InFixedCalendar, UnstableSealed};
use icu_calendar::types::{DayOfMonth, DayOfYear, MonthInfo, RataDie, Weekday, YearInfo};
use icu_calendar::{Date, Gregorian};
use icu_time::zone::{UtcOffset, ZoneNameTimestamp};
#[cfg(feature = "compiled_data")]
use icu_time::TimeZone;
use icu_time::{Hour, Minute, Nanosecond, Second, ZonedDateTime};

impl UnstableSealed for jiff::civil::Time {}
impl InFixedCalendar<()> for jiff::civil::Time {}
impl GetField<()> for jiff::civil::Time {
    fn get_field(&self) {}
}

impl GetField<Hour> for jiff::civil::Time {
    fn get_field(&self) -> Hour {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Hour::try_from(self.hour() as u8).unwrap()
    }
}

impl GetField<Minute> for jiff::civil::Time {
    fn get_field(&self) -> Minute {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Minute::try_from(self.minute() as u8).unwrap()
    }
}

impl GetField<Second> for jiff::civil::Time {
    fn get_field(&self) -> Second {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Second::try_from(self.second() as u8).unwrap()
    }
}

impl GetField<Nanosecond> for jiff::civil::Time {
    fn get_field(&self) -> Nanosecond {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Nanosecond::try_from(
            self.millisecond() as u32 * 1_000_000
                + self.microsecond() as u32 * 1_000
                + self.nanosecond() as u32,
        )
        .unwrap()
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
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Date::try_new_gregorian(self.year() as i32, self.month() as u8, self.day() as u8)
            .unwrap()
            .year()
    }
}

impl GetField<MonthInfo> for jiff::civil::Date {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Date::try_new_gregorian(self.year() as i32, self.month() as u8, self.day() as u8)
            .unwrap()
            .month()
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
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Date::try_new_gregorian(self.year() as i32, self.month() as u8, self.day() as u8)
            .unwrap()
            .to_rata_die()
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
        Some(UtcOffset::from_seconds_unchecked(self.seconds()))
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
        icu_time::zone::IanaParser::new().parse(self.time_zone().iana_name().unwrap_or_default())
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

#[test]
fn jiff() {
    use icu::datetime::{fieldsets, FixedCalendarDateTimeFormatter, NoCalendarFormatter};
    use icu::locale::locale;
    use writeable::assert_writeable_eq;

    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Asia/Tokyo").unwrap());

    let ymdt = FixedCalendarDateTimeFormatter::try_new(
        locale!("en-GB").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();
    assert_writeable_eq!(
        ymdt.format(&jiff),
        "11 Sept 2024, 08:37:20 Japan Standard Time"
    );

    let ymd =
        FixedCalendarDateTimeFormatter::try_new(locale!("en-GB").into(), fieldsets::YMD::medium())
            .unwrap();
    assert_writeable_eq!(ymd.format(&jiff.date()), "11 Sept 2024");

    let t = NoCalendarFormatter::try_new(locale!("en-GB").into(), fieldsets::T::medium()).unwrap();
    assert_writeable_eq!(t.format(&jiff.time()), "08:37:20");

    let offset = NoCalendarFormatter::try_new(
        locale!("en-GB").into(),
        fieldsets::zone::LocalizedOffsetLong,
    )
    .unwrap();
    assert_writeable_eq!(offset.format(&jiff.offset()), "GMT+09:00");
}
