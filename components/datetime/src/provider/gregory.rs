// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DateSymbolsV1 {
    pub months: months::ContextsV1,

    pub weekdays: weekdays::ContextsV1,

    pub day_periods: day_periods::ContextsV1,
}

icu_provider::impl_data_marker_no_lifetime!(
    DateSymbolsV1,
    /// Marker type for [`DateSymbolsV1`]
    DateSymbolsV1Marker
);

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DatePatternsV1 {
    pub date: patterns::LengthPatternsV1,

    pub time: patterns::LengthPatternsV1,

    pub datetime: patterns::DateTimeFormatsV1,
}

icu_provider::impl_data_marker_no_lifetime!(
    DatePatternsV1,
    /// Marker type for [`DatePatternsV1`]
    DatePatternsV1Marker
);

macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1(pub $expr);

                symbols!();
            }
        };
        ($name: ident { $($tokens: tt)* }) => {
            symbols!($name { $($tokens)* } -> ());
        };
        ($name: ident { $element: ident: Option<$ty: ty>, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { $element: ident: Option<$ty: ty> $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { } -> ($($members: tt)*)) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1 {
                    $($members)*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1 {
                pub abbreviated: SymbolsV1,
                pub narrow: SymbolsV1,
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1 {
                pub abbreviated: Option<SymbolsV1>,
                pub narrow: Option<SymbolsV1>,
                pub short: Option<SymbolsV1>,
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1 {
                pub format: FormatWidthsV1,
                pub stand_alone: Option<StandAloneWidthsV1>,
            }
        };
    }

symbols!(months, [Cow<'static, str>; 12]);

symbols!(weekdays, [Cow<'static, str>; 7]);

symbols!(
    day_periods {
        am: Cow<'static, str>,
        pm: Cow<'static, str>,
        noon: Option<Cow<'static, str>>,
        midnight: Option<Cow<'static, str>>,
    }
);

pub mod patterns {
    use super::*;
    use crate::{
        pattern::{self, Pattern},
        skeleton::{Skeleton, SkeletonError},
    };
    use litemap::LiteMap;
    use std::convert::TryFrom;

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
        type Error = pattern::Error;

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
