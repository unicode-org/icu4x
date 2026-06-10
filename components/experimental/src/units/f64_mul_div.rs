// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::*;

#[inline]
pub(super) fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    let hi = a * num;
    let err = a.mul_add(num, -hi);
    let res = hi / den;
    let rem = res.mul_add(-den, hi) + err;
    let b = res + (rem / den);

    if b.is_infinite() || b.is_nan() {
        // Fallback to simple multiplication if FMA fails due to overflow/NaN
        a * (num / den)
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_precision() {
        // Case 1: 5 grams to tonnes (1/1_000_000)
        // Naive: 5.0 * (1.0 / 1_000_000.0) = 5.000000000000001e-6
        // FMA: 5e-6
        let val = 5.0;
        let num = 1.0;
        let den = 1_000_000.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.000005);
        assert_eq!(naive, 0.0000049999999999999996);

        // Case 2: 0.1 * (1.0 / 10.0)
        // Naive: 0.010000000000000002
        // FMA: 0.01
        let val = 0.1;
        let num = 1.0;
        let den = 10.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.01);
        assert_eq!(naive, 0.010000000000000002);

        // Case 3: 0.1 * (1.0 / 5.0)
        // Naive: 0.020000000000000004
        // FMA: 0.02
        let val = 0.1;
        let num = 1.0;
        let den = 5.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.02);
        assert_eq!(naive, 0.020000000000000004);
    }
}
