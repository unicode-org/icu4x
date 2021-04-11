#![feature(test)]
#![no_main]

icu_benchmark_macros::static_setup!();

use serde_unaligned::uvec::{UVec, TEST_BUFFER_LE, TEST_SLICE};

#[no_mangle]
fn sum_u32_slice() -> u32 {
    UVec::from(std::hint::black_box(TEST_SLICE)).sum()
}

#[no_mangle]
fn sum_u8_buffer() -> u32 {
    UVec::<u32>::from_unaligned_le_bytes(std::hint::black_box(&TEST_BUFFER_LE)).sum()
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let mut total = 0;
    for _ in 0..100000000 {
        total += sum_u32_slice();
        total += sum_u8_buffer();
    }

    if total > 0 {
        0
    } else {
        1
    }
}
