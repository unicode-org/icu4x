// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod data_provider;
mod options;

use data_provider::{get_default_prefs, resolve_options};
use icu_locale_core::extensions::unicode;
use icu_preferences::{
    extensions::unicode::{errors::PreferencesParseError, keywords},
    options, preferences,
};
use options::{DayPeriod, LocaleMatcher};
use tinystr::TinyAsciiStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DatePattern(pub TinyAsciiStr<8>);

impl preferences::PreferenceKey for DatePattern {}

impl TryFrom<unicode::Value> for DatePattern {
    type Error = PreferencesParseError;

    fn try_from(_: unicode::Value) -> Result<Self, Self::Error> {
        Err(Self::Error::UnknownKeyword)
    }
}

preferences!(
    /// The locale preferences for datetime formatting.
    DateTimeFormatPreferences,
    DateTimeFormatResolvedPreferences,
    {
        /// The hour cycle
        hour_cycle: keywords::HourCycle,
        /// The calendar
        calendar: keywords::CalendarAlgorithm,
        /// The numbering system
        numbering_system: keywords::NumberingSystem,
        /// The date pattern
        date_pattern: DatePattern
    }
);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DateLength {
    Full,
    Long,
    Medium,
    Short,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TimeLength {
    Full,
    Long,
    Medium,
    Short,
}

options!(
    DateTimeFormatOptions,
    DateTimeFormatResolvedOptions,
    {
        date_length => DateLength,
        time_length => TimeLength,
        day_period => DayPeriod,
        locale_matcher => LocaleMatcher,
        time_zone => bool
    }
);

pub struct DateTimeFormat {
    prefs: DateTimeFormatResolvedPreferences,
    options: DateTimeFormatResolvedOptions,
}

impl DateTimeFormat {
    pub fn new(prefs: DateTimeFormatPreferences, options: DateTimeFormatOptions) -> Self {
        let mut resolved = get_default_prefs(&prefs.lid);

        resolved.extend(prefs);

        Self {
            prefs: resolved,
            options: resolve_options(&options),
        }
    }

    pub fn resolved_preferences(&self) -> &DateTimeFormatResolvedPreferences {
        &self.prefs
    }

    pub fn resolved_options(&self) -> &DateTimeFormatResolvedOptions {
        &self.options
    }
}
