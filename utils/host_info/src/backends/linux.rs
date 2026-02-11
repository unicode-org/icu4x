// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    backends::{HostInfoBackend, RawHostInfoBackend},
    error::HostInfoError,
    locale::PosixLocale,
};
use icu_locale_core::{preferences::extensions::unicode::keywords::HourCycle, Locale};

pub struct LinuxHostInfoBackend;

impl HostInfoBackend for LinuxHostInfoBackend {
    #[cfg(feature = "datetime")]
    fn datetime_preferences() -> Result<icu_datetime::DateTimeFormatterPreferences, HostInfoError> {
        use crate::backends::shared::posix::{raw_locale_categories, LocaleCategory};

        let mut categories = raw_locale_categories()?;

        let mut locale = Locale::UNKNOWN;
        if let Some(lc_time) = categories.remove(&LocaleCategory::Time) {
            if let Ok(loc) = PosixLocale::try_from_str(&lc_time) {
                if let Ok(loc) = Locale::try_from(loc) {
                    locale = loc;
                }
            }
        } else if let Some(lc_all) = categories.remove(&LocaleCategory::All) {
            if let Ok(loc) = PosixLocale::try_from_str(&lc_all) {
                if let Ok(loc) = Locale::try_from(loc) {
                    locale = loc;
                }
            }
        }

        let mut result = icu_datetime::DateTimeFormatterPreferences::from(locale);
        result.numbering_system = None;
        result.hour_cycle = Self::hour_cycle()?;
        result.calendar_algorithm = Self::calendar()?;
        Ok(result)
    }

    fn requested_locales() -> Result<Vec<Locale>, HostInfoError> {
        Ok(Self::raw_requested_locales()?
            .into_iter()
            .filter_map(|s| {
                PosixLocale::try_from_str(&s)
                    .ok()
                    .and_then(|posix_locale| Locale::try_from(posix_locale).ok())
            })
            .collect())
    }

    fn hour_cycle() -> Result<Option<HourCycle>, HostInfoError> {
        #[cfg(gio)]
        if let Some(hc) = gnome_clock_format_hc() {
            return Ok(Some(hc));
        }
        Ok(None)
    }
}

impl RawHostInfoBackend for LinuxHostInfoBackend {
    fn raw_requested_locales() -> Result<Vec<String>, HostInfoError> {
        // 1) LANGUAGE: colon-separated, ordered
        if let Ok(s) = std::env::var("LANGUAGE") {
            let v: Vec<String> = s
                .split(':')
                .filter(|x| !x.is_empty())
                .map(|s| s.to_string())
                .collect();
            if !v.is_empty() {
                return Ok(v);
            }
        }

        // 2) Fallbacks: LC_MESSAGES > LC_ALL > LANG
        for k in ["LC_MESSAGES", "LC_ALL", "LANG"] {
            if let Ok(s) = std::env::var(k) {
                if !s.is_empty() {
                    return Ok(vec![s]);
                }
            }
        }

        Ok(vec![])
    }
}

#[cfg(gio)]
fn gnome_clock_format_hc() -> Option<HourCycle> {
    use gio::prelude::*;
    let s = gio::Settings::new("org.gnome.desktop.interface");
    match s.string("clock-format").as_str() {
        "12h" => Some(HourCycle::H12),
        "24h" => Some(HourCycle::H23),
        _ => None,
    }
}
