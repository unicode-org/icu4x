// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This example demonstrates the use of AsciiTrie to look up data based on a region code.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
#![allow(unused_labels)]
#![allow(dead_code)]

icu_benchmark_macros::static_setup!();

use zerotrie::byte_phf::*;

fn print_byte_to_stdout(byte: u8) {
    if let Ok(c) = char::try_from(byte) {
        if c.is_ascii_alphanumeric() {
            print!("'{c}'");
            return;
        }
    }
    print!("0x{byte:X}");
}

fn random_alphanums(seed: u64, len: usize) -> Vec<u8> {
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    const BYTES: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand_pcg::Lcg64Xsh32::seed_from_u64(seed);
    BYTES.choose_multiple(&mut rng, len).copied().collect()
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let mut p_distr = vec![0; 256];
    let mut q_distr = vec![0; 256];
    for len in 0..256 {
        for seed in 0..100 {
            let bytes = random_alphanums(seed, len);
            let (p, qq) = find(bytes.as_slice()).unwrap();
            p_distr[p as usize] += 1;
            for q in qq {
                q_distr[q as usize] += 1;
            }
        }
    }
    println!("p_distr: {p_distr:?}");
    println!("q_distr: {q_distr:?}");

    let bytes = random_alphanums(0, 16);

    #[allow(non_snake_case)]
    let N = bytes.len();

    let (p, qq) = find(bytes.as_slice()).unwrap();

    println!("Results:");
    for byte in bytes.iter() {
        print_byte_to_stdout(*byte);
        let l1 = f1(*byte, p, N);
        let q = qq[l1];
        let l2 = f2(*byte, q, N);
        println!(" => l1 {l1} => q {q} => l2 {l2}");
    }

    0
}
