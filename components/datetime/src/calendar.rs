// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, indian::Indian, japanese::Japanese, Gregorian,
};
use icu_locid::unicode_ext_value;
use icu_locid::extensions::unicode::Value;

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    const IDENTIFIER: Value;
}

impl CldrCalendar for Gregorian {
    const IDENTIFIER: Value = unicode_ext_value!("gregory");
}

impl CldrCalendar for Buddhist {
    const IDENTIFIER: Value = unicode_ext_value!("buddhist");
}

impl CldrCalendar for Japanese {
    const IDENTIFIER: Value = unicode_ext_value!("japanese");
}

impl CldrCalendar for Coptic {
    const IDENTIFIER: Value = unicode_ext_value!("coptic");
}

impl CldrCalendar for Indian {
    const IDENTIFIER: Value = unicode_ext_value!("indian");
}
