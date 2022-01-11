// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_provider::prelude::ResourceKey;

    #[diplomat::opaque]
    /// An ICU4X resource key
    ///
    /// See [the rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/struct.ResourceKey.html) for more details
    pub struct ICU4XResourceKey(ResourceKey);

    impl ICU4XResourceKey {
        /// Get the resource key for DecimalSymbolsV1
        ///
        /// See [the rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/provider/key/constant.SYMBOLS_V1.html) for more information
        pub fn for_decimal_symbols() -> Box<ICU4XResourceKey> {
            // todo: Make this return &'static ICU4XResourceKey instead
            Box::new(ICU4XResourceKey(icu_decimal::provider::key::SYMBOLS_V1))
        }
    }
}
