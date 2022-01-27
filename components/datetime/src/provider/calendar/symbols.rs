// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use alloc::borrow::Cow;
use alloc::string::String;
use icu_provider::yoke::{self, *};
use litemap::LiteMap;

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DateSymbolsV1 {
    pub months: months::ContextsV1,

    pub weekdays: weekdays::ContextsV1,

    pub day_periods: day_periods::ContextsV1,

    pub eras: Eras,
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Eras {
    // TODO use ZeroMap instead: https://github.com/unicode-org/icu4x/issues/876
    pub names: LiteMap<String, String>,
    pub abbr: LiteMap<String, String>,
    pub narrow: LiteMap<String, String>,
}

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

                #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
                #[yoke(cloning_zcf)]
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
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1 {
                pub abbreviated: SymbolsV1,
                pub narrow: SymbolsV1,
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1 {
                pub abbreviated: Option<SymbolsV1>,
                pub narrow: Option<SymbolsV1>,
                pub short: Option<SymbolsV1>,
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
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
