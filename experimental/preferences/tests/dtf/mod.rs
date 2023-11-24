// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod data_provider;
mod options;
pub mod preferences;

use data_provider::get_defaults;
use icu_datetime::options::length;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{extensions_unicode_key, LanguageIdentifier, Locale};
use icu_preferences::preferences;
use options::{DayPeriod, LocaleMatcher};
use preferences::{Calendar, NumberingSystem};

preferences!(
    DateTimeFormatPreferences,
    DateTimeFormatResolvedPreferences,
    {
        hour_cycle => Option<HourCycle>, HourCycle, Some(extensions_unicode_key!("hc")),
        calendar => Option<Calendar>, Calendar, Some(extensions_unicode_key!("ca")),
        numbering_system => Option<NumberingSystem>, NumberingSystem, Some(extensions_unicode_key!("nu"))
    }
);

#[derive(Default)]
pub struct DateTimeFormatOptions {
    pub date_length: Option<length::Date>,
    pub time_length: Option<length::Time>,
    pub day_period: Option<DayPeriod>,
    pub locale_matcher: Option<LocaleMatcher>,
    pub time_zone: Option<bool>,
}

pub struct DateTimeFormat {
    prefs: DateTimeFormatResolvedPreferences,
    _options: DateTimeFormatOptions,
}

impl DateTimeFormat {
    pub fn new(prefs: DateTimeFormatPreferences, options: DateTimeFormatOptions) -> Self {
        let mut resolved = get_defaults(&prefs.lid);

        resolved.extend(&prefs);

        Self {
            prefs: resolved,
            _options: options,
        }
    }

    pub fn resolved_options(&self) -> &DateTimeFormatResolvedPreferences {
        &self.prefs
    }
}
