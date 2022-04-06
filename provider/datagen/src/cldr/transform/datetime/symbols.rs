// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr::cldr_serde;
use icu_datetime::provider::calendar::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tinystr::{tinystr, TinyStr16};

pub fn convert_dates(other: &cldr_serde::ca::Dates, calendar: &str) -> DateSymbolsV1<'static> {
    DateSymbolsV1 {
        months: (&other.months).into(),
        weekdays: (&other.days).into(),
        day_periods: (&other.day_periods).into(),
        eras: convert_eras(&other.eras, calendar),
    }
}

fn convert_eras(eras: &cldr_serde::ca::Eras, calendar: &str) -> Eras<'static> {
    let map = get_era_code_map(calendar);
    let mut out_eras = Eras::default();

    for (cldr, code) in map.into_iter() {
        if let Some(name) = eras.names.get(&cldr) {
            out_eras.names.insert(&code, name);
        }
        if let Some(abbr) = eras.abbr.get(&cldr) {
            out_eras.abbr.insert(&code, abbr);
        }
        if let Some(narrow) = eras.narrow.get(&cldr) {
            out_eras.narrow.insert(&code, narrow);
        }
    }
    out_eras
}

fn get_era_code_map(calendar: &str) -> BTreeMap<String, TinyStr16> {
    match calendar {
        "gregory" => vec![
            ("0".to_string(), tinystr!(16, "bc")),
            ("1".to_string(), tinystr!(16, "ad")),
        ]
        .into_iter()
        .collect(),
        "buddhist" => vec![("0".to_string(), tinystr!(16, "be"))]
            .into_iter()
            .collect(),
        "japanese" => crate::cldr::transform::calendar::japanese::get_era_code_map(),
        "coptic" => vec![
            ("0".to_string(), tinystr!(16, "bc")),
            ("1".to_string(), tinystr!(16, "ad")),
        ]
        .into_iter()
        .collect(),
        "indian" => vec![("0".to_string(), tinystr!(16, "saka"))]
            .into_iter()
            .collect(),
        #[allow(clippy::panic)] // TODO(#1668) Clippy exceptions need docs or fixing.
        _ => panic!("Era map unknown for {}", calendar),
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident $(,)?], [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl From<&cldr_serde::ca::$name::Symbols> for $name2::SymbolsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Symbols) -> Self {
                Self([
                    $(
                        Cow::Owned(other.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$name, $name2]);
    };
    ([$name: ident, $name2: ident $(,)?], { $($element: ident),+ $(,)? } $(,)?) => {
        impl From<&cldr_serde::ca::$name::Symbols> for $name2::SymbolsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Symbols) -> Self {
                Self {
                    $(
                        $element: other.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$name, $name]);
    };
    ([$name: ident, $name2: ident]) => {
        impl cldr_serde::ca::$name::Symbols {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl From<&cldr_serde::ca::$name::Contexts> for $name2::ContextsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::Contexts) -> Self {
                Self {
                    format: (&other.format).into(),
                    stand_alone: other.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&other.format)
                    }).map(|ref stand_alone| stand_alone.into())
                }
            }
        }

        impl cldr_serde::ca::$name::StandAloneWidths {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &cldr_serde::ca::$name::FormatWidths) -> Option<Self> {
                let abbreviated = self.abbreviated.as_ref().and_then(|v| v.get_unaliased(&other.abbreviated));
                let narrow = self.narrow.as_ref().and_then(|v| v.get_unaliased(&other.narrow));
                let short = if self.short == other.short {
                    None
                } else {
                    self.short.clone()
                };
                let wide = self.wide.as_ref().and_then(|v| v.get_unaliased(&other.wide));

                if abbreviated.is_none() && narrow.is_none() && wide.is_none() && short.is_none() {
                    None
                } else {
                    Some(Self {
                        abbreviated,
                        narrow,
                        short,
                        wide,
                    })
                }
            }
        }

        impl From<&cldr_serde::ca::$name::FormatWidths> for $name2::FormatWidthsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::FormatWidths) -> Self {
                Self {
                    abbreviated: (&other.abbreviated).into(),
                    narrow: (&other.narrow).into(),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: (&other.wide).into(),
                }
            }
        }

        impl From<&cldr_serde::ca::$name::StandAloneWidths> for $name2::StandAloneWidthsV1<'static> {
            fn from(other: &cldr_serde::ca::$name::StandAloneWidths) -> Self {
                Self {
                    abbreviated: other.abbreviated.as_ref().map(|width| width.into()),
                    narrow: other.narrow.as_ref().map(|width| width.into()),
                    short: other.short.as_ref().map(|width| width.into()),
                    wide: other.wide.as_ref().map(|width| width.into()),
                }
            }
        }
    };
}

symbols_from!(
    [months, months],
    [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12],
);

symbols_from!([days, weekdays], [sun, mon, tue, wed, thu, fri, sat]);

symbols_from!(
    [
        day_periods,
        day_periods,
    ],
    {
        am,
        pm,
        noon,
        midnight,
    },
);
