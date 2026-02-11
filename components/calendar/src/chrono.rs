// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::gregorian::GregorianDateInner;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::Date;
use crate::Gregorian;

impl From<chrono::NaiveDate> for Date<Gregorian> {
    fn from(chrono: chrono::NaiveDate) -> Self {
        use chrono::Datelike;

        // chrono stores dates as year + day of year. we work with that instead
        // of using its conversion code
        let y = chrono.year();
        let (m, d) =  calendrical_calculations::gregorian::year_day(y, chrono.ordinal() as u16);

        // chrono's MIN/MAX value are inside VALID_RD_RANGE
        Self::from_raw(
            GregorianDateInner(ArithmeticDate::new_unchecked(y, m, d)),
            Gregorian,
        )
    }
}

#[test]
fn assert_range() {
    use crate::calendar_arithmetic::VALID_RD_RANGE;
    use crate::types::RataDie;

    const EPOCH: RataDie = calendrical_calculations::gregorian::fixed_from_gregorian(1970, 1, 1);

    assert!(VALID_RD_RANGE.contains(&(EPOCH + chrono::NaiveDate::MIN.to_epoch_days() as i64)));
    assert!(VALID_RD_RANGE.contains(&(EPOCH + chrono::NaiveDate::MAX.to_epoch_days() as i64)));
}
