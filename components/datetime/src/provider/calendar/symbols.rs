// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
use zerovec::map::ZeroMap;

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct DateSymbolsV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub months: months::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub weekdays: weekdays::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub day_periods: day_periods::ContextsV1<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub eras: Eras<'data>,
}

#[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct Eras<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub names: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub abbr: ZeroMap<'data, str, str>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub narrow: ZeroMap<'data, str, str>,
}

macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default, ZeroCopyFrom, Yokeable)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1<'data>(#[cfg_attr(feature="provider_serde", serde(borrow))] pub $expr);

                symbols!();
            }
        };
        ($name: ident { $($tokens: tt)* }) => {
            symbols!($name { $($tokens)* } -> ());
        };
        ($name: ident { $element: ident: Option<$ty: ty>, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub $element: $ty,
            ));
        };
        ($name: ident { $element: ident: Option<$ty: ty> $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub $element: $ty,
            ));
        };
        ($name: ident { } -> ($($members: tt)*)) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1<'data> {
                    $($members)*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub abbreviated: SymbolsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub narrow: SymbolsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub wide: SymbolsV1<'data>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub abbreviated: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub narrow: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub short: Option<SymbolsV1<'data>>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub wide: Option<SymbolsV1<'data>>,
            }

            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1<'data> {
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub format: FormatWidthsV1<'data>,
                #[cfg_attr(feature = "provider_serde", serde(borrow))]
                pub stand_alone: Option<StandAloneWidthsV1<'data>>,
            }
        };
    }

symbols!(months, [Cow<'data, str>; 12]);

symbols!(weekdays, [Cow<'data, str>; 7]);

symbols!(
    day_periods {
        am: Cow<'data, str>,
        pm: Cow<'data, str>,
        noon: Option<Cow<'data, str>>,
        midnight: Option<Cow<'data, str>>,
    }
);
