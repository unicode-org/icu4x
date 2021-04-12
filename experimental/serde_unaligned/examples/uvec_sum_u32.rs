#![no_main]

icu_benchmark_macros::static_setup!();

use serde_unaligned::uvec::{UVec, TEST_BUFFER_LE, TEST_SLICE};

#[no_mangle]
fn sum_u32_slice() -> u32 {
    UVec::from(iai::black_box(TEST_SLICE)).sum()
}

#[no_mangle]
fn sum_u8_buffer() -> u32 {
    UVec::<u32>::from_unaligned_le_bytes(iai::black_box(&TEST_BUFFER_LE))
        .unwrap()
        .sum()
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    if sum_u32_slice() == sum_u8_buffer() {
        0
    } else {
        1
    }
}
