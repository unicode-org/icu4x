use super::*;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{langid, LanguageIdentifier};

struct DefaultPrefs {
    pub und: DTFDefaultPreferencesBag,
    pub list: &'static [(LanguageIdentifier, DTFDefaultPreferencesBag)],
}

const DEFAULT_PREFS: DefaultPrefs = DefaultPrefs {
    und: DTFDefaultPreferencesBag {
        hour_cycle: HourCycle::H12,
        calendar: Calendar::Gregory,
        numbering_system: NumberingSystem::Latn,
    },
    list: &[(
        langid!("en-US"),
        DTFDefaultPreferencesBag {
            hour_cycle: HourCycle::H12,
            calendar: Calendar::Gregory,
            numbering_system: NumberingSystem::Latn,
        },
    )],
};

pub fn get_defaults(prefs: &impl Preferences) -> DTFResolvedPreferencesBag {
    for (lid, values) in DEFAULT_PREFS.list {
        if prefs.language() == &lid.language {
            return (lid.clone(), values).into();
        }
    }
    (LanguageIdentifier::UND, &DEFAULT_PREFS.und).into()
}
