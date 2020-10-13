// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// In computers, monetary values are sometimes stored as integers representing one ten-thousandth
// (one permyriad) of a monetary unit. FixedDecimal enables a cheap representation of these
// amounts, also while retaining trailing zeros.

use fixed_decimal::FixedDecimal;

fn main() {
    let monetary_int = 19_9500;
    let fixed_decimal = FixedDecimal::from(monetary_int)
        .multiplied_pow10(-4)
        .expect("-4 is well in range");
    assert_eq!("19.9500", fixed_decimal.to_string());
}
