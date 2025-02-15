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

use calendrical_calculations::rata_die::RataDie;
use core::ops::Deref;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroMap2d, ZeroSlice, ZeroVec};

pub mod names;
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
    impl_bcp47_to_iana_map_v1!(Baked);
    impl_iana_to_bcp47_map_v3!(Baked);
    impl_windows_zones_to_bcp47_map_v1!(Baked);
    impl_zone_offset_period_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    names::Bcp47ToIanaMapV1::INFO,
    names::IanaToBcp47MapV3::INFO,
    windows::WindowsZonesToBcp47MapV1::INFO,
    ZoneOffsetPeriodV1::INFO,
];

/// TimeZone ID in BCP47 format
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeZone(pub TinyAsciiStr<8>);

impl TimeZone {
    /// The synthetic `Etc/Unknown` time zone.
    ///
    /// This is the result of parsing unknown zones. It's important that such parsing does not
    /// fail, as new zones are added all the time, and ICU4X might not be up to date.
    pub const fn unknown() -> Self {
        Self(tinystr::tinystr!(8, "unk"))
    }
}

impl Deref for TimeZone {
    type Target = TinyAsciiStr<8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsULE for TimeZone {
    type ULE = Self;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for TimeZone {
    type Container = ZeroVec<'a, TimeZone>;
    type Slice = ZeroSlice<TimeZone>;
    type GetType = TimeZone;
    type OwnedType = TimeZone;
}

/// Storage type for storing UTC offsets as eights of an hour.
pub type EighthsOfHourOffset = i8;
/// Storage type for storing `DateTime<Iso>` as minutes since [`EPOCH`].
pub type MinutesSinceEpoch = i32;
/// The epoch for [`MinutesSinceEpoch`].
///
/// This is 1970-01-01, but this is coincidental to anything UNIX and should be changed to 0 in the future.
pub const EPOCH: RataDie = RataDie::new(719163);

/// An ICU4X mapping to the time zone offsets at a given period.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(ZoneOffsetPeriodV1, "time_zone/offset_period@1", singleton))]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct ZoneOffsetPeriod<'data>(
    /// The default mapping between period and offsets. The second level key is a wall-clock time represented as
    /// the number of minutes since the local [`EPOCH`]. It represents when the offsets ended to be used.
    ///
    /// The values are the standard offset, and the daylight offset *relative to the standard offset*. As such,
    /// if the second value is 0, there is no daylight time.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ZeroMap2d<'data, TimeZone, MinutesSinceEpoch, (EighthsOfHourOffset, EighthsOfHourOffset)>,
);
