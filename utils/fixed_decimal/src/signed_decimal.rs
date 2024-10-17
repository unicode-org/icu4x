// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::uint_iterator::IntIterator;
use crate::{variations::Signed, UnsignedFixedDecimal};
use crate::{ParseError, Sign, SignDisplay};

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
    /// use fixed_decimal::FixedDecimal;
    /// use fixed_decimal::SignDisplay::*;
    ///
    /// assert_eq!(
    ///     "+1729",
    ///     FixedDecimal::from(1729)
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
