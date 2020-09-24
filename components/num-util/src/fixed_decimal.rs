use smallvec::SmallVec;

use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::ops::RangeInclusive;

use std::str::FromStr;
use static_assertions::const_assert;

use super::uint_iterator::IntIterator;
use crate::Error;

// FixedDecimal assumes usize (digits.len()) is at least as big as a u16
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u16>());

/// A struct containing decimal digits with efficient iteration and manipulation by magnitude
/// (power of 10). Supports a mantissa of non-zero digits and a number of leading and trailing
/// zeros, used for formatting and plural selection.
///
/// You can create a FixedDecimal from a standard integer type. To represent fraction digits,
/// call `.multiply_pow10()` after creating your FixedDecimal.
///
/// # Examples
///
/// ```
/// use icu_num_util::FixedDecimal;
///
/// let mut dec = FixedDecimal::from(250);
/// assert_eq!("250", dec.to_string());
///
/// dec.multiply_pow10(-2);
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
            // That should always be the case if IntIterator is used.
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
            debug_assert!(magnitude <= std::i16::MAX as usize);
            result.magnitude = magnitude as i16;
            result.upper_magnitude = result.magnitude;
            debug_assert!(i <= X);
            result.digits.extend_from_slice(&mem[(X - i)..]);
        }
        #[cfg(debug_assertions)]
        result.check_invariants();
        Ok(result)
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
        self.lower_magnitude..=self.upper_magnitude
    }

    /// Shift the digits by a power of 10, modifying self.
    ///
    /// Leading or trailing zeros may be added to keep the digit at magnitude 0 (the last digit
    /// before the decimal separator) visible.
    ///
    /// Can fail if the change in magnitude pushes the digits out of bounds; the magnitudes of all
    /// digits should fit in an i16.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(42);
    /// assert_eq!("42", dec.to_string());
    ///
    /// dec.multiply_pow10(3).expect("Bounds are small");
    /// assert_eq!("42000", dec.to_string());
    /// ```
    pub fn multiply_pow10(&mut self, delta: i16) -> Result<(), Error> {
        match delta.cmp(&0) {
            Ordering::Greater => {
                self.upper_magnitude = self
                    .upper_magnitude
                    .checked_add(delta)
                    .ok_or(Error::Limit)?;
                // If we get here, then the magnitude change is in-bounds.
                let lower_magnitude = self.lower_magnitude + delta;
                self.lower_magnitude = cmp::min(0, lower_magnitude);
            }
            Ordering::Less => {
                self.lower_magnitude = self
                    .lower_magnitude
                    .checked_add(delta)
                    .ok_or(Error::Limit)?;
                // If we get here, then the magnitude change is in-bounds.
                let upper_magnitude = self.upper_magnitude + delta;
                self.upper_magnitude = cmp::max(0, upper_magnitude);
            }
            Ordering::Equal => {}
        };
        self.magnitude += delta;
        #[cfg(debug_assertions)]
        self.check_invariants();
        Ok(())
    }

    /// Shift the digits by a power of 10, consuming self and returning a new object if successful.
    ///
    /// Leading or trailing zeros may be added to keep the digit at magnitude 0 (the last digit
    /// before the decimal separator) visible.
    ///
    /// Can fail if the change in magnitude pushes the digits out of bounds; the magnitudes of all
    /// digits should fit in an i16.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let dec = FixedDecimal::from(42).multiplied_pow10(3).expect("Bounds are small");
    /// assert_eq!("42000", dec.to_string());
    /// ```
    pub fn multiplied_pow10(mut self, delta: i16) -> Result<Self, Error> {
        match self.multiply_pow10(delta) {
            Ok(()) => Ok(self),
            Err(err) => Err(err),
        }
    }

    /// Render the FixedDecimal as a string of ASCII digits with a possible decimal point.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let dec = FixedDecimal::from(42);
    /// let mut result = String::with_capacity(dec.write_len());
    /// dec.write_to(&mut result).expect("write_to(String) should not fail");
    /// assert_eq!("42", result);
    /// ```
    pub fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result {
        if self.is_negative {
            sink.write_char('-')?;
        }
        for m in self.magnitude_range().rev() {
            if m == -1 {
                sink.write_char('.')?;
            }
            let d = self.digit_at(m);
            sink.write_char((b'0' + d) as char)?;
        }
        Ok(())
    }

    /// The number of bytes that will be written by FixedDecimal::write_to. Use this function to
    /// pre-allocate capacity in the destination buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_num_util::FixedDecimal;
    ///
    /// let dec = FixedDecimal::from(-5000).multiplied_pow10(-2).expect("Bounds are small");
    /// let mut result = String::with_capacity(dec.write_len());
    /// dec.write_to(&mut result).expect("write_to(String) should not fail");
    /// assert_eq!("-50.00", result);
    /// assert_eq!(6, dec.write_len());
    /// ```
    pub fn write_len(&self) -> usize {
        let num_digits = 1 + (self.upper_magnitude as i32 - self.lower_magnitude as i32) as usize;
        num_digits
            + (if self.is_negative { 1 } else { 0 })
            + (if self.lower_magnitude < 0 { 1 } else { 0 })
    }

    /// Assert that the invariants among struct fields are enforced. Returns true if all are okay.
    /// Call this in any method that mutates the struct fields.
    ///
    /// Example: `debug_assert!(self.check_invariants())`
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        // magnitude invariants:
        debug_assert!(self.upper_magnitude >= self.magnitude, "{:?}", self);
        debug_assert!(self.lower_magnitude <= self.magnitude, "{:?}", self);
        debug_assert!(self.upper_magnitude >= 0, "{:?}", self);
        debug_assert!(self.lower_magnitude <= 0, "{:?}", self);

        // digits invariants:
        let max_len = (self.magnitude as i32 - self.lower_magnitude as i32 + 1) as usize;
        debug_assert!(self.digits.len() <= max_len, "{:?}", self);
        if !self.digits.is_empty() {
            debug_assert_ne!(self.digits[0], 0, "{:?}", self);
            debug_assert_ne!(self.digits[self.digits.len() - 1], 0, "{:?}", self);
        }
    }
}

/// Renders the FixedDecimal according to the syntax documented in FixedDecimal::write_to.
impl fmt::Display for FixedDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_to(f)
    }
}

impl FromStr for FixedDecimal {
    type Err = Error;
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        // input_str: the input string
        // no_sign_str: the input string when the sign is removed from it
        let no_sign_str:&str; 
        let mut is_negative = false;
        if &input_str[0..1] == "-" {
            is_negative = true;
            no_sign_str = &input_str[1..];
        } else {
            no_sign_str = input_str;
        }
        // Compute length of each string once and store it, so if you use that multiple times,
        //  you don't compute it multiple times
        // has_dot: shows if your input has dot in it
        // dot_index: gives the index of dot (after removing the sign)
        let no_sign_str_len = no_sign_str.chars().count();
        let mut has_dot = false;
        let mut dot_index = no_sign_str_len;
        // The following loop computes has_dot, dot_index, and also checks to see if all
        // characters are digits and if you have at most one dot
        // Note: Input of format 111_123 is detected as syntax error here
        // Note: Input starting or ending with a dot is detected as syntax error here (Ex: .123, 123.)
        for (i, c) in no_sign_str.chars().enumerate() {
            if c == '.' {
                match has_dot{
                    false => {
                        dot_index = i;    
                        has_dot = true;
                        if i == 0 || i == no_sign_str.len()-1{
                            return Err(Error::Syntax);
                        }
                    },
                    true => {
                        return Err(Error::Syntax);
                    },
                }
            } else {
                if !c.is_digit(10) {
                    return Err(Error::Syntax)
                }
            }
        }
        // no_dot_str: shows the string when the dot is removed from it
        let no_dot_str = no_sign_str.replace(".", "");
        let no_dot_str_len = no_dot_str.chars().count();

        // left_zeros: number of zeros at the left of no_dot_str
        // right_zeros: number of zeros at the right of no_dot_str
        // These are not necessarily leading or trailing zeros
        // Example:
        //     input string     left_zeros      right_zeros        
        //     00123000             2                3
        //     0.0123000            2               3
        //     001.23000            2               3
        //     001230.00            2               3
        let mut left_zeros = 0;
        for (_i, c) in no_dot_str.chars().enumerate() {
            if c == '0' {
                left_zeros += 1;
            } else {
                break;
            }
        }
        let mut right_zeros = 0;
        for (_i, c) in no_dot_str.chars().rev().enumerate() {
            if c == '0' {
                right_zeros += 1;
            } else {
                break;
            }
        }
        // digits_str: the string after removing sign, dot, left zeros, and right zeros
        let digits_str = &no_dot_str[left_zeros .. no_dot_str_len - right_zeros];

        // shift: number of digits after dot in the input_str, 0 if there is no dot
        let mut shift:i16 = 0;
        if has_dot {
            shift = (no_sign_str_len - dot_index - 1) as i16;
        }

        // Constructing DecimalFixed only using digits_str
        let mut dec: FixedDecimal = 0.into();
        let mut v = SmallVec::<[u8; 8]>::new();
        for (i, c) in digits_str.chars().enumerate() {
            if i > std::i16::MAX as usize {
                return Err(Error::Limit);
            }
            v.push(c as u8 - '0' as u8);
        }
        let v_len = v.len();
        dec.digits = v;
        dec.magnitude = v_len as i16 -1;
        dec.upper_magnitude = v_len as i16 - 1;
        dec.lower_magnitude = 0;
        
        // Adjusting magnitude, upper_magnitude, and lower_magnitud w.r.t. left_zeros, right_zeros, and shift
        // The computed upper_magnitude value computed below is always >= 0
        // The computed lower_magnitude value computed below is always <= 0
        // The order of negative terms and positive terms is intentional, to avoid overflow if possible
        dec.upper_magnitude += -shift + left_zeros as i16 + right_zeros as i16;
        dec.magnitude += -shift + right_zeros as i16;
        dec.lower_magnitude = -1*shift;
        dec.is_negative = is_negative;

        // Outputs for debugging:
        // println!("digits_str = {}", digits_str);
        // println!("dot_index = {}", dot_index);
        // println!("right zeros = {}", right_zeros);
        // println!("left zeros = {}", left_zeros);
        // println!("shift = {}", shift);
        // println!("final_dec = {}", dec);
        // println!("final_dec = {:?}", dec);
        
        Ok(dec)
        
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
        dec.multiply_pow10(cas.delta).unwrap();
        let string = dec.to_string();
        assert_eq!(cas.expected, string, "{:?}", cas);
        assert_eq!(string.len(), dec.write_len(), "{:?}", cas);
    }
}

#[test]
fn test_from_str() {
    #[derive(Debug)]
    struct TestCase {
        pub input: &'static str,
    };
    let cases = [
        TestCase {
            input: "-00123400",
        },
        TestCase {
            input: "0.0123400",
        },
        TestCase {
            input: "-00.123400",
        },
        TestCase {
            input: "0012.3400",
        },
        TestCase {
            input: "-0012340.0",
        },
        TestCase {
            input: "1234",
        },
        TestCase {
            input: "0.000000001",
        },
        TestCase {
            input: "0.0000000010",
        },
        TestCase {
            input: "1000000",
        },
        TestCase {
            input: "10000001",
        },
        TestCase {
            input: "123",
        },
        TestCase {
            input: "922337203685477580898230948203840239384.9823094820384023938423424",
        },
        TestCase {
            input: "009223372000.003685477580898230948203840239384000",
        },
        TestCase {
            input: "009223372000.003685477580898230948203840239384000",
        },
    ];
    for cas in &cases {
        let dec: FixedDecimal = FixedDecimal::from_str(cas.input).unwrap();
        assert_eq!(dec.to_string(), cas.input);
        assert_eq!(dec.write_len(), cas.input.len());
    }
}

#[test]
pub fn test_isize_limits() {
    let dec_max: FixedDecimal = std::isize::MAX.into();
    let input = dec_max.to_string();
    let dec: FixedDecimal = FixedDecimal::from_str(&input).unwrap();
    assert_eq!(dec.to_string(), input);
    assert_eq!(dec.write_len(), input.len());
    let dec_min: FixedDecimal = std::isize::MIN.into();
    let input = dec_min.to_string();
    let dec: FixedDecimal = FixedDecimal::from_str(&input).unwrap();
    assert_eq!(dec.to_string(), input);
    assert_eq!(dec.write_len(), input.len());
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
    dec.multiply_pow10(32763).unwrap();
    assert_eq!(dec.upper_magnitude, std::i16::MAX);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.multiply_pow10(1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");
}

#[test]
fn test_lower_magnitude_bounds() {
    let mut dec: FixedDecimal = 98765.into();
    assert_eq!(dec.lower_magnitude, 0);
    dec.multiply_pow10(-32768).unwrap();
    assert_eq!(dec.lower_magnitude, std::i16::MIN);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.multiply_pow10(-1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");
}
