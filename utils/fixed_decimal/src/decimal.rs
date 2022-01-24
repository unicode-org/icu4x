// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use smallvec::SmallVec;

use core::cmp;
use core::cmp::Ordering;
use core::fmt;
use core::ops::RangeInclusive;

use core::str::FromStr;

use static_assertions::const_assert;

use crate::signum::Signum;
use crate::uint_iterator::IntIterator;

use crate::Error;

// FixedDecimal assumes usize (digits.len()) is at least as big as a u16
const_assert!(core::mem::size_of::<usize>() >= core::mem::size_of::<u16>());

/// A struct containing decimal digits with efficient iteration and manipulation by magnitude
/// (power of 10). Supports a mantissa of non-zero digits and a number of leading and trailing
/// zeros, used for formatting and plural selection.
///
/// # Data Types
///
/// The following types can be converted to a `FixedDecimal`:
///
/// - Integers, signed and unsigned
/// - Strings representing an arbitrary-precision decimal
///
/// To create a [`FixedDecimal`] with fraction digits, either create it from an integer and then
/// call [`FixedDecimal::multiplied_pow10`], or create it from a string.
///
/// Floating point numbers will be supported pending a resolution to
/// [#166](https://github.com/unicode-org/icu4x/issues/166). In the mean time, a third-party
/// float-to-string library may be used.
///
/// # Examples
///
/// ```
/// use fixed_decimal::FixedDecimal;
///
/// let mut dec = FixedDecimal::from(250);
/// assert_eq!("250", dec.to_string());
///
/// dec.multiply_pow10(-2);
/// assert_eq!("2.50", dec.to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FixedDecimal {
    /// List of digits; digits\[0\] is the most significant.
    ///
    /// Invariants:
    /// - Must not include leading or trailing zeros
    /// - Length must not exceed (magnitude - lower_magnitude + 1)
    // TODO: Consider using a nibble array
    digits: SmallVec<[u8; 8]>,

    /// Power of 10 of digits\[0\].
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
    is_negative: bool,
}

impl Default for FixedDecimal {
    /// Returns a `FixedDecimal` representing zero.
    fn default() -> Self {
        Self {
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
                let mut result = Self::from_ascending(int_iterator)
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
                Self::from_ascending(int_iterator).expect("All built-in integer types should fit")
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
    /// Initialize a `FixedDecimal` with an iterator of digits in ascending
    /// order of magnitude, starting with the digit at magnitude 0.
    ///
    /// This method is not public; use `TryFrom::<isize>` instead.
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
            // Take only up to core::i16::MAX values so that we have enough capacity
            if x > core::i16::MAX as usize {
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
            debug_assert!(magnitude <= core::i16::MAX as usize);
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
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
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
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(120);
    /// assert_eq!(0..=2, dec.magnitude_range());
    /// ```
    pub const fn magnitude_range(&self) -> RangeInclusive<i16> {
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
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
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
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
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

    /// Change the value from negative to positive or from positive to negative, modifying self.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(42);
    /// assert_eq!("42", dec.to_string());
    ///
    /// dec.negate();
    /// assert_eq!("-42", dec.to_string());
    ///
    /// dec.negate();
    /// assert_eq!("42", dec.to_string());
    /// ```
    pub fn negate(&mut self) {
        self.is_negative = !self.is_negative;
    }

    /// Change the value from negative to positive or from positive to negative, consuming self
    /// and returning a new object.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// assert_eq!(FixedDecimal::from(-42), FixedDecimal::from(42).negated());
    /// ```
    pub fn negated(mut self) -> Self {
        self.negate();
        self
    }

    /// Zero-pad the number on the left to a particular number of integer digits,
    /// returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(42);
    /// assert_eq!("42", dec.to_string());
    ///
    /// ;
    /// assert_eq!("0042", dec.clone().padded_left(4).to_string());
    ///
    /// assert_eq!("042", dec.clone().padded_left(3).to_string());
    ///
    /// assert_eq!("42", dec.clone().padded_left(2).to_string());
    ///
    /// assert_eq!("42", dec.clone().padded_left(1).to_string());
    /// ```
    pub fn padded_left(mut self, digits: u16) -> Self {
        self.pad_left(digits);
        self
    }

    /// Zero-pad the number on the left to a particular number of integer digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(42);
    /// assert_eq!("42", dec.to_string());
    ///
    /// dec.pad_left(4);
    /// assert_eq!("0042", dec.to_string());
    ///
    /// dec.pad_left(3);
    /// assert_eq!("042", dec.to_string());
    ///
    /// dec.pad_left(2);
    /// assert_eq!("42", dec.to_string());
    ///
    /// dec.pad_left(1);
    /// assert_eq!("42", dec.to_string());
    /// ```
    pub fn pad_left(&mut self, digits: u16) {
        let mut magnitude = if digits == 0 {
            0
        } else if digits > (i16::MAX as u16) + 1 {
            i16::MAX
        } else {
            (digits - 1) as i16
        };
        if magnitude < 0 {
            magnitude = 0;
        }
        // Do not truncate nonzero digits
        if magnitude <= self.magnitude {
            magnitude = self.magnitude;
        }

        self.upper_magnitude = magnitude;
    }

    /// Truncate the number on the left to a particular magnitude, deleting
    /// digits if necessary, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(4235);
    /// assert_eq!("4235", dec.to_string());
    ///
    /// assert_eq!("4235", dec.clone().truncated_left(5).to_string());
    ///
    /// assert_eq!("235", dec.clone().truncated_left(2).to_string());
    ///
    /// assert_eq!("35", dec.clone().truncated_left(1).to_string());
    ///
    /// assert_eq!("5", dec.clone().truncated_left(0).to_string());
    ///
    /// assert_eq!("0", dec.clone().truncated_left(-1).to_string());
    /// ```
    pub fn truncated_left(mut self, magnitude: i16) -> Self {
        self.truncate_left(magnitude);
        self
    }

    /// Truncate the number on the left to a particular magnitude, deleting
    /// digits if necessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    ///
    /// let mut dec = FixedDecimal::from(4235);
    /// assert_eq!("4235", dec.to_string());
    ///
    /// dec.truncate_left(5);
    /// assert_eq!("4235", dec.to_string());
    ///
    /// dec.truncate_left(2);
    /// assert_eq!("235", dec.to_string());
    ///
    /// dec.truncate_left(1);
    /// assert_eq!("35", dec.to_string());
    ///
    /// dec.truncate_left(0);
    /// assert_eq!("5", dec.to_string());
    ///
    /// dec.truncate_left(-1);
    /// assert_eq!("0", dec.to_string());
    /// ```
    pub fn truncate_left(&mut self, magnitude: i16) {
        if self.magnitude >= magnitude {
            let positive_magnitude = if magnitude > 0 { magnitude } else { 0 };
            let cut = ((self.magnitude as i32) - (magnitude as i32)) as usize;
            if cut >= self.digits.len() {
                self.digits.clear();
                self.magnitude = 0;
                self.upper_magnitude = positive_magnitude;
                #[cfg(debug_assertions)]
                self.check_invariants();
                return;
            }
            let _ = self.digits.drain(0..cut as usize).count();
            // Count number of leading zeroes
            let extra_zeroes = self.digits.iter().position(|x| *x != 0).unwrap_or(0);
            let _ = self.digits.drain(0..extra_zeroes).count();
            self.magnitude = magnitude - extra_zeroes as i16;
            self.upper_magnitude = positive_magnitude;
        }
        #[cfg(debug_assertions)]
        self.check_invariants();
    }

    /// Zero-pad the number on the right to a particular (negative) magnitude. Will truncate
    /// trailing zeros if necessary, but will not truncate other digits, returning the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = FixedDecimal::from_str("123.456").unwrap();
    /// assert_eq!("123.456", dec.to_string());
    ///
    /// assert_eq!("123.456", dec.clone().padded_right(1).to_string());
    ///
    /// assert_eq!("123.456", dec.clone().padded_right(2).to_string());
    ///
    /// assert_eq!("123.4560", dec.clone().padded_right(4).to_string());
    ///
    /// assert_eq!("123.456000", dec.clone().padded_right(6).to_string());
    /// ```
    pub fn padded_right(mut self, negative_magnitude: u16) -> Self {
        self.pad_right(negative_magnitude);
        self
    }

    /// Zero-pad the number on the right to a particular (negative) magnitude. Will truncate
    /// trailing zeros if necessary, but will not truncate other digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// # use std::str::FromStr;
    ///
    /// let mut dec = FixedDecimal::from_str("123.456").unwrap();
    /// assert_eq!("123.456", dec.to_string());
    ///
    /// dec.pad_right(1);
    /// assert_eq!("123.456", dec.to_string());
    ///
    /// dec.pad_right(2);
    /// assert_eq!("123.456", dec.to_string());
    ///
    /// dec.pad_right(4);
    /// assert_eq!("123.4560", dec.to_string());
    ///
    /// dec.pad_right(6);
    /// assert_eq!("123.456000", dec.to_string());
    /// ```
    pub fn pad_right(&mut self, negative_magnitude: u16) {
        let mut magnitude = if negative_magnitude > (i16::MAX as u16) {
            i16::MIN
        } else {
            -(negative_magnitude as i16)
        };
        let bottom_magnitude = (self.magnitude as i32 - self.digits.len() as i32 + 1) as i16;
        // Do not truncate nonzero digits
        if magnitude >= bottom_magnitude {
            magnitude = bottom_magnitude;
        }

        self.lower_magnitude = magnitude;
        #[cfg(debug_assertions)]
        self.check_invariants();
    }

    /// Returns the [Signum][Signum] of this FixedDecimal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// use fixed_decimal::Signum;
    ///
    /// assert_eq!(Signum::AboveZero, FixedDecimal::from(42).signum());
    /// assert_eq!(Signum::PositiveZero, FixedDecimal::from(0).signum());
    /// assert_eq!(Signum::NegativeZero, FixedDecimal::from(0).negated().signum());
    /// assert_eq!(Signum::BelowZero, FixedDecimal::from(-42).signum());
    /// ```
    pub fn signum(&self) -> Signum {
        let is_zero = self.digits.is_empty();
        match (self.is_negative, is_zero) {
            (false, false) => Signum::AboveZero,
            (false, true) => Signum::PositiveZero,
            (true, false) => Signum::BelowZero,
            (true, true) => Signum::NegativeZero,
        }
    }

    /// Assert that the invariants among struct fields are enforced. Returns true if all are okay.
    /// Call this in any method that mutates the struct fields.
    ///
    /// Example: `debug_assert!(self.check_invariants())`
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        // magnitude invariants:
        debug_assert!(
            self.upper_magnitude >= self.magnitude,
            "Upper magnitude too small {:?}",
            self
        );
        debug_assert!(
            self.lower_magnitude <= self.magnitude,
            "Lower magnitude too large {:?}",
            self
        );
        debug_assert!(
            self.upper_magnitude >= 0,
            "Upper magnitude below zero {:?}",
            self
        );
        debug_assert!(
            self.lower_magnitude <= 0,
            "Lower magnitude above zero {:?}",
            self
        );

        // digits invariants:
        let max_len = (self.magnitude as i32 - self.lower_magnitude as i32 + 1) as usize;
        debug_assert!(self.digits.len() <= max_len, "{:?}", self);
        if !self.digits.is_empty() {
            debug_assert_ne!(self.digits[0], 0, "Starts with a zero {:?}", self);
            debug_assert_ne!(
                self.digits[self.digits.len() - 1],
                0,
                "Ends with a zero {:?}",
                self
            );
        }
    }
}

impl writeable::Writeable for FixedDecimal {
    /// Render the `FixedDecimal` as a string of ASCII digits with a possible decimal point.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// use writeable::Writeable;
    ///
    /// let dec = FixedDecimal::from(42);
    /// let mut result = String::with_capacity(dec.write_len().capacity());
    /// dec.write_to(&mut result).expect("write_to(String) should not fail");
    /// assert_eq!("42", result);
    /// ```
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
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

    /// The number of bytes that will be written by `FixedDecimal::write_to`. Use this function to
    /// pre-allocate capacity in the destination buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// use writeable::Writeable;
    /// use writeable::LengthHint;
    ///
    /// let dec = FixedDecimal::from(-5000).multiplied_pow10(-2).expect("Bounds are small");
    /// let result = dec.writeable_to_string();
    /// assert_eq!(LengthHint::exact(6), dec.write_len());
    /// ```
    fn write_len(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(1)
            + ((self.upper_magnitude as i32 - self.lower_magnitude as i32) as usize)
            + (if self.is_negative { 1 } else { 0 })
            + (if self.lower_magnitude < 0 { 1 } else { 0 })
    }
}

/// Renders the `FixedDecimal` according to the syntax documented in `FixedDecimal::write_to`.
impl fmt::Display for FixedDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl FromStr for FixedDecimal {
    type Err = Error;
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        // input_str: the input string
        // no_sign_str: the input string when the sign is removed from it
        // Check if the input string is "" or "-"
        if input_str.is_empty() || input_str == "-" {
            return Err(Error::Syntax);
        }
        let input_str = input_str.as_bytes();
        let is_negative = input_str[0] == b'-';
        let no_sign_str = if is_negative {
            &input_str[1..]
        } else {
            input_str
        };
        // Compute length of each string once and store it, so if you use that multiple times,
        // you don't compute it multiple times
        // has_dot: shows if your input has dot in it
        // has_exponent: shows if your input has an exponent in it
        // dot_index: gives the index of dot (after removing the sign) -- equal to length of
        // the no_sign_str if there is no dot
        // exponent_index: gives the index of exponent (after removing the sign) -- equal to length of
        // the no_sign_str if there is no dot
        let mut has_dot = false;
        let mut has_exponent = false;
        let mut dot_index = no_sign_str.len();
        let mut exponent_index = no_sign_str.len();
        // The following loop computes has_dot, dot_index, and also checks to see if all
        // characters are digits and if you have at most one dot
        // Note: Input of format 111_123 is detected as syntax error here
        // Note: Input starting or ending with a dot is detected as syntax error here (Ex: .123, 123.)
        for (i, c) in no_sign_str.iter().enumerate() {
            if *c == b'.' {
                if has_dot || has_exponent {
                    // multiple dots or dots after the exponent
                    return Err(Error::Syntax);
                }
                dot_index = i;
                has_dot = true;
                if i == 0 || i == no_sign_str.len() - 1 {
                    return Err(Error::Syntax);
                }
            } else if *c == b'e' || *c == b'E' {
                if has_exponent {
                    // multiple exponents
                    return Err(Error::Syntax);
                }
                exponent_index = i;
                has_exponent = true;
                if i == 0 || i == no_sign_str.len() - 1 {
                    return Err(Error::Syntax);
                }
            } else if *c == b'-' {
                // Allow a single minus sign after the exponent
                if has_exponent && exponent_index == i - 1 {
                    continue;
                } else {
                    return Err(Error::Syntax);
                }
            } else if *c < b'0' || *c > b'9' {
                return Err(Error::Syntax);
            }
        }

        // The string without the exponent (or sign)
        // We do the bulk of the calculation on this string,
        // and extract the exponent at the end
        let no_exponent_str = &no_sign_str[..exponent_index];

        // If there was no dot, truncate the dot index
        if dot_index > exponent_index {
            dot_index = exponent_index;
        }

        // defining the output dec here and set its sign
        let mut dec = Self {
            is_negative,
            ..Default::default()
        };

        // no_dot_str_len: shows length of the string after removing the dot
        let mut no_dot_str_len = no_exponent_str.len();
        if has_dot {
            no_dot_str_len -= 1;
        }

        // Computing DecimalFixed.upper_magnitude
        let temp_upper_magnitude = dot_index - 1;
        if temp_upper_magnitude > i16::MAX as usize {
            return Err(Error::Limit);
        }
        dec.upper_magnitude = temp_upper_magnitude as i16;

        // Computing DecimalFixed.lower_magnitude
        // Note: ((i16::MIN as u16) as usize) == 32768
        let temp_lower_magnitude = no_dot_str_len - dot_index;
        if temp_lower_magnitude > (i16::MIN as u16) as usize {
            return Err(Error::Limit);
        }
        dec.lower_magnitude = (temp_lower_magnitude as i16).wrapping_neg();

        // leftmost_digit: index of the first non-zero digit
        // rightmost_digit: index of the first element after the last non-zero digit
        // Example:
        //     input string    leftmost_digit     rightmost_digit
        //     00123000              2                  5
        //     0.0123000             3                  6
        //     001.23000             2                  6
        //     001230.00             2                  5
        // Compute leftmost_digit
        let leftmost_digit = no_exponent_str
            .iter()
            .position(|c| *c != b'.' && *c != b'0');

        // If the input only has zeros (like 000, 00.0, -00.000) we handle the situation here
        // by returning the dec and don't running the rest of the code
        let leftmost_digit = if let Some(leftmost_digit) = leftmost_digit {
            leftmost_digit
        } else {
            return Ok(dec);
        };

        // Else if the input is not all zeros, we compute its magnitude:
        // Note that we can cast with "as" here because lower and upper magnitude have been checked already
        let mut temp_magnitude = ((dot_index as i32) - (leftmost_digit as i32) - 1i32) as i16;
        if dot_index < leftmost_digit {
            temp_magnitude += 1;
        }
        dec.magnitude = temp_magnitude;

        // Compute the index where the rightmost_digit ends
        let rightmost_digit_end = no_exponent_str
            .iter()
            .rposition(|c| *c != b'.' && *c != b'0')
            .map(|p| p + 1)
            .unwrap_or(no_exponent_str.len());

        // digits_str_len: shows the length of digits (Ex. 0012.8900 --> 4)
        let mut digits_str_len = rightmost_digit_end - leftmost_digit;
        if leftmost_digit < dot_index && dot_index < rightmost_digit_end {
            digits_str_len -= 1;
        }

        // Constructing DecimalFixed.digits
        let v: SmallVec<[u8; 8]> = no_exponent_str[leftmost_digit..rightmost_digit_end]
            .iter()
            .filter(|c| **c != b'.')
            .map(|c| c - b'0')
            .collect();

        let v_len = v.len();
        debug_assert_eq!(v_len, digits_str_len);
        dec.digits = v;

        // Extract the exponent part
        if has_exponent {
            let mut pow = 0;
            let mut pos_neg = 1;
            for digit in &no_sign_str[exponent_index + 1..] {
                if *digit == b'-' {
                    pos_neg = -1;
                    continue;
                }
                pow *= 10;
                pow += (digit - b'0') as i16;
            }

            dec.multiply_pow10(pos_neg * pow)?;

            // Clean up magnitude after multiplication
            if dec.magnitude > 0 {
                dec.upper_magnitude = dec.magnitude;
            }
            let neg_mag = dec.magnitude - dec.digits.len() as i16 + 1;
            if neg_mag < 0 {
                dec.lower_magnitude = neg_mag;
            }
        }

        Ok(dec)
    }
}

/// Specifies the precision of a floating point value when constructing a FixedDecimal.
///
/// IEEE 754 is a representation of a point on the number line. On the other hand, FixedDecimal
/// specifies not only the point on the number line but also the precision of the number to a
/// specific power of 10. This enum augments a floating-point value with the additional
/// information required by FixedDecimal.
#[cfg(feature = "ryu")]
#[derive(Debug, Clone, Copy)]
pub enum DoublePrecision {
    /// Specify that the floating point number is integer-valued.
    ///
    /// If the floating point is not actually integer-valued, an error will be returned.
    Integer,

    /// Specify that the floating point number is precise to a specific power of 10.
    /// The number may be rounded or trailing zeros may be added as necessary.
    Magnitude(i16, RoundingMode),

    /// Specify that the floating point number is precise to a specific number of significant digits.
    /// The number may be rounded or trailing zeros may be added as necessary.
    ///
    /// The number requested may not be zero
    SignificantDigits(u8, RoundingMode),

    /// Specify that the floating point number is precise to the maximum representable by IEEE.
    ///
    /// This results in a FixedDecimal having enough digits to recover the original floating point
    /// value, with no trailing zeros.
    Maximum,
}

/// Specifies how numbers should be rounded
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RoundingMode {
    /// Return an error if the number has trailing nonzero digits that need to be rounded.
    Unnecessary,

    /// Round toward zero (remove, or truncate, all trailing digits).
    Truncate,

    /// Round ties away from zero.
    HalfExpand,
    // TODO(#1177): Add more rounding modes.
}

#[cfg(feature = "ryu")]
impl FixedDecimal {
    /// Construct a [`FixedDecimal`] from an f64. This uses `ryu` and
    /// goes through an intermediate string representation, so is not
    /// fully efficient. See [icu4x#166](https://github.com/unicode-org/icu4x/issues/166) for
    /// more details.
    ///
    /// This function can be made available with the `"ryu"` feature.
    ///
    /// ```rust
    /// use fixed_decimal::{DoublePrecision, FixedDecimal};
    /// use writeable::Writeable;
    ///
    /// let decimal = FixedDecimal::new_from_f64(0.012345678, DoublePrecision::Maximum).unwrap();
    /// assert_eq!(decimal.writeable_to_string(), "0.012345678");
    ///
    /// let decimal = FixedDecimal::new_from_f64(-123456.78, DoublePrecision::Maximum).unwrap();
    /// assert_eq!(decimal.writeable_to_string(), "-123456.78");
    ///
    /// let decimal = FixedDecimal::new_from_f64(12345678000., DoublePrecision::Maximum).unwrap();
    /// assert_eq!(decimal.writeable_to_string(), "12345678000");
    /// ```
    pub fn new_from_f64(float: f64, precision: DoublePrecision) -> Result<Self, Error> {
        let mut decimal = Self::new_from_f64_raw(float)?;
        let n_digits = decimal.digits.len();
        // magnitude of the lowest digit in self.digits
        let lowest_magnitude = decimal.magnitude - n_digits as i16 + 1;
        // ryū will usually tack on a `.0` to integers which gets included when parsing.
        // Explicitly remove it before doing anything else
        if lowest_magnitude >= 0 && decimal.lower_magnitude < 0 {
            decimal.lower_magnitude = 0;
        }
        match precision {
            DoublePrecision::Maximum => (),
            DoublePrecision::Integer => {
                if lowest_magnitude < 0 {
                    return Err(Error::Limit);
                }
            }
            DoublePrecision::Magnitude(mag, mode) => {
                if mag > lowest_magnitude {
                    let round_by = (mag - lowest_magnitude) as u16;

                    if round_by as usize <= n_digits {
                        decimal.round_trailing_digits(round_by, mode)?;
                    } else {
                        // If we need to round by more digits than rounding can ever produce
                        // the number is zero
                        decimal = Default::default();
                    }
                }
                if mag < 0 {
                    // If the target magnitude was negative, make
                    // sure we update the lower magnitude to match it
                    decimal.lower_magnitude = mag;
                } else if decimal.lower_magnitude < 0 {
                    // If the target magnitude was positive,
                    // we may have truncated digits off the right
                    // side, reset the lower magnitude
                    decimal.lower_magnitude = 0;
                }
            }
            DoublePrecision::SignificantDigits(sig, mode) => {
                let sig = sig as usize;
                if sig == 0 {
                    return Err(Error::Limit);
                }
                if sig < n_digits {
                    let round_by = (n_digits - sig) as u16;
                    decimal.round_trailing_digits(round_by, mode)?;
                    // It may have rounded up by one
                    debug_assert!(decimal.digits.len() <= sig as usize);
                }
                let target_lowest_magnitude = decimal.magnitude - sig as i16 + 1;
                if target_lowest_magnitude <= 0 {
                    // If the target magnitude was negative, make sure we update
                    // the lower magnitude to match it
                    decimal.lower_magnitude = target_lowest_magnitude;
                } else if decimal.lower_magnitude < 0 {
                    // If the target magnitude was positive,
                    // we may have truncated digits off the right side
                    // of the decimal point, reset the lower magnitude
                    decimal.lower_magnitude = 0;
                }
            }
        }
        #[cfg(debug_assertions)]
        decimal.check_invariants();
        Ok(decimal)
    }
    /// Internal function for parsing directly from floats using ryū
    fn new_from_f64_raw(float: f64) -> Result<Self, Error> {
        if !float.is_finite() {
            return Err(Error::Limit);
        }
        // note: this does not heap allocate
        let mut buf = ryu::Buffer::new();
        let formatted = buf.format_finite(float);
        Self::from_str(formatted)
    }

    /// Internal function to round off `n` digits
    /// from the right
    ///
    /// `self` must have at least `n` digits
    ///
    /// This may end up adding a digit to the left!
    ///
    /// This will not change the number of significant digits, it simply exists
    /// to *round* them (and will typically reduce the size of `self.digits`)
    ///
    /// This function is responsible for fixing `digits`, `magnitude`, and `upper_magnitude`.
    /// It will only modify upper_magnitude when it is not large enough to fit the rounded number.
    /// The caller may fix up `lower_magnitude` by whatever scheme it desires
    fn round_trailing_digits(&mut self, n: u16, mode: RoundingMode) -> Result<(), Error> {
        /// For fixing up invariants after truncation
        ///
        /// Basically will ensure that we don't end with any zeroes
        fn fixup_invariants(dec: &mut FixedDecimal) {
            let first_nonzero = dec.digits.iter().rposition(|d| *d != 0).unwrap_or(0);
            dec.digits.truncate(first_nonzero + 1);
            if dec.digits.is_empty() {
                dec.magnitude = 0;
            }
        }

        debug_assert!(
            self.digits.len() >= n as usize,
            "Attempted to round off {} digits of number that has only {}",
            n,
            self.digits.len()
        );
        if n == 0 {
            // no point attempting to round off any digits
            #[cfg(debug_assertions)]
            self.check_invariants();
            return Ok(());
        }

        let cutoff = self.digits.len() - n as usize;

        match mode {
            RoundingMode::Unnecessary => {
                // If we got to this point then rounding was not unnecessary
                return Err(Error::Limit);
            }
            RoundingMode::Truncate => {
                self.digits.truncate(cutoff);
                fixup_invariants(self);
                #[cfg(debug_assertions)]
                self.check_invariants();
                return Ok(());
            }
            // continue to rest of routine
            RoundingMode::HalfExpand => (),
        }

        // Do we need to round our significant digits?
        // TODO(#1177): This heuristic is insufficient for most rounding modes.
        let round = self.digits[cutoff] >= 5;

        self.digits.truncate(cutoff);

        if round {
            // how much to truncate by after rounding
            let mut round_truncate = cutoff;
            for digit in self.digits[..cutoff].iter_mut().rev() {
                if *digit == 9 {
                    // Truncate this digit, the next digit can be rounded
                    round_truncate -= 1;
                } else {
                    // We need to update this digit, then we're done
                    *digit += 1;
                    self.digits.truncate(round_truncate);
                    #[cfg(debug_assertions)]
                    self.check_invariants();
                    return Ok(());
                }
            }

            // If we reached this point then the last digit was 9 and we need to insert
            // another digit at the beginning

            self.digits.clear();
            self.digits.push(1);
            debug_assert!(self.upper_magnitude >= 0);
            self.magnitude += 1;
            if self.upper_magnitude < self.magnitude {
                self.upper_magnitude = self.magnitude;
            }
        } else {
            fixup_invariants(self);
        }
        #[cfg(debug_assertions)]
        self.check_invariants();
        Ok(())
    }
}

#[cfg(feature = "ryu")]
#[test]
fn test_round() {
    #[derive(Debug)]
    struct TestCase {
        pub input: f64,
        pub round: u16,
        pub expected: &'static str,
    }
    let cases = [
        TestCase {
            input: 1.234567,
            round: 2,
            expected: "1.234600",
        },
        TestCase {
            input: 1.23456789,
            round: 2,
            expected: "1.23456800",
        },
        TestCase {
            input: 88899971.,
            round: 2,
            expected: "88900000.0",
        },
        TestCase {
            input: 999988.,
            round: 2,
            expected: "1000000.0",
        },
        TestCase {
            input: 0.9,
            round: 1,
            expected: "1.0",
        },
        TestCase {
            input: 9.9,
            round: 1,
            expected: "10.0",
        },
        TestCase {
            input: 9.9,
            round: 2,
            expected: "10.0",
        },
    ];

    for case in &cases {
        let mut dec = FixedDecimal::new_from_f64_raw(case.input).unwrap();
        dec.round_trailing_digits(case.round, RoundingMode::HalfExpand)
            .unwrap();
        writeable::assert_writeable_eq!(dec, case.expected, "{:?}", case);
    }
}

#[cfg(feature = "ryu")]
#[test]
fn test_float() {
    #[derive(Debug)]
    struct TestCase {
        pub input: f64,
        pub precision: DoublePrecision,
        pub expected: &'static str,
    }
    let cases = [
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::Maximum,
            expected: "1.234567",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::Maximum,
            expected: "888999",
        },
        // HalfExpand tests
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::Magnitude(-2, RoundingMode::HalfExpand),
            expected: "1.23",
        },
        TestCase {
            input: 1.235567,
            precision: DoublePrecision::Magnitude(-2, RoundingMode::HalfExpand),
            expected: "1.24",
        },
        TestCase {
            input: 1.2002,
            precision: DoublePrecision::Magnitude(-3, RoundingMode::HalfExpand),
            expected: "1.200",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::Magnitude(2, RoundingMode::HalfExpand),
            expected: "889000",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::Magnitude(4, RoundingMode::HalfExpand),
            expected: "890000",
        },
        TestCase {
            input: 0.9,
            precision: DoublePrecision::Magnitude(0, RoundingMode::HalfExpand),
            expected: "1",
        },
        TestCase {
            input: 0.9,
            precision: DoublePrecision::Magnitude(2, RoundingMode::HalfExpand),
            expected: "0",
        },
        TestCase {
            input: 0.009,
            precision: DoublePrecision::Magnitude(-2, RoundingMode::HalfExpand),
            expected: "0.01",
        },
        TestCase {
            input: 0.009,
            precision: DoublePrecision::Magnitude(-1, RoundingMode::HalfExpand),
            expected: "0.0",
        },
        TestCase {
            input: 0.009,
            precision: DoublePrecision::Magnitude(0, RoundingMode::HalfExpand),
            expected: "0",
        },
        TestCase {
            input: 0.0000009,
            precision: DoublePrecision::Magnitude(0, RoundingMode::HalfExpand),
            expected: "0",
        },
        TestCase {
            input: 0.0000009,
            precision: DoublePrecision::Magnitude(-7, RoundingMode::HalfExpand),
            expected: "0.0000009",
        },
        TestCase {
            input: 0.0000009,
            precision: DoublePrecision::Magnitude(-6, RoundingMode::HalfExpand),
            expected: "0.000001",
        },
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::SignificantDigits(1, RoundingMode::HalfExpand),
            expected: "1",
        },
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::SignificantDigits(2, RoundingMode::HalfExpand),
            expected: "1.2",
        },
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::SignificantDigits(4, RoundingMode::HalfExpand),
            expected: "1.235",
        },
        TestCase {
            input: 1.234567,
            precision: DoublePrecision::SignificantDigits(10, RoundingMode::HalfExpand),
            expected: "1.234567000",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::SignificantDigits(1, RoundingMode::HalfExpand),
            expected: "900000",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::SignificantDigits(2, RoundingMode::HalfExpand),
            expected: "890000",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::SignificantDigits(4, RoundingMode::HalfExpand),
            expected: "889000",
        },
        TestCase {
            input: 988999.,
            precision: DoublePrecision::SignificantDigits(1, RoundingMode::HalfExpand),
            expected: "1000000",
        },
        TestCase {
            input: 99888.,
            precision: DoublePrecision::SignificantDigits(1, RoundingMode::HalfExpand),
            expected: "100000",
        },
        TestCase {
            input: 99888.,
            precision: DoublePrecision::SignificantDigits(2, RoundingMode::HalfExpand),
            expected: "100000",
        },
        TestCase {
            input: 99888.,
            precision: DoublePrecision::SignificantDigits(3, RoundingMode::HalfExpand),
            expected: "99900",
        },
        TestCase {
            input: 9.9888,
            precision: DoublePrecision::SignificantDigits(1, RoundingMode::HalfExpand),
            expected: "10",
        },
        TestCase {
            input: 9.9888,
            precision: DoublePrecision::SignificantDigits(2, RoundingMode::HalfExpand),
            expected: "10",
        },
        TestCase {
            input: 9.9888,
            precision: DoublePrecision::SignificantDigits(3, RoundingMode::HalfExpand),
            expected: "9.99",
        },
        // truncation tests
        TestCase {
            input: 888999.,
            precision: DoublePrecision::Magnitude(4, RoundingMode::Truncate),
            expected: "880000",
        },
        TestCase {
            input: 0.009,
            precision: DoublePrecision::Magnitude(-2, RoundingMode::Truncate),
            expected: "0.00",
        },
        TestCase {
            input: 0.0000009,
            precision: DoublePrecision::Magnitude(-7, RoundingMode::Truncate),
            expected: "0.0000009",
        },
        TestCase {
            input: 9.9888,
            precision: DoublePrecision::SignificantDigits(3, RoundingMode::Truncate),
            expected: "9.98",
        },
        TestCase {
            input: 888999.,
            precision: DoublePrecision::Integer,
            expected: "888999",
        },
    ];

    for case in &cases {
        let dec = FixedDecimal::new_from_f64(case.input, case.precision).unwrap();
        writeable::assert_writeable_eq!(dec, case.expected, "{:?}", case);
    }
}

#[test]
fn test_basic() {
    #[derive(Debug)]
    struct TestCase {
        pub input: isize,
        pub delta: i16,
        pub expected: &'static str,
    }
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
        // println!("{}", cas.input + 0.01);
        dec.multiply_pow10(cas.delta).unwrap();
        writeable::assert_writeable_eq!(dec, cas.expected, "{:?}", cas);
    }
}

#[test]
fn test_from_str() {
    #[derive(Debug)]
    struct TestCase {
        pub input_str: &'static str,
    }
    let cases = [
        TestCase {
            input_str: "-00123400",
        },
        TestCase {
            input_str: "0.0123400",
        },
        TestCase {
            input_str: "-00.123400",
        },
        TestCase {
            input_str: "0012.3400",
        },
        TestCase {
            input_str: "-0012340.0",
        },
        TestCase { input_str: "1234" },
        TestCase {
            input_str: "0.000000001",
        },
        TestCase {
            input_str: "0.0000000010",
        },
        TestCase {
            input_str: "1000000",
        },
        TestCase {
            input_str: "10000001",
        },
        TestCase { input_str: "123" },
        TestCase {
            input_str: "922337203685477580898230948203840239384.9823094820384023938423424",
        },
        TestCase {
            input_str: "009223372000.003685477580898230948203840239384000",
        },
        TestCase {
            input_str: "009223372000.003685477580898230948203840239384000",
        },
        TestCase { input_str: "0" },
        TestCase { input_str: "-0" },
        TestCase { input_str: "000" },
        TestCase { input_str: "-00.0" },
    ];
    for cas in &cases {
        let input_str_roundtrip = FixedDecimal::from_str(cas.input_str).unwrap().to_string();
        assert_eq!(cas.input_str, input_str_roundtrip);
    }
}

#[test]
fn test_from_str_scientific() {
    #[derive(Debug)]
    struct TestCase {
        pub input_str: &'static str,
        pub output: &'static str,
    }
    let cases = [
        TestCase {
            input_str: "-5.4e10",
            output: "-54000000000",
        },
        TestCase {
            input_str: "5.4e-2",
            output: "0.054",
        },
        TestCase {
            input_str: "54.1e-2",
            output: "0.541",
        },
        TestCase {
            input_str: "-541e-2",
            output: "-5.41",
        },
        TestCase {
            input_str: "0.009E10",
            output: "90000000",
        },
        TestCase {
            input_str: "-9000E-10",
            output: "-0.0000009",
        },
    ];
    for cas in &cases {
        let input_str_roundtrip = FixedDecimal::from_str(cas.input_str).unwrap().to_string();
        assert_eq!(cas.output, input_str_roundtrip);
    }
}

#[test]
fn test_isize_limits() {
    for num in &[core::isize::MAX, core::isize::MIN] {
        let dec: FixedDecimal = (*num).into();
        let dec_str = dec.to_string();
        assert_eq!(num.to_string(), dec_str);
        assert_eq!(dec, FixedDecimal::from_str(&dec_str).unwrap());
        writeable::assert_writeable_eq!(dec, dec_str);
    }
}

#[test]
fn test_ui128_limits() {
    for num in &[core::i128::MAX, core::i128::MIN] {
        let dec: FixedDecimal = (*num).into();
        let dec_str = dec.to_string();
        assert_eq!(num.to_string(), dec_str);
        assert_eq!(dec, FixedDecimal::from_str(&dec_str).unwrap());
        writeable::assert_writeable_eq!(dec, dec_str);
    }
    for num in &[core::u128::MAX, core::u128::MIN] {
        let dec: FixedDecimal = (*num).into();
        let dec_str = dec.to_string();
        assert_eq!(num.to_string(), dec_str);
        assert_eq!(dec, FixedDecimal::from_str(&dec_str).unwrap());
        writeable::assert_writeable_eq!(dec, dec_str);
    }
}

#[test]
fn test_upper_magnitude_bounds() {
    let mut dec: FixedDecimal = 98765.into();
    assert_eq!(dec.upper_magnitude, 4);
    dec.multiply_pow10(32763).unwrap();
    assert_eq!(dec.upper_magnitude, core::i16::MAX);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.multiply_pow10(1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");

    // Checking from_str for dec (which is valid)
    let dec_roundtrip = FixedDecimal::from_str(&dec.to_string()).unwrap();
    assert_eq!(dec, dec_roundtrip);
}

#[test]
fn test_lower_magnitude_bounds() {
    let mut dec: FixedDecimal = 98765.into();
    assert_eq!(dec.lower_magnitude, 0);
    dec.multiply_pow10(-32768).unwrap();
    assert_eq!(dec.lower_magnitude, core::i16::MIN);
    let dec_backup = dec.clone();
    assert_eq!(Error::Limit, dec.multiply_pow10(-1).unwrap_err());
    assert_eq!(dec, dec_backup, "Value should be unchanged on failure");

    // Checking from_str for dec (which is valid)
    let dec_roundtrip = FixedDecimal::from_str(&dec.to_string()).unwrap();
    assert_eq!(dec, dec_roundtrip);
}

#[test]
fn test_zero_str_bounds() {
    #[derive(Debug)]
    struct TestCase {
        pub zeros_before_dot: usize,
        pub zeros_after_dot: usize,
        pub expected_err: Option<Error>,
    }
    // Note that core::i16::MAX = 32768
    let cases = [
        TestCase {
            zeros_before_dot: 32768,
            zeros_after_dot: 0,
            expected_err: None,
        },
        TestCase {
            zeros_before_dot: 32767,
            zeros_after_dot: 0,
            expected_err: None,
        },
        TestCase {
            zeros_before_dot: 32769,
            zeros_after_dot: 0,
            expected_err: Some(Error::Limit),
        },
        TestCase {
            zeros_before_dot: 0,
            zeros_after_dot: 32769,
            expected_err: Some(Error::Limit),
        },
        TestCase {
            zeros_before_dot: 32768,
            zeros_after_dot: 32768,
            expected_err: None,
        },
        TestCase {
            zeros_before_dot: 32769,
            zeros_after_dot: 32768,
            expected_err: Some(Error::Limit),
        },
        TestCase {
            zeros_before_dot: 32768,
            zeros_after_dot: 32769,
            expected_err: Some(Error::Limit),
        },
        TestCase {
            zeros_before_dot: 32767,
            zeros_after_dot: 32769,
            expected_err: Some(Error::Limit),
        },
        TestCase {
            zeros_before_dot: 32767,
            zeros_after_dot: 32767,
            expected_err: None,
        },
        TestCase {
            zeros_before_dot: 32768,
            zeros_after_dot: 32767,
            expected_err: None,
        },
    ];
    for cas in &cases {
        let mut input_str = format!("{:0fill$}", 0, fill = cas.zeros_before_dot);
        if cas.zeros_after_dot > 0 {
            input_str.push('.');
            input_str.push_str(&format!("{:0fill$}", 0, fill = cas.zeros_after_dot));
        }
        match FixedDecimal::from_str(&input_str) {
            Ok(dec) => {
                assert_eq!(cas.expected_err, None, "{:?}", cas);
                assert_eq!(input_str, dec.to_string(), "{:?}", cas);
            }
            Err(err) => {
                assert_eq!(cas.expected_err, Some(err), "{:?}", cas);
            }
        }
    }
}

#[test]
fn test_syntax_error() {
    #[derive(Debug)]
    struct TestCase {
        pub input_str: &'static str,
        pub expected_err: Option<Error>,
    }
    let cases = [
        TestCase {
            input_str: "-12a34",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "0.0123√400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "0.012.3400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-0-0123400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "0-0123400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-.00123400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-0.00123400",
            expected_err: None,
        },
        TestCase {
            input_str: ".00123400",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "00123400.",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "00123400.0",
            expected_err: None,
        },
        TestCase {
            input_str: "123_456",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-",
            expected_err: Some(Error::Syntax),
        },
        TestCase {
            input_str: "-1",
            expected_err: None,
        },
    ];
    for cas in &cases {
        match FixedDecimal::from_str(cas.input_str) {
            Ok(dec) => {
                assert_eq!(cas.expected_err, None, "{:?}", cas);
                assert_eq!(cas.input_str, dec.to_string(), "{:?}", cas);
            }
            Err(err) => {
                assert_eq!(cas.expected_err, Some(err), "{:?}", cas);
            }
        }
    }
}

#[test]
fn test_signum_zero() {
    #[derive(Debug)]
    struct TestCase {
        pub fixed_decimal: FixedDecimal,
        pub expected_signum: Signum,
    }
    let cases = [
        TestCase {
            fixed_decimal: Default::default(),
            expected_signum: Signum::PositiveZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from(0),
            expected_signum: Signum::PositiveZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from(0).negated(),
            expected_signum: Signum::NegativeZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from_str("0").unwrap(),
            expected_signum: Signum::PositiveZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from_str("000").unwrap(),
            expected_signum: Signum::PositiveZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from_str("-0.000").unwrap(),
            expected_signum: Signum::NegativeZero,
        },
        TestCase {
            fixed_decimal: FixedDecimal::from_str("000.000").unwrap(),
            expected_signum: Signum::PositiveZero,
        },
    ];
    for cas in &cases {
        let signum = cas.fixed_decimal.signum();
        assert_eq!(cas.expected_signum, signum, "{:?}", cas);
    }
}

#[test]
fn test_pad() {
    let mut dec = FixedDecimal::from_str("-0.42").unwrap();
    assert_eq!("-0.42", dec.to_string());

    dec.pad_left(1);
    assert_eq!("-0.42", dec.to_string());

    dec.pad_left(4);
    assert_eq!("-0000.42", dec.to_string());

    dec.pad_left(2);
    assert_eq!("-00.42", dec.to_string());
}

#[test]
fn test_truncate() {
    let mut dec = FixedDecimal::from(1000);
    assert_eq!("1000", dec.to_string());

    dec.truncate_left(2);
    assert_eq!("000", dec.to_string());

    dec.truncate_left(0);
    assert_eq!("0", dec.to_string());

    dec.truncate_left(3);
    assert_eq!("0", dec.to_string());

    let mut dec = FixedDecimal::from_str("0.456").unwrap();
    assert_eq!("0.456", dec.to_string());

    dec.truncate_left(0);
    assert_eq!("0.456", dec.to_string());

    dec.truncate_left(-1);
    assert_eq!("0.456", dec.to_string());

    dec.truncate_left(-2);
    assert_eq!("0.056", dec.to_string());

    dec.truncate_left(-3);
    assert_eq!("0.006", dec.to_string());

    dec.truncate_left(-4);
    assert_eq!("0.000", dec.to_string());

    let mut dec = FixedDecimal::from_str("100.01").unwrap();
    dec.truncate_left(1);
    assert_eq!("00.01", dec.to_string());
}
