// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::uint_iterator::IntIterator;
use crate::{variations::Signed, UnsignedFixedDecimal};
use crate::{
    IncrementLike, NoIncrement, ParseError, RoundingIncrement, RoundingMode, Sign, SignDisplay,
};

/// A Type containing a [`UnsignedFixedDecimal`] and a [`Sign`].
///
/// Supports a mantissa of non-zero digits and a number of leading and trailing
/// zeros, as well as an optional sign; used for formatting and plural selection.
///
/// # Data Types
///
/// The following types can be converted to a `SignedFixedDecimal`:
///
/// - Integers, signed and unsigned
/// - Strings representing an arbitrary-precision decimal
/// - Floating point values (using the `ryu` feature)
///
/// To create a [`SignedFixedDecimal`] with fraction digits, either create it from an integer and then
/// call [`SignedFixedDecimal::multiplied_pow10`], create it from a string, or (when the `ryu` feature is
/// enabled) create it from a floating point value using [`SignedFixedDecimal::try_from_f64`].
///
/// # Magnitude and Position
///
/// Each digit in a `SignedFixedDecimal` is indexed by a *magnitude*, or the digit's power of 10.
/// Illustration for the number "12.34":
///
/// | Magnitude | Digit | Description      |
/// |-----------|-------|------------------|
/// | 1         | 1     | Tens place       |
/// | 0         | 2     | Ones place       |
/// | -1        | 3     | Tenths place     |
/// | -2        | 4     | Hundredths place |
///
/// Some functions deal with a *position* for the purpose of padding, truncating, or rounding a
/// number. In these cases, the position sits between the corresponding magnitude of that position
/// and the next lower significant digit.
/// Illustration:
///
/// ```text
/// Position:   2   0  -2
/// Number:     |1|2.3|4|
/// Position:     1  -1
/// ```
///
/// Expected output of various operations, all with input "12.34":
///
/// | Operation                | Position  | Expected Result |
/// |--------------------------|-----------|-----------------|
/// | Truncate to tens         |         1 |   10            |
/// | Truncate to tenths       |        -1 |   12.3          |
/// | Pad to ten thousands     |         4 | 0012.34         |
/// | Pad to ten thousandths   |        -4 |   12.3400       |
///
/// # Examples
///
/// ```
/// use fixed_decimal::UnsignedFixedDecimal;
///
/// let mut dec = UnsignedFixedDecimal::from(250);
/// assert_eq!("250", dec.to_string());
///
/// dec.multiply_pow10(-2);
/// assert_eq!("2.50", dec.to_string());
/// ```
pub type SignedFixedDecimal = Signed<UnsignedFixedDecimal>;

impl SignedFixedDecimal {
    #[inline]
    /// Parses a [`SignedFixedDecimal`].
    pub fn try_from_str(s: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(s.as_bytes())
    }

    pub fn try_from_utf8(input_str: &[u8]) -> Result<Self, ParseError> {
        // input_str: the input string
        // no_sign_str: the input string when the sign is removed from it
        if input_str.is_empty() {
            return Err(ParseError::Syntax);
        }
        #[allow(clippy::indexing_slicing)] // The string is not empty.
        let sign = match input_str[0] {
            b'-' => Sign::Negative,
            b'+' => Sign::Positive,
            _ => Sign::None,
        };
        #[allow(clippy::indexing_slicing)] // The string is not empty.
        let no_sign_str = if sign == Sign::None {
            input_str
        } else {
            &input_str[1..]
        };
        if no_sign_str.is_empty() {
            return Err(ParseError::Syntax);
        }

        let unsigned_decimal = UnsignedFixedDecimal::try_from_no_sign_utf8(no_sign_str)?;
        Ok(Self {
            sign,
            value: unsigned_decimal,
        })
    }

    /// Returns the sign of this number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// use fixed_decimal::Sign;
    /// # use std::str::FromStr;
    ///
    /// assert_eq!(SignedFixedDecimal::from(1729).sign(), Sign::None);
    /// assert_eq!(
    ///     SignedFixedDecimal::from(-1729).sign(),
    ///     Sign::Negative
    /// );
    /// assert_eq!(
    ///     SignedFixedDecimal::from(1729).sign(),
    ///     Sign::Positive
    /// );
    /// ```
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Clears all the fields and sets the number to zero.
    fn clear(&mut self) {
        self.value.clear();
        self.sign = Sign::None;
    }

    /// Changes the sign of this number to the one given.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// use fixed_decimal::Sign;
    ///
    /// let mut dec = SignedFixedDecimal::from(1729);
    /// assert_eq!("1729", dec.to_string());
    ///
    /// dec.set_sign(Sign::Negative);
    /// assert_eq!("-1729", dec.to_string());
    ///
    /// dec.set_sign(Sign::Positive);
    /// assert_eq!("+1729", dec.to_string());
    ///
    /// dec.set_sign(Sign::None);
    /// assert_eq!("1729", dec.to_string());
    /// ```
    pub fn set_sign(&mut self, sign: Sign) {
        self.sign = sign;
    }

    /// Returns this number with the sign changed to the one given.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// use fixed_decimal::Sign;
    ///
    /// assert_eq!(
    ///     "+1729",
    ///     SignedFixedDecimal::from(1729)
    ///         .with_sign(Sign::Positive)
    ///         .to_string()
    /// );
    /// assert_eq!(
    ///     "1729",
    ///     SignedFixedDecimal::from(-1729).with_sign(Sign::None).to_string()
    /// );
    /// assert_eq!(
    ///     "-1729",
    ///     SignedFixedDecimal::from(1729)
    ///         .with_sign(Sign::Negative)
    ///         .to_string()
    /// );
    /// ```
    pub fn with_sign(mut self, sign: Sign) -> Self {
        self.set_sign(sign);
        self
    }

    /// Sets the sign of this number according to the given sign display strategy.
    ///
    /// # Examples
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// use fixed_decimal::SignDisplay::*;
    ///
    /// let mut dec = SignedFixedDecimal::from(1729);
    /// assert_eq!("1729", dec.to_string());
    /// dec.apply_sign_display(Always);
    /// assert_eq!("+1729", dec.to_string());
    /// ```
    pub fn apply_sign_display(&mut self, sign_display: SignDisplay) {
        use Sign::*;
        match sign_display {
            SignDisplay::Auto => {
                if self.sign != Negative {
                    self.sign = None
                }
            }
            SignDisplay::Always => {
                if self.sign != Negative {
                    self.sign = Positive
                }
            }
            SignDisplay::Never => self.sign = None,
            SignDisplay::ExceptZero => {
                if self.value.is_zero() {
                    self.sign = None
                } else if self.sign != Negative {
                    self.sign = Positive
                }
            }
            SignDisplay::Negative => {
                if self.sign != Negative || self.value.is_zero() {
                    self.sign = None
                }
            }
        }
    }

    /// Returns this number with its sign set according to the given sign display strategy.
    ///
    /// # Examples
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// use fixed_decimal::SignDisplay::*;
    ///
    /// assert_eq!(
    ///     "+1729",
    ///     SignedFixedDecimal::from(1729)
    ///         .with_sign_display(ExceptZero)
    ///         .to_string()
    /// );
    /// ```
    pub fn with_sign_display(mut self, sign_display: SignDisplay) -> Self {
        self.apply_sign_display(sign_display);
        self
    }
}

impl FromStr for SignedFixedDecimal {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

macro_rules! impl_from_signed_integer_type {
    ($itype:ident, $utype: ident) => {
        impl From<$itype> for SignedFixedDecimal {
            fn from(value: $itype) -> Self {
                let int_iterator: IntIterator<$utype> = value.into();
                let sign = if int_iterator.is_negative {
                    Sign::Negative
                } else {
                    Sign::None
                };
                let value = UnsignedFixedDecimal::from_ascending(int_iterator)
                    .expect("All built-in integer types should fit");
                SignedFixedDecimal { sign, value }
            }
        }
    };
}

impl_from_signed_integer_type!(isize, usize);
impl_from_signed_integer_type!(i128, u128);
impl_from_signed_integer_type!(i64, u64);
impl_from_signed_integer_type!(i32, u32);
impl_from_signed_integer_type!(i16, u16);
impl_from_signed_integer_type!(i8, u8);

macro_rules! impl_from_unsigned_integer_type {
    ($utype: ident) => {
        impl From<$utype> for SignedFixedDecimal {
            fn from(value: $utype) -> Self {
                let int_iterator: IntIterator<$utype> = value.into();
                Self {
                    sign: Sign::None,
                    value: UnsignedFixedDecimal::from_ascending(int_iterator)
                        .expect("All built-in integer types should fit"),
                }
            }
        }
    };
}

impl_from_unsigned_integer_type!(usize);
impl_from_unsigned_integer_type!(u128);
impl_from_unsigned_integer_type!(u64);
impl_from_unsigned_integer_type!(u32);
impl_from_unsigned_integer_type!(u16);
impl_from_unsigned_integer_type!(u8);

/// All the rounding and rounding related logic is implmented in this implmentation block.
impl SignedFixedDecimal {
    /// Rounds this number at a particular digit position.
    ///
    /// This uses half to even rounding, which rounds to the nearest integer and resolves ties by
    /// selecting the nearest even integer to the original value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// dec.round(0);
    /// assert_eq!("-2", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// dec.round(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// dec.round(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// dec.round(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// dec.round(0);
    /// assert_eq!("2", dec.to_string());
    /// ```
    pub fn round(&mut self, position: i16) {
        self.half_even_to_increment_internal(position, NoIncrement)
    }

    /// Returns this number rounded at a particular digit position.
    ///
    /// This uses half to even rounding by default, which rounds to the nearest integer and
    /// resolves ties by selecting the nearest even integer to the original value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// assert_eq!("-2", dec.rounded(0).to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// assert_eq!("0", dec.rounded(0).to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// assert_eq!("0", dec.rounded(0).to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// assert_eq!("1", dec.rounded(0).to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// assert_eq!("2", dec.rounded(0).to_string());
    /// ```
    pub fn rounded(mut self, position: i16) -> Self {
        self.round(position);
        self
    }

    /// Rounds this number towards positive infinity at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// dec.ceil(0);
    /// assert_eq!("-1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// dec.ceil(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// dec.ceil(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// dec.ceil(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// dec.ceil(0);
    /// assert_eq!("2", dec.to_string());
    /// ```
    #[inline(never)]
    pub fn ceil(&mut self, position: i16) {
        self.ceil_to_increment_internal(position, NoIncrement);
    }

    /// Returns this number rounded towards positive infinity at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// assert_eq!("-1", dec.ceiled(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// assert_eq!("1", dec.ceiled(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// assert_eq!("1", dec.ceiled(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// assert_eq!("1", dec.ceiled(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// assert_eq!("2", dec.ceiled(0).to_string());
    /// ```
    pub fn ceiled(mut self, position: i16) -> Self {
        self.ceil(position);
        self
    }

    /// Rounds this number away from zero at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// dec.expand(0);
    /// assert_eq!("-2", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// dec.expand(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// dec.expand(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// dec.expand(0);
    /// assert_eq!("1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// dec.expand(0);
    /// assert_eq!("2", dec.to_string());
    /// ```
    #[inline(never)]
    pub fn expand(&mut self, position: i16) {
        self.expand_to_increment_internal(position, NoIncrement)
    }

    /// Returns this number rounded away from zero at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// assert_eq!("-2", dec.expanded(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// assert_eq!("1", dec.expanded(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// assert_eq!("1", dec.expanded(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// assert_eq!("1", dec.expanded(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// assert_eq!("2", dec.expanded(0).to_string());
    /// ```
    pub fn expanded(mut self, position: i16) -> Self {
        self.expand(position);
        self
    }

    /// Rounds this number towards negative infinity at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// dec.floor(0);
    /// assert_eq!("-2", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// dec.floor(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// dec.floor(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// dec.floor(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// dec.floor(0);
    /// assert_eq!("1", dec.to_string());
    /// ```
    #[inline(never)]
    pub fn floor(&mut self, position: i16) {
        self.floor_to_increment_internal(position, NoIncrement);
    }

    /// Returns this number rounded towards negative infinity at a particular digit position.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// assert_eq!("-2", dec.floored(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// assert_eq!("0", dec.floored(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// assert_eq!("0", dec.floored(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// assert_eq!("0", dec.floored(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// assert_eq!("1", dec.floored(0).to_string());
    /// ```
    pub fn floored(mut self, position: i16) -> Self {
        self.floor(position);
        self
    }

    /// Rounds this number towards zero at a particular digit position.
    ///
    /// Also see [`FixedDecimal::pad_end()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// dec.trunc(0);
    /// assert_eq!("-1", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// dec.trunc(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// dec.trunc(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// dec.trunc(0);
    /// assert_eq!("0", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// dec.trunc(0);
    /// assert_eq!("1", dec.to_string());
    /// ```
    #[inline(never)]
    pub fn trunc(&mut self, position: i16) {
        self.trunc_to_increment_internal(position, NoIncrement)
    }

    /// Returns this number rounded towards zero at a particular digit position.
    ///
    /// Also see [`FixedDecimal::padded_end()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::SignedFixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let dec = SignedFixedDecimal::from_str("-1.5").unwrap();
    /// assert_eq!("-1", dec.trunced(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.4").unwrap();
    /// assert_eq!("0", dec.trunced(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.5").unwrap();
    /// assert_eq!("0", dec.trunced(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("0.6").unwrap();
    /// assert_eq!("0", dec.trunced(0).to_string());
    /// let dec = SignedFixedDecimal::from_str("1.5").unwrap();
    /// assert_eq!("1", dec.trunced(0).to_string());
    /// ```
    pub fn trunced(mut self, position: i16) -> Self {
        self.trunc(position);
        self
    }

    /// Rounds this number at a particular digit position, using the specified rounding mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::{FixedDecimal, RoundingMode};
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// dec.round_with_mode(0, RoundingMode::Floor);
    /// assert_eq!("-4", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// dec.round_with_mode(0, RoundingMode::Ceil);
    /// assert_eq!("-3", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("5.455").unwrap();
    /// dec.round_with_mode(-2, RoundingMode::HalfExpand);
    /// assert_eq!("5.46", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("-7.235").unwrap();
    /// dec.round_with_mode(-2, RoundingMode::HalfTrunc);
    /// assert_eq!("-7.23", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("9.75").unwrap();
    /// dec.round_with_mode(-1, RoundingMode::HalfEven);
    /// assert_eq!("9.8", dec.to_string());
    /// ```
    pub fn round_with_mode(&mut self, position: i16, mode: RoundingMode) {
        match mode {
            RoundingMode::Ceil => self.ceil_to_increment_internal(position, NoIncrement),
            RoundingMode::Expand => self.expand_to_increment_internal(position, NoIncrement),
            RoundingMode::Floor => self.floor_to_increment_internal(position, NoIncrement),
            RoundingMode::Trunc => self.trunc_to_increment_internal(position, NoIncrement),
            RoundingMode::HalfCeil => self.half_ceil_to_increment_internal(position, NoIncrement),
            RoundingMode::HalfExpand => {
                self.half_expand_to_increment_internal(position, NoIncrement)
            }
            RoundingMode::HalfFloor => self.half_floor_to_increment_internal(position, NoIncrement),
            RoundingMode::HalfTrunc => self.half_trunc_to_increment_internal(position, NoIncrement),
            RoundingMode::HalfEven => self.half_even_to_increment_internal(position, NoIncrement),
        }
    }

    /// Returns this number rounded at a particular digit position, using the specified rounding mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::{FixedDecimal, RoundingMode};
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// assert_eq!(
    ///     "-4",
    ///     dec.rounded_with_mode(0, RoundingMode::Floor).to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// assert_eq!(
    ///     "-3",
    ///     dec.rounded_with_mode(0, RoundingMode::Ceil).to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("5.455").unwrap();
    /// assert_eq!(
    ///     "5.46",
    ///     dec.rounded_with_mode(-2, RoundingMode::HalfExpand)
    ///         .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("-7.235").unwrap();
    /// assert_eq!(
    ///     "-7.23",
    ///     dec.rounded_with_mode(-2, RoundingMode::HalfTrunc)
    ///         .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("9.75").unwrap();
    /// assert_eq!(
    ///     "9.8",
    ///     dec.rounded_with_mode(-1, RoundingMode::HalfEven)
    ///         .to_string()
    /// );
    /// ```
    pub fn rounded_with_mode(mut self, position: i16, mode: RoundingMode) -> Self {
        self.round_with_mode(position, mode);
        self
    }

    /// Rounds this number at a particular digit position and increment, using the specified rounding mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::{FixedDecimal, RoundingIncrement, RoundingMode};
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// dec.round_with_mode_and_increment(
    ///     0,
    ///     RoundingMode::Floor,
    ///     RoundingIncrement::MultiplesOf1,
    /// );
    /// assert_eq!("-4", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("-3.59").unwrap();
    /// dec.round_with_mode_and_increment(
    ///     -1,
    ///     RoundingMode::Ceil,
    ///     RoundingIncrement::MultiplesOf2,
    /// );
    /// assert_eq!("-3.4", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("5.455").unwrap();
    /// dec.round_with_mode_and_increment(
    ///     -2,
    ///     RoundingMode::HalfExpand,
    ///     RoundingIncrement::MultiplesOf5,
    /// );
    /// assert_eq!("5.45", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("-7.235").unwrap();
    /// dec.round_with_mode_and_increment(
    ///     -2,
    ///     RoundingMode::HalfTrunc,
    ///     RoundingIncrement::MultiplesOf25,
    /// );
    /// assert_eq!("-7.25", dec.to_string());
    /// let mut dec = SignedFixedDecimal::from_str("9.75").unwrap();
    /// dec.round_with_mode_and_increment(
    ///     -1,
    ///     RoundingMode::HalfEven,
    ///     RoundingIncrement::MultiplesOf5,
    /// );
    /// assert_eq!("10.0", dec.to_string());
    /// ```
    pub fn round_with_mode_and_increment(
        &mut self,
        position: i16,
        mode: RoundingMode,
        increment: RoundingIncrement,
    ) {
        match mode {
            RoundingMode::Ceil => self.ceil_to_increment_internal(position, increment),
            RoundingMode::Expand => self.expand_to_increment_internal(position, increment),
            RoundingMode::Floor => self.floor_to_increment_internal(position, increment),
            RoundingMode::Trunc => self.trunc_to_increment_internal(position, increment),
            RoundingMode::HalfCeil => self.half_ceil_to_increment_internal(position, increment),
            RoundingMode::HalfExpand => self.half_expand_to_increment_internal(position, increment),
            RoundingMode::HalfFloor => self.half_floor_to_increment_internal(position, increment),
            RoundingMode::HalfTrunc => self.half_trunc_to_increment_internal(position, increment),
            RoundingMode::HalfEven => self.half_even_to_increment_internal(position, increment),
        }
    }

    /// Returns this number rounded at a particular digit position and increment, using the specified rounding mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::{FixedDecimal, RoundingIncrement, RoundingMode};
    /// # use std::str::FromStr;
    ///
    /// let mut dec = SignedFixedDecimal::from_str("-3.5").unwrap();
    /// assert_eq!(
    ///     "-4",
    ///     dec.rounded_with_mode_and_increment(
    ///         0,
    ///         RoundingMode::Floor,
    ///         RoundingIncrement::MultiplesOf1
    ///     )
    ///     .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("-3.59").unwrap();
    /// assert_eq!(
    ///     "-3.4",
    ///     dec.rounded_with_mode_and_increment(
    ///         -1,
    ///         RoundingMode::Ceil,
    ///         RoundingIncrement::MultiplesOf2
    ///     )
    ///     .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("5.455").unwrap();
    /// assert_eq!(
    ///     "5.45",
    ///     dec.rounded_with_mode_and_increment(
    ///         -2,
    ///         RoundingMode::HalfExpand,
    ///         RoundingIncrement::MultiplesOf5
    ///     )
    ///     .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("-7.235").unwrap();
    /// assert_eq!(
    ///     "-7.25",
    ///     dec.rounded_with_mode_and_increment(
    ///         -2,
    ///         RoundingMode::HalfTrunc,
    ///         RoundingIncrement::MultiplesOf25
    ///     )
    ///     .to_string()
    /// );
    /// let mut dec = SignedFixedDecimal::from_str("9.75").unwrap();
    /// assert_eq!(
    ///     "10.0",
    ///     dec.rounded_with_mode_and_increment(
    ///         -1,
    ///         RoundingMode::HalfEven,
    ///         RoundingIncrement::MultiplesOf5
    ///     )
    ///     .to_string()
    /// );
    /// ```
    pub fn rounded_with_mode_and_increment(
        mut self,
        position: i16,
        mode: RoundingMode,
        increment: RoundingIncrement,
    ) -> Self {
        self.round_with_mode_and_increment(position, mode, increment);
        self
    }

    fn ceil_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        if self.sign == Sign::Negative {
            self.value.trunc_to_increment_internal(position, increment);
            return;
        }

        self.value.expand_to_increment_internal(position, increment);
    }

    fn trunc_to_increment_internal<R: IncrementLike>(&mut self, position: i16, inner_increment: R) {
        self.value
            .trunc_to_increment_internal(position, inner_increment);
    }

    fn half_trunc_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        self.value
            .half_trunc_to_increment_internal(position, increment);
    }

    fn half_ceil_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        if self.sign == Sign::Negative {
            self.half_trunc_to_increment_internal(position, increment);
            return;
        }

        self.half_expand_to_increment_internal(position, increment);
    }

    fn expand_to_increment_internal<R: IncrementLike>(
        &mut self,
        position: i16,
        inner_increment: R,
    ) {
        self.value
            .expand_to_increment_internal(position, inner_increment);
    }

    fn half_expand_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        self.value
            .half_expand_to_increment_internal(position, increment);
    }

    fn half_even_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        self.value
            .half_even_to_increment_internal(position, increment);
    }

    fn floor_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        if self.sign == Sign::Negative {
            self.expand_to_increment_internal(position, increment);
            return;
        }

        self.trunc_to_increment_internal(position, increment);
    }

    fn half_floor_to_increment_internal<R: IncrementLike>(&mut self, position: i16, increment: R) {
        if self.sign == Sign::Negative {
            self.half_expand_to_increment_internal(position, increment);
            return;
        }

        self.half_trunc_to_increment_internal(position, increment);
    }
}
