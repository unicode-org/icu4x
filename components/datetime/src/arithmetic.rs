// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! Assorted functions to help with date calculations.

use crate::date::WeekDay;
use crate::date_new::{Era, Year};

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

// Temporary simplified function to get the day of the week
pub fn iso_date_to_weekday(year: i32, month: usize, day: usize) -> WeekDay {
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = (if month < 2 { year - 1 } else { year }).rem_euclid(400) as usize;
    let result = (year + year / 4 - year / 100 + year / 400 + t[month as usize] + day + 1) % 7;
    WeekDay::new_unchecked(result as u8)
}

#[test]
fn test_iso_date_to_weekday() {
    use crate::date::weekdays;
    assert_eq!(weekdays::SAT, iso_date_to_weekday(2000, 0, 0));
    assert_eq!(weekdays::WED, iso_date_to_weekday(2021, 1, 2));
    assert_eq!(weekdays::SAT, iso_date_to_weekday(-400, 0, 0));
    assert_eq!(weekdays::WED, iso_date_to_weekday(-379, 1, 2));
}
