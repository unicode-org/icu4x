// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

pub mod symbols;

use crate::pattern;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
pub use symbols::{DateSymbolsV1, DateSymbolsV1Marker};

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DatePatternsV1 {
    pub date: patterns::LengthPatternsV1,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    pub time_h11_h12: patterns::LengthPatternsV1,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    pub time_h23_h24: patterns::LengthPatternsV1,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: pattern::CoarseHourCycle,

    pub datetime: patterns::DateTimeFormatsV1,
}

pub mod patterns {
    use super::*;
    use crate::{
        pattern::{self, reference::Pattern},
        skeleton::{Skeleton, SkeletonError},
    };
    use core::convert::TryFrom;
    use litemap::LiteMap;

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct LengthPatternsV1 {
        pub full: Cow<'static, str>,
        pub long: Cow<'static, str>,
        pub medium: Cow<'static, str>,
        pub short: Cow<'static, str>,
    }

    /// This struct is a public wrapper around the internal [`Pattern`] struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the pattern machinery.
    ///
    /// The [`Pattern`] is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternV1(pub Pattern);

    impl From<Pattern> for PatternV1 {
        fn from(pattern: Pattern) -> Self {
            Self(pattern)
        }
    }

    impl TryFrom<&str> for PatternV1 {
        type Error = pattern::PatternError;

        fn try_from(pattern_string: &str) -> Result<Self, Self::Error> {
            let pattern = Pattern::from_bytes(pattern_string);
            match pattern {
                Ok(pattern) => Ok(Self::from(pattern)),
                Err(err) => Err(err),
            }
        }
    }

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

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct SkeletonsV1(pub LiteMap<SkeletonV1, PatternV1>);

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DateTimeFormatsV1 {
        pub length_patterns: LengthPatternsV1,
        pub skeletons: SkeletonsV1,
    }
}
