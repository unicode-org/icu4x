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

// In this scenario we showcase retrieval of OS regional preferences.
// The result chain is: OS > Locale > Defaults.
#[test]
fn dtf_get_os_prefs() {
    let loc = locale!("en-US");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let mut prefs: DateTimeFormatPreferences = loc.into();
    if let Some(os_prefs) = os_prefs {
        prefs.extend(os_prefs);
    }

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H23);
}

// In this scenario we showcase retrieval of OS regional preferences.
// The priority is in locale unicode extension overriding OS preferences.
// The result chain is: Locale > OS > Defaults.
#[test]
fn dtf_locale_override_os_prefs() {
    let loc = locale!("en-US-u-hc-h11");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let prefs = if let Some(mut os_prefs) = os_prefs {
        os_prefs.extend(loc);
        os_prefs
    } else {
        loc.into()
    };

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H11);
}

// In this scenario we showcase retrieval of OS regional preferences.
// The priority is in OS preferences overriding locale unicode extension.
// The result chain is: OS > Locale > Defaults.
#[test]
fn dtf_os_prefs_override_locale() {
    let loc = locale!("en-US-u-hc-h11");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let mut prefs = DateTimeFormatPreferences::from(loc);
    if let Some(os_prefs) = os_prefs {
        prefs.extend(os_prefs);
    }

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H23);
}

// In this scenario we showcase retrieval of OS regional preferences,
// preferences bag, and locale.
// The result chain is: Bag > Locale > OS > Defaults.
#[test]
fn dtf_call_override_locale_override_os_prefs() {
    let loc = locale!("en-US-u-hc-h11");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let mut prefs = if let Some(mut os_prefs) = os_prefs {
        os_prefs.extend(loc);
        os_prefs
    } else {
        loc.into()
    };

    let bag = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };

    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H24);
}
