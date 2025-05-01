// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::zone::ZoneNameTimestamp;
use crate::Time;
use calendrical_calculations::rata_die::RataDie;
use icu_calendar::{Date, Iso};
#[cfg(feature = "datagen")]
use icu_provider::prelude::*;
use zerovec::maps::ZeroMapKV;
use zerovec::ule::AsULE;
use zerovec::{ZeroMap2d, ZeroSlice, ZeroVec};

pub use crate::zone::ule::TimeZoneVariantULE;
pub use crate::zone::TimeZone;
pub mod iana;
pub mod windows;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_time_data::*;
    pub mod icu {
        pub use crate as time;
    }
    make_provider!(Baked);
    impl_timezone_identifiers_iana_extended_v1!(Baked);
    impl_timezone_identifiers_iana_core_v1!(Baked);
    impl_timezone_identifiers_windows_v1!(Baked);
    impl_timezone_variants_offsets_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    iana::TimezoneIdentifiersIanaExtendedV1::INFO,
    iana::TimezoneIdentifiersIanaCoreV1::INFO,
    windows::TimezoneIdentifiersWindowsV1::INFO,
    TimezoneVariantsOffsetsV1::INFO,
];

/// Storage type for storing UTC offsets as eights of an hour.
pub type EighthsOfHourOffset = i8;

icu_provider::data_marker!(
    /// The default mapping between period and offsets. The second level key is a wall-clock time encoded as
    /// [`ZoneNameTimestamp`]. It represents when the offsets started to be used.
    ///
    /// The values are the standard offset, and the daylight offset *relative to the standard offset*. As such,
    /// if the second value is 0, there is no daylight time.
    TimezoneVariantsOffsetsV1,
    "timezone/variants/offsets/v1",
    ZeroMap2d<'static, TimeZone, ZoneNameTimestamp, (EighthsOfHourOffset, EighthsOfHourOffset)>,
    is_singleton = true
);
