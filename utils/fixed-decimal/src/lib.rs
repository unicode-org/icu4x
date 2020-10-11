// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `fixed-decimal` is a utility crate of the [`ICU4X`] project.
//!
//! It includes [`FixedDecimal`], a core API for representing numbers in a human-readable form
//! appropriate for formatting and plural rule selection. It is optimized for operations involving
//! the individual digits of a number.
//!
//! # Example
//!
//! ```
//! use fixed_decimal::FixedDecimal;
//!
//! let dec = FixedDecimal::from(250)
//!     .multiplied_pow10(-2)
//!     .expect("Bounds are small");
//! assert_eq!("2.50", format!("{}", dec));
//!
//! #[derive(Debug, PartialEq)]
//! struct MagnitudeAndDigit(i16, u8);
//!
//! let digits: Vec<MagnitudeAndDigit> = dec
//!     .magnitude_range()
//!     .map(|m| MagnitudeAndDigit(m, dec.digit_at(m)))
//!     .collect();
//!
//! assert_eq!(
//!     vec![
//!         MagnitudeAndDigit(-2, 0),
//!         MagnitudeAndDigit(-1, 5),
//!         MagnitudeAndDigit(0, 2)
//!     ],
//!     digits
//! );
//! ```
//!
//! [`FixedDecimal`]: ./struct.FixedDecimal.html
//! [`ICU4X`]: https://github.com/unicode-org/icu4x

pub mod decimal;
mod uint_iterator;

pub use decimal::FixedDecimal;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The magnitude or number of digits exceeds the limit of the FixedDecimal. The highest
    /// magnitude of the most significant digit is std::i16::MAX, and the lowest magnitude of the
    /// least significant digit is std::i16::MIN.
    ///
    /// # Example
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// use fixed_decimal::Error;
    ///
    /// let mut dec1 = FixedDecimal::from(123);
    /// assert_eq!(Error::Limit, dec1.multiply_pow10(std::i16::MAX).unwrap_err());
    /// ```
    Limit,
    /// The input of a string that is supposed to be converted to FixedDecimal is not accepted.
    ///
    /// Any string with non-digit characters (except for one '.' and one '-' at the beginning of the string) is not accepted.
    /// Also, empty string ("") and its negation ("-") are not accepted.
    /// Strings of form "12_345_678" are not accepted, the accepted format is "12345678".
    /// Also '.' shouldn't be first or the last characters, i. e. .123 and 123. are not accepted, and instead 0.123 and
    /// 123 (or 123.0) must be used.
    Syntax,
}
