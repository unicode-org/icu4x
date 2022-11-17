// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a example of how to build a size-optimized WASM library file using ICU4X.

extern crate icu_capi;

mod baked {
    include!("../icu4x_data_skiawasm_bake/mod.rs");
    include!("../icu4x_data_skiawasm_bake/any.rs");
}

#[no_mangle]
pub extern "C" fn skiawasm_get_provider() -> Box<icu_capi::provider::ffi::ICU4XDataProvider> {
    Box::new(icu_capi::provider::ffi::ICU4XDataProvider(icu_capi::provider::ICU4XDataProviderInner::Any(Box::new(baked::BakedDataProvider))))
}

