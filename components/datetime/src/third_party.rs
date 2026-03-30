// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Support for third-party datetime libraries.
//!
//! ICU4X provides traits to allow using types from popular third-party datetime libraries
//! directly with ICU4X formatters like [`DateTimeFormatter`](crate::DateTimeFormatter).
//!
//! To enable this support, the corresponding feature must be enabled:
//! - `chrono_0_4` for [chrono](https://crates.io/crates/chrono)
//! - `jiff_0_2` for [jiff](https://crates.io/crates/jiff)
//! - `time_0_3` for [time](https://crates.io/crates/time)
//!
//! # Chrono
//!
//! The following examples show how to use [`chrono`] types with ICU4X formatters.
//!
//! ```
//! # #[cfg(feature = "chrono_0_4")] {
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//! use icu::locale::locale;
//! use writeable::assert_writeable_eq;
//!
//! // Format a chrono::NaiveDate using fieldsets::YMD
//! let chrono_date = chrono::NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
//! let dtf_date = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMD::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_date.format(&chrono_date), "Jan 15, 2025");
//!
//! // Format a chrono::NaiveTime using fieldsets::T
//! let chrono_time = chrono::NaiveTime::from_hms_opt(16, 9, 35).unwrap();
//! let dtf_time = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::T::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_time.format(&chrono_time), "4:09:35 PM");
//!
//! // Format a chrono::NaiveDateTime using fieldsets::YMDT
//! let chrono_datetime = chrono_date.and_time(chrono_time);
//! let dtf_datetime = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_datetime.format(&chrono_datetime), "Jan 15, 2025, 4:09:35 PM");
//!
//! // Format a chrono::DateTime using fieldsets::YMDT with a zone style
//! let chrono_zoned = chrono_datetime.and_local_timezone(chrono::Utc).unwrap();
//! let dtf_zoned = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_zoned.format(&chrono_zoned), "Jan 15, 2025, 4:09:35 PM GMT+0");
//!
//! // Format a chrono::Weekday using fieldsets::E
//! let chrono_weekday = chrono::Weekday::Wed;
//! let dtf_weekday = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::E::long(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_weekday.format(&chrono_weekday), "Wednesday");
//! # }
//! ```
//!
//! # Jiff
//!
//! The following examples show how to use [`jiff`] types with ICU4X formatters.
//!
//! ```
//! # #[cfg(feature = "jiff_0_2")] {
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//! use icu::locale::locale;
//! use writeable::assert_writeable_eq;
//!
//! // Format a jiff::civil::Date using fieldsets::YMD
//! let jiff_date = jiff::civil::date(2025, 1, 15);
//! let dtf_date = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMD::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_date.format(&jiff_date), "Jan 15, 2025");
//!
//! // Format a jiff::civil::Time using fieldsets::T
//! let jiff_time = jiff::civil::time(16, 9, 35, 0);
//! let dtf_time = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::T::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_time.format(&jiff_time), "4:09:35 PM");
//!
//! // Format a jiff::civil::DateTime using fieldsets::YMDT
//! let jiff_datetime = jiff_date.at(16, 9, 35, 0);
//! let dtf_datetime = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_datetime.format(&jiff_datetime), "Jan 15, 2025, 4:09:35 PM");
//!
//! // Format a jiff::Zoned using fieldsets::YMDT with a zone style
//! let jiff_zoned = jiff_datetime.in_tz("UTC").unwrap();
//! let dtf_zoned = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_zoned.format(&jiff_zoned), "Jan 15, 2025, 4:09:35 PM GMT+0");
//!
//! // Format a jiff::civil::Weekday using fieldsets::E
//! let jiff_weekday = jiff::civil::Weekday::Wednesday;
//! let dtf_weekday = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::E::long(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_weekday.format(&jiff_weekday), "Wednesday");
//! # }
//! ```
//!
//! # Time
//!
//! The following examples show how to use [`time`] types with ICU4X formatters.
//!
//! ```
//! # #[cfg(feature = "time_0_3")] {
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//! use icu::locale::locale;
//! use writeable::assert_writeable_eq;
//!
//! // Format a time::Date using fieldsets::YMD
//! let time_date = time::Date::from_calendar_date(2025, time::Month::January, 15).unwrap();
//! let dtf_date = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMD::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_date.format(&time_date), "Jan 15, 2025");
//!
//! // Format a time::Time using fieldsets::T
//! let time_time = time::Time::from_hms(16, 9, 35).unwrap();
//! let dtf_time = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::T::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_time.format(&time_time), "4:09:35 PM");
//!
//! // Format a time::PrimitiveDateTime using fieldsets::YMDT
//! let time_datetime = time::PrimitiveDateTime::new(time_date, time_time);
//! let dtf_datetime = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_datetime.format(&time_datetime), "Jan 15, 2025, 4:09:35 PM");
//!
//! // Format a time::OffsetDateTime using fieldsets::YMDT with a zone style
//! let time_offset = time_datetime.assume_utc();
//! let dtf_zoned = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_zoned.format(&time_offset), "Jan 15, 2025, 4:09:35 PM GMT+0");
//!
//! // Format a time::Weekday using fieldsets::E
//! let time_weekday = time::Weekday::Wednesday;
//! let dtf_weekday = DateTimeFormatter::try_new(
//!     locale!("en-US").into(),
//!     fieldsets::E::long(),
//! )
//! .unwrap();
//! assert_writeable_eq!(dtf_weekday.format(&time_weekday), "Wednesday");
//! # }
//! ```
