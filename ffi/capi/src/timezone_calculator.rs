// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::provider::ffi::DataProvider;

    /// An object capable of computing the metazone from a timezone.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::MetazoneCalculator, Struct)]
    pub struct TimeZoneCalculator(pub icu_timezone::TimeZoneCalculator);

    impl TimeZoneCalculator {
        #[diplomat::rust_link(icu::timezone::MetazoneCalculator::new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        pub fn create(provider: &DataProvider) -> Result<Box<TimeZoneCalculator>, DataError> {
            Ok(Box::new(TimeZoneCalculator(call_constructor!(
                icu_timezone::TimeZoneCalculator::new [r => Ok(r)],
                icu_timezone::TimeZoneCalculator::try_new_with_any_provider,
                icu_timezone::TimeZoneCalculator::try_new_with_buffer_provider,
                provider,
            )?)))
        }
    }
}
