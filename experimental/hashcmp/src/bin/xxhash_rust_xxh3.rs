// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use std::hash::Hasher;
use std::collections::BTreeSet;

const ALPHANUMS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let seed = std::hint::black_box(2023u64);
    let mut hashes = BTreeSet::new();
    let mut hashes_32bit = BTreeSet::new();
    let mut record = |hashed| {
        hashes.insert(hashed);
        hashes_32bit.insert(hashed as u32);
    };
    let mut cases = 0;
    for i in 0u8..255 {
        let mut hasher = xxhash_rust::xxh3::Xxh3::with_seed(seed);
        hasher.write_u8(i);
        record(hasher.digest128());
        cases += 1;
    }
    for a in ALPHANUMS.iter() {
        for b in ALPHANUMS.iter() {
            for c in ALPHANUMS.iter() {
                let mut hasher = xxhash_rust::xxh3::Xxh3::with_seed(seed);
                hasher.write_u8(*a);
                hasher.write_u8(*b);
                hasher.write_u8(*c);
                record(hasher.digest128());
                cases += 1;
            }
        }
    }
    for start in 0..63 {
        for stride in 1..=63 {
            for count in 4..=63 {
                let mut hasher = xxhash_rust::xxh3::Xxh3::with_seed(seed);
                for i in 0..count {
                    let j = (start + i*stride) % 63;
                    hasher.write_u8(ALPHANUMS[j]);
                }
                record(hasher.digest128());
                cases += 1;
            }
        }
    }
    println!("unique hashes: {} / {} (32-bit: {})", hashes.len(), cases, hashes_32bit.len());
    0
}
