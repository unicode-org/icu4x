// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::types::Moment;
use core::ops::{Add, AddAssign, Sub, SubAssign};

/// The *Rata Die*, or *R.D.*, or `fixed_date`: number of days since January 1, 1 CE.
///
/// See: <https://en.wikipedia.org/wiki/Rata_Die>
///
/// Keep this class INTERNAL because it shouldn't be possible to construct a RataDie
/// except from a date that is in range of one of the official calendars.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RataDie(i64);

impl RataDie {
    pub const fn new(fixed_date: i64) -> Self {
        let result = Self(fixed_date);
        #[cfg(debug_assertions)]
        result.check();
        result
    }

    #[cfg(debug_assertions)]
    pub const fn check(&self) {
        if self.0 > i64::MAX / 256 {
            debug_assert!(
                false,
                "RataDie is not designed to store values near to the overflow boundary"
            );
        }
        if self.0 < i64::MIN / 256 {
            debug_assert!(
                false,
                "RataDie is not designed to store values near to the overflow boundary"
            );
        }
    }

    /// A valid RataDie that is intended to be below all dates representable in calendars
    #[cfg(test)]
    pub const fn big_negative() -> Self {
        Self::new(i64::MIN / 256 / 256)
    }

    pub const fn to_i64_date(self) -> i64 {
        self.0
    }

    pub const fn to_f64_date(self) -> f64 {
        self.0 as f64
    }

    /// Calculate the number of days between two RataDie in a const-friendly way
    pub const fn const_diff(self, rhs: Self) -> i64 {
        self.0 - rhs.0
    }

    pub const fn as_moment(&self) -> Moment {
        Moment::new(self.0 as f64)
    }
}

/// Shift a RataDie N days into the future
impl Add<i64> for RataDie {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        let result = Self(self.0 + rhs);
        #[cfg(debug_assertions)]
        result.check();
        result
    }
}

impl AddAssign<i64> for RataDie {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
        #[cfg(debug_assertions)]
        self.check();
    }
}

/// Shift a RataDie N days into the past
impl Sub<i64> for RataDie {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        let result = Self(self.0 - rhs);
        #[cfg(debug_assertions)]
        result.check();
        result
    }
}

impl SubAssign<i64> for RataDie {
    fn sub_assign(&mut self, rhs: i64) {
        self.0 -= rhs;
        #[cfg(debug_assertions)]
        self.check();
    }
}

/// Calculate the number of days between two RataDie
impl Sub for RataDie {
    type Output = i64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}
