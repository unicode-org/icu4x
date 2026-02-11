// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::gregorian::GregorianDateInner;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::Date;
use crate::Gregorian;

impl From<jiff::civil::Date> for Date<Gregorian> {
    fn from(jiff: jiff::civil::Date) -> Self {
        // jiff's MIN/MAX value are inside VALID_RD_RANGE
        Self::from_raw(
            GregorianDateInner(ArithmeticDate::new_unchecked(
                jiff.year() as i32,
                jiff.month() as u8,
                jiff.day() as u8,
            )),
            Gregorian,
        )
    }
}

#[test]
fn assert_range() {
    use crate::calendar_arithmetic::VALID_RD_RANGE;

    assert!(VALID_RD_RANGE.contains(
        &(calendrical_calculations::gregorian::fixed_from_gregorian(
            jiff::civil::Date::MIN.year() as i32,
            jiff::civil::Date::MIN.month() as u8,
            jiff::civil::Date::MIN.day() as u8
        ))
    ));
    assert!(VALID_RD_RANGE.contains(
        &(calendrical_calculations::gregorian::fixed_from_gregorian(
            jiff::civil::Date::MAX.year() as i32,
            jiff::civil::Date::MAX.month() as u8,
            jiff::civil::Date::MAX.day() as u8
        ))
    ));
}
