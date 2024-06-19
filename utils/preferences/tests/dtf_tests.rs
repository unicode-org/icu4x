// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod dtf;

use icu_locale_core::{
    locale,
    subtags::{language, region},
    Locale,
};
use icu_preferences::extensions::unicode::keywords;
use tinystr::tinystr;

use dtf::*;

// In this scenario, the locale is the only source of preferences
// and since it's empty, the defaults for the resolved locale will be taken.
// The result chain is: Defaults.
#[test]
fn dtf_default() {
    let loc = locale!("en-US");

    let dtf = DateTimeFormat::new(loc.into(), Default::default());

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H12
    );
}

// In this scenario, we resolve the locale, and then apply the regional
// preferences from unicode extensions of the Locale on top of it.
// The result chain is: Locale > Defaults.
#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());

    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H11
    );
}

// In this scenario, we will take the preferences bag, and extend
// the preferences from the locale with it.
// The result chain is: Bag > Locale > Defaults.
#[test]
fn dtf_prefs() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();

    let bag = DateTimeFormatPreferences {
        hour_cycle: Some(keywords::HourCycle::H24),
        ..Default::default()
    };
    let mut prefs = DateTimeFormatPreferences::from(loc);
    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H24
    );
    assert_eq!(
        dtf.resolved_preferences().calendar,
        keywords::CalendarAlgorithm::Gregory,
    );
}

// In this scenario we showcase two preferences in locale extensions,
// and one of them overridden in the preferences bag.
// The result chain is: Bag > Locale > Defaults.
#[test]
fn dtf_prefs_with_ca() {
    let loc: Locale = "en-US-u-hc-h11-ca-buddhist".parse().unwrap();
    let bag = DateTimeFormatPreferences {
        hour_cycle: Some(keywords::HourCycle::H24),
        ..Default::default()
    };
    let mut prefs = DateTimeFormatPreferences::from(loc);
    prefs.extend(bag);

    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H24
    );
    assert_eq!(
        dtf.resolved_preferences().calendar,
        keywords::CalendarAlgorithm::Buddhist,
    );
}

// In this scenario we pass `en` but resolve to `en-US`.
#[test]
fn dtf_prefs_default_region() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(dtf.resolved_preferences().lid.language, language!("en"));
    assert_eq!(dtf.resolved_preferences().lid.region, Some(region!("US")));
    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H12
    );
}

#[test]
fn dtf_options_default() {
    use icu_datetime::options::length;

    let loc: Locale = "en".parse().unwrap();

    let options = DateTimeFormatOptions {
        ..Default::default()
    };
    let dtf = DateTimeFormat::new(loc.into(), options);
    assert_eq!(dtf.resolved_options().date_length, length::Date::Short);
}

#[test]
fn dtf_options_manual() {
    use icu_datetime::options::length;

    let loc: Locale = "en".parse().unwrap();

    let options = DateTimeFormatOptions {
        date_length: Some(length::Date::Medium),
        ..Default::default()
    };
    let dtf = DateTimeFormat::new(loc.into(), options);
    assert_eq!(dtf.resolved_options().date_length, length::Date::Medium);
}

#[test]
fn dtf_prefs_unknown_ue_keu() {
    let loc: Locale = "en-u-bb-h99".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(dtf.resolved_preferences().lid.language, language!("en"));
    assert_eq!(dtf.resolved_preferences().lid.region, Some(region!("US")));
    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H12
    );
}

#[test]
fn dtf_prefs_unknown_ue_value() {
    let loc: Locale = "en-u-hc-h99".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(dtf.resolved_preferences().lid.language, language!("en"));
    assert_eq!(dtf.resolved_preferences().lid.region, Some(region!("US")));
    assert_eq!(
        dtf.resolved_preferences().hour_cycle,
        keywords::HourCycle::H12
    );
}

#[test]
fn dtf_prefs_non_ue_preference() {
    let loc: Locale = "en-US".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(dtf.resolved_preferences().lid.language, language!("en"));
    assert_eq!(dtf.resolved_preferences().lid.region, Some(region!("US")));
    assert_eq!(
        dtf.resolved_preferences().date_pattern,
        DatePattern(tinystr!(8, "m/d/Y"))
    );
}

#[test]
fn dtf_prefs_unknown_ue_value_skipped() {
    let loc: Locale = "en-u-hc-h99".parse().unwrap();
    let prefs: DateTimeFormatPreferences = loc.into();

    assert_eq!(prefs.hour_cycle, None);
}

#[test]
fn dtf_prefs_into_locale() {
    let loc: Locale = "en-u-hc-h23-ca-buddhist".parse().unwrap();
    let prefs: DateTimeFormatPreferences = loc.into();
    let loc2: Locale = prefs.into();

    assert_eq!(loc2.to_string(), "en-u-ca-buddhist-hc-h23");
}

#[test]
fn dtf_prefs_ca_islamic() {
    use icu_preferences::extensions::unicode::keywords;

    let loc: Locale = "en-u-ca-islamic".parse().unwrap();
    let prefs: DateTimeFormatPreferences = loc.into();
    assert_eq!(
        prefs.calendar,
        Some(keywords::CalendarAlgorithm::Islamic(None))
    );
    let loc2: Locale = prefs.into();
    assert_eq!(loc2.to_string(), "en-u-ca-islamic");

    let loc: Locale = "en-u-ca-islamic-civil".parse().unwrap();
    let prefs: DateTimeFormatPreferences = loc.into();
    assert_eq!(
        prefs.calendar,
        Some(keywords::CalendarAlgorithm::Islamic(Some(
            keywords::IslamicCalendarAlgorithm::Civil
        )))
    );
    let loc2: Locale = prefs.into();
    assert_eq!(loc2.to_string(), "en-u-ca-islamic-civil");

    let loc: Locale = "en-u-ca-islamic-foo".parse().unwrap();
    let prefs: DateTimeFormatPreferences = loc.into();
    assert_eq!(
        prefs.calendar,
        Some(keywords::CalendarAlgorithm::Islamic(None))
    );
    let loc2: Locale = prefs.into();
    assert_eq!(loc2.to_string(), "en-u-ca-islamic");
}
