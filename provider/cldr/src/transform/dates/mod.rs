// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod patterns;
pub mod symbols;

/// Serde structs for the CLDR JSON dates files.
pub(self) mod cldr_json {
    use crate::cldr_langid::CldrLangID;
    use serde::Deserialize;
    use std::borrow::Cow;

    macro_rules! symbols {
        ($name: ident, $([$alias: expr, $element: ident, $ty: ty]),+ $(,)?) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize)]
                pub struct Symbols {
                    $(
                        #[serde(rename = $alias)]
                        pub $element: $ty
                    ),*
                }

                symbols!();
            }
        };
        ($name: ident, $([$element: ident, $ty: ty]),+ $(,)?) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Deserialize)]
                pub struct Symbols {
                    $(pub $element: $ty),*
                }

                symbols!();
            }
        };
        () => {
            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct FormatWidths {
                pub abbreviated: Symbols,
                pub narrow: Symbols,
                pub short: Option<Symbols>,
                pub wide: Symbols,
            }

            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct StandAloneWidths {
                pub abbreviated: Option<Symbols>,
                pub narrow: Option<Symbols>,
                pub short: Option<Symbols>,
                pub wide: Option<Symbols>,
            }

            #[derive(Debug, PartialEq, Clone, Deserialize)]
            pub struct Contexts {
                pub format: FormatWidths,
                #[serde(rename = "stand-alone")]
                pub stand_alone: Option<StandAloneWidths>,
            }
        }
    }

    symbols!(
        months,
        ["1", m1, String],
        ["2", m2, String],
        ["3", m3, String],
        ["4", m4, String],
        ["5", m5, String],
        ["6", m6, String],
        ["7", m7, String],
        ["8", m8, String],
        ["9", m9, String],
        ["10", m10, String],
        ["11", m11, String],
        ["12", m12, String],
    );

    symbols!(
        days,
        [sun, String],
        [mon, String],
        [tue, String],
        [wed, String],
        [thu, String],
        [fri, String],
        [sat, String],
    );

    // The day period symbols are Cow<'static, str> instead of String because the Option
    // needs to be retained when converting them into Cow for the data provider.
    symbols!(
        day_periods,
        ["am", am, Cow<'static, str>],
        ["pm", pm, Cow<'static, str>],
        ["noon", noon, Option<Cow<'static, str>>],
        ["midnight", midnight, Option<Cow<'static, str>>],
    );

    #[derive(PartialEq, Debug, Deserialize)]
    #[serde(untagged)]
    pub enum LengthPattern {
        Plain(String),
        WithNumberingSystems {
            #[serde(rename = "_value")]
            pattern: String,
            #[serde(rename = "_numbers")]
            numbering_systems: String,
        },
    }

    impl LengthPattern {
        /// Get the pattern, dropping the numbering system if present.
        pub fn get_pattern(&self) -> &String {
            match self {
                Self::Plain(pattern) => pattern,
                Self::WithNumberingSystems { pattern, .. } => pattern,
            }
        }
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LengthPatterns {
        pub full: LengthPattern,
        pub long: LengthPattern,
        pub medium: LengthPattern,
        pub short: LengthPattern,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct DateTimeFormats {
        pub full: LengthPattern,
        pub long: LengthPattern,
        pub medium: LengthPattern,
        pub short: LengthPattern,
        #[serde(rename = "availableFormats")]
        pub available_formats: AvailableFormats,
    }

    #[derive(PartialEq, Clone, Debug, Deserialize)]
    pub struct AvailableFormats(
        #[serde(with = "tuple_vec_map")] pub(crate) Vec<(Cow<'static, str>, Cow<'static, str>)>,
    );

    /// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
    /// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
    ///
    /// e.g.
    /// https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json
    #[derive(PartialEq, Debug, Deserialize)]
    pub struct GregoryDates {
        pub months: months::Contexts,
        pub days: days::Contexts,
        #[serde(rename = "dayPeriods")]
        pub day_periods: day_periods::Contexts,
        #[serde(rename = "dateFormats")]
        pub date_formats: LengthPatterns,
        #[serde(rename = "timeFormats")]
        pub time_formats: LengthPatterns,
        #[serde(rename = "dateTimeFormats")]
        pub datetime_formats: DateTimeFormats,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Calendars {
        pub gregorian: GregoryDates,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Dates {
        pub calendars: Calendars,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangDates {
        pub dates: Dates,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangData(#[serde(with = "tuple_vec_map")] pub(crate) Vec<(CldrLangID, LangDates)>);

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}
