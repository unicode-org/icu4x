// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use fixed_decimal::FixedDecimal;
    use writeable::Writeable;

    #[diplomat::opaque]
    /// A decimal number. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    pub struct ICU4XCreateFixedDecimalResult {
        /// Will be None if `success` is `false`
        pub fd: Option<Box<ICU4XFixedDecimal>>,
        /// Currently just a boolean, but we might add a proper error enum as necessary
        pub success: bool,
    }

    impl ICU4XFixedDecimal {
        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
        fn create(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Construct an [`ICU4XFixedDecimal`] from a string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
        fn create_fromstr(v: &str) -> ICU4XCreateFixedDecimalResult {
            v.parse::<FixedDecimal>()
                .map(|v| ICU4XCreateFixedDecimalResult {
                    fd: Some(Box::new(ICU4XFixedDecimal(v))),
                    success: true,
                })
                .unwrap_or(ICU4XCreateFixedDecimalResult {
                    fd: None,
                    success: false,
                })
        }

        /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
        fn multiply_pow10(&mut self, power: i16) -> bool {
            self.0.multiply_pow10(power).is_ok()
        }

        /// Invert the sign of the [`ICU4XFixedDecimal`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate) for more information.
        fn negate(&mut self) {
            self.0.negate()
        }

        /// Format the [`ICU4XFixedDecimal`] as a string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to) for more information.
        fn to_string(&self, to: &mut diplomat_runtime::DiplomatWriteable) {
            self.0.write_to(to).unwrap();
        }
    }
}
