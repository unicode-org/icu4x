// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(feature = "buffer_provider")]
    use crate::errors::ffi::DataError;
    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;

    #[diplomat::rust_link(icu::time::zone::UtcOffsetCalculator, Struct)]
    #[diplomat::opaque]
    pub struct UtcOffsetCalculator(pub icu_time::zone::UtcOffsetCalculator);

    impl UtcOffsetCalculator {
        /// Construct a new [`UtcOffsetCalculator`] instance using compiled data.
        #[diplomat::rust_link(icu::time::zone::UtcOffsetCalculator::new, FnInStruct)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<UtcOffsetCalculator> {
            Box::new(UtcOffsetCalculator(
                icu_time::zone::UtcOffsetCalculator::new(),
            ))
        }
        /// Construct a new [`UtcOffsetCalculator`] instance using a particular data source.
        #[diplomat::rust_link(icu::time::zone::UtcOffsetCalculator::new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<UtcOffsetCalculator>, DataError> {
            Ok(Box::new(UtcOffsetCalculator(
                icu_time::zone::UtcOffsetCalculator::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }
    }
}
