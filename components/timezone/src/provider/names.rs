// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Property names-related data for this component
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use alloc::boxed::Box;
use core::cmp::Ordering;
use core::str;

use icu_provider::prelude::*;

use crate::TimeZoneBcp47Id;
use zerotrie::ZeroTrie;
use zerovec::ule::{UnvalidatedStr, VarULE};
use zerovec::{maps::ZeroMapKV, VarZeroSlice, VarZeroVec, ZeroVec};

/// This is a time zone identifier that can be "loose matched" as according to
/// [ECMAScript Temporal](https://tc39.es/proposal-temporal/#sec-isavailabletimezonename)
///
/// (matched case-insensitively in ASCII)
///
/// This is expected to be ASCII, but we do not rely on this invariant anywhere except during
/// datagen.
///
/// The Ord impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that a map can be searched for both strict and loose equality.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
///
/// # Examples
///
/// Using a [`NormalizedTimeZoneIdStr`] as the key of a [`ZeroMap`]:
///
/// ```
/// use icu_timezone::provider::names::NormalizedTimeZoneIdStr;
/// use zerovec::ZeroMap;
///
/// let map: ZeroMap<NormalizedTimeZoneIdStr, usize> = [
///     (NormalizedTimeZoneIdStr::from_str("America/Los_Angeles"), 11),
///     (NormalizedTimeZoneIdStr::from_str("Asia/Kolkata"), 22),
///     (NormalizedTimeZoneIdStr::from_str("Europe/Berlin"), 33),
/// ]
/// .into_iter()
/// .collect();
///
/// let key_approx = NormalizedTimeZoneIdStr::from_str("europe/berlin");
/// let key_exact = NormalizedTimeZoneIdStr::from_str("Europe/Berlin");
///
/// // Strict lookup:
/// assert_eq!(None, map.get_copied(key_approx));
/// assert_eq!(Some(33), map.get_copied(key_exact));
///
/// // Loose lookup:
/// assert_eq!(Some(33), map.get_copied_by(|u| u.cmp_loose(key_approx)));
/// assert_eq!(Some(33), map.get_copied_by(|u| u.cmp_loose(key_exact)));
/// ```
#[derive(PartialEq, Eq)] // VarULE wants these to be byte equality
#[derive(Debug, VarULE)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(transparent)]
pub struct NormalizedTimeZoneIdStr(UnvalidatedStr);

/// This impl requires enabling the optional `serde` Cargo feature of the `icu_properties` crate
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<NormalizedTimeZoneIdStr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Box<UnvalidatedStr>>::deserialize(deserializer).map(NormalizedTimeZoneIdStr::cast_box)
    }
}

/// This impl requires enabling the optional `serde` Cargo feature of the `icu_properties` crate
#[cfg(feature = "serde")]
impl<'de, 'a> serde::Deserialize<'de> for &'a NormalizedTimeZoneIdStr
where
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <&UnvalidatedStr>::deserialize(deserializer).map(NormalizedTimeZoneIdStr::cast_ref)
    }
}

impl<'a> ZeroMapKV<'a> for NormalizedTimeZoneIdStr {
    type Container = VarZeroVec<'a, NormalizedTimeZoneIdStr>;
    type Slice = VarZeroSlice<NormalizedTimeZoneIdStr>;
    type GetType = NormalizedTimeZoneIdStr;
    type OwnedType = Box<NormalizedTimeZoneIdStr>;
}

/// The Ord/PartialOrd impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that a map can be searched for both strict and loose equality.
impl PartialOrd for NormalizedTimeZoneIdStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The Ord impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that a map can be searched for both strict and loose equality.
impl Ord for NormalizedTimeZoneIdStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.cmp_loose(other);
        // When loose equality holds, fall back to strict equality
        if cmp == Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            cmp
        }
    }
}

impl NormalizedTimeZoneIdStr {
    /// Perform the loose comparison as defined in [`NormalizedTimeZoneIdStr`].
    pub fn cmp_loose(&self, other: &Self) -> Ordering {
        let self_iter = self.0.iter().map(u8::to_ascii_lowercase);
        let other_iter = other.0.iter().map(u8::to_ascii_lowercase);
        self_iter.cmp(other_iter)
    }

    /// Convert a string reference to a [`NormalizedTimeZoneIdStr`].
    pub const fn from_str(s: &str) -> &Self {
        Self::cast_ref(UnvalidatedStr::from_str(s))
    }

    /// Convert a [`UnvalidatedStr`] reference to a [`NormalizedTimeZoneIdStr`] reference.
    pub const fn cast_ref(value: &UnvalidatedStr) -> &Self {
        // Safety: repr(transparent)
        unsafe { core::mem::transmute(value) }
    }

    /// Convert a [`UnvalidatedStr`] box to a [`NormalizedTimeZoneIdStr`] box.
    pub const fn cast_box(value: Box<UnvalidatedStr>) -> Box<Self> {
        // Safety: repr(transparent)
        unsafe { core::mem::transmute(value) }
    }

    /// Get a [`NormalizedPropertyName`] box from a byte slice.
    pub fn boxed_from_bytes(b: &[u8]) -> Box<Self> {
        Self::cast_box(UnvalidatedStr::from_boxed_bytes(b.into()))
    }
}

/// A mapping from IANA time zone identifiers to BCP-47 time zone identifiers.
///
/// Multiple IANA time zone IDs can map to the same BCP-47 time zone ID.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(marker(
    IanaToBcp47MapV1Marker,
    "time_zone/iana_to_bcp47@1",
    singleton
))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::names),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct IanaToBcp47MapV1<'data> {
    /// A map from IANA time zone identifiers to indexes of BCP-47 time zone identifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroTrie<ZeroVec<'data, u8>>,
    /// A sorted list of BCP-47 time zone identifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    // Note: this is 9739B as ZeroVec<TinyStr8> and 9335B as VarZeroVec<str>
    pub bcp47_ids: ZeroVec<'data, TimeZoneBcp47Id>,
    /// An XxHash64 checksum of [`Self::bcp47_ids`].
    pub bcp47_ids_checksum: u64,
}

/// A mapping from IANA time zone identifiers to BCP-47 time zone identifiers.
///
/// The BCP-47 time zone ID maps to the default IANA time zone ID according to the CLDR data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(marker(
    Bcp47ToIanaMapV1Marker,
    "time_zone/bcp47_to_iana@1",
    singleton
))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::names),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct Bcp47ToIanaMapV1<'data> {
    /// A sorted list of BCP-47 time zone identifiers.
    ///
    /// If empty, the list from [`IanaToBcp47MapV1`] should be used so long as
    /// the checksums match.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bcp47_ids: ZeroVec<'data, TimeZoneBcp47Id>,
    /// An XxHash64 checksum of [`Self::bcp47_ids`].
    pub bcp47_ids_checksum: u64,
    /// The IANA time zone identifier corresponding to the BCP-47 ID in
    /// [`Self::bcp47_ids`].
    ///
    /// Since there can be more than one IANA identifier for a particular
    /// BCP-47 identifier, this list contains only the current canonical
    /// IANA identifier.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub canonical_iana_ids: VarZeroVec<'data, str>,
}
