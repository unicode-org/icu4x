// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::*;
use crate::Date;
use crate::Ref;

#[test]
fn check_extrema() {
    // Minimum and maximum dates allowed in ECMA-262 Temporal.
    let min_date_iso = Date::try_new_iso(-271821, 4, 19).unwrap();
    let max_date_iso = Date::try_new_iso(275760, 9, 13).unwrap();

    // Assertion failure:
    // > Month should be in range of u8! Value -1428 failed for RD RataDie(-99280838)
    {
        let cal = Chinese::new();
        let min_date = min_date_iso.to_calendar(Ref(&cal));
        let max_date = max_date_iso.to_calendar(Ref(&cal));

        println!(
            "min.year = {:?}, max.year = {:?}",
            min_date.year(),
            max_date.year()
        );
    }

    // Assertion failure:
    // > Month should be in range of u8! Value -1428 failed for RD RataDie(-99280838)
    {
        let cal = Dangi::new();
        let min_date = min_date_iso.to_calendar(Ref(&cal));
        let max_date = max_date_iso.to_calendar(Ref(&cal));

        println!(
            "min.year = {:?}, max.year = {:?}",
            min_date.year(),
            max_date.year()
        );
    }

    // Assertion failure:
    // > assertion failed: Date::try_new_observational_islamic_date(y, m, d, IslamicObservational::new_always_calculating()).is_ok()
    {
        let cal = HijriSimulated::new_mecca();
        let min_date = min_date_iso.to_calendar(Ref(&cal));
        let max_date = max_date_iso.to_calendar(Ref(&cal));

        println!(
            "min.year = {:?}, max.year = {:?}",
            min_date.year(),
            max_date.year()
        );
    }

    // Assertion failure:
    // > assertion failed: Date::try_new_ummalqura_date(y, m, d, IslamicUmmAlQura::new_always_calculating()).is_ok()
    {
        let cal = HijriUmmAlQura::new();
        let min_date = min_date_iso.to_calendar(Ref(&cal));
        let max_date = max_date_iso.to_calendar(Ref(&cal));

        println!(
            "min.year = {:?}, max.year = {:?}",
            min_date.year(),
            max_date.year()
        );
    }
}
