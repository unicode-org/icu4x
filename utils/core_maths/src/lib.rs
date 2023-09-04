// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Extension trait for full float functionality in `#[no_std]` backed by [`libm`].
//!
//! Method signatures and documentation are the same as `std` as of 1.72.
//!
//! # Usage
//! ```rust
//! #[allow(unused_imports)] // will be unused on std targets
//! use core_maths::*;
//!
//! 3.9.floor();
//! ```

#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

/// See [`crate`].
pub trait CoreFloat: Sized + Copy {
    /// Returns the largest integer less than or equal to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.7_f64;
    /// let g = 3.0_f64;
    /// let h = -3.7_f64;
    ///
    /// assert_eq!(f.floor(), 3.0);
    /// assert_eq!(g.floor(), 3.0);
    /// assert_eq!(h.floor(), -4.0);
    /// ```
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.01_f64;
    /// let g = 4.0_f64;
    ///
    /// assert_eq!(f.ceil(), 4.0);
    /// assert_eq!(g.ceil(), 4.0);
    /// ```
    fn ceil(self) -> Self;

    /// Returns the nearest integer to `self`. If a value is half-way between two
    /// integers, round away from `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.3_f64;
    /// let g = -3.3_f64;
    /// let h = -3.7_f64;
    /// let i = 3.5_f64;
    /// let j = 4.5_f64;
    ///
    /// assert_eq!(f.round(), 3.0);
    /// assert_eq!(g.round(), -3.0);
    /// assert_eq!(h.round(), -4.0);
    /// assert_eq!(i.round(), 4.0);
    /// assert_eq!(j.round(), 5.0);
    /// ```
    fn round(self) -> Self;

    /// Returns the nearest integer to a number. Rounds half-way cases to the number
    /// with an even least significant digit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core_maths::*;
    /// let f = 3.3_f64;
    /// let g = -3.3_f64;
    /// let h = 3.5_f64;
    /// let i = 4.5_f64;
    ///
    /// assert_eq!(f.round_ties_even(), 3.0);
    /// assert_eq!(g.round_ties_even(), -3.0);
    /// assert_eq!(h.round_ties_even(), 4.0);
    /// assert_eq!(i.round_ties_even(), 4.0);
    /// ```
    fn round_ties_even(self) -> Self;

    /// Returns the integer part of `self`.
    /// This means that non-integer numbers are always truncated towards zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.7_f64;
    /// let g = 3.0_f64;
    /// let h = -3.7_f64;
    ///
    /// assert_eq!(f.trunc(), 3.0);
    /// assert_eq!(g.trunc(), 3.0);
    /// assert_eq!(h.trunc(), -3.0);
    /// ```
    fn trunc(self) -> Self;

    /// Returns the fractional part of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.6_f64;
    /// let y = -3.6_f64;
    /// let abs_difference_x = (x.fract() - 0.6).abs();
    /// let abs_difference_y = (y.fract() - (-0.6)).abs();
    ///
    /// assert!(abs_difference_x < 1e-10);
    /// assert!(abs_difference_y < 1e-10);
    /// ```
    fn fract(self) -> Self;

    /// Computes the absolute value of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.5_f64;
    /// let y = -3.5_f64;
    ///
    /// let abs_difference_x = (x.abs() - x).abs();
    /// let abs_difference_y = (y.abs() - (-y)).abs();
    ///
    /// assert!(abs_difference_x < 1e-10);
    /// assert!(abs_difference_y < 1e-10);
    ///
    /// assert!(f64::NAN.abs().is_nan());
    /// ```
    fn abs(self) -> Self;

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f64;
    ///
    /// assert_eq!(f.signum(), 1.0);
    /// assert_eq!(f64::NEG_INFINITY.signum(), -1.0);
    ///
    /// assert!(f64::NAN.signum().is_nan());
    /// ```
    fn signum(self) -> Self;

    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise
    /// equal to `-self`. If `self` is a NaN, then a NaN with the sign bit of
    /// `sign` is returned. Note, however, that conserving the sign bit on NaN
    /// across arithmetical operations is not generally guaranteed.
    /// See [explanation of NaN as a special value](primitive@f32) for more info.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 3.5_f64;
    ///
    /// assert_eq!(f.copysign(0.42), 3.5_f64);
    /// assert_eq!(f.copysign(-0.42), -3.5_f64);
    /// assert_eq!((-f).copysign(0.42), 3.5_f64);
    /// assert_eq!((-f).copysign(-0.42), -3.5_f64);
    ///
    /// assert!(f64::NAN.copysign(1.0).is_nan());
    /// ```
    fn copysign(self, sign: Self) -> Self;

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if
    /// the target architecture has a dedicated `fma` CPU instruction. However,
    /// this is not always true, and will be heavily dependant on designing
    /// algorithms with specific target hardware in mind.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = 10.0_f64;
    /// let x = 4.0_f64;
    /// let b = 60.0_f64;
    ///
    /// // 100.0
    /// let abs_difference = (m.mul_add(x, b) - ((m * x) + b)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n`
    /// such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: f64 = 7.0;
    /// let b = 4.0;
    /// assert_eq!(a.div_euclid(b), 1.0); // 7.0 > 4.0 * 1.0
    /// assert_eq!((-a).div_euclid(b), -2.0); // -7.0 >= 4.0 * -2.0
    /// assert_eq!(a.div_euclid(-b), -1.0); // 7.0 >= -4.0 * -1.0
    /// assert_eq!((-a).div_euclid(-b), 2.0); // -7.0 >= -4.0 * 2.0
    /// ```
    fn div_euclid(self, rhs: Self) -> Self;

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == rhs.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `rhs.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)`
    /// approximately.
    ///
    /// # Examples
    ///
    /// ```
    /// let a: f64 = 7.0;
    /// let b = 4.0;
    /// assert_eq!(a.rem_euclid(b), 3.0);
    /// assert_eq!((-a).rem_euclid(b), 1.0);
    /// assert_eq!(a.rem_euclid(-b), 3.0);
    /// assert_eq!((-a).rem_euclid(-b), 1.0);
    /// // limitation due to round-off error
    /// assert!((-f64::EPSILON).rem_euclid(3.0) != 0.0);
    /// ```
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`.
    /// It might have a different sequence of rounding operations than `powf`,
    /// so the results are not guaranteed to agree.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let abs_difference = (x.powi(2) - (x * x)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn powi(self, n: i32) -> Self;

    /// Raises a number to a floating point power.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn powf(self, n: Self) -> Self;

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let positive = 4.0_f64;
    /// let negative = -4.0_f64;
    /// let negative_zero = -0.0_f64;
    ///
    /// let abs_difference = (positive.sqrt() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// assert!(negative.sqrt().is_nan());
    /// assert!(negative_zero.sqrt() == negative_zero);
    /// ```
    fn sqrt(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0_f64;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 2.0_f64;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0_f64;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and
    /// `self.log10()` can produce more accurate results for base 10.
    ///
    /// # Examples
    ///
    /// ```
    /// let twenty_five = 25.0_f64;
    ///
    /// // log5(25) - 2 == 0
    /// let abs_difference = (twenty_five.log(5.0) - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let four = 4.0_f64;
    ///
    /// // log2(4) - 2 == 0
    /// let abs_difference = (four.log2() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let hundred = 100.0_f64;
    ///
    /// // log10(100) - 2 == 0
    /// let abs_difference = (hundred.log10() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn log10(self) -> Self;

    /// Returns the cube root of a number.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 8.0_f64;
    ///
    /// // x^(1/3) - 2 == 0
    /// let abs_difference = (x.cbrt() - 2.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn cbrt(self) -> Self;

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f64;
    /// let y = 3.0_f64;
    ///
    /// // sqrt(x^2 + y^2)
    /// let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn hypot(self, other: Self) -> Self;

    /// Computes the sine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn sin(self) -> Self;

    /// Computes the cosine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0 * std::f64::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn cos(self) -> Self;

    /// Computes the tangent of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-14);
    /// ```
    fn tan(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f64::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f64::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn asin(self) -> Self;

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f64::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f64::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn acos(self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 1.0_f64;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Examples
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0_f64;
    /// let y1 = -3.0_f64;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0_f64;
    /// let y2 = 3.0_f64;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-std::f64::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f64::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 < 1e-10);
    /// assert!(abs_difference_2 < 1e-10);
    /// ```
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-16_f64;
    ///
    /// // for very small x, e^x is approximately 1 + x + x^2 / 2
    /// let approx = x + x * x / 2.0;
    /// let abs_difference = (x.exp_m1() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-20);
    /// ```
    fn exp_m1(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-16_f64;
    ///
    /// // for very small x, ln(1 + x) is approximately x - x^2 / 2
    /// let approx = x - x * x / 2.0;
    /// let abs_difference = (x.ln_1p() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-20);
    /// ```
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let x = 1.0_f64;
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0_f64;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f64::consts::E;
    /// let f = e.tanh().atanh();
    ///
    /// let abs_difference = (f - e).abs();
    ///
    /// assert!(abs_difference < 1.0e-10);
    /// ```
    fn atanh(self) -> Self;
}

impl CoreFloat for f32 {
    #[inline]
    fn floor(self) -> Self {
        libm::floorf(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }

    #[inline]
    fn round(self) -> Self {
        libm::roundf(self)
    }

    #[inline]
    fn round_ties_even(self) -> Self {
        libm::rintf(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        libm::truncf(self)
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            1.0_f32.copysign(self)
        }
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        libm::copysignf(self, sign)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fmaf(self, a, b)
    }

    fn div_euclid(self, rhs: Self) -> Self {
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }

    #[inline]
    fn powi(self, mut exp: i32) -> Self {
        if exp == 0 {
            return 1.0;
        }
        let mut base = self;
        let mut acc = 1.0;

        while exp > 1 {
            if (exp & 1) == 1 {
                acc *= base;
            }
            exp /= 2;
            base = base * base;
        }

        // since exp!=0, finally the exp must be 1.
        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        acc * base
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        libm::powf(self, n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    #[inline]
    fn exp(self) -> Self {
        libm::expf(self)
    }

    #[inline]
    fn exp2(self) -> Self {
        libm::exp2f(self)
    }

    #[inline]
    fn ln(self) -> Self {
        libm::logf(self)
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    #[inline]
    fn log2(self) -> Self {
        libm::log2f(self)
    }

    #[inline]
    fn log10(self) -> Self {
        libm::log10f(self)
    }

    #[inline]
    fn cbrt(self) -> Self {
        libm::cbrtf(self)
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        libm::hypotf(self, other)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sinf(self)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cosf(self)
    }

    #[inline]
    fn tan(self) -> Self {
        libm::tanf(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asinf(self)
    }

    #[inline]
    fn acos(self) -> Self {
        libm::acosf(self)
    }

    #[inline]
    fn atan(self) -> Self {
        libm::atanf(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }

    #[inline]
    fn exp_m1(self) -> Self {
        libm::expm1f(self)
    }

    #[inline]
    fn ln_1p(self) -> Self {
        libm::log1pf(self)
    }

    #[inline]
    fn sinh(self) -> Self {
        libm::sinhf(self)
    }

    #[inline]
    fn cosh(self) -> Self {
        libm::coshf(self)
    }

    #[inline]
    fn tanh(self) -> Self {
        libm::tanhf(self)
    }

    #[inline]
    fn asinh(self) -> Self {
        let ax = self.abs();
        let ix = 1.0 / ax;
        (ax + (ax / (Self::hypot(1.0, ix) + ix)))
            .ln_1p()
            .copysign(self)
    }

    #[inline]
    fn acosh(self) -> Self {
        if self < 1.0 {
            Self::NAN
        } else {
            (self + ((self - 1.0).sqrt() * (self + 1.0).sqrt())).ln()
        }
    }

    #[inline]
    fn atanh(self) -> Self {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }
}

impl CoreFloat for f64 {
    #[inline]
    fn floor(self) -> Self {
        libm::floor(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        libm::ceil(self)
    }

    #[inline]
    fn round(self) -> Self {
        libm::round(self)
    }

    #[inline]
    fn round_ties_even(self) -> Self {
        libm::rint(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        libm::trunc(self)
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn abs(self) -> Self {
        libm::fabs(self)
    }

    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            1.0_f64.copysign(self)
        }
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        libm::copysign(self, sign)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fma(self, a, b)
    }

    fn div_euclid(self, rhs: Self) -> Self {
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }

    #[inline]
    fn powi(self, mut exp: i32) -> Self {
        if exp == 0 {
            return 1.0;
        }
        let mut base = self;
        let mut acc = 1.0;

        while exp > 1 {
            if (exp & 1) == 1 {
                acc *= base;
            }
            exp /= 2;
            base = base * base;
        }

        // since exp!=0, finally the exp must be 1.
        // Deal with the final bit of the exponent separately, since
        // squaring the base afterwards is not necessary and may cause a
        // needless overflow.
        acc * base
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        libm::pow(self, n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    #[inline]
    fn exp(self) -> Self {
        libm::exp(self)
    }

    #[inline]
    fn exp2(self) -> Self {
        libm::exp2(self)
    }

    #[inline]
    fn ln(self) -> Self {
        libm::log(self)
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    #[inline]
    fn log2(self) -> Self {
        libm::log2(self)
    }

    #[inline]
    fn log10(self) -> Self {
        libm::log10(self)
    }

    #[inline]
    fn cbrt(self) -> Self {
        libm::cbrt(self)
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        libm::hypot(self, other)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sin(self)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cos(self)
    }

    #[inline]
    fn tan(self) -> Self {
        libm::tan(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asin(self)
    }

    #[inline]
    fn acos(self) -> Self {
        libm::acos(self)
    }

    #[inline]
    fn atan(self) -> Self {
        libm::atan(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }

    #[inline]
    fn exp_m1(self) -> Self {
        libm::expm1(self)
    }

    #[inline]
    fn ln_1p(self) -> Self {
        libm::log1p(self)
    }

    #[inline]
    fn sinh(self) -> Self {
        libm::sinh(self)
    }

    #[inline]
    fn cosh(self) -> Self {
        libm::cosh(self)
    }

    #[inline]
    fn tanh(self) -> Self {
        libm::tanh(self)
    }

    #[inline]
    fn asinh(self) -> Self {
        let ax = self.abs();
        let ix = 1.0 / ax;
        (ax + (ax / (Self::hypot(1.0, ix) + ix)))
            .ln_1p()
            .copysign(self)
    }

    #[inline]
    fn acosh(self) -> Self {
        if self < 1.0 {
            Self::NAN
        } else {
            (self + ((self - 1.0).sqrt() * (self + 1.0).sqrt())).ln()
        }
    }

    #[inline]
    fn atanh(self) -> Self {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }
}
