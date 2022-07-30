// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use core::str::FromStr;

    use alloc::boxed::Box;

    use icu_plurals::{PluralCategory, PluralOperands, PluralRules};

    use crate::{locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    /// FFI version of `PluralCategory`.
    #[diplomat::rust_link(icu_plurals::PluralCategory, Enum)]
    pub enum ICU4XPluralCategory {
        Zero,
        One,
        Two,
        Few,
        Many,
        Other,
    }

    /// FFI version of `PluralRules`.
    #[diplomat::rust_link(icu_plurals::PluralRules, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XPluralRules(PluralRules);

    impl ICU4XPluralRules {
        /// FFI version of `PluralRules::try_new_cardinal()`.
        #[diplomat::rust_link(icu_plurals::PluralRules::try_new, FnInStruct)]
        pub fn try_new_cardinal(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XPluralRules>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let locale = &locale.0.as_ref().into();
            let provider = provider.0.as_deserializing();
            PluralRules::try_new_cardinal(&provider, locale)
                .map(|r| Box::new(ICU4XPluralRules(r)))
                .map_err(Into::into)
                .into()
        }

        /// FFI version of `PluralRules::try_new_ordinal()`.
        #[diplomat::rust_link(icu_plurals::PluralRules::try_new, FnInStruct)]
        pub fn try_new_ordinal(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XPluralRules>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let locale = &locale.0.as_ref().into();
            let provider = provider.0.as_deserializing();
            PluralRules::try_new_ordinal(&provider, locale)
                .map(|r| Box::new(ICU4XPluralRules(r)))
                .map_err(Into::into)
                .into()
        }

        /// FFI version of `PluralRules::select()`.
        #[diplomat::rust_link(icu_plurals::PluralRules::select, FnInStruct)]
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
        #[diplomat::rust_link(icu_plurals::PluralRules::categories, FnInStruct)]
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

    /// FFI version of `PluralOperands`.
    #[diplomat::rust_link(icu_plurals::PluralOperands, Struct)]
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
        #[diplomat::rust_link(icu_plurals::PluralOperands::from_str, FnInStruct)]
        pub fn create(s: &str) -> DiplomatResult<ICU4XPluralOperands, ICU4XError> {
            PluralOperands::from_str(s)
                .map(|ops| ICU4XPluralOperands {
                    i: ops.i,
                    v: ops.v,
                    w: ops.w,
                    f: ops.f,
                    t: ops.t,
                    c: ops.c,
                })
                // XXX should this have its own errors?
                .map_err(|_| ICU4XError::PluralParserError)
                .into()
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
