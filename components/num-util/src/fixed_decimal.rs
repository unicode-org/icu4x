use smallvec::SmallVec;

use std::cmp;
use std::ops::RangeInclusive;
use std::string::ToString;

use static_assertions::const_assert;

use super::uint_iterator::IntIterator;

// FixedDecimal assumes usize (digits.len()) is at least as big as a u16
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u16>());

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The magnitude or number of digits exceeds the limit of the FixedDecimal. The highest
    /// magnitude of the most significant digit is std::i16::MAX, and the lowest magnitude of the
    /// least significant digit is std::i16::MIN.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    /// use icu_num_util::fixed_decimal::Error;
    ///
    /// let mut dec1 = FixedDecimal::from(123);
    /// assert_eq!(Error::Limit, dec1.adjust_magnitude(std::i16::MAX).unwrap_err());
    /// ```
    Limit,
}

/// A struct containing decimal digits with efficient iteration and manipulation by magnitude
/// (power of 10). Supports a mantissa of non-zero digits and a number of leading and trailing
/// zeros, used for formatting and plural selection.
///
/// You can create a FixedDecimal from a standard integer type. To represent fraction digits,
/// call `.adjust_magnitude()` after creating your FixedDecimal.
///
/// # Examples
///
/// ```
/// use icu_num_util::FixedDecimal;
///
/// let mut dec = FixedDecimal::from(250);
/// assert_eq!("250", dec.to_string());
///
/// dec.adjust_magnitude(-2);
/// assert_eq!("2.50", dec.to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FixedDecimal {
    /// List of digits; digits[0] is the most significant.
    ///
    /// Invariants:
    /// - Must not include leading or trailing zeros
    /// - Length must not exceed (magnitude - lower_magnitude + 1)
    // TODO: Consider using a nibble array
    digits: SmallVec<[u8; 8]>,

    /// Power of 10 of digits[0].
    ///
    /// Invariants:
    /// - <= upper_magnitude
    /// - >= lower_magnitude
    magnitude: i16,

    /// Power of 10 of the most significant digit, which may be zero.
    ///
    /// Invariants:
    /// - >= 0
    /// - >= magnitude
    upper_magnitude: i16,

    /// Power of 10 of the least significant digit, which may be zero.
    ///
    /// Invariants:
    /// - <= 0
    /// - <= magnitude
    lower_magnitude: i16,

    /// Whether the number is negative. Negative zero is supported.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let mut dec: FixedDecimal = Default::default();
    /// dec.is_negative = true;
    /// assert_eq!("-0", dec.to_string());
    /// ```
    pub is_negative: bool,
}

impl Default for FixedDecimal {
    /// Returns a FixedDecimal representing zero.
    fn default() -> Self {
        FixedDecimal {
            digits: SmallVec::new(),
            magnitude: 0,
            upper_magnitude: 0,
            lower_magnitude: 0,
            is_negative: false,
        }
    }
}

macro_rules! impl_from_signed_integer_type {
    ($itype:ident, $utype: ident) => {
        impl From<$itype> for FixedDecimal {
            fn from(value: $itype) -> Self {
                let int_iterator: IntIterator<$utype> = value.into();
                let is_negative = int_iterator.is_negative;
                let mut result = FixedDecimal::from_ascending(int_iterator)
                    .expect("All built-in integer types should fit");
                result.is_negative = is_negative;
                result
            }
        }
    };
}

macro_rules! impl_from_unsigned_integer_type {
    ($utype: ident) => {
        impl From<$utype> for FixedDecimal {
            fn from(value: $utype) -> Self {
                let int_iterator: IntIterator<$utype> = value.into();
                FixedDecimal::from_ascending(int_iterator)
                    .expect("All built-in integer types should fit")
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

impl_from_unsigned_integer_type!(usize);
impl_from_unsigned_integer_type!(u128);
impl_from_unsigned_integer_type!(u64);
impl_from_unsigned_integer_type!(u32);
impl_from_unsigned_integer_type!(u16);
impl_from_unsigned_integer_type!(u8);

impl FixedDecimal {
    /// Initialize a FixedDecimal with an iterator of digits in ascending
    /// order of magnitude, starting with the digit at magnitude 0.
    ///
    /// This method is not public; use TryFrom::<isize> instead.
    fn from_ascending<T>(digits_iter: T) -> Result<Self, Error>
    where
        T: Iterator<Item = u8>,
    {
        // TODO: make X a usize generic to customize the size of this array
        // https://github.com/rust-lang/rust/issues/44580
        // NOTE: 39 is the size required for u128: ceil(log10(u128::MAX)) == 39.
        const X: usize = 39;
        // A temporary structure to allow the digits in the iterator to be reversed.
        // The digits are inserted started from the end, and then a slice is copied
        // into its final destination (result.digits).
        let mut mem: [u8; X] = [0u8; X];
        let mut trailing_zeros: usize = 0;
        let mut i: usize = 0;
        for (x, d) in digits_iter.enumerate() {
            // Take only up to std::i16::MAX values so that we have enough capacity
            if x > std::i16::MAX as usize {
                return Err(Error::Limit);
            }
            // TODO: Should we check here that `d` is between 0 and 9?
            // That should always be the case if UintIterator is used.
            if i != 0 || d != 0 {
                i += 1;
                match X.checked_sub(i) {
                    Some(v) => mem[v] = d,
                    // This error should be obsolete after X is made generic
                    None => return Err(Error::Limit),
                }
            } else {
                trailing_zeros += 1;
            }
        }
        let mut result: Self = Default::default();
        if i != 0 {
            let magnitude = trailing_zeros + i - 1;
            debug_assert_le!(magnitude, std::i16::MAX as usize);
            result.magnitude = magnitude as i16;
            result.upper_magnitude = result.magnitude;
            debug_assert_le!(i, X);
            result.digits.extend_from_slice(&mem[(X - i)..]);
        }
        #[cfg(debug_assertions)]
        result.check_invariants();
        return Ok(result);
    }

    /// Gets the digit at the specified order of magnitude. Returns 0 if the magnitude is out of
    /// range of the currently visible digits.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let dec = FixedDecimal::from(945);
    /// assert_eq!(0, dec.digit_at(-1));
    /// assert_eq!(5, dec.digit_at(0));
    /// assert_eq!(4, dec.digit_at(1));
    /// assert_eq!(9, dec.digit_at(2));
    /// assert_eq!(0, dec.digit_at(3));
    /// ```
    pub fn digit_at(&self, magnitude: i16) -> u8 {
        if magnitude > self.magnitude {
            0 // Leading zero
        } else {
            // The following line can't fail: magnitude <= self.magnitude, by
            // the if statement above, and u16::MAX == i16::MAX - i16::MIN, and
            // usize is asserted to be at least as big as u16.
            let j = (self.magnitude as i32 - magnitude as i32) as usize;
            match self.digits.get(j) {
                Some(v) => *v,
                None => 0, // Trailing zero
            }
        }
    }

    /// Gets the visible range of digit magnitudes, in ascending order of magnitude. Call `.rev()`
    /// on the return value to get the range in descending order. Magnitude 0 is always included,
    /// even if the number has leading or trailing zeros.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(120);
    /// assert_eq!(0..=2, dec.magnitude_range());
    /// ```
    pub fn magnitude_range(&self) -> RangeInclusive<i16> {
        RangeInclusive::new(self.lower_magnitude, self.upper_magnitude)
    }

    /// Shift the digits by a power of 10. Leading or trailing zeros may be added to keep the digit
    /// at magnitude 0 (the last digit before the decimal separator) visible.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(42);
    /// assert_eq!("42", dec.to_string());
    ///
    /// dec.adjust_magnitude(3);
    /// assert_eq!("42000", dec.to_string());
    /// ```
    pub fn adjust_magnitude(&mut self, delta: i16) -> Result<(), Error> {
        if delta > 0 {
            self.upper_magnitude = match self.upper_magnitude.checked_add(delta) {
                Some(v) => v,
                None => return Err(Error::Limit),
            };
            // If we get here, then the magnitude change is in-bounds.
            let lower_magnitude = self.lower_magnitude + delta;
            self.lower_magnitude = cmp::min(0, lower_magnitude);
        } else if delta < 0 {
            self.lower_magnitude = match self.lower_magnitude.checked_add(delta) {
                Some(v) => v,
                None => return Err(Error::Limit),
            };
            // If we get here, then the magnitude change is in-bounds.
            let upper_magnitude = self.upper_magnitude + delta;
            self.upper_magnitude = cmp::max(0, upper_magnitude);
        }
        self.magnitude += delta;
        #[cfg(debug_assertions)]
        self.check_invariants();
        return Ok(());
    }

    /// Assert that the invariants among struct fields are enforced. Returns true if all are okay.
    /// Call this in any method that mutates the struct fields.
    ///
    /// Example: `debug_assert!(self.check_invariants())`
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        // magnitude invariants:
        debug_assert_ge!(self.upper_magnitude, self.magnitude, "{:?}", self);
        debug_assert_le!(self.lower_magnitude, self.magnitude, "{:?}", self);
        debug_assert_ge!(self.upper_magnitude, 0, "{:?}", self);
        debug_assert_le!(self.lower_magnitude, 0, "{:?}", self);

        // digits invariants:
        let max_len = (self.magnitude as i32 - self.lower_magnitude as i32 + 1) as usize;
        debug_assert_le!(self.digits.len(), max_len, "{:?}", self);
        if self.digits.len() > 0 {
            debug_assert_ne!(self.digits[0], 0, "{:?}", self);
            debug_assert_ne!(self.digits[self.digits.len() - 1], 0, "{:?}", self);
        }
    }
}

// TODO(review): This should be fmt::Display, but that is very no_std-unfriendly for code size.
impl ToString for FixedDecimal {
    fn to_string(&self) -> String {
        let max_len = self.magnitude_range().len() + 2;
        let mut result = String::with_capacity(max_len);
        if self.is_negative {
            result.push('-');
        }
        for m in self.magnitude_range().rev() {
            if m == -1 {
                result.push('.');
            }
            let d = self.digit_at(m);
            result.push((b'0' + d) as char);
        }
        debug_assert_ge!(max_len, result.len());
        return result;
    }
}

#[test]
fn test_basic() {
    #[derive(Debug)]
    struct TestCase {
        pub input: isize,
        pub delta: i16,
        pub expected: &'static str,
    };
    let cases = [
        TestCase {
            input: 51423,
            delta: 0,
            expected: "51423",
        },
        TestCase {
            input: 51423,
            delta: -2,
            expected: "514.23",
        },
        TestCase {
            input: 51423,
            delta: -5,
            expected: "0.51423",
        },
        TestCase {
            input: 51423,
            delta: -8,
            expected: "0.00051423",
        },
        TestCase {
            input: 51423,
            delta: 3,
            expected: "51423000",
        },
        TestCase {
            input: 0,
            delta: 0,
            expected: "0",
        },
        TestCase {
            input: 0,
            delta: -2,
            expected: "0.00",
        },
        TestCase {
            input: 0,
            delta: 3,
            expected: "0000",
        },
        TestCase {
            input: 500,
            delta: 0,
            expected: "500",
        },
        TestCase {
            input: 500,
            delta: -1,
            expected: "50.0",
        },
        TestCase {
            input: 500,
            delta: -2,
            expected: "5.00",
        },
        TestCase {
            input: 500,
            delta: -3,
            expected: "0.500",
        },
        TestCase {
            input: 500,
            delta: -4,
            expected: "0.0500",
        },
        TestCase {
            input: 500,
            delta: 3,
            expected: "500000",
        },
        TestCase {
            input: -123,
            delta: 0,
            expected: "-123",
        },
        TestCase {
            input: -123,
            delta: -2,
            expected: "-1.23",
        },
        TestCase {
            input: -123,
            delta: -5,
            expected: "-0.00123",
        },
        TestCase {
            input: -123,
            delta: 3,
            expected: "-123000",
        },
    ];
    for cas in &cases {
        let mut dec: FixedDecimal = cas.input.into();
        dec.adjust_magnitude(cas.delta).unwrap();
        assert_eq!(cas.expected, dec.to_string(), "{:?}", cas);
    }
}

#[test]
fn test_isize_limits() {
    let dec_max: FixedDecimal = std::isize::MAX.into();
    assert_eq!(std::isize::MAX.to_string(), dec_max.to_string());
    let dec_min: FixedDecimal = std::isize::MIN.into();
    assert_eq!(std::isize::MIN.to_string(), dec_min.to_string());
}

#[test]
fn test_ui128_limits() {
    let udec_max: FixedDecimal = std::u128::MAX.into();
    assert_eq!(std::u128::MAX.to_string(), udec_max.to_string());
    let idec_min: FixedDecimal = std::i128::MIN.into();
    assert_eq!(std::i128::MIN.to_string(), idec_min.to_string());
}

#[test]
fn test_upper_magnitude_bounds() {
    let mut dec: FixedDecimal = 98765.into();
    assert_eq!(dec.upper_magnitude, 4);
    dec.adjust_magnitude(32763).unwrap();
    assert_eq!(dec.upper_magnitude, std::i16::MAX);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.adjust_magnitude(1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");
}

#[test]
fn test_lower_magnitude_bounds() {
    let mut dec: FixedDecimal = 98765.into();
    assert_eq!(dec.lower_magnitude, 0);
    dec.adjust_magnitude(-32768).unwrap();
    assert_eq!(dec.lower_magnitude, std::i16::MIN);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.adjust_magnitude(-1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");
}
