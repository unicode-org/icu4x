// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// Date
#[cfg(feature = "invariant")]
use crate::prelude::*;

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != DataCategory::Dates {
        return None;
    }
    match (data_key.sub_category.as_str(), data_key.version) {
        ("gregory", 1) => make_inv_response::<gregory::DatesV1>(),
        _ => None,
    }
}

pub mod gregory {
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;
    #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
    pub struct DatesV1 {
        pub symbols: DateSymbolsV1,

        pub patterns: PatternsV1,
    }

    #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
    pub struct DateSymbolsV1 {
        pub months: months::ContextsV1,

        pub weekdays: weekdays::ContextsV1,

        pub day_periods: day_periods::ContextsV1,
    }

    #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
    pub struct PatternsV1 {
        pub date: patterns::StylePatternsV1,

        pub time: patterns::StylePatternsV1,

        pub date_time: patterns::StylePatternsV1,
    }

    macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
                pub struct SymbolsV1(pub $expr);

                symbols!();
            }
        };
        ($name: ident, $($element: ident: $ty: ty),*) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
                pub struct SymbolsV1 {
                    $(pub $element: $ty),*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
            pub struct FormatWidthsV1 {
                pub abbreviated: SymbolsV1,
                pub narrow: SymbolsV1,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
            pub struct StandAloneWidthsV1 {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub abbreviated: Option<SymbolsV1>,

                #[serde(skip_serializing_if = "Option::is_none")]
                pub narrow: Option<SymbolsV1>,

                #[serde(skip_serializing_if = "Option::is_none")]
                pub short: Option<SymbolsV1>,

                #[serde(skip_serializing_if = "Option::is_none")]
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
            pub struct ContextsV1 {
                pub format: FormatWidthsV1,

                #[serde(skip_serializing_if = "Option::is_none")]
                pub stand_alone: Option<StandAloneWidthsV1>,
            }
        };
    }

    symbols!(months, [Cow<'static, str>; 12]);

    symbols!(weekdays, [Cow<'static, str>; 7]);

    symbols!(day_periods, am: Cow<'static, str>, pm: Cow<'static, str>);

    pub mod patterns {
        use super::*;
        #[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default)]
        pub struct StylePatternsV1 {
            pub full: Cow<'static, str>,
            pub long: Cow<'static, str>,
            pub medium: Cow<'static, str>,
            pub short: Cow<'static, str>,
        }
    }
}
