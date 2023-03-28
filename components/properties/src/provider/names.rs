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
use core::fmt;

use core::str;

use icu_provider::prelude::*;

use zerovec::{maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec, ZeroMap};

/// This is a property name that can be "loose matched" as according to
/// [PropertyValueAliases.txt](https://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt)
///
/// (matched case-insensitively in ASCII, ignoring underscores, whitespace, and hyphens)
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
#[derive(PartialEq, Eq)] // VarULE wants these to be byte equality
#[derive(VarULE)]
#[repr(transparent)]
pub struct NormalizedPropertyNameStr([u8]);

#[cfg(feature = "datagen")]
impl serde::Serialize for NormalizedPropertyNameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        if serializer.is_human_readable() {
            let s = str::from_utf8(&self.0)
                .map_err(|_| S::Error::custom("Attempted to datagen invalid string property"))?;
            serializer.serialize_str(s)
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<NormalizedPropertyNameStr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use alloc::borrow::Cow;
        let s; // lifetime lengthening
        let b;
        // Can be improved with https://github.com/unicode-org/icu4x/issues/2310
        // the allocations here are fine, in normal ICU4X code they'll only get hit
        // during human-readable deserialization
        let bytes = if deserializer.is_human_readable() {
            s = <Cow<str>>::deserialize(deserializer)?;
            s.as_bytes()
        } else {
            b = <Cow<[u8]>>::deserialize(deserializer)?;
            &b
        };
        Ok(NormalizedPropertyNameStr::boxed_from_bytes(bytes))
    }
}

impl<'a> ZeroMapKV<'a> for NormalizedPropertyNameStr {
    type Container = VarZeroVec<'a, NormalizedPropertyNameStr>;
    type Slice = VarZeroSlice<NormalizedPropertyNameStr>;
    type GetType = NormalizedPropertyNameStr;
    type OwnedType = Box<NormalizedPropertyNameStr>;
}

/// The Ord/PartialOrd impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that a map can be searched for both strict and loose equality.
impl PartialOrd for NormalizedPropertyNameStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Normalize a character based on the "loose matching" described in PropertyValueAliases.txt,
/// returning None for skippable characters
///
/// ICU has [code for this][1] (and [during property lookup][2]) which we emulate.
/// In particular, ICU only does normalization within ASCII, which makes sense since character names
/// seem to be only ASCII.
///
/// [1]: https://github.com/unicode-org/icu/blob/288c4c7555915ce7b1fb675d94ddd495058fc039/icu4c/source/common/propname.cpp#L35
/// [2]: https://github.com/unicode-org/icu/blob/288c4c7555915ce7b1fb675d94ddd495058fc039/icu4c/source/common/propname.cpp#L226-L230
fn normalize_char(ch: u8) -> Option<u8> {
    match ch {
        // all ascii whitespace
        ch if ch.is_ascii_whitespace() => None,
        // underscores, hyphens, and the vertical tab character
        // not covered by is_ascii_whitespace()
        b'_' | b'-' | 0x0B => None,
        // ignore case by lowercasing
        ch => Some(ch.to_ascii_lowercase()),
    }
}

/// The Ord impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that a map can be searched for both strict and loose equality.
impl Ord for NormalizedPropertyNameStr {
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

impl fmt::Debug for NormalizedPropertyNameStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(s) = str::from_utf8(&self.0) {
            f.write_str(s)
        } else {
            f.write_str("(invalid utf8)")
        }
    }
}

impl NormalizedPropertyNameStr {
    pub(crate) fn cmp_loose(&self, other: &Self) -> Ordering {
        let self_iter = self.0.iter().copied().filter_map(normalize_char);
        let other_iter = other.0.iter().copied().filter_map(normalize_char);
        self_iter.cmp(other_iter)
    }
    #[cfg(feature = "serde")]
    /// Get a Box<NormalizedPropertyName> from a byte slice
    pub fn boxed_from_bytes(b: &[u8]) -> Box<Self> {
        #[allow(clippy::expect_used)] // Self has no invariants
        // can be cleaned up with https://github.com/unicode-org/icu4x/issues/2310
        let this = Self::parse_byte_slice(b).expect("NormalizedPropertyName has no invariants");

        zerovec::ule::encode_varule_to_box(&this)
    }
}

/// A set of characters and strings which share a particular property value.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone)]
#[icu_provider::data_struct(marker(GeneralCategoryMaskNameToValueV1Marker, "props/names/gcm@1"))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider::names),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyValueNameToEnumMapV1<'data> {
    /// A map from names to their value discriminant
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap<'data, NormalizedPropertyNameStr, u16>,
}

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone)]
#[icu_provider::data_struct]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider::names),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyEnumToValueNameMapV1<'data> {
    /// A map from the value discriminant (the index) to the names
    /// (empty strings count as missing)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: VarZeroVec<'data, str>,
}
