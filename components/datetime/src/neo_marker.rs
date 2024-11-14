// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo formatter markers.
//!
//! # Examples
//!
//! ## Alignment
//!
//! By default, datetimes are formatted for a variable-width context. You can
//! give a hint that the strings will be displayed in a column-like context,
//! which will coerce numerics to be padded with zeros.
//!
//! ```
//! use icu::calendar::Date;
//! use icu::calendar::Gregorian;
//! use icu::datetime::fieldset::YMD;
//! use icu::datetime::neo_skeleton::Alignment;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let plain_formatter =
//!     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!         &locale!("en-US").into(),
//!         YMD::short(),
//!     )
//!     .unwrap();
//!
//! let column_formatter =
//!     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!         &locale!("en-US").into(),
//!         YMD::short().with_alignment(Alignment::Column),
//!     )
//!     .unwrap();
//!
//! // By default, en-US does not pad the month and day with zeros.
//! assert_try_writeable_eq!(
//!     plain_formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/25"
//! );
//!
//! // The column alignment option hints that they should be padded.
//! assert_try_writeable_eq!(
//!     column_formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "01/01/25"
//! );
//! ```
//!
//! ## Year Style
//!
//! The precision of the rendered year can be adjusted using the [`YearStyle`] option.
//!
//! ```
//! use icu::calendar::Date;
//! use icu::calendar::Gregorian;
//! use icu::datetime::fieldset::YMD;
//! use icu::datetime::neo_skeleton::YearStyle;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     YMD::short().with_year_style(YearStyle::Auto),
//! )
//! .unwrap();
//!
//! // Era displayed when needed for disambiguation,
//! // such as years before year 0 and small year numbers:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // Era elided for modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900"
//! );
//! // Era and century both elided for nearby years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/25"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     YMD::short().with_year_style(YearStyle::Full),
//! )
//! .unwrap();
//!
//! // Era still displayed in cases with ambiguity:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // Era elided for modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900"
//! );
//! // But now we always get a full-precision year:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/2025"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     YMD::short().with_year_style(YearStyle::Always),
//! )
//! .unwrap();
//!
//! // Era still displayed in cases with ambiguity:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // But now it is shown even on modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900 AD"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/2025 AD"
//! );
//! ```
//!
//! ## Hour Cycle
//!
//! Hours can be switched between 12-hour and 24-hour time via the `u-hc` locale keyword.
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::fieldset::T;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! // By default, en-US uses 12-hour time and fr-FR uses 24-hour time,
//! // but we can set overrides.
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US-u-hc-h12").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US-u-hc-h23").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "16:12"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("fr-FR-u-hc-h12").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("fr-FR-u-hc-h23").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "16:12"
//! );
//! ```
//!
//! Hour cycles `h11` and `h24` are supported, too:
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::fieldset::T;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("und-u-hc-h11").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
//!     "0:00 AM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("und-u-hc-h24").into(),
//!     T::short().hm(),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
//!     "24:00"
//! );
//! ```
//!
//! ## Time Precision
//!
//! The time can be displayed with hour, minute, or second precision, and
//! zero-valued fields can be automatically hidden.
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::fieldset::T;
//! use icu::datetime::neo_skeleton::FractionalSecondDigits;
//! use icu::datetime::neo_skeleton::TimePrecision;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatters = [
//!     TimePrecision::HourPlus,
//!     TimePrecision::HourExact,
//!     TimePrecision::MinutePlus,
//!     TimePrecision::MinuteExact,
//!     TimePrecision::SecondPlus,
//!     TimePrecision::SecondExact(FractionalSecondDigits::F0),
//! ].map(|time_precision| {
//!     FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!         &locale!("en-US").into(),
//!         T::short().with_time_precision(time_precision),
//!     )
//!     .unwrap()
//! });
//!
//! let times = [
//!     Time::try_new(7, 0, 0, 0).unwrap(),
//!     Time::try_new(7, 0, 10, 0).unwrap(),
//!     Time::try_new(7, 12, 20, 5).unwrap(),
//! ];
//!
//! // TODO(#5782): the Plus variants should render fractional digits
//! let expected_value_table = [
//!     // 7:00:00, 7:00:10, 7:12:20.5432
//!     ["7 AM", "7:00:10 AM", "7:12:20 AM"], // HourPlus
//!     ["7 AM", "7 AM", "7 AM"], // HourExact
//!     ["7:00 AM", "7:00:10 AM", "7:12:20 AM"], // MinutePlus
//!     ["7:00 AM", "7:00 AM", "7:12 AM"], // MinuteExact
//!     ["7:00:00 AM", "7:00:10 AM", "7:12:20 AM"], // SecondPlus
//!     ["7:00:00 AM", "7:00:10 AM", "7:12:20 AM"], // SecondExact
//! ];
//!
//! for (expected_value_row, formatter) in expected_value_table.iter().zip(formatters.iter()) {
//!     for (expected_value, time) in expected_value_row.iter().zip(times.iter()) {
//!         assert_try_writeable_eq!(
//!             formatter.format(time),
//!             *expected_value,
//!             Ok(()),
//!             "{formatter:?} @ {time:?}"
//!         );
//!     }
//! }
//! ```
//!
//! ## Fractional Second Digits
//!
//! Times can be displayed with a custom number of fractional digits from 0-9:
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::calendar::Time;
//! use icu::datetime::fieldset::T;
//! use icu::datetime::neo_skeleton::FractionalSecondDigits;
//! use icu::datetime::neo_skeleton::TimePrecision;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US").into(),
//!     T::short().with_time_precision(TimePrecision::SecondExact(FractionalSecondDigits::F2)),
//! )
//! .unwrap();
//!
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 543200000).unwrap()),
//!     "4:12:20.54 PM"
//! );
//! ```
//!
//! ## Time Zone Formatting
//!
//! Here, we configure a [`DateTimeFormatter`] to format with generic non-location short,
//! which falls back to the offset when unavailable (see [`V`]).
//!
//! ```
//! use icu::calendar::{Date, Time};
//! use icu::timezone::{TimeZoneInfo, UtcOffset, TimeZoneIdMapper, TimeZoneBcp47Id};
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::V;
//! use icu::datetime::DateTimeWriteError;
//! use icu::locale::locale;
//! use tinystr::tinystr;
//! use writeable::assert_try_writeable_eq;
//!
//! // Set up the formatter
//! let mut tzf = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en").into(),
//!     V::short(),
//! )
//! .unwrap();
//!
//! // "uschi" - has symbol data for short generic non-location
//! let time_zone = TimeZoneIdMapper::new()
//!     .iana_to_bcp47("America/Chicago")
//!     .with_offset("-05".parse().ok())
//!     .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "CT"
//! );
//!
//! // "ushnl" - has time zone override symbol data for short generic non-location
//! let time_zone = TimeZoneIdMapper::new()
//!     .iana_to_bcp47("Pacific/Honolulu")
//!     .with_offset("-10".parse().ok())
//!     .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "HST"
//! );
//!
//! // Mis-spelling of "America/Chicago" results in a fallback to offset format
//! let time_zone = TimeZoneIdMapper::new()
//!     .iana_to_bcp47("America/Chigagou")
//!     .with_offset("-05".parse().ok())
//!     .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "GMT-5"
//! );
//! ```

#[cfg(doc)]
use crate::{fieldset::*, DateTimeFormatter};
