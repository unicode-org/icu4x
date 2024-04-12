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
use num_traits::ToPrimitive;
use num_traits::{One, Pow, Zero};

// TODO: implement AsULE for IcuRatio and use it in Data module.
// TODO: add test cases for IcuRatio.
/// A ratio type that uses `BigInt` as the underlying type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IcuRatio(Ratio<BigInt>);

#[derive(Debug)]
pub enum IcuRatioError {
    BigIntParseError(num_bigint::ParseBigIntError),
    InvalidRatioString,
}

impl IcuRatio {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    pub fn numerator(&self) -> &BigInt {
        self.0.numer()
    }

    pub fn numerator_str(&self) -> String {
        self.0.numer().to_string()
    }

    pub fn denominator(&self) -> &BigInt {
        self.0.denom()
    }

    pub fn denominator_str(&self) -> String {
        self.0.denom().to_string()
    }

    /// Returns the reciprocal of the ratio.
    /// For example, the reciprocal of 2/3 is 3/2.
    pub fn recip(&self) -> Self {
        Self(self.0.recip())
    }

    /// Returns the absolute value of the ratio.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn ten() -> Self {
        Self(Ratio::from_integer(10.into()))
    }

    pub fn two() -> Self {
        Self(Ratio::from_integer(2.into()))
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl Mul for IcuRatio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for IcuRatio {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
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
        let num_str = parts.next().unwrap_or("0");
        let den_str = parts.next().unwrap_or("1");
        if parts.next().is_some() {
            return Err(IcuRatioError::InvalidRatioString);
        }

        let numerator = BigInt::from_str(num_str).map_err(IcuRatioError::BigIntParseError)?;
        let denominator = BigInt::from_str(den_str).map_err(IcuRatioError::BigIntParseError)?;
        Ok(Self::new(numerator, denominator))
    }
}
