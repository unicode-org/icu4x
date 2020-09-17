// In computers, monetary values are sometimes stored as integers representing one ten-thousandth
// (one permyriad) of a monetary unit. FixedDecimal enables a cheap representation of these
// amounts, also while retaining trailing zeros.

use icu_num_util::FixedDecimal;

fn main() {
    let monetary_int = 19_9500;
    let fixed_decimal = FixedDecimal::from(monetary_int)
        .multiplied_pow10(-4)
        .expect("-4 is well in range");
    assert_eq!("19.9500", fixed_decimal.to_string());
}
