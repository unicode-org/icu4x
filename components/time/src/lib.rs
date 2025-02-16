// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

//! TODO

extern crate alloc;

mod error;
mod ids;
#[cfg(feature = "ixdtf")]
mod ixdtf;
pub mod provider;
pub mod scaffold;
mod time_zone;
mod types;
mod windows_tz;
mod zone_offset;

pub use error::InvalidOffsetError;
#[cfg(feature = "ixdtf")]
pub use ixdtf::ParseError;

pub use provider::TimeZone;
pub use time_zone::TimeZoneInfo;

pub use types::{DateTime, Hour, Minute, Nanosecond, Second, Time, ZonedDateTime};

pub mod zone {
    //! Types for resolving and manipulating time zones.
    //!
    //! # Fields
    //!
    //! In ICU4X, a [formattable time zone](super::TimeZoneInfo) consists of up to four different fields:
    //!
    //! 1. The time zone ID
    //! 2. The offset from UTC
    //! 3. A timestamp, as time zone names can change over time
    //! 4. The zone variant, representing concepts such as Standard, Summer, Daylight, and Ramadan time
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
    //! IANA time zone, use [`IanaParser`].
    //!
    //! ## UTC Offset
    //!
    //! The UTC offset precisely states the time difference between the time zone in question and
    //! Coordinated Universal Time (UTC).
    //!
    //! In localized strings, it is often rendered as "UTC-6", meaning 6 hours less than UTC (some locales
    //! use the term "GMT" instead of "UTC").
    //!
    //! ## Timestamp
    //!
    //! Some time zones change names over time, such as when changing "metazone". For example, Portugal changed from
    //! "Western European Time" to "Central European Time" and back in the 1990s, without changing time zone id
    //! (`Europe/Lisbon`/`ptlis`). Therefore, a timestamp is needed to resolve such generic time zone names.
    //!
    //! It is not required to set the timestamp on [`TimeZoneInfo`](super::TimeZoneInfo). If it is not set, some string
    //! formats may be unsupported.
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
    //! Note: It is not required to set the zone variant on [`TimeZoneInfo`](super::TimeZoneInfo). If it is not set, some string
    //! formats may be unsupported.
    //!
    //! # Examples
    //!
    //! ```
    //! use icu::calendar::Date;
    //! use icu::time::Time;
    //! use icu::time::TimeZone;
    //! use icu::time::zone::IanaParser;
    //! use icu::time::zone::TimeZoneVariant;
    //! use tinystr::tinystr;
    //!
    //! // Parse the IANA ID
    //! let id = IanaParser::new().iana_to_bcp47("America/Chicago");
    //!
    //! // Alternatively, use the BCP47 ID directly
    //! let id = TimeZone(tinystr!(8, "uschi"));
    //!
    //! // Create a TimeZoneInfo<Base> by associating the ID with an offset
    //! let time_zone = id.with_offset("-0600".parse().ok());
    //!
    //! // Extend to a TimeZoneInfo<AtTime> by adding a local time
    //! let time_zone_at_time = time_zone
    //!     .at_time((Date::try_new_iso(2023, 12, 2).unwrap(), Time::midnight()));
    //!
    //! // Extend to a TimeZoneInfo<Full> by adding a zone variant
    //! let time_zone_with_variant =
    //!     time_zone_at_time.with_zone_variant(TimeZoneVariant::Standard);
    //! ```

    pub use crate::types::TimeZoneVariant;
    pub use crate::types::UtcOffset;
    pub use crate::zone_offset::UtcOffsetCalculator;
    pub use crate::zone_offset::UtcOffsets;

    pub use iana::IanaParser;
    pub use windows::WindowsTimeZoneMapper;

    /// TODO
    pub mod models {
        pub use crate::time_zone::models::*;
        pub use crate::time_zone::TimeZoneModel;
    }

    /// TODO
    pub mod iana {
        pub use crate::ids::{
            IanaParser, IanaParserBorrowed, IanaParserExtended, IanaParserExtendedBorrowed,
            TimeZoneBcp47Iter, TimeZoneCanonicalIanaIter,
        };
    }

    /// TODO
    pub mod windows {
        pub use crate::windows_tz::WindowsTimeZoneMapper;
        pub use crate::windows_tz::WindowsTimeZoneMapperBorrowed;
    }
}
