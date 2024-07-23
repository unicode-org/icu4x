// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::provider::ffi::DataProvider;

    /// An object capable of computing the metazone from a timezone.
    ///
    /// This can be used via `maybe_calculate_metazone()` on [`CustomTimeZone`].
    ///
    /// [`CustomTimeZone`]: crate::timezone::ffi::CustomTimeZone
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::MetazoneCalculator, Struct)]
    pub struct MetazoneCalculator(pub icu_timezone::MetazoneCalculator);

    impl MetazoneCalculator {
        #[diplomat::rust_link(icu::timezone::MetazoneCalculator::new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        pub fn create(provider: &DataProvider) -> Result<Box<MetazoneCalculator>, DataError> {
            Ok(Box::new(MetazoneCalculator(call_constructor!(
                icu_timezone::MetazoneCalculator::new [r => Ok(r)],
                icu_timezone::MetazoneCalculator::try_new_with_any_provider,
                icu_timezone::MetazoneCalculator::try_new_with_buffer_provider,
                provider,
            )?)))
        }
    }
}
