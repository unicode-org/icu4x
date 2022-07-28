// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::{yoke, zerofrom};
use tinystr::TinyAsciiStr;
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroMap2d, ZeroSlice, ZeroVec};

/// TimeZone ID in BCP47 format
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeZoneBcp47Id(pub TinyAsciiStr<8>);

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

/// MetaZone ID in a compact format
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct MetaZoneId(pub TinyAsciiStr<4>);

impl AsULE for MetaZoneId {
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

impl<'a> zerovec::maps::ZeroMapKV<'a> for MetaZoneId {
    type Container = ZeroVec<'a, MetaZoneId>;
    type Slice = ZeroSlice<MetaZoneId>;
    type GetType = MetaZoneId;
    type OwnedType = MetaZoneId;
}

/// An ICU4X mapping to the metazones at a given period.
/// See CLDR-JSON metaZones.json for more context.
#[icu_provider::data_struct(MetaZonePeriodV1Marker = "time_zone/metazone_period@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetaZonePeriodV1<'data>(
    /// The default mapping between period and metazone id. The second level key is a wall-clock time represented as the number of minutes since the local unix epoch. It represents when the metazone started to be used.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ZeroMap2d<'data, TimeZoneBcp47Id, i32, Option<MetaZoneId>>,
);
