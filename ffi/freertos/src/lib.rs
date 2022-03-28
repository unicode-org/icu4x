// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]
#![allow(clippy::upper_case_acronyms)]

#![feature(alloc_error_handler)]

extern crate alloc;

// Necessary to for symbols to be linked in
extern crate icu_capi;

use alloc::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m::asm;
use freertos_rust::FreeRtosAllocator;

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
