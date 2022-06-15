// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_datetime::provider::calendar::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tinystr::{tinystr, TinyStr16};

pub fn convert_dates(other: &cldr_serde::ca::Dates, calendar: &str) -> DateSymbolsV1<'static> {
    DateSymbolsV1 {
        months: other.months.get(&()),
        weekdays: other.days.get(&()),
        day_periods: other.day_periods.get(&()),
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

fn get_month_code_map(calendar: &str) -> &'static [&'static str] {
    const THIRTEEN_MONTH_CODES: &[&str] = &[
        "M01", "M02", "M03", "M04", "M05", "M06", "M07", "M08", "M09", "M10", "M11", "M12", "M13",
    ];
    const TWELVE_MONTH_CODES: &[&str] = &THIRTEEN_MONTH_CODES[..12];

    match calendar {
        "gregory" | "buddhist" | "japanese" | "indian" => TWELVE_MONTH_CODES,
        "coptic" | "ethiopic" => THIRTEEN_MONTH_CODES,
        #[allow(clippy::panic)] // Panics okay in datagen
        _ => panic!("Month map unknown for {}", calendar),
    }
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
        "japanese" => crate::transform::cldr::calendar::japanese::get_era_code_map(),
        "coptic" => vec![
            ("0".to_string(), tinystr!(16, "bc")),
            ("1".to_string(), tinystr!(16, "ad")),
        ]
        .into_iter()
        .collect(),
        "indian" => vec![("0".to_string(), tinystr!(16, "saka"))]
            .into_iter()
            .collect(),
        "ethiopic" => vec![
            ("0".to_string(), tinystr!(16, "incarnation")),
            ("1".to_string(), tinystr!(16, "before-incar")),
            ("2".to_string(), tinystr!(16, "mundi")),
        ]
        .into_iter()
        .collect(),
        #[allow(clippy::panic)] // Panics okay in datagen
        _ => panic!("Era map unknown for {}", calendar),
    }
}

macro_rules! symbols_from {
    ([$name: ident, $name2: ident $(,)?], $ctx:ty, [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl cldr_serde::ca::$name::Symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::SymbolsV1<'static> {
                $name2::SymbolsV1([
                    $(
                        Cow::Owned(self.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$name, $name2], $ctx);
    };
    ([$name: ident, $name2: ident $(,)?], $ctx:ty, { $($element: ident),+ $(,)? } $(,)?) => {
        impl cldr_serde::ca::$name::Symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::SymbolsV1<'static> {
                $name2::SymbolsV1 {
                    $(
                        $element: self.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$name, $name], $ctx);
    };
    ([$name: ident, $name2: ident], $ctx:ty) => {
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

        impl cldr_serde::ca::$name::Contexts {
            fn get(&self, ctx: &$ctx) -> $name2::ContextsV1<'static> {
                $name2::ContextsV1 {
                    format: self.format.get(ctx),
                    stand_alone: self.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&self.format)
                    }).map(|ref stand_alone| stand_alone.get(ctx))
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

        impl cldr_serde::ca::$name::FormatWidths {
            fn get(&self, ctx: &$ctx) -> $name2::FormatWidthsV1<'static> {
                $name2::FormatWidthsV1 {
                    abbreviated: self.abbreviated.get(ctx),
                    narrow: self.narrow.get(ctx),
                    short: self.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.wide.get(ctx),
                }
            }
        }

        impl cldr_serde::ca::$name::StandAloneWidths {
            fn get(&self, ctx: &$ctx) -> $name2::StandAloneWidthsV1<'static> {
                $name2::StandAloneWidthsV1 {
                    abbreviated: self.abbreviated.as_ref().map(|width| width.get(ctx)),
                    narrow: self.narrow.as_ref().map(|width| width.get(ctx)),
                    short: self.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.wide.as_ref().map(|width| width.get(ctx)),
                }
            }
        }
    };
}
symbols_from!(
    [months, months],
    (),
    [m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12],
);

symbols_from!([days, weekdays], (), [sun, mon, tue, wed, thu, fri, sat]);

symbols_from!(
    [
        day_periods,
        day_periods,
    ],
    (),
    {
        am,
        pm,
        noon,
        midnight,
    },
);
