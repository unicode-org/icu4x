// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::Sign;
use fixed_decimal::SignDisplay;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use fixed_decimal::{DoublePrecision, FixedDecimal};
    use writeable::Writeable;

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    /// The sign of a FixedDecimal, as shown in formatting.
    #[diplomat::rust_link(fixed_decimal::Sign, Enum)]
    pub enum ICU4XFixedDecimalSign {
        /// No sign (implicitly positive, e.g., 1729).
        None,
        /// A negative sign, e.g., -1729.
        Negative,
        /// An explicit positive sign, e.g., +1729.
        Positive,
    }

    /// ECMA-402 compatible sign display preference.
    #[diplomat::rust_link(fixed_decimal::decimal::SignDisplay, Enum)]
    pub enum ICU4XFixedDecimalSignDisplay {
        Auto,
        Never,
        Always,
        ExceptZero,
        Negative,
    }

    fn convert(dec: FixedDecimal) -> Box<ICU4XFixedDecimal> {
        Box::new(ICU4XFixedDecimal(dec))
    }

    impl ICU4XFixedDecimal {
        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
        pub fn create_from_i32(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
        pub fn create_from_u32(v: u32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
        pub fn create_from_i64(v: i64) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
        pub fn create_from_u64(v: u64) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from an integer-valued float
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::try_from_f64, FnInStruct)]
        #[diplomat::rust_link(fixed_decimal::DoublePrecision, Enum)]
        pub fn create_from_f64_with_integer_precision(
            f: f64,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::Integer;
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, with a given power of 10 for the lower magnitude
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::try_from_f64, FnInStruct)]
        #[diplomat::rust_link(fixed_decimal::DoublePrecision, Enum)]
        pub fn create_from_f64_with_lower_magnitude(
            f: f64,
            magnitude: i16,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::Magnitude(magnitude);
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, for a given number of significant digits
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::try_from_f64, FnInStruct)]
        #[diplomat::rust_link(fixed_decimal::DoublePrecision, Enum)]
        pub fn create_from_f64_with_significant_digits(
            f: f64,
            digits: u8,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::SignificantDigits(digits);
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, with enough digits to recover
        /// the original floating point in IEEE 754 without needing trailing zeros
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::try_from_f64, FnInStruct)]
        #[diplomat::rust_link(fixed_decimal::decimal::DoublePrecision, Enum)]
        pub fn create_from_f64_with_floating_precision(
            f: f64,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::Floating;
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from a string.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
        pub fn create_from_str(v: &str) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            v.parse::<FixedDecimal>()
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::digit_at, FnInStruct)]
        pub fn digit_at(&self, magnitude: i16) -> u8 {
            self.0.digit_at(magnitude)
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::magnitude_range, FnInStruct)]
        pub fn magnitude_start(&self) -> i16 {
            *self.0.magnitude_range().start()
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::magnitude_range, FnInStruct)]
        pub fn magnitude_end(&self) -> i16 {
            *self.0.magnitude_range().end()
        }

        #[diplomat::rust_link(
            fixed_decimal::decimal::FixedDecimal::nonzero_magnitude_start,
            FnInStruct
        )]
        pub fn nonzero_magnitude_start(&self) -> i16 {
            self.0.nonzero_magnitude_start()
        }

        #[diplomat::rust_link(
            fixed_decimal::decimal::FixedDecimal::nonzero_magnitude_end,
            FnInStruct
        )]
        pub fn nonzero_magnitude_end(&self) -> i16 {
            self.0.nonzero_magnitude_end()
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::is_zero, FnInStruct)]
        pub fn is_zero(&self) -> bool {
            self.0.is_zero()
        }

        /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::multiply_pow10, FnInStruct)]
        pub fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power)
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::sign, FnInStruct)]
        pub fn sign(&self) -> ICU4XFixedDecimalSign {
            self.0.sign().into()
        }

        /// Set the sign of the [`ICU4XFixedDecimal`].
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::set_sign, FnInStruct)]
        pub fn set_sign(&mut self, sign: ICU4XFixedDecimalSign) {
            self.0.set_sign(sign.into())
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::apply_sign_display, FnInStruct)]
        pub fn apply_sign_display(&mut self, sign_display: ICU4XFixedDecimalSignDisplay) {
            self.0.apply_sign_display(sign_display.into())
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::trim_start, FnInStruct)]
        pub fn trim_start(&mut self) {
            self.0.trim_start()
        }

        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::trim_end, FnInStruct)]
        pub fn trim_end(&mut self) {
            self.0.trim_end()
        }

        /// Zero-pad the [`ICU4XFixedDecimal`] on the left to a particular position
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::pad_start, FnInStruct)]
        pub fn pad_start(&mut self, position: i16) {
            self.0.pad_start(position)
        }

        /// Zero-pad the [`ICU4XFixedDecimal`] on the right to a particular position
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::pad_end, FnInStruct)]
        pub fn pad_end(&mut self, position: i16) {
            self.0.pad_end(position)
        }

        /// Truncate the [`ICU4XFixedDecimal`] on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years
        /// ("2022" -> "22")
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::set_max_position, FnInStruct)]
        pub fn set_max_position(&mut self, position: i16) {
            self.0.set_max_position(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::trunc, FnInStruct)]
        pub fn trunc(&mut self, position: i16) {
            self.0.trunc(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::half_trunc, FnInStruct)]
        pub fn half_trunc(&mut self, position: i16) {
            self.0.half_trunc(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::expand, FnInStruct)]
        pub fn expand(&mut self, position: i16) {
            self.0.expand(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::half_expand, FnInStruct)]
        pub fn half_expand(&mut self, position: i16) {
            self.0.half_expand(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::ceil, FnInStruct)]
        pub fn ceil(&mut self, position: i16) {
            self.0.ceil(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::half_ceil, FnInStruct)]
        pub fn half_ceil(&mut self, position: i16) {
            self.0.half_ceil(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::floor, FnInStruct)]
        pub fn floor(&mut self, position: i16) {
            self.0.floor(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::half_floor, FnInStruct)]
        pub fn half_floor(&mut self, position: i16) {
            self.0.half_floor(position)
        }

        #[diplomat::rust_link(fixed_decimal::FixedDecimal::half_even, FnInStruct)]
        pub fn half_even(&mut self, position: i16) {
            self.0.half_even(position)
        }

        /// Concatenates `other` to the end of `self`.
        ///
        /// If successful, `other` will be set to 0 and a successful status is returned.
        ///
        /// If not successful, `other` will be unchanged and an error is returned.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::concatenate_end, FnInStruct)]
        pub fn concatenate_end(&mut self, other: &mut ICU4XFixedDecimal) -> DiplomatResult<(), ()> {
            let x = core::mem::take(&mut other.0);
            match self.0.concatenate_end(x) {
                Ok(()) => Ok(()),
                Err(y) => {
                    other.0 = y;
                    Err(())
                }
            }
            .into()
        }

        /// Format the [`ICU4XFixedDecimal`] as a string.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::write_to, FnInStruct)]
        pub fn to_string(&self, to: &mut diplomat_runtime::DiplomatWriteable) {
            let _ = self.0.write_to(to);
        }
    }
}

impl From<ffi::ICU4XFixedDecimalSign> for Sign {
    fn from(other: ffi::ICU4XFixedDecimalSign) -> Self {
        match other {
            ffi::ICU4XFixedDecimalSign::None => Self::None,
            ffi::ICU4XFixedDecimalSign::Negative => Self::Negative,
            ffi::ICU4XFixedDecimalSign::Positive => Self::Positive,
        }
    }
}

impl From<Sign> for ffi::ICU4XFixedDecimalSign {
    fn from(other: Sign) -> Self {
        match other {
            Sign::None => Self::None,
            Sign::Negative => Self::Negative,
            Sign::Positive => Self::Positive,
        }
    }
}

impl From<ffi::ICU4XFixedDecimalSignDisplay> for SignDisplay {
    fn from(other: ffi::ICU4XFixedDecimalSignDisplay) -> Self {
        match other {
            ffi::ICU4XFixedDecimalSignDisplay::Auto => Self::Auto,
            ffi::ICU4XFixedDecimalSignDisplay::Never => Self::Never,
            ffi::ICU4XFixedDecimalSignDisplay::Always => Self::Always,
            ffi::ICU4XFixedDecimalSignDisplay::ExceptZero => Self::ExceptZero,
            ffi::ICU4XFixedDecimalSignDisplay::Negative => Self::Negative,
        }
    }
}
