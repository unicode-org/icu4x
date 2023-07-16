// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::error::Error;
use alloc::vec;
use alloc::vec::Vec;

/// To speed up the search algorithm, we limit the number of times the level-2 parameter (q)
/// can hit its max value of 255 before we try the next level-1 parameter (p). In practice,
/// this has a small impact on the resulting perfect hash, resulting in about 1 in 10000
/// hash maps that fall back to the slow path.
const MAX_L2_SEARCH_MISSES: usize = 24;

/// Directly compute the perfect hash function.
///
/// Returns `(p, [q_0, q_1, ..., q_(N-1)])`, or an error if the PHF could not be computed.
#[allow(unused_labels)] // for readability
pub fn find(bytes: &[u8]) -> Result<(u8, Vec<u8>), Error> {
    #[allow(non_snake_case)]
    let N = bytes.len();

    let mut p = 0u8;
    let mut qq = vec![0u8; N];

    let mut bqs = vec![0u8; N];
    let mut seen = vec![false; N];
    let mut max_allowable_p = P_FAST_MAX;
    let mut max_allowable_q = Q_FAST_MAX;

    'p_loop: loop {
        let mut buckets: Vec<(usize, Vec<u8>)> = (0..N).map(|i| (i, vec![])).collect();
        for byte in bytes {
            buckets[f1(*byte, p, N)].1.push(*byte);
        }
        buckets.sort_by_key(|(_, v)| -(v.len() as isize));
        // println!("New P: p={p:?}, buckets={buckets:?}");
        let mut i = 0;
        let mut num_max_q = 0;
        bqs.fill(0);
        seen.fill(false);
        'q_loop: loop {
            if i == buckets.len() {
                for (local_j, real_j) in buckets.iter().map(|(j, _)| *j).enumerate() {
                    qq[real_j] = bqs[local_j];
                }
                // println!("Success: p={p:?}, num_max_q={num_max_q:?}, bqs={bqs:?}, qq={qq:?}");
                // if num_max_q > 0 {
                //     println!("num_max_q={num_max_q:?}");
                // }
                return Ok((p, qq));
            }
            let mut bucket = buckets[i].1.as_slice();
            'byte_loop: for (j, byte) in bucket.iter().enumerate() {
                if seen[f2(*byte, bqs[i], N)] {
                    // println!("Skipping Q: p={p:?}, i={i:?}, byte={byte:}, q={i:?}, l2={:?}", f2(*byte, bqs[i], N));
                    for k_byte in &bucket[0..j] {
                        assert!(seen[f2(*k_byte, bqs[i], N)]);
                        seen[f2(*k_byte, bqs[i], N)] = false;
                    }
                    'reset_loop: loop {
                        if bqs[i] < max_allowable_q {
                            bqs[i] += 1;
                            continue 'q_loop;
                        }
                        num_max_q += 1;
                        bqs[i] = 0;
                        if i == 0 || num_max_q > MAX_L2_SEARCH_MISSES {
                            if p == max_allowable_p && max_allowable_p != P_REAL_MAX {
                                // println!("Could not solve fast function: trying again: {bytes:?}");
                                max_allowable_p = P_REAL_MAX;
                                max_allowable_q = Q_REAL_MAX;
                                p = 0;
                                continue 'p_loop;
                            } else if p == P_REAL_MAX {
                                // println!("Could not solve PHF function");
                                return Err(Error::CouldNotSolvePerfectHash);
                            } else {
                                p += 1;
                                continue 'p_loop;
                            }
                        }
                        i -= 1;
                        bucket = buckets[i].1.as_slice();
                        for byte in bucket {
                            assert!(seen[f2(*byte, bqs[i], N)]);
                            seen[f2(*byte, bqs[i], N)] = false;
                        }
                    }
                } else {
                    // println!("Marking as seen: i={i:?}, byte={byte:}, l2={:?}", f2(*byte, bqs[i], N));
                    seen[f2(*byte, bqs[i], N)] = true;
                }
            }
            // println!("Found Q: i={i:?}, q={:?}", bqs[i]);
            i += 1;
        }
    }
}

impl PerfectByteHashMap<Vec<u8>> {
    pub fn try_new(keys: &[u8]) -> Result<Self, Error> {
        let n = keys.len();
        let (p, mut qq) = find(keys)?;
        let mut keys_permuted = vec![0; n];
        for key in keys {
            let l1 = f1(*key, p, n);
            let q = qq[l1];
            let l2 = f2(*key, q, n);
            keys_permuted[l2] = *key;
        }
        let mut result = Vec::with_capacity(n * 2 + 1);
        result.push(p);
        result.append(&mut qq);
        result.append(&mut keys_permuted);
        Ok(Self(result))
    }
}
