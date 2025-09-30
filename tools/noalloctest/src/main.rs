// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// We have icu4x_noalloctest so that this crate does not trip up our all-crates CI
#![cfg_attr(icu4x_noalloctest, no_std)]
#![cfg_attr(icu4x_noalloctest, no_main)]

use icu_collections as _;
use icu_locale_core as _;
use icu_properties as _;
use icu_provider as _;
use litemap as _;
use potential_utf as _;
use tinystr as _;
use writeable as _;
use yoke as _;
use zerofrom as _;
use zerotrie as _;
use zerovec as _;

#[cfg(icu4x_noalloctest)]
mod real {
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        // don't care
        loop {}
    }

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
        // don't care
        loop {}
    }
}

#[cfg(not(icu4x_noalloctest))]
fn main() {}
