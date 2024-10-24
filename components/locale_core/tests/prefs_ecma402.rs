// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod preferences;

use icu_locale_core::locale;
use icu_locale_core::preferences::extensions::unicode::keywords;

use preferences::time_formatter::{ecma402::TimeFormatter, options::TimeStyle};

#[test]
fn prefs_ecma402_tf_default() {
    let loc = locale!("en-US");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.resolved_options().hour_cycle, keywords::HourCycle::H12);
    assert_eq!(tf.resolved_options().time_style, TimeStyle::Medium);
    assert_eq!(tf.resolved_options().locale, locale!("en"));

    // Locale with no data
    let loc = locale!("fr");

    let tf = TimeFormatter::new((&loc).into(), Default::default());

    assert_eq!(tf.resolved_options().hour_cycle, keywords::HourCycle::H23);
    assert_eq!(tf.resolved_options().time_style, TimeStyle::Medium);
    assert_eq!(tf.resolved_options().locale, locale!("und"));
}
