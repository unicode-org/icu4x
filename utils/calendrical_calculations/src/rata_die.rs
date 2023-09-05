// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use core::ops::{Add, AddAssign, Sub, SubAssign};

/// The *Rata Die*, or *R.D.*, or `fixed_date`: number of days since January 1, 1 CE.
///
/// See: <https://en.wikipedia.org/wiki/Rata_Die>
///
/// It is a logic error to construct a RataDie
/// except from a date that is in range of one of the official calendars.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RataDie(i64);

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
    ///
    /// For testing only
    #[doc(hidden)]
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

/// A moment is a RataDie with a fractional part giving the time of day.
///
/// NOTE: This should not cause overflow errors for most cases, but consider
/// alternative implementations if necessary.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Moment(f64);

/// Add a number of days to a Moment
impl Add<f64> for Moment {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<f64> for Moment {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

/// Subtract a number of days from a Moment
impl Sub<f64> for Moment {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<f64> for Moment {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

/// Calculate the number of days between two moments
impl Sub for Moment {
    type Output = f64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Moment {
    /// Create a new moment
    pub const fn new(value: f64) -> Moment {
        Moment(value)
    }

    /// Get the inner field of a Moment
    pub const fn inner(&self) -> f64 {
        self.0
    }

    /// Get the RataDie of a Moment
    pub fn as_rata_die(&self) -> RataDie {
        RataDie::new(libm::floor(self.0) as i64)
    }
}

#[test]
fn test_moment_to_rata_die_conversion() {
    for i in -1000..=1000 {
        let moment = Moment::new(i as f64);
        let rata_die = moment.as_rata_die();
        assert_eq!(rata_die.to_i64_date(), i);
    }
}
