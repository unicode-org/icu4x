// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use super::patterns::PatternV1;
use crate::skeleton::{Skeleton, SkeletonError};
use core::convert::TryFrom;
use icu_provider::yoke::{self, *};
use litemap::LiteMap;

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DateSkeletonPatternsV1(pub LiteMap<SkeletonV1, PatternV1>);

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
