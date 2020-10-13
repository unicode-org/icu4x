// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use fixed_decimal::FixedDecimal;
use std::convert::TryFrom;
use std::io::Error as IOError;
use std::isize;
use std::num::ParseIntError;
use std::str::FromStr;

/// A full plural operands representation of a number. See [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) for complete operands description.
/// Plural operands in compliance with [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
///
/// See [full operands description](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
///
/// # Examples
///
/// From int
///
/// ```
/// use icu_plurals::PluralOperands;
/// assert_eq!(PluralOperands {
///    i: 2,
///    v: 0,
///    w: 0,
///    f: 0,
///    t: 0,
/// }, PluralOperands::from(2_usize))
/// ```
///
/// From float
///
/// ```
/// use std::str::FromStr;
/// use icu_plurals::PluralOperands;
/// assert_eq!(Ok(PluralOperands {
///    i: 1234,
///    v: 3,
///    w: 3,
///    f: 567,
///    t: 567,
/// }), "-1234.567".parse())
/// ```
///
/// From &str
///
/// ```
/// use std::convert::TryFrom;
/// use icu_plurals::PluralOperands;
/// assert_eq!(Ok(PluralOperands {
///    i: 123,
///    v: 2,
///    w: 2,
///    f: 45,
///    t: 45,
/// }), "123.45".parse())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PluralOperands {
    /// Integer value of input
    pub i: u64,
    /// Number of visible fraction digits with trailing zeros
    pub v: usize,
    /// Number of visible fraction digits without trailing zeros
    pub w: usize,
    /// Visible fraction digits with trailing zeros
    pub f: u64,
    /// Visible fraction digits without trailing zeros
    pub t: u64,
}

impl PluralOperands {
    /// Returns the number represented by this [PluralOperands] as floating point.
    /// The precision of the number returned is up to the representation accuracy
    /// of a double.
    pub fn n(&self) -> f64 {
        let fraction = self.t as f64 / 10_f64.powi(self.v as i32);
        self.i as f64 + fraction
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperandsError {
    /// Input to the Operands parsing was empty.
    Empty,
    /// Input to the Operands parsing was invalid.
    Invalid,
}

impl From<ParseIntError> for OperandsError {
    fn from(_: ParseIntError) -> Self {
        Self::Invalid
    }
}

impl From<IOError> for OperandsError {
    fn from(_: IOError) -> Self {
        Self::Invalid
    }
}

impl FromStr for PluralOperands {
    type Err = OperandsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(OperandsError::Empty);
        }

        let abs_str = if input.starts_with('-') {
            &input[1..]
        } else {
            &input
        };

        let (
            integer_digits,
            num_fraction_digits0,
            num_fraction_digits,
            fraction_digits0,
            fraction_digits,
        ) = if let Some(sep_idx) = abs_str.find('.') {
            let int_str = &abs_str[..sep_idx];
            let dec_str = &abs_str[(sep_idx + 1)..];

            let integer_digits = u64::from_str(&int_str)?;

            let dec_str_no_zeros = dec_str.trim_end_matches('0');

            let num_fraction_digits0 = dec_str.len() as usize;
            let num_fraction_digits = dec_str_no_zeros.len() as usize;

            let fraction_digits0 = u64::from_str(&dec_str)?;
            let fraction_digits =
                if num_fraction_digits == 0 || num_fraction_digits == num_fraction_digits0 {
                    fraction_digits0
                } else {
                    u64::from_str(&dec_str_no_zeros)?
                };

            (
                integer_digits,
                num_fraction_digits0,
                num_fraction_digits,
                fraction_digits0,
                fraction_digits,
            )
        } else {
            let integer_digits = u64::from_str(&abs_str)?;
            (integer_digits, 0, 0, 0, 0)
        };

        Ok(PluralOperands {
            i: integer_digits,
            v: num_fraction_digits0,
            w: num_fraction_digits,
            f: fraction_digits0,
            t: fraction_digits,
        })
    }
}

macro_rules! impl_integer_type {
    ($ty:ident) => {
        impl From<$ty> for PluralOperands {
            fn from(input: $ty) -> Self {
                PluralOperands {
                    i: input as u64,
                    v: 0,
                    w: 0,
                    f: 0,
                    t: 0,
                }
            }
        }
    };
    ($($ty:ident)+) => {
        $(impl_integer_type!($ty);)+
    };
}

macro_rules! impl_signed_integer_type {
    ($ty:ident) => {
        impl TryFrom<$ty> for PluralOperands {
            type Error = OperandsError;
            fn try_from(input: $ty) -> Result<Self, Self::Error> {
                let x = input.checked_abs().ok_or(OperandsError::Invalid)?;
                Ok(PluralOperands {
                    i: x as u64,
                    v: 0,
                    w: 0,
                    f: 0,
                    t: 0,
                })
            }
        }
    };
    ($($ty:ident)+) => {
        $(impl_signed_integer_type!($ty);)+
    };
}

impl_integer_type!(u8 u16 u32 u64 u128 usize);
impl_signed_integer_type!(i8 i16 i32 i64 i128 isize);

impl From<&FixedDecimal> for PluralOperands {
    /// Converts a [fixed_decimal::FixedDecimal] to [PluralOperands]. Retains at most 18
    /// digits each from the integer and fraction parts.
    fn from(dec: &FixedDecimal) -> Self {
        let mag_range = dec.magnitude_range();
        let mag_high = std::cmp::min(17, *mag_range.end());
        let mag_low = std::cmp::max(-18, *mag_range.start());

        let mut i: u64 = 0;
        for magnitude in (0..=mag_high).rev() {
            i *= 10;
            i += dec.digit_at(magnitude) as u64;
        }

        let mut f: u64 = 0;
        let mut t: u64 = 0;
        let mut w: usize = 0;
        for magnitude in (mag_low..=-1).rev() {
            let digit = dec.digit_at(magnitude) as u64;
            f *= 10;
            f += digit;
            if digit != 0 {
                t = f;
                w = (-magnitude) as usize;
            }
        }

        PluralOperands {
            i,
            v: (-mag_low) as usize,
            w,
            f,
            t,
        }
    }
}
