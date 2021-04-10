#![feature(test)]
#![no_main]

icu_benchmark_macros::static_setup!();

use serde_unaligned::uvec::{UVec, TEST_BUFFER_LE};

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let uvec = UVec::<u32>::from_unaligned_le_bytes(std::hint::black_box(&TEST_BUFFER_LE));

    if uvec.get(0) == Some(5) && uvec.get(1) == Some(5) && uvec.get(2) == Some(5) {
        0
    } else {
        1
    }
}
