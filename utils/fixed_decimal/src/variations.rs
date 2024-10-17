// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// This module defines variations of numeric types, including signed values,
/// values with infinity, and values with NaN.

/// Specifies the precision of a floating point value when constructing a FixedDecimal.
///
/// IEEE 754 is a representation of a point on the number line. On the other hand, FixedDecimal
/// specifies not only the point on the number line but also the precision of the number to a
/// specific power of 10. This enum augments a floating-point value with the additional
/// information required by FixedDecimal.
#[non_exhaustive]
#[cfg(feature = "ryu")]
#[derive(Debug, Clone, Copy)]
pub enum FloatPrecision {
    /// Specify that the floating point number is integer-valued.
    ///
    /// If the floating point is not actually integer-valued, an error will be returned.
    Integer,

    /// Specify that the floating point number is precise to a specific power of 10.
    /// The number may be rounded or trailing zeros may be added as necessary.
    Magnitude(i16),

    /// Specify that the floating point number is precise to a specific number of significant digits.
    /// The number may be rounded or trailing zeros may be added as necessary.
    ///
    /// The number requested may not be zero
    SignificantDigits(u8),

    /// Specify that the floating point number is precise to the maximum representable by IEEE.
    ///
    /// This results in a FixedDecimal having enough digits to recover the original floating point
    /// value, with no trailing zeros.
    RoundTrip,
}

// TODO: move to sign.rs
/// A specification of the sign used when formatting a number.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(clippy::exhaustive_enums)]
// There are only 3 sign values, and they correspond to the low-level data model of FixedDecimal and UTS 35.
pub enum Sign {
    /// No sign (implicitly positive, e.g., 1729).
    None,
    /// A negative sign, e.g., -1729.
    Negative,
    /// An explicit positive sign, e.g., +1729.
    Positive,
}

/// Configuration for when to render the minus sign or plus sign.
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SignDisplay {
    /// Render the sign according to locale preferences. In most cases, this means a minus sign
    /// will be shown on negative numbers, and no sign will be shown on positive numbers.
    Auto,

    /// Do not display the sign. Positive and negative numbers are indistinguishable.
    Never,

    /// Show a minus sign on negative numbers and a plus sign on positive numbers, including zero.
    Always,

    /// Show a minus sign on negative numbers and a plus sign on positive numbers, except do not
    /// show any sign on positive or negative zero.
    ExceptZero,

    /// Show a minus sign on strictly negative numbers. Do not show a sign on positive numbers or
    /// on positive or negative zero.
    ///
    /// This differs from [`Auto`](SignDisplay::Auto) in that it does not render a sign on negative zero.
    Negative,
}

// Adapters to convert runtime dispatched calls into const-inlined methods.
// This allows reducing the codesize for the common case of no increment.

#[derive(Copy, Clone, PartialEq)]
struct NoIncrement;

trait IncrementLike: Copy + Sized + PartialEq {
    const MULTIPLES_OF_1: Option<Self>;
    const MULTIPLES_OF_2: Option<Self>;
    const MULTIPLES_OF_5: Option<Self>;
    const MULTIPLES_OF_25: Option<Self>;
}

impl IncrementLike for RoundingIncrement {
    const MULTIPLES_OF_1: Option<Self> = Some(Self::MultiplesOf1);

    const MULTIPLES_OF_2: Option<Self> = Some(Self::MultiplesOf2);

    const MULTIPLES_OF_5: Option<Self> = Some(Self::MultiplesOf5);

    const MULTIPLES_OF_25: Option<Self> = Some(Self::MultiplesOf25);
}

impl IncrementLike for NoIncrement {
    const MULTIPLES_OF_1: Option<Self> = Some(Self);

    const MULTIPLES_OF_2: Option<Self> = None;

    const MULTIPLES_OF_5: Option<Self> = None;

    const MULTIPLES_OF_25: Option<Self> = None;
}

/// Mode used in a rounding operation.
///
/// # Comparative table of rounding modes
///
/// | Value | Ceil | Expand | Floor | Trunc | HalfCeil | HalfExpand | HalfFloor | HalfTrunc | HalfEven |
/// |:-----:|:----:|:------:|:-----:|:-----:|:--------:|:----------:|:---------:|:---------:|:--------:|
/// |  +1.8 |  +2  |   +2   |   +1  |   +1  |    +2    |     +2     |     +2    |     +2    |    +2    |
/// |  +1.5 |   "  |    "   |   "   |   "   |     "    |      "     |     +1    |     +1    |     "    |
/// |  +1.2 |   "  |    "   |   "   |   "   |    +1    |     +1     |     "     |     "     |    +1    |
/// |  +0.8 |  +1  |   +1   |   0   |   0   |     "    |      "     |     "     |     "     |     "    |
/// |  +0.5 |   "  |    "   |   "   |   "   |     "    |      "     |     0     |     0     |     0    |
/// |  +0.2 |   "  |    "   |   "   |   "   |     0    |      0     |     "     |     "     |     "    |
/// |  -0.2 |   0  |   -1   |   -1  |   "   |     "    |      "     |     "     |     "     |     "    |
/// |  -0.5 |   "  |    "   |   "   |   "   |     "    |     -1     |     -1    |     "     |     "    |
/// |  -0.8 |   "  |    "   |   "   |   "   |    -1    |      "     |     "     |     -1    |    -1    |
/// |  -1.2 |  -1  |   -2   |   -2  |   -1  |     "    |      "     |     "     |     "     |     "    |
/// |  -1.5 |   "  |    "   |   "   |   "   |     "    |     -2     |     -2    |     "     |    -2    |
/// |  -1.8 |   "  |    "   |   "   |   "   |    -2    |      "     |     "     |     -2    |     "    |
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RoundingMode {
    /// Round up, or towards positive infinity.
    Ceil,
    /// Round away from zero, or towards infinity.
    Expand,
    /// Round down, or towards negative infinity.
    Floor,
    /// Round towards zero, or away from infinity.
    Trunc,
    /// Round to the nearest integer, resolving ties by rounding up.
    HalfCeil,
    /// Round to the nearest integer, resolving ties by rounding away from zero.
    HalfExpand,
    /// Round to the nearest integer, resolving ties by rounding down.
    HalfFloor,
    /// Round to the nearest integer, resolving ties by rounding towards zero.
    HalfTrunc,
    /// Round to the nearest integer, resolving ties by rounding towards the nearest even integer.
    HalfEven,
}

/// Increment used in a rounding operation.
///
/// Forces a rounding operation to round to only multiples of the specified increment.
///
/// # Example
///
/// ```
/// use fixed_decimal::{FixedDecimal, RoundingIncrement, RoundingMode};
/// use writeable::assert_writeable_eq;
/// # use std::str::FromStr;
/// let dec = FixedDecimal::from_str("-7.266").unwrap();
/// let mode = RoundingMode::Expand;
/// let increments = [
///     // .266 normally expands to .27 when rounding on position -2...
///     (RoundingIncrement::MultiplesOf1, "-7.27"),
///     // ...however, when rounding to multiples of 2, .266 expands to .28, since the next multiple
///     // of 2 bigger than the least significant digit of the rounded value (7) is 8.
///     (RoundingIncrement::MultiplesOf2, "-7.28"),
///     // .266 expands to .30, since the next multiple of 5 bigger than 7 is 10.
///     (RoundingIncrement::MultiplesOf5, "-7.30"),
///     // .266 expands to .50, since the next multiple of 25 bigger than 27 is 50.
///     // Note how we compare against 27 instead of only 7, because the increment applies to
///     // the two least significant digits of the rounded value instead of only the least
///     // significant digit.
///     (RoundingIncrement::MultiplesOf25, "-7.50"),
/// ];
///
/// for (increment, expected) in increments {
///     assert_writeable_eq!(
///         dec.clone().rounded_with_mode_and_increment(
///             -2,
///             mode,
///             increment
///         ),
///         expected
///     );
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum RoundingIncrement {
    /// Round the least significant digit to any digit (0-9).
    ///
    /// This is the default rounding increment for all the methods that don't take a
    /// `RoundingIncrement` as an argument.
    #[default]
    MultiplesOf1,
    /// Round the least significant digit to multiples of two (0, 2, 4, 6, 8).
    MultiplesOf2,
    /// Round the least significant digit to multiples of five (0, 5).
    MultiplesOf5,
    /// Round the two least significant digits to multiples of twenty-five (0, 25, 50, 75).
    ///
    /// With this increment, the rounding position index will match the least significant digit
    /// of the multiple of 25; e.g. the number .264 expanded at position -2 using increments of 25
    /// will give .50 as a result, since the next multiple of 25 bigger than 26 is 50.
    MultiplesOf25,
}

/// The `Signed` struct represents a numeric value with an associated sign.
#[derive(Debug, Clone, PartialEq)]
pub struct Signed<T> {
    pub sign: Sign,
    pub value: T,
}

/// The `WithInfinity` enum represents a numeric value that may be either infinite or finite.
#[derive(Debug)]
pub enum WithInfinity<T> {
    Infinity,
    Finite(T),
}

/// The `WithNaN` enum represents a numeric value that may be NaN.
#[derive(Debug)]
pub enum WithNaN<T> {
    NaN,
    N(T),
}

/// The `WithCompactExponent` struct represents a numeric value with a compact exponent.
#[derive(Debug)]
pub struct WithCompactExponent<T> {
    pub exponent: u8,
    pub significand: T,
}

/// The `WithScientificExponent` struct represents a numeric value with a scientific exponent.
#[derive(Debug)]
pub struct WithScientificExponent<T> {
    pub exponent: i16,
    pub significand: T,
}
