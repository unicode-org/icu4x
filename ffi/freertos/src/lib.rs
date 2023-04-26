// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This crate is a shim that enables one to use icu4x on Cortex-M + FreeRTOS by setting up the
//! relevant Rust runtime hooks.
//!
//! Note that compiling to this platform needs Rust nightly, and this crate attempts to
//! build across multiple nightly versions.
//!
//! This crate has a build script that will attempt to detect the nightly version and configure
//! things appropriately, where possible. Older nightlies will end up setting the
//! `--cfg needs_alloc_error_handler` flag: if using a custom build system and a nightly from
//! 2022 or earlier, please set this flag.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![allow(clippy::upper_case_acronyms)]
#![cfg_attr(
    all(target_os = "none", needs_alloc_error_handler),
    feature(alloc_error_handler)
)]

// Necessary to for symbols to be linked in
extern crate icu_capi;

// CFG it off so that it doesn't break the --all-features build due to needing unstable rust
#[cfg(target_os = "none")]
mod stuff {
    extern crate alloc;

    use core::panic::PanicInfo;
    use freertos_rust::FreeRtosAllocator;

    #[global_allocator]
    static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

    #[cfg(needs_alloc_error_handler)] // this defaults to the panic handler on newer nightlies
    #[alloc_error_handler]
    fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
        cortex_m::asm::bkpt();
        loop {}
    }

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

// Needed for rust runtime stuff
//
// renamed so you can't accidentally use it
#[cfg(not(target_os = "none"))]
extern crate std as rust_std;
