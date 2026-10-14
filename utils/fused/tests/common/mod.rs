// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;
use num_traits::sign::Signed;

/// Finds the binary exponent `e` such that `2^e <= p/q < 2^(e+1)`.
fn find_exponent(p: &BigInt, q: &BigInt) -> i32 {
    let p_bits = p.bits() as i32;
    let q_bits = q.bits() as i32;
    let mut e = p_bits - q_bits - 1;

    loop {
        let cond = if e >= 0 {
            p >= &(q << e as usize)
        } else {
            &(p << (-e) as usize) >= q
        };
        if !cond {
            e -= 1;
            break;
        }
        e += 1;
    }
    e
}

/// Rounds an arbitrary-precision rational number to the nearest `f64`
/// using the IEEE 754 round-to-nearest-even rule and direct bit construction.
pub fn round_ratio_to_f64(r: &Ratio<BigInt>) -> f64 {
    if r.numer() == &BigInt::from(0) {
        return 0.0;
    }
    let is_neg = r.numer() < &BigInt::from(0);
    let r_abs = r.abs();
    let p = r_abs.numer();
    let q = r_abs.denom();

    let e = find_exponent(p, q);

    // f64 minimum exponent for normal numbers is -1022
    let e_scaling = std::cmp::max(e, -1022);

    // Scale the rational by 2^(52 - e_scaling)
    let shift = 52 - e_scaling;
    let n = if shift >= 0 {
        p << shift as usize
    } else {
        p >> (-shift) as usize
    };

    let mut m = &n / q;
    let rem = &n % q;

    // Rounding decision (round to nearest, tie to even)
    let two_rem = &rem << 1;
    let round_up = if &two_rem < q {
        false
    } else if &two_rem > q {
        true
    } else {
        // Tie: round to even (if m is odd, round up)
        &m % 2 != BigInt::from(0)
    };

    if round_up {
        m += 1;
    }

    // Handle significand overflow (if m >= 2^53, scale down)
    let mut final_e = e_scaling;
    let limit = BigInt::from(1) << 53;
    if m >= limit {
        m >>= 1;
        final_e += 1;
    }

    // Handle exponent overflow (overflow to Infinity)
    if final_e > 1023 {
        return if is_neg {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }

    let sign_bit = if is_neg { 1_u64 << 63 } else { 0 };
    let m_u64 = m.to_u64().unwrap();

    let bits = if m_u64 >= (1_u64 << 52) {
        // Normal number
        let exponent_bits = ((final_e + 1023) as u64) << 52;
        let fraction_bits = m_u64 & 0xf_ffff_ffff_ffff;
        sign_bit | exponent_bits | fraction_bits
    } else {
        // Subnormal number (exponent_bits = 0)
        sign_bit | m_u64
    };

    f64::from_bits(bits)
}

/// Computes the ULP distance between two `f64` values.
pub fn ulp_distance_f64(x: f64, y: f64) -> u64 {
    if x.is_nan() || y.is_nan() {
        return u64::MAX;
    }
    if x == y {
        return 0;
    }
    if x.signum() != y.signum() {
        let x_abs = x.abs();
        let y_abs = y.abs();
        if (x_abs == 0.0 && y_abs.to_bits() == 1) || (y_abs == 0.0 && x_abs.to_bits() == 1) {
            return 1;
        }
        return u64::MAX;
    }
    let x_bits = x.to_bits();
    let y_bits = y.to_bits();
    x_bits.abs_diff(y_bits)
}
