// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Activation functions without branches or table lookups, so that loops calling them
//! can be vectorized, and without a dependency on `libm` in `no_std` builds.

#[inline]
pub(super) fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + exp(-x))
}

#[inline]
pub(super) fn tanh(x: f32) -> f32 {
    // tanh(x) = 1 - 2 / (e^2x + 1); the absolute error near 0 is about 1e-7,
    // which is below what the model can resolve.
    1.0 - 2.0 / (exp(2.0 * x) + 1.0)
}

/// `e^x`.
///
/// Splits `x = n ln 2 + r` with `|r| <= ln 2 / 2` and evaluates `e^r` with a degree-6
/// polynomial (Cephes `expf`), relative error about 2e-7. The input is clamped so that
/// `2^n` stays a normal `f32`.
#[inline]
pub(super) fn exp(x: f32) -> f32 {
    const LOG2_E: f32 = core::f32::consts::LOG2_E;
    // ln 2 split into a high part with few mantissa bits and a correction
    const LN2_HI: f32 = 0.693_359_4;
    const LN2_LO: f32 = -2.121_944_4e-4;
    let x = x.clamp(-87.0, 88.0);
    // Round to nearest by adding and subtracting 1.5 * 2^23 (|x * LOG2_E| < 2^22);
    // `f32::round` is a libm call in no_std builds.
    const ROUND: f32 = 12_582_912.0;
    let t = x * LOG2_E + ROUND;
    let n = t - ROUND;
    let r = x - n * LN2_HI - n * LN2_LO;
    let p = 1.987_569_1e-4_f32;
    let p = p * r + 1.398_199_9e-3;
    let p = p * r + 8.333_452e-3;
    let p = p * r + 4.166_579_6e-2;
    let p = p * r + 1.666_666_5e-1;
    let p = p * r + 0.5;
    let er = p * r * r + r + 1.0;
    // 2^n, built from the exponent bits; n is in [-126, 127] after the clamp. The low
    // mantissa bits of `t` hold n, which avoids a float-to-int cast that would stop
    // the loop from being vectorized.
    let n_bits = t.to_bits().wrapping_sub(ROUND.to_bits());
    let scale = f32::from_bits(n_bits.wrapping_add(127) << 23);
    er * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> impl Iterator<Item = f32> {
        (-20_000..=20_000).map(|i| i as f32 / 100.0)
    }

    #[test]
    fn exp_is_accurate() {
        for x in inputs().filter(|x| (-87.0..=88.0).contains(x)) {
            let expected = f64::from(x).exp();
            let rel = ((f64::from(exp(x)) - expected) / expected).abs();
            assert!(rel < 1e-6, "exp({x}): relative error {rel}");
        }
        // Clamped instead of overflowing or becoming subnormal.
        for x in [-1e30, -1000.0, -88.0, 89.0, 1000.0, 1e30, f32::INFINITY] {
            assert!(exp(x).is_normal(), "exp({x}) = {}", exp(x));
        }
        assert!(exp(f32::NEG_INFINITY).is_normal());
    }

    #[test]
    fn activations_are_accurate() {
        for x in inputs().chain([-1e30, 1e30, f32::INFINITY, f32::NEG_INFINITY]) {
            let x64 = f64::from(x);
            let s = sigmoid(x);
            let t = tanh(x);
            assert!((0.0..=1.0).contains(&s), "sigmoid({x}) = {s}");
            assert!((-1.0..=1.0).contains(&t), "tanh({x}) = {t}");
            assert!(
                (f64::from(s) - 1.0 / (1.0 + (-x64).exp())).abs() < 1e-6,
                "sigmoid({x})"
            );
            assert!((f64::from(t) - x64.tanh()).abs() < 1e-6, "tanh({x})");
        }
    }
}
