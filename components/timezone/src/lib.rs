// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for resolving and manipulating time zones.
//!
//! # `TimeZone`
//!
//! In ICU4X, a [`TimeZone`] consists of two optional fields:
//!
//! 1. The offset from UTC
//! 2. The BCP-47 ID
//!
//! ## UTC Offset
//!
//! The UTC offset precisely states the time difference between the time zone in question and
//! Coordinated Universal Time (UTC).
//!
//! In localized strings, it is often rendered as "UTC-6", meaning 6 hours less than UTC (some locales
//! use the term "GMT" instead of "UTC").
//!
//! ## ID
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
//! # `ResolvedTimeZone`
//!
//! A [`ResolvedTimeZone`] represents a [`TimeZone`] interpreted at a specific local time, and
//! extended with two more formatting-only fields:
//!
//! 1. The metazone ID
//! 2. The zone variant, representing concepts such as Standard, Summer, Daylight, and Ramadan time
//!
//! A [`ResolvedTimeZone`] is constructed using [`TimeZone::resolve_at`].
//!
//! ## Metazone
//!
//! A metazone is a collection of multiple time zones that share the same localized formatting
//! at a particular date and time.
//!
//! For example, "America/Chicago" and "America/Indiana/Knox" both map to US Central Time, or
//! `"America_Central"`.
//!
//! The mapping from time zone to metazone depends on the date. For example, from 1991 to 2006,
//! "America/Indiana/Knox" mapped to US Eastern Time instead of US Central Time.
//!
//! As with time zone IDs, there are two interchangeable forms:
//!
//! 1. Long form, like `"America_Central"`
//! 2. Short form compatible with BCP-47, like `"amce"`
//!
//! ICU4X uses the short form.
//!
//! Note: in ICU4X, "metazone" is one word and "time zone" is two words, except for this crate
//! and module name, where "timezone" is used with no separators. See
//! <https://github.com/unicode-org/icu4x/issues/2507>.
//!
//! ## Zone Variant
//!
//! Many zones use different names and offsets in the summer than in the winter. In ICU4X,
//! this is called the _zone variant_.
//!
//! CLDR has two zone variants, named `"standard"` and `"daylight"`. However, the mapping of these
//! variants to specific observed offsets varies from time zone to time zone, and they may not
//! consistently represent winter versus summer time.
//!
//! # Examples
//!
//! Create a time zone for which the offset and time zone ID are already known, and create a
//! formattable time zone for a certain local datetime:
//!
//! ```
//! use icu::calendar::DateTime;
//! use icu::timezone::TimeZone;
//! use icu::timezone::UtcOffset;
//! use icu::timezone::MetazoneId;
//! use icu::timezone::MetazoneCalculator;
//! use icu::timezone::TimeZoneBcp47Id;
//! use icu::timezone::TimeZoneIdMapper;
//! use icu::timezone::ZoneOffsetCalculator;
//! use icu::timezone::ZoneVariant;
//! use tinystr::tinystr;
//!
//! // Create a time zone for America/Chicago at UTC-6:
//! let bcp47_id = TimeZoneIdMapper::new().as_borrowed().iana_to_bcp47("America/Chicago").unwrap();
//! let time_zone = TimeZone::new("-0600".parse().unwrap(), bcp47_id);
//!
//! // Create a `ResolvedTimeZone` at January 1, 2022:
//! let formattable = time_zone.resolve_at(&DateTime::try_new_iso_datetime(2022, 1, 1, 0, 0, 0).unwrap());
//!
//! assert_eq!(formattable.metazone_id().unwrap(), MetazoneId(tinystr!(4, "amce")));
//! assert_eq!(formattable.zone_variant().unwrap(), ZoneVariant::standard());
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
mod metazone;
pub mod provider;
mod time_zone;
mod types;
mod windows_tz;
mod zone_offset;
mod zoned_datetime;

#[cfg(all(feature = "ixdtf", feature = "compiled_data"))]
mod ixdtf;

pub use error::{InvalidOffsetError, UnknownTimeZoneError};
pub use ids::{
    TimeZoneIdMapper, TimeZoneIdMapperBorrowed, TimeZoneIdMapperWithFastCanonicalization,
    TimeZoneIdMapperWithFastCanonicalizationBorrowed,
};
pub use metazone::MetazoneCalculator;
pub use provider::{MetazoneId, TimeZoneBcp47Id};
pub use time_zone::{ResolvedTimeZone, TimeZone};
pub use types::{UtcOffset, ZoneVariant};
pub use windows_tz::{WindowsTimeZoneMapper, WindowsTimeZoneMapperBorrowed};
pub use zone_offset::ZoneOffsetCalculator;
pub use zoned_datetime::{ResolvedZonedDateTime, ZonedDateTime};

#[cfg(all(feature = "ixdtf", feature = "compiled_data"))]
pub use crate::ixdtf::ParseError;
