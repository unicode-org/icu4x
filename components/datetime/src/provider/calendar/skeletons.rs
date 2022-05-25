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

#[cfg(feature = "datagen")]
// This cannot be const constructed. Instead we bake it to a BakedDateSkeletonPatternsV1
// for which we then define ZeroFrom to the struct itself.
impl crabbake::Bakeable for DateSkeletonPatternsV1<'_> {
    fn bake(&self, env: &crabbake::CrateEnv) -> crabbake::TokenStream {
        env.insert("icu_datetime");
        let vals = self.0.iter().map(|(skeleton, pattern)| {
            let fields = skeleton.0 .0.iter().map(|f| f.bake(env));
            let pattern = pattern.bake(env);
            crabbake::quote! {
                (&[#(#fields),*], #pattern)
            }
        });
        crabbake::quote! {
            [#(#vals),*]
        }
    }
}

#[cfg(feature = "datagen")]
impl Default for DateSkeletonPatternsV1Marker {
    fn default() -> Self {
        Self
    }
}

#[cfg(feature = "datagen")]
impl crabbake::Bakeable for DateSkeletonPatternsV1Marker {
    fn bake(&self, env: &crabbake::CrateEnv) -> crabbake::TokenStream {
        env.insert("icu_datetime");
        crabbake::quote! {
            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker
        }
    }
}

type BakedDateSkeletonPatternsV1 = [(&'static [crate::fields::Field], PatternPlurals<'static>)];

/// This is not actually zero-alloc. Whoops
impl zerofrom::ZeroFrom<'static, BakedDateSkeletonPatternsV1> for DateSkeletonPatternsV1<'static> {
    fn zero_from(other: &'static BakedDateSkeletonPatternsV1) -> Self {
        Self(
            other
                .iter()
                .map(|(fields, pattern)| {
                    (
                        SkeletonV1(Skeleton(fields.iter().cloned().collect())),
                        zerofrom::ZeroFrom::zero_from(pattern),
                    )
                })
                .collect(),
        )
    }
}
