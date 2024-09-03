// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A provider for mapping Windows Zones to IANA identifiers.
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
/// Windows Timezones may map to multiple IANA identifiers so providing a [`WindowsGeoName`]
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
#[yoke(prove_covariance_manually)]
pub struct WindowsZonesToIanaMapV1<'data>(
    /// A map from a `WindowsZoneIdentifier` and `WindowsGeoName` to IANA identifier(s).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ZeroMap2d<'data, WindowsZoneIdentifier, WindowsGeoName, str>,
);

/// The `WindowsGeoName` is a value that is either a two-letter ISO 3166-1 code or a numeric UN M49 code.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_timezone::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct WindowsGeoName(pub TinyAsciiStr<3>);

impl Default for WindowsGeoName {
    fn default() -> Self {
        Self(tinystr!(3, "001"))
    }
}

impl From<TinyAsciiStr<3>> for WindowsGeoName {
    fn from(value: TinyAsciiStr<3>) -> Self {
        Self(value)
    }
}

impl AsULE for WindowsGeoName {
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

impl<'a> zerovec::maps::ZeroMapKV<'a> for WindowsGeoName {
    type Container = ZeroVec<'a, WindowsGeoName>;
    type Slice = ZeroSlice<WindowsGeoName>;
    type GetType = WindowsGeoName;
    type OwnedType = WindowsGeoName;
}

/// The designated Windows standard name identifier for a certain time zone.
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
