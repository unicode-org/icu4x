// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// In computers, monetary values are sometimes stored as integers representing one ten-thousandth
// (one permyriad) of a monetary unit. FixedDecimal enables a cheap representation of these
// amounts, also while retaining trailing zeros.#![no_std]

// Replacing the allocator and using the `alloc` crate are still unstable.
#![feature(start, core_intrinsics, lang_items, panic_handler, alloc_error_handler)]

// #![no_std]

/////////////////////////////////
// BEGIN WEE_ALLOC BOILERPLATE //
/////////////////////////////////

// Use `wee_alloc` as the global allocator.
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort();
}

// Need to provide an allocation error handler which just aborts
// the execution with trap.
#[alloc_error_handler]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    ::core::intrinsics::abort();
}
*/

///////////////////////////////
// END WEE_ALLOC BOILERPLATE //
///////////////////////////////

use fixed_decimal::FixedDecimal;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    let monetary_int = 19_9500;
    let fixed_decimal = FixedDecimal::default()
    // let fixed_decimal = FixedDecimal::from(monetary_int)
        // .multiplied_pow10(-4)
        // .expect("-4 is well in range")
        // .ok()
        ;

    let mut output = String::with_capacity(fixed_decimal.write_len());
    fixed_decimal
        .write_to(&mut output)
        .expect("Writing to a string is infallible");
        ;
    // assert_eq!("19.9500", output);
}
