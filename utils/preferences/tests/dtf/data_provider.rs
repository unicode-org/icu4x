use super::*;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{langid, LanguageIdentifier};
use tinystr::tinystr;

struct DefaultPrefs {
    pub und: DateTimeFormatResolvedPreferences,
    pub list: &'static [DateTimeFormatResolvedPreferences],
}

const DEFAULT_PREFS: DefaultPrefs = DefaultPrefs {
    und: DateTimeFormatResolvedPreferences {
        lid: LanguageIdentifier::UND,
        hour_cycle: HourCycle::H23,
        calendar: Calendar::Gregory,
        numbering_system: NumberingSystem(tinystr!(8, "latn")),
    },
    list: &[DateTimeFormatResolvedPreferences {
        lid: langid!("en-US"),
        hour_cycle: HourCycle::H12,
        calendar: Calendar::Gregory,
        numbering_system: NumberingSystem(tinystr!(8, "latn")),
    }],
};

pub fn get_defaults(lid: &Option<LanguageIdentifier>) -> DateTimeFormatResolvedPreferences {
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
