// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Assorted functions to help with date calculations.

use crate::date::{Era, IsoWeekday, Year};
use crate::pattern::{Pattern, TimeGranularity};

use tinystr::tinystr8;

pub fn iso_year_to_gregorian(iso_year: i32) -> Year {
    if iso_year > 0 {
        Year {
            era: Era(tinystr8!("ce")),
            number: iso_year,
            related_iso: iso_year,
        }
    } else {
        Year {
            era: Era(tinystr8!("bce")),
            number: -iso_year + 1,
            related_iso: iso_year,
        }
    }
}

#[test]
fn test_iso_year_to_gregorian() {
    assert_eq!(
        iso_year_to_gregorian(2020),
        Year {
            era: Era(tinystr8!("ce")),
            number: 2020,
            related_iso: 2020,
        }
    );
    assert_eq!(
        iso_year_to_gregorian(1),
        Year {
            era: Era(tinystr8!("ce")),
            number: 1,
            related_iso: 1,
        }
    );
    assert_eq!(
        iso_year_to_gregorian(0),
        Year {
            era: Era(tinystr8!("bce")),
            number: 1,
            related_iso: 0,
        }
    );
    assert_eq!(
        iso_year_to_gregorian(-1),
        Year {
            era: Era(tinystr8!("bce")),
            number: 2,
            related_iso: -1,
        }
    );
}

/// Temporary simplified function to get the day of the week
/// month and day are both zero-indexed.
///
/// The caller should guarantee that `month` is between 0 and 11.
pub fn iso_date_to_weekday(year: i32, month: usize, day: usize) -> IsoWeekday {
    debug_assert!(month < 12);
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = (if month < 2 { year - 1 } else { year }).rem_euclid(400) as usize;
    let result = (year + year / 4 - year / 100 + year / 400 + t[month] + day + 1) % 7;
    result.into()
}

#[test]
fn test_iso_date_to_weekday() {
    assert_eq!(IsoWeekday::Saturday, iso_date_to_weekday(2000, 0, 0));
    assert_eq!(IsoWeekday::Wednesday, iso_date_to_weekday(2021, 1, 2));
    assert_eq!(IsoWeekday::Saturday, iso_date_to_weekday(-400, 0, 0));
    assert_eq!(IsoWeekday::Wednesday, iso_date_to_weekday(-379, 1, 2));
}

/// Returns [`true`] if the most granular time being displayed will align with
/// the top of the hour, otherwise returns [`false`].
/// e.g. `12:00:00` is at the top of the hour for any display granularity.
/// e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed.
pub fn is_top_of_hour(pattern: &Pattern, minute: u8, second: u8) -> bool {
    match pattern.most_granular_time() {
        None | Some(TimeGranularity::Hours) => true,
        Some(TimeGranularity::Minutes) => minute == 0,
        Some(TimeGranularity::Seconds) => minute + second == 0,
    }
}
