// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use core::str::FromStr;

    use alloc::boxed::Box;

    use icu_plurals::{PluralCategory, PluralOperands, PluralRules};

    use crate::{locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    pub struct ICU4XCreatePluralRulesResult {
        pub rules: Option<Box<ICU4XPluralRules>>,
        pub success: bool,
    }

    /// FFI version of `PluralCategory`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralCategory.html) for more details.
    pub enum ICU4XPluralCategory {
        Zero,
        One,
        Two,
        Few,
        Many,
        Other,
    }

    /// FFI version of `PluralRules`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html) for more details.
    #[diplomat::opaque]
    pub struct ICU4XPluralRules(PluralRules);

    impl ICU4XPluralRules {
        /// FFI version of `PluralRules::try_new_cardinal()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new) for more details.
        pub fn try_new_cardinal(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
        ) -> ICU4XCreatePluralRulesResult {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            PluralRules::try_new_cardinal(locale.0.as_ref().clone(), &provider)
                .ok()
                .map(|r| ICU4XCreatePluralRulesResult {
                    rules: Some(Box::new(ICU4XPluralRules(r))),
                    success: true,
                })
                .unwrap_or(ICU4XCreatePluralRulesResult {
                    rules: None,
                    success: false,
                })
        }

        /// FFI version of `PluralRules::try_new_ordinal()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new) for more details.
        pub fn try_new_ordinal(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
        ) -> ICU4XCreatePluralRulesResult {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            PluralRules::try_new_ordinal(locale.0.as_ref().clone(), &provider)
                .ok()
                .map(|r| ICU4XCreatePluralRulesResult {
                    rules: Some(Box::new(ICU4XPluralRules(r))),
                    success: true,
                })
                .unwrap_or(ICU4XCreatePluralRulesResult {
                    rules: None,
                    success: false,
                })
        }

        /// FFI version of `PluralRules::select()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select) for more details.
        pub fn select(&self, op: ICU4XPluralOperands) -> ICU4XPluralCategory {
            let res = self.0.select(PluralOperands {
                i: op.i,
                v: op.v,
                w: op.w,
                f: op.f,
                t: op.t,
                c: op.c,
            });

            match res {
                PluralCategory::Zero => ICU4XPluralCategory::Zero,
                PluralCategory::One => ICU4XPluralCategory::One,
                PluralCategory::Two => ICU4XPluralCategory::Two,
                PluralCategory::Few => ICU4XPluralCategory::Few,
                PluralCategory::Many => ICU4XPluralCategory::Many,
                PluralCategory::Other => ICU4XPluralCategory::Other,
            }
        }

        /// FFI version of `PluralRules::categories()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories) for more details.
        pub fn categories(&self) -> ICU4XPluralCategories {
            self.0.categories().fold(
                ICU4XPluralCategories {
                    zero: false,
                    one: false,
                    two: false,
                    few: false,
                    many: false,
                    other: false,
                },
                |mut categories, category| {
                    match category {
                        PluralCategory::Zero => categories.zero = true,
                        PluralCategory::One => categories.one = true,
                        PluralCategory::Two => categories.two = true,
                        PluralCategory::Few => categories.few = true,
                        PluralCategory::Many => categories.many = true,
                        PluralCategory::Other => categories.other = true,
                    };
                    categories
                },
            )
        }
    }

    /// This is the result returned by `ICU4XPluralOperands::create()`
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html) for more details.
    pub struct ICU4XCreatePluralOperandsResult {
        pub operands: ICU4XPluralOperands,
        pub success: bool,
    }

    /// FFI version of `PluralOperands`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html) for more details.
    pub struct ICU4XPluralOperands {
        pub i: u64,
        pub v: usize,
        pub w: usize,
        pub f: u64,
        pub t: u64,
        pub c: usize,
    }

    impl ICU4XPluralOperands {
        /// FFI version of `PluralOperands::from_str()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str) for more details.
        pub fn create(s: &str) -> ICU4XCreatePluralOperandsResult {
            PluralOperands::from_str(s)
                .ok()
                .map(|ops| ICU4XCreatePluralOperandsResult {
                    operands: ICU4XPluralOperands {
                        i: ops.i,
                        v: ops.v,
                        w: ops.w,
                        f: ops.f,
                        t: ops.t,
                        c: ops.c,
                    },
                    success: true,
                })
                .unwrap_or(ICU4XCreatePluralOperandsResult {
                    operands: ICU4XPluralOperands {
                        i: 0,
                        v: 0,
                        w: 0,
                        f: 0,
                        t: 0,
                        c: 0,
                    },
                    success: false,
                })
        }
    }

    /// FFI version of `PluralRules::categories()` data.
    pub struct ICU4XPluralCategories {
        pub zero: bool,
        pub one: bool,
        pub two: bool,
        pub few: bool,
        pub many: bool,
        pub other: bool,
    }
}
