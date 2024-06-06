// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON date time display name files.
//!
//! CLDR JSON files having this structure include "ca-gregorian.json", "ca-buddhist.json", etc.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>

use icu_datetime::provider::neo::marker_attrs::{Context, Length, PatternLength};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct FormatWidths<Symbols> {
    pub(in crate::provider) abbreviated: Symbols,
    pub(in crate::provider) narrow: Symbols,
    pub(in crate::provider) short: Option<Symbols>,
    pub(in crate::provider) wide: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct StandAloneWidths<Symbols> {
    pub(in crate::provider) abbreviated: Option<Symbols>,
    pub(in crate::provider) narrow: Option<Symbols>,
    pub(in crate::provider) short: Option<Symbols>,
    pub(in crate::provider) wide: Option<Symbols>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct Numeric<Symbols> {
    pub(in crate::provider) all: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct Contexts<Symbols> {
    pub(in crate::provider) format: FormatWidths<Symbols>,
    #[serde(rename = "stand-alone")]
    pub(in crate::provider) stand_alone: Option<StandAloneWidths<Symbols>>,
    // currently only found on monthPatterns
    pub(in crate::provider) numeric: Option<Numeric<Symbols>>,
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
    pub(in crate::provider) fn get_symbols(&self, context: Context, length: Length) -> &Symbols {
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
pub(in crate::provider) struct MonthSymbols(pub(in crate::provider) HashMap<String, String>);
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct MonthPatternSymbols {
    pub(in crate::provider) leap: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct DaySymbols {
    pub(in crate::provider) sun: String,
    pub(in crate::provider) mon: String,
    pub(in crate::provider) tue: String,
    pub(in crate::provider) wed: String,
    pub(in crate::provider) thu: String,
    pub(in crate::provider) fri: String,
    pub(in crate::provider) sat: String,
}

// The day period symbols are Cow<'static, str> instead of String because the Option
// needs to be retained when converting them into Cow for the data provider.
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(in crate::provider) struct DayPeriodSymbols {
    pub(in crate::provider) am: Cow<'static, str>,
    pub(in crate::provider) pm: Cow<'static, str>,
    pub(in crate::provider) noon: Option<Cow<'static, str>>,
    pub(in crate::provider) midnight: Option<Cow<'static, str>>,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(in crate::provider) enum LengthPattern {
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
    pub(in crate::provider) fn get_pattern(&self) -> &String {
        match self {
            Self::Plain(pattern) => pattern,
            Self::WithNumberingSystems { pattern, .. } => pattern,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Clone, Default)]
pub(in crate::provider) struct Eras {
    #[serde(rename = "eraNames")]
    pub(in crate::provider) names: HashMap<String, String>,
    #[serde(rename = "eraAbbr")]
    pub(in crate::provider) abbr: HashMap<String, String>,
    #[serde(rename = "eraNarrow")]
    pub(in crate::provider) narrow: HashMap<String, String>,
}

impl Eras {
    /// Load the era corresponding to a [`Length`] value
    ///
    /// Panics on Length::Short
    pub(in crate::provider) fn load(&self, length: Length) -> &HashMap<String, String> {
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
pub(in crate::provider) struct LengthPatterns {
    pub(in crate::provider) full: LengthPattern,
    pub(in crate::provider) long: LengthPattern,
    pub(in crate::provider) medium: LengthPattern,
    pub(in crate::provider) short: LengthPattern,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(in crate::provider) struct DateTimeFormats {
    pub(in crate::provider) full: LengthPattern,
    pub(in crate::provider) long: LengthPattern,
    pub(in crate::provider) medium: LengthPattern,
    pub(in crate::provider) short: LengthPattern,
    #[serde(rename = "availableFormats")]
    pub(in crate::provider) available_formats: AvailableFormats,
}

impl LengthPatterns {
    pub(in crate::provider) fn get_pattern(&self, length: PatternLength) -> &LengthPattern {
        match length {
            PatternLength::Full => &self.full,
            PatternLength::Long => &self.long,
            PatternLength::Medium => &self.medium,
            PatternLength::Short => &self.short,
        }
    }
}

impl DateTimeFormats {
    pub(in crate::provider) fn get_pattern(&self, length: PatternLength) -> &LengthPattern {
        match length {
            PatternLength::Full => &self.full,
            PatternLength::Long => &self.long,
            PatternLength::Medium => &self.medium,
            PatternLength::Short => &self.short,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub(in crate::provider) struct AvailableFormats(pub(in crate::provider) HashMap<String, String>);

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub(in crate::provider) struct CyclicNameSets {
    pub(in crate::provider) years: Option<Contexts<BTreeMap<u8, String>>>,
}

/// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
/// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
///
/// e.g.
/// <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(in crate::provider) struct Dates {
    pub(in crate::provider) months: Contexts<MonthSymbols>,
    #[serde(rename = "monthPatterns")]
    pub(in crate::provider) month_patterns: Option<Contexts<MonthPatternSymbols>>,
    pub(in crate::provider) days: Contexts<DaySymbols>,
    pub(in crate::provider) eras: Option<Eras>,
    #[serde(rename = "cyclicNameSets")]
    pub(in crate::provider) cyclic_name_sets: Option<CyclicNameSets>,
    #[serde(rename = "dayPeriods")]
    pub(in crate::provider) day_periods: Contexts<DayPeriodSymbols>,
    #[serde(rename = "dateFormats")]
    pub(in crate::provider) date_formats: LengthPatterns,
    #[serde(rename = "timeFormats")]
    pub(in crate::provider) time_formats: LengthPatterns,
    #[serde(rename = "dateTimeFormats")]
    pub(in crate::provider) datetime_formats: DateTimeFormats,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct DatesCalendars {
    pub(in crate::provider) calendars: HashMap<String, Dates>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct LangDates {
    pub(in crate::provider) dates: DatesCalendars,
}

pub(in crate::provider) type Resource = super::LocaleResource<LangDates>;
