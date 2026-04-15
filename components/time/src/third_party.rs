// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Support for third-party time libraries.
//!
//! ICU4X provides conversions from types from popular third-party time libraries
//! into [`Time`](crate::Time), [`DateTime`](crate::DateTime), and [`ZonedDateTime`](crate::ZonedDateTime).
//!
//! ✨ *To enable this support, the corresponding Cargo feature must be enabled:*
//!
//! - `unstable_chrono_0_4` for [chrono](https://crates.io/crates/chrono)
//! - `unstable_jiff_0_2` for [jiff](https://crates.io/crates/jiff)
//! - `unstable_time_0_3` for [time](https://crates.io/crates/time)
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. Use with caution.
//! </div>
//!
//! # Chrono
//!
//! The following examples show how to use [`chrono`] types with ICU4X time.
//!
//! ```
//! # #[cfg(feature = "unstable_chrono_0_4")] {
//! use icu::calendar::Gregorian;
//! use icu::time::{Time, DateTime, ZonedDateTime, zone::UtcOffset, zone::TimeZoneInfo, zone::models::AtTime};
//!
//! // Convert a chrono::NaiveTime into an ICU4X Time
//! let chrono_time = chrono::NaiveTime::from_hms_opt(16, 9, 35).unwrap();
//! let icu_time = Time::from(chrono_time);
//! assert_eq!(u8::from(icu_time.hour), 16);
//!
//! // Convert a chrono::NaiveDateTime into an ICU4X DateTime<Gregorian>
//! let chrono_date = chrono::NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
//! let chrono_datetime = chrono_date.and_time(chrono_time);
//! let icu_datetime = DateTime::<Gregorian>::from(chrono_datetime);
//! assert_eq!(icu_datetime.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_datetime.time.hour), 16);
//!
//! // Convert a chrono::FixedOffset into an ICU4X UtcOffset
//! let chrono_offset = chrono::FixedOffset::east_opt(3600).unwrap();
//! let icu_offset = UtcOffset::from(chrono_offset);
//! assert_eq!(icu_offset.to_seconds(), 3600);
//!
//! // Convert a chrono::DateTime<Tz> into an ICU4X ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>>
//! let chrono_zoned = chrono_datetime.and_local_timezone(chrono::Utc).unwrap();
//! let icu_zoned = ZonedDateTime::<Gregorian, TimeZoneInfo<AtTime>>::from(&chrono_zoned);
//! assert_eq!(icu_zoned.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_zoned.time.hour), 16);
//! assert_eq!(icu_zoned.zone.offset().unwrap().to_seconds(), 0);
//! # }
//! ```
//!
//! # Jiff
//!
//! The following examples show how to use [`jiff`] types with ICU4X time.
//!
//! ```
//! # #[cfg(feature = "unstable_jiff_0_2")] {
//! use icu::calendar::Gregorian;
//! use icu::time::{Time, DateTime, ZonedDateTime, zone::UtcOffset, zone::TimeZoneInfo, zone::models::AtTime};
//!
//! // Convert a jiff::civil::Time into an ICU4X Time
//! let jiff_time = jiff::civil::time(16, 9, 35, 0);
//! let icu_time = Time::from(jiff_time);
//! assert_eq!(u8::from(icu_time.hour), 16);
//!
//! // Convert a jiff::civil::DateTime into an ICU4X DateTime<Gregorian>
//! let jiff_date = jiff::civil::date(2025, 1, 15);
//! let jiff_datetime = jiff_date.at(16, 9, 35, 0);
//! let icu_datetime = DateTime::<Gregorian>::from(jiff_datetime);
//! assert_eq!(icu_datetime.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_datetime.time.hour), 16);
//!
//! // Convert a jiff::tz::Offset into an ICU4X UtcOffset
//! let jiff_offset = jiff::tz::offset(1);
//! let icu_offset = UtcOffset::from(jiff_offset);
//! assert_eq!(icu_offset.to_seconds(), 3600);
//!
//! // Convert a jiff::Zoned into an ICU4X ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>>
//! # #[cfg(feature = "compiled_data")] {
//! let jiff_zoned = jiff_datetime.to_zoned(jiff::tz::TimeZone::UTC).unwrap();
//! let icu_zoned = ZonedDateTime::<Gregorian, TimeZoneInfo<AtTime>>::from(&jiff_zoned);
//! assert_eq!(icu_zoned.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_zoned.time.hour), 16);
//! assert_eq!(icu_zoned.zone.offset().unwrap().to_seconds(), 0);
//! # }
//! # }
//! ```
//!
//! # Time
//!
//! The following examples show how to use [`time`] types with ICU4X time.
//!
//! ```
//! # #[cfg(feature = "unstable_time_0_3")] {
//! use icu::calendar::Gregorian;
//! use icu::time::{Time, DateTime, ZonedDateTime, zone::UtcOffset};
//!
//! // Convert a time::Time into an ICU4X Time
//! let time_time = time::Time::from_hms(16, 9, 35).unwrap();
//! let icu_time = Time::from(time_time);
//! assert_eq!(u8::from(icu_time.hour), 16);
//!
//! // Convert a time::PrimitiveDateTime into an ICU4X DateTime<Gregorian>
//! let time_date = time::Date::from_calendar_date(2025, time::Month::January, 15).unwrap();
//! let time_datetime = time::PrimitiveDateTime::new(time_date, time_time);
//! let icu_datetime = DateTime::<Gregorian>::from(time_datetime);
//! assert_eq!(icu_datetime.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_datetime.time.hour), 16);
//!
//! // Convert a time::UtcOffset into an ICU4X UtcOffset
//! let time_offset = time::UtcOffset::from_hms(1, 0, 0).unwrap();
//! let icu_offset = UtcOffset::from(time_offset);
//! assert_eq!(icu_offset.to_seconds(), 3600);
//!
//! // Convert a time::OffsetDateTime into an ICU4X ZonedDateTime<Gregorian, UtcOffset>
//! let time_offset_dt = time_datetime.assume_utc();
//! let icu_zoned = ZonedDateTime::<Gregorian, UtcOffset>::from(&time_offset_dt);
//! assert_eq!(icu_zoned.date.era_year().year, 2025);
//! assert_eq!(u8::from(icu_zoned.time.hour), 16);
//! assert_eq!(icu_zoned.zone.to_seconds(), 0);
//! # }
//! ```
