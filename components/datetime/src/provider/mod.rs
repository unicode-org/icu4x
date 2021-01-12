// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! The `provider` module contains struct definitions for ICU4X [`DataProvider`].
//!
//! [`DataProvider`]: icu_provider::DataProvider

pub(crate) mod helpers;

pub mod key {
    use icu_provider::{resource::ResourceKey, resource_key};
    pub const GREGORY_V1: ResourceKey = resource_key!(dates, "gregory", 1);
}

pub mod gregory {
    use std::borrow::Cow;

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DatesV1 {
        pub symbols: DateSymbolsV1,

        pub patterns: PatternsV1,
    }

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

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternsV1 {
        pub date: patterns::StylePatternsV1,

        pub time: patterns::StylePatternsV1,

        pub date_time: patterns::StylePatternsV1,
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
        ($name: ident, $($element: ident: $ty: ty),+ $(,)?) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1 {
                    $(pub $element: $ty),*
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
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1 {
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub abbreviated: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub narrow: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub short: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1 {
                pub format: FormatWidthsV1,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub stand_alone: Option<StandAloneWidthsV1>,
            }
        };
    }

    symbols!(months, [Cow<'static, str>; 12]);

    symbols!(weekdays, [Cow<'static, str>; 7]);

    symbols!(
        day_periods,
        am: Cow<'static, str>,
        pm: Cow<'static, str>,
        noon: Option<Cow<'static, str>>,
        midnight: Option<Cow<'static, str>>,
    );

    pub mod patterns {
        use super::*;
        #[derive(Debug, PartialEq, Clone, Default)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        pub struct StylePatternsV1 {
            pub full: Cow<'static, str>,
            pub long: Cow<'static, str>,
            pub medium: Cow<'static, str>,
            pub short: Cow<'static, str>,
        }
    }
}
