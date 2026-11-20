// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features in here are a mess

use alloc::boxed::Box;
use icu_calendar::Gregorian;
use icu_datetime::{
    DateTimeFormatter, DateTimeFormatterLoadError, FixedCalendarDateTimeFormatter,
    fieldsets::Combo, fieldsets::builder::BuilderError, fieldsets::enums::*, pattern::*,
    scaffold::*,
};

pub(crate) fn map_or_default<Input, Output>(input: Option<Input>) -> Output
where
    Output: From<Input> + Default,
{
    input.map(Output::from).unwrap_or_default()
}

pub(super) fn date_formatter_with_zone<Zone>(
    formatter: &DateTimeFormatter<DateFieldSet>,
    locale: &crate::unstable::locale_core::ffi::Locale,
    zone: Zone,
    load: impl FnOnce(
        &mut DateTimeNames<Combo<DateFieldSet, Zone>>,
    ) -> Result<(), crate::unstable::errors::ffi::DateTimeFormatterLoadError>,
    to_formatter: impl FnOnce(
        DateTimeNames<Combo<DateFieldSet, Zone>>,
        Combo<DateFieldSet, Zone>,
    ) -> Result<
        DateTimeFormatter<Combo<DateFieldSet, Zone>>,
        (
            DateTimeFormatterLoadError,
            DateTimeNames<Combo<DateFieldSet, Zone>>,
        ),
    >,
) -> Result<
    Box<crate::unstable::zoned_date_formatter::ffi::ZonedDateFormatter>,
    crate::unstable::errors::ffi::DateTimeFormatterLoadError,
>
where
    Zone: DateTimeMarkers + ZoneMarkers,
    <Zone as DateTimeMarkers>::Z: ZoneMarkers,
    Combo<DateFieldSet, Zone>: DateTimeNamesFrom<DateFieldSet>,
    ZonedDateFieldSet: DateTimeNamesFrom<Combo<DateFieldSet, Zone>>,
{
    let prefs = (&locale.0).into();
    let mut names = DateTimeNames::from_formatter(prefs, formatter.clone())
        .cast_into_fset::<Combo<DateFieldSet, Zone>>();
    load(&mut names)?;
    let field_set = formatter
        .to_field_set_builder()
        .build_date()
        .map_err(|e| match e {
            BuilderError::InvalidDateFields => {
                // This can fail if the date fields are for a calendar period
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::InvalidDateFields
            }
            _ => {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::Unknown
            }
        })?
        .with_zone(zone);
    let formatter = to_formatter(names, field_set)
        // This can fail if the locale doesn't match and the fields conflict
        .map_err(|(e, _)| e)?
        .cast_into_fset();
    Ok(Box::new(
        crate::unstable::zoned_date_formatter::ffi::ZonedDateFormatter(formatter),
    ))
}

pub(super) fn datetime_formatter_with_zone<Zone>(
    formatter: &DateTimeFormatter<CompositeDateTimeFieldSet>,
    locale: &crate::unstable::locale_core::ffi::Locale,
    zone: Zone,
    load: impl FnOnce(
        &mut DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
    ) -> Result<(), crate::unstable::errors::ffi::DateTimeFormatterLoadError>,
    to_formatter: impl FnOnce(
        DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
        Combo<DateAndTimeFieldSet, Zone>,
    ) -> Result<
        DateTimeFormatter<Combo<DateAndTimeFieldSet, Zone>>,
        (
            DateTimeFormatterLoadError,
            DateTimeNames<Combo<DateAndTimeFieldSet, Zone>>,
        ),
    >,
) -> Result<
    Box<crate::unstable::zoned_date_time_formatter::ffi::ZonedDateTimeFormatter>,
    crate::unstable::errors::ffi::DateTimeFormatterLoadError,
>
where
    Zone: DateTimeMarkers + ZoneMarkers,
    <Zone as DateTimeMarkers>::Z: ZoneMarkers,
    Combo<DateAndTimeFieldSet, Zone>: DateTimeNamesFrom<CompositeDateTimeFieldSet>,
    ZonedDateAndTimeFieldSet: DateTimeNamesFrom<Combo<DateAndTimeFieldSet, Zone>>,
{
    let prefs = (&locale.0).into();
    let mut names = DateTimeNames::from_formatter(prefs, formatter.clone())
        .cast_into_fset::<Combo<DateAndTimeFieldSet, Zone>>();
    load(&mut names)?;
    let field_set = formatter
        .to_field_set_builder()
        .build_date_and_time()
        .map_err(|e| match e {
            BuilderError::InvalidDateFields => {
                debug_assert!(false, "fields were already validated in DateTimeFormatter");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::InvalidDateFields
            }
            _ => {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::Unknown
            }
        })?
        .with_zone(zone);
    let formatter = to_formatter(names, field_set)
        // This can fail if the locale doesn't match and the fields conflict
        .map_err(|(e, _)| e)?
        .cast_into_fset();
    Ok(Box::new(
        crate::unstable::zoned_date_time_formatter::ffi::ZonedDateTimeFormatter(formatter),
    ))
}

pub(super) fn date_formatter_gregorian_with_zone<Zone>(
    formatter: &FixedCalendarDateTimeFormatter<Gregorian, DateFieldSet>,
    locale: &crate::unstable::locale_core::ffi::Locale,
    zone: Zone,
    load: impl FnOnce(
        &mut FixedCalendarDateTimeNames<Gregorian, Combo<DateFieldSet, Zone>>,
    ) -> Result<(), crate::unstable::errors::ffi::DateTimeFormatterLoadError>,
    to_formatter: impl FnOnce(
        FixedCalendarDateTimeNames<Gregorian, Combo<DateFieldSet, Zone>>,
        Combo<DateFieldSet, Zone>,
    ) -> Result<
        FixedCalendarDateTimeFormatter<Gregorian, Combo<DateFieldSet, Zone>>,
        (
            DateTimeFormatterLoadError,
            FixedCalendarDateTimeNames<Gregorian, Combo<DateFieldSet, Zone>>,
        ),
    >,
) -> Result<
    Box<crate::unstable::zoned_date_formatter::ffi::ZonedDateFormatterGregorian>,
    crate::unstable::errors::ffi::DateTimeFormatterLoadError,
>
where
    Zone: DateTimeMarkers + ZoneMarkers,
    <Zone as DateTimeMarkers>::Z: ZoneMarkers,
    Combo<DateFieldSet, Zone>: DateTimeNamesFrom<DateFieldSet>,
    ZonedDateFieldSet: DateTimeNamesFrom<Combo<DateFieldSet, Zone>>,
{
    let prefs = (&locale.0).into();
    let mut names = FixedCalendarDateTimeNames::from_formatter(prefs, formatter.clone())
        .cast_into_fset::<Combo<DateFieldSet, Zone>>();
    load(&mut names)?;
    let field_set = formatter
        .to_field_set_builder()
        .build_date()
        .map_err(|e| match e {
            BuilderError::InvalidDateFields => {
                // This can fail if the date fields are for a calendar period
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::InvalidDateFields
            }
            _ => {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::Unknown
            }
        })?
        .with_zone(zone);
    let formatter = to_formatter(names, field_set)
        // This can fail if the locale doesn't match and the fields conflict
        .map_err(|(e, _)| e)?
        .cast_into_fset();
    Ok(Box::new(
        crate::unstable::zoned_date_formatter::ffi::ZonedDateFormatterGregorian(formatter),
    ))
}

pub(super) fn datetime_formatter_gregorian_with_zone<Zone>(
    formatter: &FixedCalendarDateTimeFormatter<Gregorian, CompositeDateTimeFieldSet>,
    locale: &crate::unstable::locale_core::ffi::Locale,
    zone: Zone,
    load: impl FnOnce(
        &mut FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
    ) -> Result<(), crate::unstable::errors::ffi::DateTimeFormatterLoadError>,
    to_formatter: impl FnOnce(
        FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
        Combo<DateAndTimeFieldSet, Zone>,
    ) -> Result<
        FixedCalendarDateTimeFormatter<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
        (
            DateTimeFormatterLoadError,
            FixedCalendarDateTimeNames<Gregorian, Combo<DateAndTimeFieldSet, Zone>>,
        ),
    >,
) -> Result<
    Box<crate::unstable::zoned_date_time_formatter::ffi::ZonedDateTimeFormatterGregorian>,
    crate::unstable::errors::ffi::DateTimeFormatterLoadError,
>
where
    Zone: DateTimeMarkers + ZoneMarkers,
    <Zone as DateTimeMarkers>::Z: ZoneMarkers,
    Combo<DateAndTimeFieldSet, Zone>: DateTimeNamesFrom<CompositeDateTimeFieldSet>,
    ZonedDateAndTimeFieldSet: DateTimeNamesFrom<Combo<DateAndTimeFieldSet, Zone>>,
{
    let prefs = (&locale.0).into();
    let mut names = FixedCalendarDateTimeNames::from_formatter(prefs, formatter.clone())
        .cast_into_fset::<Combo<DateAndTimeFieldSet, Zone>>();
    load(&mut names)?;
    let field_set = formatter
        .to_field_set_builder()
        .build_date_and_time()
        .map_err(|e| match e {
            BuilderError::InvalidDateFields => {
                debug_assert!(false, "fields were already validated in DateTimeFormatter");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::InvalidDateFields
            }
            _ => {
                debug_assert!(false, "should be infallible, but got: {e:?}");
                crate::unstable::errors::ffi::DateTimeFormatterLoadError::Unknown
            }
        })?
        .with_zone(zone);
    let formatter = to_formatter(names, field_set)
        // This can fail if the locale doesn't match and the fields conflict
        .map_err(|(e, _)| e)?
        .cast_into_fset();
    Ok(Box::new(
        crate::unstable::zoned_date_time_formatter::ffi::ZonedDateTimeFormatterGregorian(formatter),
    ))
}

#[cfg(all(test, feature = "compiled_data", feature = "unstable"))]
mod tests {
    use crate::unstable::{
        date::ffi::IsoDate,
        date_formatter::ffi::{DateFormatter, DateFormatterGregorian},
        date_time_formatter::ffi::{DateTimeFormatter, DateTimeFormatterGregorian},
        errors::ffi::DateTimeFormatterLoadError,
        iana_parser::ffi::IanaParser,
        locale_core::ffi::Locale,
        time::ffi::Time,
        time_formatter::ffi::TimeFormatter,
        timezone_formatter::ffi::TimeZoneFormatter,
        zoned_date_formatter::ffi::{ZonedDateFormatter, ZonedDateFormatterGregorian},
        zoned_date_time_formatter::ffi::{ZonedDateTimeFormatter, ZonedDateTimeFormatterGregorian},
        zoned_datetime::ffi::ZonedIsoDateTime,
        zoned_time_formatter::ffi::ZonedTimeFormatter,
    };
    use diplomat_runtime::rust_interop::RustWriteVec;

    fn with_write(f: impl FnOnce(&mut diplomat_runtime::DiplomatWrite)) -> alloc::string::String {
        let mut buf = RustWriteVec::with_capacity(0);
        // SAFETY: `buf` is the only `DiplomatWrite` in scope.
        f(unsafe { buf.borrow_mut() });
        alloc::string::String::from_utf8(buf.borrow().as_bytes().to_vec()).unwrap()
    }

    #[test]
    fn test_from_skeleton_formatters() {
        let locale = Locale::from_string(b"en-US").unwrap();
        let iso_date = IsoDate::create(2026, 9, 23).unwrap();
        let time = Time::create(16, 5, 0, 0).unwrap();
        let iana = IanaParser::create();
        let zdt = ZonedIsoDateTime::strict_from_string(
            b"2026-09-23T16:05:00-07:00[America/Los_Angeles]",
            &iana,
        )
        .unwrap();

        // DateFormatter: date skeleton ("yMd") and calendar period skeleton ("yMMM")
        let df = DateFormatter::create_from_skeleton(&locale, b"yMd").unwrap();
        assert_eq!(with_write(|w| df.format_iso(&iso_date, w)), "9/23/2026");

        let df_ym = DateFormatter::create_from_skeleton(&locale, b"yMMM").unwrap();
        assert_eq!(with_write(|w| df_ym.format_iso(&iso_date, w)), "Sep 2026");

        let df_greg = DateFormatterGregorian::create_from_skeleton(&locale, b"y-MM-dd").unwrap();
        assert_eq!(
            with_write(|w| df_greg.format_iso(&iso_date, w)),
            "09/23/2026"
        );

        // TimeFormatter
        let tf = TimeFormatter::create_from_skeleton(&locale, b"jm").unwrap();
        assert_eq!(with_write(|w| tf.format(&time, w)), "4:05\u{202f}PM");

        // DateTimeFormatter & DateTimeFormatterGregorian
        let dtf = DateTimeFormatter::create_from_skeleton(&locale, b"yMdjm").unwrap();
        assert_eq!(
            with_write(|w| dtf.format_iso(&iso_date, &time, w)),
            "9/23/2026, 4:05\u{202f}PM"
        );

        let dtf_greg = DateTimeFormatterGregorian::create_from_skeleton(&locale, b"yMdjm").unwrap();
        assert_eq!(
            with_write(|w| dtf_greg.format_iso(&iso_date, &time, w)),
            "9/23/2026, 4:05\u{202f}PM"
        );

        // TimeZoneFormatter
        let tzf = TimeZoneFormatter::create_from_skeleton(&locale, b"vvvv").unwrap();
        assert_eq!(
            with_write(|w| tzf.format(&zdt.zone, w).unwrap()),
            "Pacific Time"
        );

        // ZonedDateFormatter & ZonedDateFormatterGregorian
        let zdf = ZonedDateFormatter::create_from_skeleton(&locale, b"yMdz").unwrap();
        assert_eq!(
            with_write(|w| zdf.format_iso(&zdt.date, &zdt.zone, w).unwrap()),
            "9/23/2026 PDT"
        );

        let zdf_greg = ZonedDateFormatterGregorian::create_from_skeleton(&locale, b"yMdz").unwrap();
        assert_eq!(
            with_write(|w| zdf_greg.format_iso(&zdt.date, &zdt.zone, w).unwrap()),
            "9/23/2026 PDT"
        );

        // ZonedTimeFormatter
        let ztf = ZonedTimeFormatter::create_from_skeleton(&locale, b"jmz").unwrap();
        assert_eq!(
            with_write(|w| ztf.format(&zdt.time, &zdt.zone, w).unwrap()),
            "4:05\u{202f}PM PDT"
        );

        // ZonedDateTimeFormatter & ZonedDateTimeFormatterGregorian
        let zdtf = ZonedDateTimeFormatter::create_from_skeleton(&locale, b"yMdjmz").unwrap();
        assert_eq!(
            with_write(|w| zdtf.format_iso(&zdt.date, &zdt.time, &zdt.zone, w).unwrap()),
            "9/23/2026, 4:05\u{202f}PM PDT"
        );

        let zdtf_greg =
            ZonedDateTimeFormatterGregorian::create_from_skeleton(&locale, b"yMdjmz").unwrap();
        assert_eq!(
            with_write(|w| zdtf_greg
                .format_iso(&zdt.date, &zdt.time, &zdt.zone, w)
                .unwrap()),
            "9/23/2026, 4:05\u{202f}PM PDT"
        );

        // Category validation errors
        assert_eq!(
            DateFormatter::create_from_skeleton(&locale, b"jm").err(),
            Some(DateTimeFormatterLoadError::InvalidSkeletonForFormatter)
        );
        assert_eq!(
            TimeFormatter::create_from_skeleton(&locale, b"yMd").err(),
            Some(DateTimeFormatterLoadError::InvalidSkeletonForFormatter)
        );
        assert_eq!(
            ZonedDateTimeFormatter::create_from_skeleton(&locale, b"yMdjm").err(),
            Some(DateTimeFormatterLoadError::InvalidSkeletonForFormatter)
        );
        assert_eq!(
            DateFormatter::create_from_skeleton(&locale, b"yMdz").err(),
            Some(DateTimeFormatterLoadError::InvalidSkeletonForFormatter)
        );

        // Invalid UTF-8 bytes are leniently ignored just like other non-field characters
        let df_lenient_utf8 = DateFormatter::create_from_skeleton(&locale, b"y\xffMd").unwrap();
        assert_eq!(
            with_write(|w| df_lenient_utf8.format_iso(&iso_date, w)),
            "9/23/2026"
        );
    }
}
