// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::uint_iterator::IntIterator;
use crate::{variations::Signed, UnsignedFixedDecimal};
use crate::{ParseError, Sign};

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

        let unsigned_decimal = UnsignedFixedDecimal::try_from_utf8(no_sign_str)?;
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
