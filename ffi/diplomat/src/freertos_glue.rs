// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
