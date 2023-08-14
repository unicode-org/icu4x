// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde::{self, ca};
use icu_calendar::types::MonthCode;
use icu_datetime::provider::calendar::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tinystr::{tinystr, TinyStr16, TinyStr4};

pub fn convert_dates(other: &cldr_serde::ca::Dates, calendar: &str) -> DateSymbolsV1<'static> {
    DateSymbolsV1 {
        months: other.months.get(&(get_month_code_map(calendar), calendar)),
        weekdays: other.days.get(&()),
        eras: convert_eras(&other.eras, calendar),
    }
}

pub fn convert_times(other: &cldr_serde::ca::Dates) -> TimeSymbolsV1<'static> {
    TimeSymbolsV1 {
        day_periods: other.day_periods.get(&()),
    }
}

fn convert_eras(eras: &cldr_serde::ca::Eras, calendar: &str) -> Eras<'static> {
    let map = get_era_code_map(calendar);
    let mut out_eras = Eras::default();

    for (cldr, code) in map.into_iter() {
        if let Some(name) = eras.names.get(&cldr) {
            out_eras.names.insert(code.as_str().into(), name);
        }
        if let Some(abbr) = eras.abbr.get(&cldr) {
            out_eras.abbr.insert(code.as_str().into(), abbr);
        }
        if let Some(narrow) = eras.narrow.get(&cldr) {
            out_eras.narrow.insert(code.as_str().into(), narrow);
        }
    }
    out_eras
}

fn get_month_code_map(calendar: &str) -> &'static [TinyStr4] {
    // This will need to be more complicated to handle lunar calendars
    // https://github.com/unicode-org/icu4x/issues/2066
    static SOLAR_MONTH_CODES: &[TinyStr4] = &[
        tinystr!(4, "M01"),
        tinystr!(4, "M02"),
        tinystr!(4, "M03"),
        tinystr!(4, "M04"),
        tinystr!(4, "M05"),
        tinystr!(4, "M06"),
        tinystr!(4, "M07"),
        tinystr!(4, "M08"),
        tinystr!(4, "M09"),
        tinystr!(4, "M10"),
        tinystr!(4, "M11"),
        tinystr!(4, "M12"),
        tinystr!(4, "M13"),
    ];
    // CLDR labels the regular months and M05L by their ordinals
    // whereas M06L is stored as 7-yeartype-leap
    static HEBREW_MONTH_CODES: &[TinyStr4] = &[
        tinystr!(4, "M01"),
        tinystr!(4, "M02"),
        tinystr!(4, "M03"),
        tinystr!(4, "M04"),
        tinystr!(4, "M05"),
        tinystr!(4, "M05L"),
        tinystr!(4, "M06"),
        tinystr!(4, "M07"),
        tinystr!(4, "M08"),
        tinystr!(4, "M09"),
        tinystr!(4, "M10"),
        tinystr!(4, "M11"),
        tinystr!(4, "M12"),
        // M06L is handled separately in MonthSymbols code
    ];
    match calendar {
        "gregory" | "buddhist" | "japanese" | "japanext" | "indian" | "persian" | "roc"
        | "islamic" | "islamicc" | "umalqura" | "tbla" => &SOLAR_MONTH_CODES[0..12],
        "coptic" | "ethiopic" | "chinese" | "dangi" => SOLAR_MONTH_CODES,
        "hebrew" => HEBREW_MONTH_CODES,
        _ => panic!("Month map unknown for {calendar}"),
    }
}

fn get_era_code_map(calendar: &str) -> BTreeMap<String, TinyStr16> {
    match calendar {
        "gregory" => vec![
            ("0".to_string(), tinystr!(16, "bce")),
            ("1".to_string(), tinystr!(16, "ce")),
        ]
        .into_iter()
        .collect(),
        "buddhist" => vec![("0".to_string(), tinystr!(16, "be"))]
            .into_iter()
            .collect(),
        "chinese" => vec![].into_iter().collect(),
        "japanese" | "japanext" => crate::transform::cldr::calendar::japanese::get_era_code_map(),
        "coptic" => vec![
            // Before Diocletian
            ("0".to_string(), tinystr!(16, "bd")),
            // Anno Diocletian/After Diocletian
            ("1".to_string(), tinystr!(16, "ad")),
        ]
        .into_iter()
        .collect(),
        "dangi" => vec![].into_iter().collect(),
        "indian" => vec![("0".to_string(), tinystr!(16, "saka"))]
            .into_iter()
            .collect(),
        "islamic" => vec![("0".to_string(), tinystr!(16, "islamic"))]
            .into_iter()
            .collect(),
        "islamicc" => vec![("0".to_string(), tinystr!(16, "islamic"))]
            .into_iter()
            .collect(),
        "umalqura" => vec![("0".to_string(), tinystr!(16, "islamic"))]
            .into_iter()
            .collect(),
        "tbla" => vec![("0".to_string(), tinystr!(16, "islamic"))]
            .into_iter()
            .collect(),
        "persian" => vec![("0".to_string(), tinystr!(16, "ah"))]
            .into_iter()
            .collect(),
        "hebrew" => vec![("0".to_string(), tinystr!(16, "hebrew"))]
            .into_iter()
            .collect(),
        "ethiopic" => vec![
            ("0".to_string(), tinystr!(16, "incar")),
            ("1".to_string(), tinystr!(16, "pre-incar")),
            ("2".to_string(), tinystr!(16, "mundi")),
        ]
        .into_iter()
        .collect(),
        "roc" => vec![
            ("0".to_string(), tinystr!(16, "roc-inverse")),
            ("1".to_string(), tinystr!(16, "roc")),
        ]
        .into_iter()
        .collect(),
        _ => panic!("Era map unknown for {calendar}"),
    }
}

macro_rules! symbols_from {
    ([$symbols: path, $name2: ident $(,)?], $ctx:ty, [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl $symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::SymbolsV1<'static> {
                $name2::SymbolsV1([
                    $(
                        Cow::Owned(self.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$symbols, $name2], $ctx);
    };
    ([$symbols: path, $name2: ident $(,)?], $ctx:ty, { $($element: ident),+ $(,)? } $(,)?) => {
        impl $symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::SymbolsV1<'static> {
                $name2::SymbolsV1 {
                    $(
                        $element: self.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$symbols, $name2], $ctx);
    };
    ([$symbols: path, $name2: ident], $ctx:ty) => {
        impl $symbols {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl ca::Contexts<$symbols> {
            fn get(&self, ctx: &$ctx) -> $name2::ContextsV1<'static> {
                $name2::ContextsV1 {
                    format: self.format.get(ctx),
                    stand_alone: self.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&self.format)
                    }).map(|ref stand_alone| stand_alone.get(ctx))
                }
            }
        }

        impl ca::StandAloneWidths<$symbols> {
            // Helper function which returns None if the two groups of symbols overlap.
            pub fn get_unaliased(&self, other: &ca::FormatWidths<$symbols>) -> Option<Self> {
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

        impl ca::FormatWidths<$symbols> {
            fn get(&self, ctx: &$ctx) -> $name2::FormatWidthsV1<'static> {
                $name2::FormatWidthsV1 {
                    abbreviated: self.abbreviated.get(ctx),
                    narrow: self.narrow.get(ctx),
                    short: self.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.wide.get(ctx),
                }
            }
        }

        impl ca::StandAloneWidths<$symbols> {
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
    [cldr_serde::ca::MonthSymbols, months],
    (&'static [TinyStr4], &str)
);

impl cldr_serde::ca::MonthSymbols {
    fn get(&self, ctx: &(&'static [TinyStr4], &str)) -> months::SymbolsV1<'static> {
        if ctx.0.len() == 12 && self.0.len() == 12 {
            let mut arr: [Cow<'static, str>; 12] = Default::default();
            for (k, v) in self.0.iter() {
                let index: usize = k
                    .parse()
                    .expect("CLDR month indices must parse as numbers!");
                if index == 0 {
                    panic!("CLDR month indices cannot be zero");
                }

                arr[index - 1] = Cow::Owned(v.into());
            }

            for (i, val) in arr.iter().enumerate() {
                if val.is_empty() {
                    panic!("Solar calendar does not have data for month {i}");
                }
            }
            months::SymbolsV1::SolarTwelve(arr)
        } else {
            let mut map = BTreeMap::new();
            for (k, v) in self.0.iter() {
                let code = if k == "7-yeartype-leap" && ctx.1 == "hebrew" {
                    tinystr!(4, "M06L")
                } else {
                    let index: usize = k
                        .parse()
                        .expect("CLDR month indices must parse as numbers!");

                    if index == 0 {
                        panic!("CLDR month indices cannot be zero");
                    }
                    *ctx.0
                        .get(index - 1)
                        .expect("Found out of bounds month index for calendar")
                };

                map.insert(MonthCode(code), v.as_ref());
            }
            months::SymbolsV1::Other(map.into_iter().collect())
        }
    }
}

symbols_from!(
    [cldr_serde::ca::DaySymbols, weekdays],
    (),
    [sun, mon, tue, wed, thu, fri, sat]
);

symbols_from!(
    [
        cldr_serde::ca::DayPeriodSymbols,
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
