// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub unsafe extern "C" fn icu4x_init() {
    #[cfg(feature = "logging")]
    crate::logging::ffi::ICU4XLogger::init_console_logger();
}
