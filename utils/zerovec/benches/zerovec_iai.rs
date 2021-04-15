// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;

use zerovec::samples::*;
use zerovec::ZeroVec;

fn sum_slice() -> u32 {
    black_box(TEST_SLICE).iter().sum::<u32>()
}

fn sum_zerovec() -> u32 {
    ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE))
        .unwrap()
        .iter()
        .sum::<u32>()
}

fn binarysearch_slice() -> Result<usize, usize> {
    black_box(TEST_SLICE).binary_search(&0x0c0d0c)
}

fn binarysearch_zerovec() -> Result<usize, usize> {
    ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE))
        .unwrap()
        .binary_search(&0x0c0d0c)
}

iai::main!(
    sum_slice,
    sum_zerovec,
    binarysearch_slice,
    binarysearch_zerovec,
);
