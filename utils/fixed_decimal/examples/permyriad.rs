// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// In computers, monetary values are sometimes stored as integers representing one ten-thousandth
// (one permyriad) of a monetary unit. FixedDecimal enables a cheap representation of these
// amounts, also while retaining trailing zeros.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use fixed_decimal::FixedDecimal;
use writeable::Writeable;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();
    let monetary_int = 19_9500;
    let fixed_decimal = FixedDecimal::from(monetary_int).multiplied_pow10(-4);

    assert_eq!(fixed_decimal.write_to_string(), "19.9500");

    0
}
