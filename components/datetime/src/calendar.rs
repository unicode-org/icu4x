// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, ethiopic::Ethiopic, indian::Indian, japanese::Japanese,
    Gregorian,
};
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_value as value;

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    const BCP_47_IDENTIFIER: Value;
}

impl CldrCalendar for Gregorian {
    const BCP_47_IDENTIFIER: Value = value!("gregory");
}

impl CldrCalendar for Buddhist {
    const BCP_47_IDENTIFIER: Value = value!("buddhist");
}

impl CldrCalendar for Japanese {
    const BCP_47_IDENTIFIER: Value = value!("japanese");
}

impl CldrCalendar for Coptic {
    const BCP_47_IDENTIFIER: Value = value!("coptic");
}

impl CldrCalendar for Indian {
    const BCP_47_IDENTIFIER: Value = value!("indian");
}

impl CldrCalendar for Ethiopic {
    const BCP_47_IDENTIFIER: Value = value!("ethiopic");
}
