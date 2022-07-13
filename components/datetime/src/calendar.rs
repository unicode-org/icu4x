// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DateTimeFormatError;
use core::any;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, ethiopic::Ethiopic, indian::Indian, japanese::Japanese,
    Gregorian,
};
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_key as key;
use icu_locid::extensions_unicode_value as value;
use icu_locid::Locale;
use tinystr::{tinystr, TinyStr16};

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    /// If multiple BCP 47 identifiers work, this should be
    /// the default one when no others are provided
    const DEFAULT_BCP_47_IDENTIFIER: Value;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

impl CldrCalendar for Gregorian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("gregory");
}

impl CldrCalendar for Buddhist {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("buddhist");
}

impl CldrCalendar for Japanese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanese");
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("japanese") || *value == value!("japanext")
    }
}

impl CldrCalendar for Coptic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("coptic");
}

impl CldrCalendar for Indian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("indian");
}

impl CldrCalendar for Ethiopic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("ethiopic");
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("ethiopic") || *value == value!("ethioaa")
    }
}

pub(crate) fn potentially_fixup_calendar<C: CldrCalendar>(
    locale: &mut Locale,
) -> Result<(), DateTimeFormatError> {
    let cal = locale.extensions.unicode.keywords.get(&key!("ca"));

    if let Some(cal) = cal {
        if !C::is_identifier_allowed_for_calendar(cal) {
            let mut string = cal.to_string();
            string.truncate(16);
            let tiny = TinyStr16::from_str(&string).unwrap_or(tinystr!(16, "unknown"));
            return Err(DateTimeFormatError::MismatchedCalendarLocale(
                any::type_name::<C>(),
                tiny,
            ));
        }
    } else {
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), C::DEFAULT_BCP_47_IDENTIFIER);
    }

    Ok(())
}
