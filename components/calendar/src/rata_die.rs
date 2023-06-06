// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        Self(fixed_date)
    }
    pub const fn to_fixed_date(self) -> i64 {
        self.0
    }
    /// Calculate the number of days between two RataDie in a const-friendly way
    pub const fn const_diff(self, rhs: Self) -> i64 {
        self.0 - rhs.0
    }
}

/// Shift a RataDie N days into the future
impl Add<i64> for RataDie {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<i64> for RataDie {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
    }
}

/// Shift a RataDie N days into the past
impl Sub<i64> for RataDie {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<i64> for RataDie {
    fn sub_assign(&mut self, rhs: i64) {
        self.0 -= rhs;
    }
}

/// Calculate the number of days between two RataDie
impl Sub for RataDie {
    type Output = i64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}
