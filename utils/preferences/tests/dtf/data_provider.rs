use super::*;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{langid, LanguageIdentifier};

struct DefaultPrefs {
    pub und: DateTimeFormatDefaultPreferences,
    pub list: &'static [(LanguageIdentifier, DateTimeFormatDefaultPreferences)],
}

const DEFAULT_PREFS: DefaultPrefs = DefaultPrefs {
    und: DateTimeFormatDefaultPreferences {
        hour_cycle: HourCycle::H12,
        calendar: Calendar::Gregory,
        numbering_system: NumberingSystem::Latn,
    },
    list: &[(
        langid!("en-US"),
        DateTimeFormatDefaultPreferences {
            hour_cycle: HourCycle::H12,
            calendar: Calendar::Gregory,
            numbering_system: NumberingSystem::Latn,
        },
    )],
};

pub fn get_defaults(lid: &Option<LanguageIdentifier>) -> DateTimeFormatResolvedPreferences {
    if let Some(lid) = lid {
        for (k, v) in DEFAULT_PREFS.list {
            if k.language == lid.language {
                return (k.clone(), v).into();
            }
        }
    }
    (LanguageIdentifier::UND, &DEFAULT_PREFS.und).into()
}
