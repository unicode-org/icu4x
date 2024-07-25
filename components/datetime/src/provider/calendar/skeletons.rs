// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    pattern::runtime::PatternPlurals,
    skeleton::{reference::Skeleton, SkeletonError},
};
use core::convert::TryFrom;
use icu_provider::prelude::*;
use litemap::LiteMap;

size_test!(DateSkeletonPatternsV1, date_skeleton_patterns_v1_size, 24);

/// Skeleton data for dates and times, along with the corresponding plural pattern
/// information.
#[doc = date_skeleton_patterns_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(DateSkeletonPatternsV1Marker = "datetime/skeletons@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateSkeletonPatternsV1<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[zerofrom(clone)]
    pub LiteMap<SkeletonV1, PatternPlurals<'data>>,
);

/// This struct is a public wrapper around the internal `Skeleton` struct. This allows
/// access to the serialization and deserialization capabilities, without exposing the
/// internals of the skeleton machinery.
///
/// The `Skeleton` is an "exotic type" in the serialization process, and handles its own
/// custom serialization practices.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SkeletonV1(pub Skeleton);

impl TryFrom<&str> for SkeletonV1 {
    type Error = SkeletonError;

    fn try_from(skeleton_string: &str) -> Result<Self, Self::Error> {
        match Skeleton::try_from(skeleton_string) {
            Ok(skeleton) => Ok(Self(skeleton)),
            Err(err) => Err(err),
        }
    }
}
