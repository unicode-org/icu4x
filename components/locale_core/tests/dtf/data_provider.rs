// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use icu_locale_core::preferences::extensions::unicode::keywords;
use icu_locale_core::{langid, subtags::subtag, LanguageIdentifier};
use tinystr::tinystr;

struct DefaultPrefs {
    pub und: DateTimeFormatResolvedPreferences,
    pub list: &'static [DateTimeFormatResolvedPreferences],
}

const DEFAULT_PREFS: DefaultPrefs = DefaultPrefs {
    und: DateTimeFormatResolvedPreferences {
        lid: LanguageIdentifier::default(),
        hour_cycle: keywords::HourCycle::H23,
        calendar: keywords::CalendarAlgorithm::Gregory,
        numbering_system: keywords::NumberingSystem(subtag!("latn")),
        date_pattern: DatePattern(tinystr!(8, "y-M-d")),
    },
    list: &[DateTimeFormatResolvedPreferences {
        lid: langid!("en-US"),
        hour_cycle: keywords::HourCycle::H12,
        calendar: keywords::CalendarAlgorithm::Gregory,
        numbering_system: keywords::NumberingSystem(subtag!("latn")),
        date_pattern: DatePattern(tinystr!(8, "M/d/y")),
    }],
};

pub fn get_default_prefs(lid: &Option<LanguageIdentifier>) -> DateTimeFormatResolvedPreferences {
    lid.as_ref()
        .and_then(|lid| {
            DEFAULT_PREFS
                .list
                .iter()
                .find(|dtfrp| dtfrp.lid.language == lid.language)
        })
        .cloned()
        .unwrap_or(DEFAULT_PREFS.und)
}

pub fn resolve_options(options: &DateTimeFormatOptions) -> DateTimeFormatResolvedOptions {
    DateTimeFormatResolvedOptions {
        date_length: options.date_length.unwrap_or(DateLength::Short),
        time_length: options.time_length.unwrap_or(TimeLength::Short),
        day_period: DayPeriod::Short,
        locale_matcher: LocaleMatcher::BestFit,
        time_zone: options.time_zone.unwrap_or(false),
    }
}
