// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod dtf;

use icu_datetime::options::preferences::HourCycle;
use icu_locid::{locale, LanguageIdentifier};

use dtf::*;

fn get_os_dtf_preferences(lid: &LanguageIdentifier) -> Option<dtf::DateTimeFormatPreferences> {
    Some(dtf::DateTimeFormatPreferences {
        lid: Some(lid.clone()),
        hour_cycle: Some(HourCycle::H23),
        ..Default::default()
    })
}

#[test]
fn dtf_get_os_prefs() {
    let loc = locale!("en-US");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let mut prefs: DateTimeFormatPreferences = loc.into();
    if let Some(os_prefs) = os_prefs {
        prefs.merge(os_prefs);
    }

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H23);
}

#[test]
fn dtf_locale_override_os_prefs() {
    let loc = locale!("en-US-u-hc-h11");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let mut prefs: DateTimeFormatPreferences = loc.into();
    if let Some(os_prefs) = os_prefs {
        prefs.merge(os_prefs);
    }

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H11);
}

#[test]
fn dtf_os_prefs_override_locale() {
    let loc = locale!("en-US-u-hc-h11");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let locale_prefs: DateTimeFormatPreferences = loc.into();
    let prefs = if let Some(mut os_prefs) = os_prefs {
        os_prefs.merge(locale_prefs);
        os_prefs
    } else {
        locale_prefs
    };

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H23);
}

#[test]
fn dtf_call_override_locale_override_os_prefs() {
    let loc = locale!("en-US-u-hc-h11");

    let mut prefs = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let locale_prefs: DateTimeFormatPreferences = loc.into();

    prefs.merge(locale_prefs);

    if let Some(os_prefs) = os_prefs {
        prefs.merge(os_prefs);
    };

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H24);
}
