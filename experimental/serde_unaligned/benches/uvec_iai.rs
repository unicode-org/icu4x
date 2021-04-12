// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;

use serde_unaligned::uvec::*;

fn uvec_sum_u32_slice() -> u32 {
    UVec::from(black_box(TEST_SLICE)).sum()
}

fn uvec_sum_u8_buffer() -> u32 {
    UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
        .unwrap()
        .sum()
}

fn uvec_sum_u32_u32_slice() -> u32 {
    UVec::from(black_box(TEST_SLICE)).sum_u32()
}

fn uvec_sum_u32_u8_buffer() -> u32 {
    UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
        .unwrap()
        .sum_u32()
}

iai::main!(
    uvec_sum_u32_slice,
    uvec_sum_u8_buffer,
    uvec_sum_u32_u32_slice,
    uvec_sum_u32_u8_buffer,
);
