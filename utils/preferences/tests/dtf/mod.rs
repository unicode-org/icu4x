mod data_provider;
mod options;
mod preferences;

use data_provider::get_defaults;
use icu_datetime::options::length;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{extensions_unicode_key, LanguageIdentifier, Locale};
use icu_preferences::{preferences, Preferences};
use options::{DayPeriod, LocaleMatcher};
use preferences::{Calendar, NumberingSystem};

preferences!(
    DTFPreferencesBag,
    DTFDefaultPreferencesBag,
    DTFResolvedPreferencesBag,
    {
        hour_cycle => Option<HourCycle>, HourCycle, Some(extensions_unicode_key!("hc")),
        calendar => Option<Calendar>, Calendar, Some(extensions_unicode_key!("ca")),
        numbering_system => Option<NumberingSystem>, NumberingSystem, Some(extensions_unicode_key!("nu"))
    },
    DTFOptionsBag,
    {
        date_length => Option<length::Date>,
        time_length => Option<length::Time>,
        day_period => Option<DayPeriod>,
        locale_matcher => Option<LocaleMatcher>,
        time_zone => Option<bool>
    }
);

pub struct DateTimeFormat {
    prefs: DTFResolvedPreferencesBag,
    options: Option<DTFOptionsBag>,
}

impl DateTimeFormat {
    pub fn new(prefs: DTFPreferencesBag, options: Option<DTFOptionsBag>) -> Self {
        let mut resolved = get_defaults(&prefs);

        resolved.resolve(&prefs);

        Self {
            prefs: resolved,
            options,
        }
    }

    fn get_time(&self) -> String {
        match self.prefs.hour_cycle {
            HourCycle::H11 => "00:13 am",
            HourCycle::H12 => "12:13 am",
            HourCycle::H23 => "00:13",
            HourCycle::H24 => "24:13",
        }
        .to_string()
    }

    fn get_date(&self) -> String {
        "Monday, June 23rd 2022".to_string()
    }

    pub fn format(&self, _input: u64) -> String {
        if let Some(options) = &self.options {
            match (options.date_length, options.time_length) {
                (Some(_), None) => {
                    return self.get_date();
                }
                (None, Some(_)) => {
                    return self.get_time();
                }
                (Some(_), Some(_)) => {
                    return format!("{}{}{}", self.get_date(), ", ", self.get_time(),);
                }
                _ => {}
            }
        }
        return format!("{}{}{}", self.get_date(), ", ", self.get_time(),);
    }

    pub fn resolved_options(&self) -> &DTFResolvedPreferencesBag {
        &self.prefs
    }
}
