// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::map::{MutableZeroVecLike, ZeroMapKV, ZeroVecLike};
use crate::ZeroVec;
use alloc::borrow::Borrow;
use alloc::vec;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use t1ha::T1haHasher;

const SEED: u64 = 0xaabbccdd;
// Split the 64bit hash into (g, f0, f1)
// g == highest 16 bits of h
// f0 == middle 24 bits of h
// f1 == lowest 24 bits of h
#[inline]
const fn split_hash64(hash: u64, m: u32) -> (usize, u32, u32) {
    (
        ((hash >> 48) as u32 % m) as usize,
        (hash >> 24) as u32 & 0xffffff,
        ((hash & 0xffffff) as u32),
    )
}

// Compute (f0 + f1 * d0 + d1) % m
// where
// f0, f1 are 24 bits
#[inline]
fn compute_displacement(f: (u32, u32), d: (u32, u32), m: u32) -> usize {
    (f.1.wrapping_mul(d.0).wrapping_add(f.0).wrapping_add(d.1) % m) as usize
}

#[inline]
fn compute_hash<K: Hash>(k: &K, m: usize) -> (usize, u32, u32) {
    let mut hasher = T1haHasher::with_seed(SEED);
    k.hash(&mut hasher);
    split_hash64(hasher.finish(), m as u32)
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct HashIndex<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    displacements: ZeroVec<'a, (u32, u32)>,
}

impl<'a> HashIndex<'a> {
    /// Build the hashIndex and permute keys, values according to the hash.
    #[inline]
    #[allow(clippy::indexing_slicing, clippy::unwrap_used)] // proper documentation at each occurence
    pub fn build_from_kv_containers<'b, K, V>(
        keys: &mut K::Container,
        values: &mut V::Container,
    ) -> Self
    where
        K: ZeroMapKV<'a> + 'b + ?Sized + Hash,
        V: ZeroMapKV<'a> + ?Sized,
    {
        let len = keys.zvl_len();

        // A vector to track the size of buckets for sorting.
        let mut bucket_sizes = vec![0; len];

        // A flattened representation of items in the buckets after applying first level hash function
        let mut bucket_flatten = Vec::with_capacity(len);

        // Compute initial displacement and bucket sizes
        for i in 0..len {
            // Compute (g, f0, f1) of the key bytes.
            let h = K::Container::zvl_get_as_t(
                // 0 <= i < keys.len()
                keys.zvl_get(i).unwrap(),
                |k| compute_hash(&k, len),
            );

            // 0 <= g < keys.len()
            bucket_sizes[h.0] += 1;
            bucket_flatten.push((h, i));
        }

        // Sort by decreasing order of bucket_sizes.
        bucket_flatten.sort_by(|&(ha, _), &(hb, _)| {
            // ha.0, hb.0 are always within bounds of `bucket_sizes`
            (bucket_sizes[hb.0], hb).cmp(&(bucket_sizes[ha.0], ha))
        });

        // Generation count while iterating buckets.
        // Each trial of ((d0, d1), bucket chain) is a new generation.
        // We use this to track which all slots are assigned for the current bucket chain.
        let mut generation = 0;

        // Whether a slot has been occupied by previous buckets with a different first level hash (different
        // bucket chain).
        let mut occupied = vec![false; len];

        // Track generation count for the slots.
        // A slot is empty if either it is unoccupied by the previous bucket chains and the
        // assignment is not equal to generation.
        let mut assignments = vec![0; len];

        // Vec to store the displacements (saves us a recomputation of hash while assigning slots).
        let mut current_displacements = Vec::with_capacity(16);

        // (d0, d1) which splits the bucket into different slots
        let mut displacements = vec![(0, 0); len];

        // Vec to store mapping to the original order of keys.
        // This is a permutation which will be applied to keys, values at the end.
        let mut reverse_mapping = vec![0; len];

        let mut start = 0;
        while start < len {
            // Bucket span with the same first level hash
            // start is always within bounds of `bucket_flatten`
            let g = bucket_flatten[start].0 .0;
            // g is always within bounds of `bucket_sizes`
            let end = start + bucket_sizes[g];
            // start, end - 1 are always within bounds of `bucket_sizes`
            let buckets = &bucket_flatten[start..end];

            'd0: for d0 in 0..len as u32 {
                'd1: for d1 in 0..len as u32 {
                    current_displacements.clear();
                    generation += 1;

                    for ((_, f0, f1), _) in buckets {
                        let displacement_idx =
                            compute_displacement((*f0, *f1), (d0, d1), len as u32);

                        // displacement_idx is always within bounds
                        if occupied[displacement_idx] || assignments[displacement_idx] == generation
                        {
                            continue 'd1;
                        }
                        assignments[displacement_idx] = generation;

                        current_displacements.push(displacement_idx);
                    }

                    // Successfully found a (d0, d1), store it as index g.
                    // g < displacements.len() due to modulo operation
                    displacements[g] = (d0, d1);

                    for (i, displacement_idx) in current_displacements.iter().enumerate() {
                        // `current_displacements` has same size as `buckets`
                        let (_, idx) = &buckets[i];

                        // displacement_idx is always within bounds
                        occupied[*displacement_idx] = true;
                        reverse_mapping[*displacement_idx] = *idx;
                    }
                    break 'd0;
                }
            }

            start = end;
        }

        keys.zvl_permute(&mut reverse_mapping.clone());
        values.zvl_permute(&mut reverse_mapping);

        Self {
            displacements: ZeroVec::alloc_from_slice(&displacements),
        }
    }

    #[inline]
    pub fn index<K>(&self, k: &K) -> Option<usize>
    where
        K: Hash,
    {
        let (g, f0, f1) = compute_hash(k, self.displacements.len());
        #[allow(clippy::unwrap_used)] // g is in-range
        let (d0, d1) = self.displacements.get(g).unwrap();
        Some(compute_displacement(
            (f0, f1),
            (d0, d1),
            self.displacements.len() as u32,
        ))
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZeroHashMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[cfg_attr(feature = "serde", serde(borrow))]
    index: HashIndex<'a>,
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> ZeroHashMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, K, V> ZeroHashMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized + Hash + Eq,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[inline]
    pub fn get<'b>(&'b self, key: &'b K) -> Option<&'b V::GetType> {
        let i = self.index.index(&key)?;
        #[allow(clippy::unwrap_used)] // i is in 0..self.keys.len()
        let found = self.keys.zvl_get(i).unwrap();
        if K::Container::zvl_get_as_t(found, |found| found == key) {
            self.values.zvl_get(i)
        } else {
            None
        }
    }

    /// Build a [`ZeroHashMap`] from an iterator returning (K, V) tuples.
    ///
    /// # Example
    /// ```
    /// use zerovec::ZeroHashMap;
    ///
    /// let kv: Vec<(i32, &str)> = vec![(1,"a"), (2, "b"),(3, "c"),(4 , "d")];
    /// let hashmap: ZeroHashMap<i32, str> = ZeroHashMap::build_from_iter(kv.into_iter());
    /// assert_eq!(hashmap.get(&1), Some("a"));
    /// assert_eq!(hashmap.get(&2), Some("b"));
    /// assert_eq!(hashmap.get(&3), Some("c"));
    /// assert_eq!(hashmap.get(&4), Some("d"));
    /// ```
    pub fn build_from_iter<A, B, I>(iter: I) -> Self
    where
        A: Borrow<K>,
        B: Borrow<V>,
        I: Iterator<Item = (A, B)>,
    {
        let size_hint = match iter.size_hint() {
            (_, Some(upper)) => upper,
            (lower, None) => lower,
        };

        let mut keys = K::Container::zvl_with_capacity(size_hint);
        let mut values = V::Container::zvl_with_capacity(size_hint);
        for (k, v) in iter {
            keys.zvl_push(k.borrow());
            values.zvl_push(v.borrow());
        }
        let index = HashIndex::build_from_kv_containers::<K, V>(&mut keys, &mut values);
        Self {
            index,
            values,
            keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ule::AsULE;
    use rand::{distributions::Standard, Rng, SeedableRng};
    use rand_pcg::Lcg64Xsh32;

    #[test]
    fn test_zhms_u64k_u64v() {
        const N: usize = 65530;
        let seed = u64::from_le_bytes(*b"testseed");
        let rng = Lcg64Xsh32::seed_from_u64(seed);
        let kv: Vec<(u64, u64)> = rng.sample_iter(&Standard).take(N).collect();
        let hashmap: ZeroHashMap<u64, u64> =
            ZeroHashMap::build_from_iter(kv.iter().map(|e| (&e.0, &e.1)));
        for (k, v) in kv {
            assert_eq!(
                hashmap.get(&k).copied().map(<u64 as AsULE>::from_unaligned),
                Some(v),
            );
        }
    }
}
