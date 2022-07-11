// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON date time display name files.
//!
//! CLDR JSON files having this structure include "ca-gregorian.json", "ca-buddhist.json", etc.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;

macro_rules! symbols {
    ($name: ident, $symbols:item) => {
        pub mod $name {
            use super::*;

            #[derive(Debug, PartialEq, Clone, Deserialize)]
            $symbols

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

symbols!(months, pub struct Symbols(pub HashMap<String, String>););

symbols!(
    days,
    pub struct Symbols {
        pub sun: String,
        pub mon: String,
        pub tue: String,
        pub wed: String,
        pub thu: String,
        pub fri: String,
        pub sat: String,
    }
);

// The day period symbols are Cow<'static, str> instead of String because the Option
// needs to be retained when converting them into Cow for the data provider.
symbols!(
    day_periods,
    pub struct Symbols {
        pub am: Cow<'static, str>,
        pub pm: Cow<'static, str>,
        pub noon: Option<Cow<'static, str>>,
        pub midnight: Option<Cow<'static, str>>,
    }
);

#[derive(PartialEq, Debug, Deserialize, Clone)]
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

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Eras {
    #[serde(rename = "eraNames")]
    pub names: HashMap<String, String>,
    #[serde(rename = "eraAbbr")]
    pub abbr: HashMap<String, String>,
    #[serde(rename = "eraNarrow")]
    pub narrow: HashMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct LengthPatterns {
    pub full: LengthPattern,
    pub long: LengthPattern,
    pub medium: LengthPattern,
    pub short: LengthPattern,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct DateTimeFormats {
    pub full: LengthPattern,
    pub long: LengthPattern,
    pub medium: LengthPattern,
    pub short: LengthPattern,
    #[serde(rename = "availableFormats")]
    pub available_formats: AvailableFormats,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct AvailableFormats(pub HashMap<String, String>);

/// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
/// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
///
/// e.g.
/// https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Dates {
    pub months: months::Contexts,
    pub days: days::Contexts,
    pub eras: Eras,
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
pub struct DatesCalendars {
    pub calendars: HashMap<String, Dates>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangDates {
    pub dates: DatesCalendars,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, LangDates>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
