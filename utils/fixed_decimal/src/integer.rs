// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;
use core::fmt;

use core::str::FromStr;

use crate::Error;
use crate::FixedDecimal;

/// A [`FixedInteger`] is a [`FixedDecimal`] with no fractional part.
///
///
/// # Examples
///
/// ```
/// # use std::convert::TryFrom;
/// # use std::str::FromStr;
/// use fixed_decimal::Error;
/// use fixed_decimal::FixedDecimal;
/// use fixed_decimal::FixedInteger;
///
/// assert_eq!(
///     FixedDecimal::from(FixedInteger::from(5)),
///     FixedDecimal::from(5)
/// );
/// assert_eq!(
///     FixedInteger::try_from(FixedDecimal::from(5)),
///     Ok(FixedInteger::from(5))
/// );
/// assert_eq!(
///     FixedInteger::try_from(FixedDecimal::from_str("05").unwrap()),
///     Ok(FixedInteger::from_str("05").unwrap())
/// );
/// assert_eq!(
///     FixedInteger::try_from(FixedDecimal::from_str("5.0").unwrap()),
///     Err(Error::Limit)
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FixedInteger(FixedDecimal);

impl From<FixedInteger> for FixedDecimal {
    fn from(value: FixedInteger) -> Self {
        value.0
    }
}

macro_rules! impl_fixed_integer_from_integer_type {
    ($type:ident) => {
        impl From<$type> for FixedInteger {
            fn from(value: $type) -> Self {
                FixedInteger(FixedDecimal::from(value))
            }
        }
    };
}

impl_fixed_integer_from_integer_type!(isize);
impl_fixed_integer_from_integer_type!(i128);
impl_fixed_integer_from_integer_type!(i64);
impl_fixed_integer_from_integer_type!(i32);
impl_fixed_integer_from_integer_type!(i16);
impl_fixed_integer_from_integer_type!(i8);
impl_fixed_integer_from_integer_type!(usize);
impl_fixed_integer_from_integer_type!(u128);
impl_fixed_integer_from_integer_type!(u64);
impl_fixed_integer_from_integer_type!(u32);
impl_fixed_integer_from_integer_type!(u16);
impl_fixed_integer_from_integer_type!(u8);

impl writeable::Writeable for FixedInteger {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        self.0.write_to(sink)
    }
}

impl TryFrom<FixedDecimal> for FixedInteger {
    type Error = Error;
    fn try_from(value: FixedDecimal) -> Result<Self, Error> {
        if value.magnitude_range().start() != &0 {
            Err(Error::Limit)
        } else {
            Ok(FixedInteger(value))
        }
    }
}

impl TryFrom<&[u8]> for FixedInteger {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Error> {
        FixedInteger::try_from(FixedDecimal::try_from(value)?)
    }
}

impl FromStr for FixedInteger {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        FixedInteger::try_from(FixedDecimal::from_str(value)?)
    }
}
