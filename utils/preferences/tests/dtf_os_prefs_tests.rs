// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod dtf;

use icu_locale_core::{locale, LanguageIdentifier};
use icu_preferences::extensions::unicode::keywords;

use dtf::*;

fn get_os_dtf_preferences(lid: &LanguageIdentifier) -> Option<dtf::DateTimeFormatPreferences> {
    // This optionally may different
    Some(dtf::DateTimeFormatPreferences {
        lid: Some(lid.clone()),
        hour_cycle: Some(keywords::HourCycle::H23),
        date_pattern: Some(DatePattern(tinystr::tinystr!(8, "d.m.Y"))),
        ..Default::default()
    })
}

fn get_os_dtf_options() -> Option<dtf::DateTimeFormatOptions> {
    use icu_datetime::options::length;

    Some(dtf::DateTimeFormatOptions {
        date_length: Some(length::Date::Long),
        time_zone: Some(true),
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

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H23
    );
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

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H11
    );
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

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H23
    );
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
        hour_cycle: Some(keywords::HourCycle::H24),
        ..Default::default()
    };

    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H24
    );
}

#[test]
fn dtf_options_override_os_options() {
    use icu_datetime::options::length;

    let loc = locale!("en");

    let dev_options = DateTimeFormatOptions {
        date_length: Some(length::Date::Medium),
        ..Default::default()
    };

    let options = if let Some(mut os_options) = get_os_dtf_options() {
        os_options.extend(dev_options);
        os_options
    } else {
        dev_options
    };

    let dtf = DateTimeFormat::new(loc.into(), options);

    // This is taken from dev options
    assert_eq!(dtf.resolved_options().date_length, length::Date::Medium);

    // Dev didn't specify time zone field presence, so this is taken from os_prefs
    assert!(dtf.resolved_options().time_zone);
}

#[test]
fn dtf_prefs_non_ue_preference() {
    let loc = locale!("en-US");

    let os_prefs = get_os_dtf_preferences(&loc.id);
    let prefs = if let Some(mut os_prefs) = os_prefs {
        os_prefs.extend(loc);
        os_prefs
    } else {
        loc.into()
    };

    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(
        dtf.resolved_preferences().date_pattern,
        DatePattern(tinystr::tinystr!(8, "d.m.Y"))
    );
}
