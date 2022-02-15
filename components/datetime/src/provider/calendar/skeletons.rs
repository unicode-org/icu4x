// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use crate::{
    pattern::runtime::PatternPlurals,
    skeleton::{reference::Skeleton, SkeletonError},
};
use core::convert::TryFrom;
use icu_provider::{yoke, zerofrom};
use litemap::LiteMap;

#[icu_provider::data_struct(DateSkeletonPatternsV1Marker = "datetime/skeletons@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zf)]
pub struct DateSkeletonPatternsV1<'data>(
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub  LiteMap<SkeletonV1, PatternPlurals<'data>>,
);

/// This struct is a public wrapper around the internal `Skeleton` struct. This allows
/// access to the serialization and deserialization capabilities, without exposing the
/// internals of the skeleton machinery.
///
/// The `Skeleton` is an "exotic type" in the serialization process, and handles its own
/// custom serialization practices.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
