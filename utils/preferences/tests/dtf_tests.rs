mod dtf;

use icu_datetime::options::length;
use icu_datetime::options::preferences::HourCycle;
use icu_locid::{locale, subtags_language, subtags_region, Locale};

use dtf::*;

#[test]
fn dtf_default() {
    let loc = locale!("en-US");

    let options = DateTimeFormatOptions {
        time_length: Some(length::Time::Long),
        ..Default::default()
    };

    let dtf = DateTimeFormat::new(loc.into(), options);

    assert_eq!(dtf.format(0), String::from("12:13 am"));
}

#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(
        dtf.format(0),
        String::from("Monday, June 23rd 2022, 00:13 am")
    );
}

#[test]
fn dtf_prefs() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let mut prefs = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    prefs.merge_locale(loc);
    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(dtf.format(0), String::from("Monday, June 23rd 2022, 24:13"));
}

#[test]
fn dtf_prefs_with_ca() {
    let loc: Locale = "en-US-u-hc-h11-ca-buddhist".parse().unwrap();
    let mut prefs = DateTimeFormatPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    prefs.merge_locale(loc);
    let dtf = DateTimeFormat::new(prefs, Default::default());
    assert_eq!(dtf.format(0), String::from("Monday, June 23rd 2022, 24:13"));
}

#[test]
fn dtf_prefs_default_region() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    let dtf = DateTimeFormat::new(loc.into(), Default::default());
    assert_eq!(
        dtf.format(0),
        String::from("Monday, June 23rd 2022, 12:13 am")
    );
    assert_eq!(dtf.resolved_options().lid.language, subtags_language!("en"));
    assert_eq!(
        dtf.resolved_options().lid.region,
        Some(subtags_region!("US"))
    );
}
