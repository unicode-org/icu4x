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

// Manually implement DataMarker so that we can keep it in the proper experimental feature
// #[icu_provider::data_struct(marker(
//     DateSkeletonPatternsV1Marker,
//     "datetime/skeletons@1",
//     extension_key = "ca"
// ))]
//
/// Skeleton data for dates and times, along with the corresponding plural pattern
/// information.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(
    DateSkeletonPatternsV1Marker,
    "datetime/skeletons@1",
    fallback_by = "language",
    extension_key = "ca",
))]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
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
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
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

// DateSkeletonPatternsV1 uses heap-allocations, so it cannot be const-constructed
// (and it also isn't zero-copy). For baking, we work around this by using an equivalent
// const type (BakedDateSkeletonPatternsV1) and then zero-froming that into the final
// struct. This operation contains the required allocation (which also happens in the
// serde codepath, ZeroFrom<Self> uses #[zerofrom(clone)]).
// See https://github.com/unicode-org/icu4x/issues/1678.

#[cfg(feature = "datagen")]
impl databake::Bake for DateSkeletonPatternsV1<'_> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        use zerofrom::ZeroFrom;
        env.insert("icu_datetime");
        databake::Bake::bake(
            &self
                .0
                .iter()
                .map(|(skeleton, pattern)| {
                    (skeleton.0 .0.as_slice(), PatternPlurals::zero_from(pattern))
                })
                .collect::<Vec<_>>()
                .as_slice(),
            env,
        )
    }
}

#[cfg(feature = "datagen")]
impl Default for DateSkeletonPatternsV1Marker {
    fn default() -> Self {
        Self
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for DateSkeletonPatternsV1Marker {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_datetime");
        databake::quote! {
            icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker
        }
    }
}
