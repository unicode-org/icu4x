// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for resolving and manipulating time zones.
//!
//! # Fields
//!
//! In ICU4X, a [formattable time zone](TimeZoneInfo) consists of four different fields:
//!
//! 1. The time zone ID
//! 2. The offset from UTC
//! 3. The zone variant, representing concepts such as Standard, Summer, Daylight, and Ramadan time
//! 4. A timestamp, as time zone names can change over time
//!
//! ## Time Zone
//!
//! The time zone ID corresponds to a time zone from the time zone database. The time zone ID
//! usually corresponds to the largest city in the time zone.
//!
//! There are two mostly-interchangeable standards for time zone IDs:
//!
//! 1. IANA time zone IDs, like `"America/Chicago"`
//! 2. BCP-47 time zone IDs, like `"uschi"`
//!
//! ICU4X uses BCP-47 time zone IDs for all of its APIs. To get a BCP-47 time zone from an
//! IANA time zone, use [`TimeZoneIdMapper`].
//!
//! ## UTC Offset
//!
//! The UTC offset precisely states the time difference between the time zone in question and
//! Coordinated Universal Time (UTC).
//!
//! In localized strings, it is often rendered as "UTC-6", meaning 6 hours less than UTC (some locales
//! use the term "GMT" instead of "UTC").
//!
//! ## Zone Variant
//!
//! Many metazones use different names and offsets in the summer than in the winter. In ICU4X,
//! this is called the _zone variant_.
//!
//! CLDR has two zone variants, named `"standard"` and `"daylight"`. However, the mapping of these
//! variants to specific observed offsets varies from time zone to time zone, and they may not
//! consistently represent winter versus summer time.
//!
//! Note: It is optional (not required) to set the zone variant when constructing a
//! [`TimeZoneInfo`]. Therefore, the list of possible variants does not include a generic variant
//! to represent the lack of a preference.
//! 
//! # Examples
//!
//! Create a time zone for which the offset and time zone ID are already known, and calculate
//! the metazone based on a certain local datetime:
//!
//! ```
//! use icu::calendar::DateTime;
//! use icu::timezone::TimeZoneInfo;
//! use icu::timezone::UtcOffset;
//! use icu::timezone::MetazoneCalculator;
//! use icu::timezone::TimeZoneBcp47Id;
//! use icu::timezone::TimeZoneIdMapper;
//! use tinystr::{tinystr, TinyAsciiStr};
//!
//! let mapper = TimeZoneIdMapper::new();
//! // Create a time zone for America/Chicago at UTC-6:
//! let mut time_zone = TimeZoneInfo {
//!     time_zone_id: mapper.as_borrowed().iana_to_bcp47("America/Chicago"),
//!     offset: Some("-0600".parse().unwrap()),
//!     ..TimeZoneInfo::unknown()
//! };
//!
//! // Alternatively, set it directly from the BCP-47 ID
//! assert_eq!(time_zone.time_zone_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
//! ```

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod error;
mod ids;
pub mod provider;
mod time_zone;
mod types;
mod windows_tz;
mod zoned_datetime;

#[cfg(all(feature = "ixdtf", feature = "compiled_data"))]
mod ixdtf;

pub use error::InvalidOffsetError;
pub use ids::{
    TimeZoneIdMapper, TimeZoneIdMapperBorrowed, TimeZoneIdMapperWithFastCanonicalization,
    TimeZoneIdMapperWithFastCanonicalizationBorrowed,
};
pub use provider::TimeZoneBcp47Id;
pub use time_zone::TimeZoneInfo;
pub use types::{UtcOffset, ZoneVariant};
pub use windows_tz::{WindowsTimeZoneMapper, WindowsTimeZoneMapperBorrowed};
pub use zoned_datetime::CustomZonedDateTime;

#[cfg(all(feature = "ixdtf", feature = "compiled_data"))]
pub use crate::ixdtf::ParseError;
