#![no_main]

icu_benchmark_macros::static_setup!();

use serde_unaligned::uvec::{UVec, TEST_BUFFER_LE, TEST_SLICE};

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let uvec = UVec::<u32>::from_unaligned_le_bytes(iai::black_box(&TEST_BUFFER_LE)).unwrap();

    if uvec.get(0) == TEST_SLICE.get(0).copied()
        && uvec.get(1) == TEST_SLICE.get(1).copied()
        && uvec.get(2) == TEST_SLICE.get(2).copied()
    {
        0
    } else {
        1
    }
}
