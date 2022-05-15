use super::Preferences;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::language;
use icu_locid::subtags::{Language, Region, Script};
use icu_locid::unicode_ext_key;
use icu_locid::Locale;

pub struct DateTimeFormat {
    preferences: ResolvedDTFPreferencesBag,
}

impl DateTimeFormat {
    pub fn try_new(preferences: &impl DTFPreferences) -> Self {
        let defaults = ResolvedDTFPreferencesBag {
            language: Language::UND,
            script: None,
            region: None,
            hour_cycle: if *preferences.language() == language!("en") {
                HourCycle::H12
            } else {
                HourCycle::H24
            },
        };
        Self {
            preferences: defaults.resolve(preferences),
        }
    }

    pub fn format(&self) -> String {
        let hour = if self.preferences.hour_cycle == HourCycle::H24 {
            "15:00"
        } else {
            "3:00pm"
        };

        let date = match self.preferences.language.as_str() {
            "en" => String::from("05/13/2022"),
            "fr" => String::from("13/05/2022"),
            _ => String::from("ISO DATE"),
        };
        format!("{date} {hour}")
    }
}

pub trait DTFPreferences: Preferences {
    fn hour_cycle(&self) -> Option<HourCycle> {
        None
    }
}

#[derive(Default)]
pub struct DTFPreferencesBag {
    pub language: Option<Language>,
    pub script: Option<Script>,
    pub region: Option<Region>,
    pub hour_cycle: Option<HourCycle>,
}

pub struct ResolvedDTFPreferencesBag {
    pub language: Language,
    pub script: Option<Script>,
    pub region: Option<Region>,
    pub hour_cycle: HourCycle,
}

impl ResolvedDTFPreferencesBag {
    fn resolve(&self, prefs: &impl DTFPreferences) -> Self {
        Self {
            language: *prefs.language(),
            script: prefs.script().copied(),
            region: prefs.region().copied(),
            hour_cycle: prefs.hour_cycle().unwrap_or(self.hour_cycle),
        }
    }
}

impl DTFPreferencesBag {
    #[allow(clippy::result_unit_err)]
    pub fn merge_locale(&mut self, locale: &Locale) -> Result<(), ()> {
        if self.language.is_none() && !locale.id.language.is_empty() {
            self.language = Some(locale.id.language);
        };
        if self.script.is_none() && locale.id.script.is_some() {
            self.script = locale.id.script;
        }
        if self.region.is_none() && locale.id.region.is_some() {
            self.region = locale.id.region;
        }
        if self.hour_cycle.is_none() {
            if let Some(value) = locale
                .extensions
                .unicode
                .keywords
                .get(&unicode_ext_key!("hc"))
            {
                self.hour_cycle = Some(HourCycle::try_from(value)?);
            }
        }
        Ok(())
    }
}

impl Preferences for DTFPreferencesBag {
    fn language(&self) -> &Language {
        self.language.as_ref().unwrap_or(&Language::UND)
    }

    fn script(&self) -> Option<&Script> {
        self.script.as_ref()
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }
}

impl DTFPreferences for DTFPreferencesBag {
    fn hour_cycle(&self) -> Option<HourCycle> {
        self.hour_cycle
    }
}

impl DTFPreferences for Locale {}
