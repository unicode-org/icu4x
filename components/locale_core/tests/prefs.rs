// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod preferences;

use icu_locale_core::{langid, locale, preferences::extensions::unicode::keywords::HourCycle};

use preferences::time_formatter::{
    options::TimeStyle, TimeFormatter, TimeFormatterOptions, TimeFormatterPreferences,
};

#[test]
fn prefs_tf_default() {
    let loc = locale!("en-US");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.format_to_string(11), "(en) 11:00 am");

    // Locale with no data
    let loc = locale!("fr");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.format_to_string(11), "(und) 11:00");
}

#[test]
fn prefs_tf_custom_options() {
    let loc = locale!("en-US");

    let tf = TimeFormatter::new(
        (&loc).into(),
        TimeFormatterOptions {
            time_style: Some(TimeStyle::Short),
        },
    );

    assert_eq!(tf.format_to_string(11), "(en) 11:00 am");

    // Locale with no data
    let loc = locale!("fr");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.format_to_string(11), "(und) 11:00");
}

#[test]
fn prefs_tf_custom_preferences() {
    let loc = locale!("en-US");
    let bag = TimeFormatterPreferences {
        hour_cycle: Some(HourCycle::H23),
        ..Default::default()
    };
    let mut prefs = TimeFormatterPreferences::from(&loc);
    prefs.extend(bag);

    let tf = TimeFormatter::new(prefs, Default::default());

    assert_eq!(tf.format_to_string(11), "(en) 11:00");
}

#[test]
fn prefs_tf_from_lid() {
    let lid = langid!("en-US");

    let tf = TimeFormatter::new((&lid).into(), Default::default());

    assert_eq!(tf.format_to_string(11), "(en) 11:00 am");

    // Locale with no data
    let loc = locale!("fr");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.format_to_string(11), "(und) 11:00");
}

#[test]
fn prefs_tf_from_lid_with_custom_preferences() {
    let lid = langid!("en-US");
    let bag = TimeFormatterPreferences {
        hour_cycle: Some(HourCycle::H23),
        ..Default::default()
    };
    let mut prefs = TimeFormatterPreferences::from(&lid);
    prefs.extend(bag);

    let tf = TimeFormatter::new(prefs, Default::default());

    assert_eq!(tf.format_to_string(11), "(en) 11:00");
}
