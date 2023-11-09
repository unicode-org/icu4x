// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON date time display name files.
//!
//! CLDR JSON files having this structure include "ca-gregorian.json", "ca-buddhist.json", etc.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>

use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FormatWidths<Symbols> {
    pub abbreviated: Symbols,
    pub narrow: Symbols,
    pub short: Option<Symbols>,
    pub wide: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct StandAloneWidths<Symbols> {
    pub abbreviated: Option<Symbols>,
    pub narrow: Option<Symbols>,
    pub short: Option<Symbols>,
    pub wide: Option<Symbols>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Numeric<Symbols> {
    pub all: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Contexts<Symbols> {
    pub format: FormatWidths<Symbols>,
    #[serde(rename = "stand-alone")]
    pub stand_alone: Option<StandAloneWidths<Symbols>>,
    // currently only found on monthPatterns
    pub numeric: Option<Numeric<Symbols>>,
}

/// A length, for querying Contexts
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Length {
    Abbr,
    Narrow,
    Wide,
    Short,
    Numeric,
}

/// A context, for querying Contexts
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Context {
    Format,
    Standalone,
}

impl<Symbols> Contexts<Symbols> {
    fn get_symbols_exact(&self, context: Context, length: Length) -> Option<&Symbols> {
        use {Context::*, Length::*};

        match (context, length) {
            (_, Numeric) => self.numeric.as_ref().map(|n| &n.all),
            (Format, Abbr) => Some(&self.format.abbreviated),
            (Format, Narrow) => Some(&self.format.narrow),
            (Format, Wide) => Some(&self.format.wide),
            (Format, Short) => self.format.short.as_ref(),
            (Standalone, Abbr) => self
                .stand_alone
                .as_ref()
                .and_then(|s| s.abbreviated.as_ref()),
            (Standalone, Narrow) => self.stand_alone.as_ref().and_then(|s| s.narrow.as_ref()),
            (Standalone, Wide) => self.stand_alone.as_ref().and_then(|s| s.wide.as_ref()),
            (Standalone, Short) => self.stand_alone.as_ref().and_then(|s| s.short.as_ref()),
        }
    }

    /// Load the symbols for a given context/length pair, performing horizontal fallback
    /// if necessary
    ///
    /// Horizontal fallback is performed as specified in
    /// <https://unicode.org/reports/tr35/tr35-dates.html#months_days_quarters_eras>
    ///
    /// I.e. missing `standalone`s fall back to `format`, missing `short` falls back to
    /// `abbr`.
    pub fn get_symbols(&self, context: Context, length: Length) -> &Symbols {
        if context == Context::Standalone {
            if let Some(sym) = self.get_symbols_exact(context, length) {
                return sym;
            }
            // fall back to format
        }

        if let Some(sym) = self.get_symbols_exact(Context::Format, length) {
            return sym;
        }

        // The only case where we reach this far without error is when we're looking
        // for short lengths
        debug_assert!(
            length == Length::Short,
            "Short is the only nullable format length!"
        );

        &self.format.abbreviated
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MonthSymbols(pub HashMap<String, String>);
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MonthPatternSymbols {
    pub leap: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DaySymbols {
    pub sun: String,
    pub mon: String,
    pub tue: String,
    pub wed: String,
    pub thu: String,
    pub fri: String,
    pub sat: String,
}

// The day period symbols are Cow<'static, str> instead of String because the Option
// needs to be retained when converting them into Cow for the data provider.
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DayPeriodSymbols {
    pub am: Cow<'static, str>,
    pub pm: Cow<'static, str>,
    pub noon: Option<Cow<'static, str>>,
    pub midnight: Option<Cow<'static, str>>,
}

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

#[derive(PartialEq, Debug, Deserialize, Clone, Default)]
pub struct Eras {
    #[serde(rename = "eraNames")]
    pub names: HashMap<String, String>,
    #[serde(rename = "eraAbbr")]
    pub abbr: HashMap<String, String>,
    #[serde(rename = "eraNarrow")]
    pub narrow: HashMap<String, String>,
}

impl Eras {
    /// Load the era corresponding to a [`Length`] value
    ///
    /// Panics on Length::Short
    pub(crate) fn load(&self, length: Length) -> &HashMap<String, String> {
        match length {
            Length::Abbr => &self.abbr,
            Length::Narrow => &self.narrow,
            Length::Wide => &self.names,
            Length::Short | Length::Numeric => {
                unreachable!("Years do not have short/numeric symbols!")
            }
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PatternLength {
    Full,
    Long,
    Medium,
    Short,
}

impl LengthPatterns {
    pub fn get_pattern(&self, length: PatternLength) -> &LengthPattern {
        match length {
            PatternLength::Full => &self.full,
            PatternLength::Long => &self.long,
            PatternLength::Medium => &self.medium,
            PatternLength::Short => &self.short,
        }
    }
}

impl DateTimeFormats {
    pub(crate) fn get_pattern(&self, length: PatternLength) -> &LengthPattern {
        match length {
            PatternLength::Full => &self.full,
            PatternLength::Long => &self.long,
            PatternLength::Medium => &self.medium,
            PatternLength::Short => &self.short,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct AvailableFormats(pub HashMap<String, String>);

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct CyclicNameSets {
    pub years: Option<Contexts<BTreeMap<u8, String>>>,
}

/// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
/// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
///
/// e.g.
/// <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub struct Dates {
    pub months: Contexts<MonthSymbols>,
    #[serde(rename = "monthPatterns")]
    pub month_patterns: Option<Contexts<MonthPatternSymbols>>,
    pub days: Contexts<DaySymbols>,
    pub eras: Option<Eras>,
    #[serde(rename = "cyclicNameSets")]
    pub cyclic_name_sets: Option<CyclicNameSets>,
    #[serde(rename = "dayPeriods")]
    pub day_periods: Contexts<DayPeriodSymbols>,
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

pub type Resource = super::LocaleResource<LangDates>;
