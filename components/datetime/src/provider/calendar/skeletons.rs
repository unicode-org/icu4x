// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    pattern::runtime::PatternPlurals,
    skeleton::{reference::Skeleton, SkeletonError},
};
use core::convert::TryFrom;
use icu_provider::{yoke, zerofrom, DataKeyMetadata};
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
#[derive(yoke::Yokeable, zerofrom::ZeroFrom, Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateSkeletonPatternsV1<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[zerofrom(clone)]
    pub LiteMap<SkeletonV1, PatternPlurals<'data>>,
);

/// Marker type for [`DateSkeletonPatternsV1`].
#[cfg(feature = "experimental")]
pub struct DateSkeletonPatternsV1Marker;
#[cfg(feature = "experimental")]
impl icu_provider::DataMarker for DateSkeletonPatternsV1Marker {
    type Yokeable = DateSkeletonPatternsV1<'static>;
}
#[cfg(feature = "experimental")]
impl icu_provider::KeyedDataMarker for DateSkeletonPatternsV1Marker {
    const KEY: icu_provider::DataKey = icu_provider::data_key!(
        "datetime/skeletons@1",
        DataKeyMetadata::construct_internal(
            icu_provider::FallbackPriority::Language,
            Some(icu_locid::extensions_unicode_key!("ca")),
            None
        )
    );
}

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

// DateSkeletonPatternsV1 uses heap-allocations, so it cannot be const-constructed
// (and it also isn't zero-copy). For baking, we work around this by using an equivalent
// const type (BakedDateSkeletonPatternsV1) and then zero-froming that into the final
// struct. This operation contains the required allocation (which also happens in the
// serde codepath, ZeroFrom<Self> uses #[zerofrom(clone)]).
// See https://github.com/unicode-org/icu4x/issues/1678.

#[cfg(feature = "datagen")]
impl databake::Bake for DateSkeletonPatternsV1<'_> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_datetime");
        let vals = self.0.iter().map(|(skeleton, pattern)| {
            let fields = skeleton.0 .0.iter().map(|f| f.bake(env));
            let pattern = pattern.bake(env);
            databake::quote! {
                (&[#(#fields),*], #pattern)
            }
        });
        databake::quote! {
            &[#(#vals),*]
        }
    }
}

#[cfg(all(feature = "datagen", feature = "experimental"))]
impl Default for DateSkeletonPatternsV1Marker {
    fn default() -> Self {
        Self
    }
}

#[cfg(all(feature = "datagen", feature = "experimental"))]
impl databake::Bake for DateSkeletonPatternsV1Marker {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_datetime");
        databake::quote! {
            ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker
        }
    }
}

type BakedDateSkeletonPatternsV1 = &'static [(&'static [crate::fields::Field], PatternPlurals<'static>)];

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
