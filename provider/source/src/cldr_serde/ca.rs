// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON date time display name files.
//!
//! CLDR JSON files having this structure include "ca-gregorian.json", "ca-buddhist.json", etc.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>

use icu::datetime::provider::neo::marker_attrs::{Context, Length, PatternLength};
use icu_pattern::PatternString;
use icu_pattern::SinglePlaceholder;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct FormatWidths<Symbols> {
    pub(crate) abbreviated: Symbols,
    pub(crate) narrow: Symbols,
    pub(crate) short: Option<Symbols>,
    pub(crate) wide: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct StandAloneWidths<Symbols> {
    pub(crate) abbreviated: Option<Symbols>,
    pub(crate) narrow: Option<Symbols>,
    pub(crate) short: Option<Symbols>,
    pub(crate) wide: Option<Symbols>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct Numeric<Symbols> {
    pub(crate) all: Symbols,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct Contexts<Symbols> {
    pub(crate) format: FormatWidths<Symbols>,
    #[serde(rename = "stand-alone")]
    pub(crate) stand_alone: Option<StandAloneWidths<Symbols>>,
    // currently only found on monthPatterns
    pub(crate) numeric: Option<Numeric<Symbols>>,
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
    pub(crate) fn get_symbols(&self, context: Context, length: Length) -> &Symbols {
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
pub(crate) struct MonthSymbols(pub(crate) HashMap<String, String>);
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct MonthPatternSymbols {
    pub(crate) leap: PatternString<SinglePlaceholder>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct DaySymbols {
    pub(crate) sun: String,
    pub(crate) mon: String,
    pub(crate) tue: String,
    pub(crate) wed: String,
    pub(crate) thu: String,
    pub(crate) fri: String,
    pub(crate) sat: String,
}

// The day period symbols are Cow<'static, str> instead of String because the Option
// needs to be retained when converting them into Cow for the data provider.
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub(crate) struct DayPeriodSymbols {
    pub(crate) am: Cow<'static, str>,
    pub(crate) pm: Cow<'static, str>,
    pub(crate) noon: Option<Cow<'static, str>>,
    pub(crate) midnight: Option<Cow<'static, str>>,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum LengthPattern {
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
    pub(crate) fn get_pattern(&self) -> &String {
        match self {
            Self::Plain(pattern) => pattern,
            Self::WithNumberingSystems { pattern, .. } => pattern,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Clone, Default)]
pub(crate) struct Eras {
    #[serde(rename = "eraNames")]
    pub(crate) names: HashMap<String, String>,
    #[serde(rename = "eraAbbr")]
    pub(crate) abbr: HashMap<String, String>,
    #[serde(rename = "eraNarrow")]
    pub(crate) narrow: HashMap<String, String>,
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
pub(crate) struct LengthPatterns {
    pub(crate) full: LengthPattern,
    pub(crate) long: LengthPattern,
    pub(crate) medium: LengthPattern,
    pub(crate) short: LengthPattern,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct DateTimeFormats {
    pub(crate) full: LengthPattern,
    pub(crate) long: LengthPattern,
    pub(crate) medium: LengthPattern,
    pub(crate) short: LengthPattern,
    #[serde(rename = "availableFormats")]
    pub(crate) available_formats: AvailableFormats,
    #[serde(rename = "appendItems")]
    pub(crate) append_items: AppendItems,
}

#[derive(PartialEq, Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct AppendItems {
    #[serde(rename = "Day")]
    pub(crate) day: String,
    #[serde(rename = "Day-Of-Week")]
    pub(crate) day_of_week: String,
    #[serde(rename = "Era")]
    pub(crate) era: String,
    #[serde(rename = "Hour")]
    pub(crate) hour: String,
    #[serde(rename = "Minute")]
    pub(crate) minute: String,
    #[serde(rename = "Month")]
    pub(crate) month: String,
    #[serde(rename = "Quarter")]
    pub(crate) quarter: String,
    #[serde(rename = "Second")]
    pub(crate) second: String,
    #[serde(rename = "Timezone")]
    pub(crate) timezone: String,
    #[serde(rename = "Week")]
    pub(crate) week: String,
    #[serde(rename = "Year")]
    pub(crate) year: String,
}

/// dateTimeFormats-atTime, dateTimeFormats-relative
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct DateTimeFormatsVariant {
    pub(crate) standard: LengthPatterns,
}

impl DateTimeFormatsVariant {
    pub(crate) fn get_pattern(&self, length: PatternLength) -> &LengthPattern {
        match length {
            PatternLength::Long => &self.standard.long,
            PatternLength::Medium => &self.standard.medium,
            PatternLength::Short => &self.standard.short,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub(crate) struct AvailableFormats(pub(crate) HashMap<String, String>);

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub(crate) struct CyclicNameSets {
    pub(crate) years: Option<Contexts<BTreeMap<u8, String>>>,
}

/// This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key
/// "main.LANGID.dates.calendars.gregorian" where "LANGID" is the identifier.
///
/// e.g.
/// <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>
#[derive(PartialEq, Debug, Deserialize, Clone)]
pub(crate) struct Dates {
    pub(crate) months: Contexts<MonthSymbols>,
    #[serde(rename = "monthPatterns")]
    pub(crate) month_patterns: Option<Contexts<MonthPatternSymbols>>,
    pub(crate) days: Contexts<DaySymbols>,
    pub(crate) eras: Option<Eras>,
    #[serde(rename = "cyclicNameSets")]
    pub(crate) cyclic_name_sets: Option<CyclicNameSets>,
    #[serde(rename = "dayPeriods")]
    pub(crate) day_periods: Contexts<DayPeriodSymbols>,
    #[serde(rename = "dateFormats")]
    pub(crate) date_formats: LengthPatterns,
    #[serde(rename = "timeFormats")]
    pub(crate) time_formats: LengthPatterns,
    #[serde(rename = "dateSkeletons")]
    pub(crate) date_skeletons: LengthPatterns,
    #[serde(rename = "timeSkeletons")]
    pub(crate) time_skeletons: LengthPatterns,
    #[serde(rename = "dateTimeFormats")]
    pub(crate) datetime_formats: DateTimeFormats,
    #[serde(rename = "dateTimeFormats-atTime")]
    pub(crate) datetime_formats_at_time: DateTimeFormatsVariant,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DatesCalendars {
    pub(crate) calendars: HashMap<String, Dates>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangDates {
    pub(crate) dates: DatesCalendars,
}

pub(crate) type Resource = super::LocaleResource<LangDates>;
