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
    options: DateTimeFormatOptions,
}

impl DateTimeFormat {
    pub fn new(prefs: DateTimeFormatPreferences, options: DateTimeFormatOptions) -> Self {
        let mut resolved = get_defaults(&prefs.lid);

        resolved.resolve(&prefs);

        Self {
            prefs: resolved,
            options,
        }
    }

    fn get_time(&self) -> String {
        println!("{:#?}", self.prefs.hour_cycle);
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
        match (self.options.date_length, self.options.time_length) {
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
        return format!("{}{}{}", self.get_date(), ", ", self.get_time(),);
    }

    pub fn resolved_options(&self) -> &DateTimeFormatResolvedPreferences {
        &self.prefs
    }
}
