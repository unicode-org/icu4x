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

use core::ops::Deref;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroSlice, ZeroVec};

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
    use icu_timezone_data::*;
    pub mod icu {
        pub use crate as timezone;
    }
    make_provider!(Baked);
    impl_bcp47_to_iana_map_v1_marker!(Baked);
    impl_iana_to_bcp47_map_v3_marker!(Baked);
    impl_windows_zones_to_bcp47_map_v1_marker!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    names::Bcp47ToIanaMapV1Marker::INFO,
    names::IanaToBcp47MapV3Marker::INFO,
    windows::WindowsZonesToBcp47MapV1Marker::INFO,
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
#[cfg_attr(feature = "datagen", databake(path = icu_timezone::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeZoneBcp47Id(pub TinyAsciiStr<8>);

impl TimeZoneBcp47Id {
    /// The synthetic `Etc/Unknown` time zone.
    ///
    /// This is the result of parsing unknown zones. It's important that such parsing does not
    /// fail, as new zones are added all the time, and ICU4X might not be up to date.
    pub const fn unknown() -> Self {
        Self(tinystr::tinystr!(8, "unk"))
    }
}

impl Deref for TimeZoneBcp47Id {
    type Target = TinyAsciiStr<8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsULE for TimeZoneBcp47Id {
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

impl<'a> zerovec::maps::ZeroMapKV<'a> for TimeZoneBcp47Id {
    type Container = ZeroVec<'a, TimeZoneBcp47Id>;
    type Slice = ZeroSlice<TimeZoneBcp47Id>;
    type GetType = TimeZoneBcp47Id;
    type OwnedType = TimeZoneBcp47Id;
}
