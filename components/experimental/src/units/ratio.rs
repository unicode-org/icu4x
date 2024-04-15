// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Signed;
use num_traits::{One, Pow, Zero};

use super::provider::{Base, SiPrefix};

// TODO: add test cases for IcuRatio.
/// A ratio type that uses `BigInt` as the underlying type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IcuRatio(Ratio<BigInt>);

#[derive(Debug)]
pub enum IcuRatioError {
    /// Represents an error when parsing a `BigInt` from a string.
    /// For example, the string "1/23" is invalid because it contains a '/' character.
    ///              the string "3A" is invalid because it contains a non-numeric character.
    BigIntParseError(num_bigint::ParseBigIntError),

    /// Represents an error when parsing a ratio from a string.
    /// For example, the string "1/0/2" is invalid because it contains more than one '/' character.
    InvalidRatioString,
}

impl IcuRatio {
    /// Creates a new `IcuRatio` from the given numerator and denominator.
    pub(crate) fn from_big_ints(numerator: BigInt, denominator: BigInt) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    /// Returns the reciprocal of the ratio.
    /// For example, the reciprocal of 2/3 is 3/2.
    pub(crate) fn recip(&self) -> Self {
        Self(self.0.recip())
    }

    /// Returns the absolute value of the ratio.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a Ratio with value of 2.
    pub fn two() -> Self {
        Self(Ratio::from_integer(2.into()))
    }

    /// Returns a Ratio with value of 10.
    pub fn ten() -> Self {
        Self(Ratio::from_integer(10.into()))
    }
}

impl Mul for IcuRatio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Mul<&IcuRatio> for &IcuRatio {
    type Output = IcuRatio;

    fn mul(self, rhs: &IcuRatio) -> IcuRatio {
        IcuRatio(&self.0 * &rhs.0)
    }
}

impl MulAssign<IcuRatio> for IcuRatio {
    fn mul_assign(&mut self, rhs: IcuRatio) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<&SiPrefix> for IcuRatio {
    fn mul_assign(&mut self, rhs: &SiPrefix) {
        match rhs.base {
            Base::Decimal => {
                *self *= IcuRatio::ten().pow(rhs.power as i32);
            }
            Base::Binary => {
                *self *= IcuRatio::two().pow(rhs.power as i32);
            }
        }
    }
}

impl Div for IcuRatio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for IcuRatio {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Add for IcuRatio {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for IcuRatio {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for IcuRatio {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for IcuRatio {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Pow<i32> for IcuRatio {
    type Output = Self;

    fn pow(self, exp: i32) -> Self {
        Self(self.0.pow(exp))
    }
}

impl Zero for IcuRatio {
    fn zero() -> Self {
        Self(Ratio::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for IcuRatio {
    fn one() -> Self {
        Self(Ratio::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl From<Ratio<BigInt>> for IcuRatio {
    fn from(ratio: Ratio<BigInt>) -> Self {
        Self(ratio)
    }
}

impl From<u32> for IcuRatio {
    fn from(value: u32) -> Self {
        Self(Ratio::from_integer(value.into()))
    }
}

impl FromStr for IcuRatio {
    type Err = IcuRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');

        let numerator = match parts.next() {
            Some(num_str) => BigInt::from_str(num_str).map_err(IcuRatioError::BigIntParseError)?,
            None => BigInt::from(0),
        };

        let denominator = match parts.next() {
            Some(den_str) => BigInt::from_str(den_str).map_err(IcuRatioError::BigIntParseError)?,
            None => BigInt::from(1),
        };

        if parts.next().is_some() {
            return Err(IcuRatioError::InvalidRatioString);
        }

        Ok(Self::from_big_ints(numerator, denominator))
    }
}
