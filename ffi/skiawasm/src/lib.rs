// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a example of how to build a size-optimized WASM library file using ICU4X.

extern crate icu_capi;

#[cfg(not(feature = "empty_data"))]
mod data {
    include!("../icu4x_data_skiawasm_bake/mod.rs");
    include!("../icu4x_data_skiawasm_bake/any.rs");

    pub fn get_provider() -> Box<dyn icu_provider::AnyProvider> {
        Box::new(BakedDataProvider)
    }
}

#[cfg(feature = "empty_data")]
mod data {
    use icu_provider_adapters::empty::EmptyDataProvider;

    pub fn get_provider() -> Box<dyn icu_provider::AnyProvider> {
        Box::new(EmptyDataProvider::new())
    }
}

#[no_mangle]
pub extern "C" fn skiawasm_get_provider() -> Box<icu_capi::provider::ffi::ICU4XDataProvider> {
    Box::new(icu_capi::provider::ffi::ICU4XDataProvider(icu_capi::provider::ICU4XDataProviderInner::Any(data::get_provider())))
}

