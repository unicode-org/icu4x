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
        hour_cycle: HourCycle::H12,
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
    if let Some(lid) = lid {
        for v in DEFAULT_PREFS.list {
            if v.lid.language == lid.language {
                return v.clone();
            }
        }
    }
    (LanguageIdentifier::UND, &DEFAULT_PREFS.und).into()
}
