// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod data_provider;
mod options;
pub mod preferences;

use data_provider::{get_default_prefs, resolve_options};
use icu_datetime::options::length;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{extensions_unicode_key, LanguageIdentifier, Locale};
use icu_preferences::{options, preferences};
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

options!(
    DateTimeFormatOptions,
    DateTimeFormatResolvedOptions,
    {
        date_length => Option<length::Date>, length::Date,
        time_length => Option<length::Time>, length::Time,
        day_period => Option<DayPeriod>, DayPeriod,
        locale_matcher => Option<LocaleMatcher>, LocaleMatcher,
        time_zone => Option<bool>, bool
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
