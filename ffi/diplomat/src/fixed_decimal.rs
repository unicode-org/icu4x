// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::decimal::Sign;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use fixed_decimal::decimal::{DoublePrecision, FixedDecimal};
    use writeable::Writeable;

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal, Struct)]
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    /// The sign of a FixedDecimal, as shown in formatting.
    #[diplomat::rust_link(fixed_decimal::decimal::Sign, Enum)]
    pub enum ICU4XFixedDecimalSign {
        /// No sign (implicitly positive, e.g., 1729).
        None,
        /// A negative sign, e.g., -1729.
        Negative,
        /// An explicit positive sign, e.g., +1729.
        Positive,
    }

    fn convert(dec: FixedDecimal) -> Box<ICU4XFixedDecimal> {
        Box::new(ICU4XFixedDecimal(dec))
    }

    impl ICU4XFixedDecimal {
        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal, Struct)]
        pub fn create(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, with enough digits to recover
        /// the original floating point in IEEE 754 without needing trailing zeros
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::try_from_f64, FnInStruct)]
        pub fn create_from_f64_with_max_precision(
            f: f64,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::Floating;
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, with a given power of 10 for the lower magnitude
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::try_from_f64, FnInStruct)]
        pub fn create_from_f64_with_lower_magnitude(
            f: f64,
            precision: i16,
        ) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            let precision = DoublePrecision::Magnitude(precision);
            FixedDecimal::try_from_f64(f, precision)
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Construct an [`ICU4XFixedDecimal`] from an float, for a given number of significant digits
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::try_from_f64, FnInStruct)]
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

        /// Construct an [`ICU4XFixedDecimal`] from a string.
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal, Struct)]
        pub fn create_fromstr(v: &str) -> DiplomatResult<Box<ICU4XFixedDecimal>, ICU4XError> {
            v.parse::<FixedDecimal>()
                .map(convert)
                .map_err(Into::into)
                .into()
        }

        /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::multiply_pow10, FnInStruct)]
        pub fn multiply_pow10(&mut self, power: i16) -> bool {
            self.0.multiply_pow10(power).is_ok()
        }

        /// Set the sign of the [`ICU4XFixedDecimal`].
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::set_sign, FnInStruct)]
        pub fn set_sign(&mut self, sign: ICU4XFixedDecimalSign) {
            self.0.set_sign(sign.into())
        }

        /// Zero-pad the [`ICU4XFixedDecimal`] on the left to a particular position
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::pad_start, FnInStruct)]
        pub fn pad_start(&mut self, position: i16) {
            self.0.pad_start(position)
        }

        /// Truncate the [`ICU4XFixedDecimal`] on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years
        /// ("2022" -> "22")
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::set_max_position, FnInStruct)]
        pub fn set_max_position(&mut self, position: i16) {
            self.0.set_max_position(position)
        }

        /// Zero-pad the [`ICU4XFixedDecimal`] on the right to a particular position
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::pad_end, FnInStruct)]
        pub fn pad_end(&mut self, position: i16) {
            self.0.pad_end(position)
        }

        /// Format the [`ICU4XFixedDecimal`] as a string.
        #[diplomat::rust_link(fixed_decimal::decimal::FixedDecimal::write_to, FnInStruct)]
        pub fn to_string(&self, to: &mut diplomat_runtime::DiplomatWriteable) {
            #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
            self.0.write_to(to).unwrap();
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
