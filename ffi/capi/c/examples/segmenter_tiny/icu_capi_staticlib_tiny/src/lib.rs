// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![feature(alloc_error_handler)]
#![no_std]

extern crate icu_capi;

extern crate dlmalloc;

use core::alloc::Layout;
use core::panic::PanicInfo;
use dlmalloc::GlobalDlmalloc;

#[global_allocator]
static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
