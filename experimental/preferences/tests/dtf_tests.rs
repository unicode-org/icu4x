// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod dtf;

use icu_datetime::options::preferences::HourCycle;
use icu_locid::{
    locale,
    subtags::{language, region},
    Locale,
};

use dtf::*;

// In this scenario, the locale is the only source of preferences
// and since it's empty, the defaults for the resolved locale will be taken.
// The result chain is: Defaults.
#[test]
fn dtf_default() {
    let loc = locale!("en-US");

    let dtf = DateTimeFormat::new(loc.into(), Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H12);
}

// In this scenario, we resolve the locale, and then apply the regional
// preferences from unicode extensions of the Locale on top of it.
// The result chain is: Locale > Defaults.
#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());

    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H11);
}

// In this scenario, we will take the preferences bag, and extend
// the preferences from the locale with it.
// The result chain is: Bag > Locale > Defaults.
#[test]
fn dtf_prefs() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();

    let bag = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    let mut prefs = DateTimeFormatPreferences::from(loc);
    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H24);
    assert_eq!(
        dtf.resolved_options().calendar,
        dtf::preferences::Calendar::Gregory,
    );
}

// In this scenario we showcase two preferences in locale extensions,
// and one of them overridden in the preferences bag.
// The result chain is: Bag > Locale > Defaults.
#[test]
fn dtf_prefs_with_ca() {
    let loc: Locale = "en-US-u-hc-h11-ca-buddhist".parse().unwrap();
    let bag = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    let mut prefs = DateTimeFormatPreferences::from(loc);
    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H24);
    assert_eq!(
        dtf.resolved_options().calendar,
        dtf::preferences::Calendar::Buddhist,
    );
}

// In this scenario we pass `en` but resolve to `en-US`.
#[test]
fn dtf_prefs_default_region() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(dtf.resolved_options().lid.language, language!("en"));
    assert_eq!(dtf.resolved_options().lid.region, Some(region!("US")));
    assert_eq!(dtf.resolved_options().hour_cycle, HourCycle::H12);
}
