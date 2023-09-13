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

use crate::TimeZoneBcp47Id;
use core::str;
use icu_provider::prelude::*;
use zerotrie::ZeroTrie;
use zerovec::{VarZeroVec, ZeroVec};

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
    /// An XxHash64 checksum of [`Self::bcp47_ids`].
    ///
    /// The actual list should be loaded from [`IanaToBcp47MapV1`]. The checksums
    /// should match if these were generated from the same data set.
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
