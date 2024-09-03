// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Windows Zones to IANA data for this component
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::{tinystr, TinyAsciiStr};
use zerovec::{
    ule::{AsULE, ULE},
    ZeroMap2d, ZeroSlice, ZeroVec,
};

/// A mapping from Windows Timezone names to the corresponding IANA identifier(s).
///
/// Windows Timezones may map to multiple IANA identifiers so providing a windows territory/region
/// code is required to differentiate IANA identifiers.
/// 
/// Not all return values are guaranteed to be a single value. The return may be a space delimited
/// string of IANA identifier values.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone)]
#[icu_provider::data_struct(marker(
    WindowsZonesToIanaMapV1Marker,
    "time_zone/windows_zones_to_iana@1",
    singleton
))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_timezone::provider::windows))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)] // TODO: Prove
pub struct WindowsZonesToIanaMapV1<'data> {
    /// A map from a `WindowsZoneIdentifier` and `WindowsRegionCode` to IANA identifier(s).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap2d<'data, WindowsZoneIdentifier, WindowsRegionCode, str>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_timezone::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct WindowsRegionCode(pub TinyAsciiStr<3>);

impl Default for WindowsRegionCode {
    fn default() -> Self {
        Self(tinystr!(3, "001"))
    }
}

impl From<TinyAsciiStr<3>> for WindowsRegionCode {
    fn from(value: TinyAsciiStr<3>) -> Self {
        Self(value)
    }
}

impl AsULE for WindowsRegionCode {
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

impl<'a> zerovec::maps::ZeroMapKV<'a> for WindowsRegionCode {
    type Container = ZeroVec<'a, WindowsRegionCode>;
    type Slice = ZeroSlice<WindowsRegionCode>;
    type GetType = WindowsRegionCode;
    type OwnedType = WindowsRegionCode;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_timezone::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct WindowsZoneIdentifier(pub TinyAsciiStr<32>);

impl From<TinyAsciiStr<32>> for WindowsZoneIdentifier {
    fn from(value: TinyAsciiStr<32>) -> Self {
        Self(value)
    }
}

impl AsULE for WindowsZoneIdentifier {
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

impl<'a> zerovec::maps::ZeroMapKV<'a> for WindowsZoneIdentifier {
    type Container = ZeroVec<'a, WindowsZoneIdentifier>;
    type Slice = ZeroSlice<WindowsZoneIdentifier>;
    type GetType = WindowsZoneIdentifier;
    type OwnedType = WindowsZoneIdentifier;
}
