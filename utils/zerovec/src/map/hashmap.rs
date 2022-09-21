// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{MutableZeroVecLike, ZeroMapKV, ZeroVecLike};
use crate::flexzerovec::{FlexZeroVec, FlexZeroVecOwned};
use ahash::AHasher;
use alloc::borrow::Borrow;
use alloc::vec;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

#[inline]
fn compute_hash<K: Hash>(seed: u32, k: &K, m: usize) -> usize {
    let mut hasher = AHasher::new_with_keys(seed.into(), 0xaabbccdd);
    k.hash(&mut hasher);
    hasher.finish() as usize % m
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct HashIndex<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    displacements: FlexZeroVec<'a>,
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
            // Compute first level hash of the key bytes.
            // First level uses a seed value of 0.
            let l1 = K::Container::zvl_get_as_t(
                // 0 <= i < keys.len()
                keys.zvl_get(i).unwrap(),
                |k| compute_hash(0x00, &k, len),
            );

            // 0 <= l1 < keys.len()
            bucket_sizes[l1] += 1;
            bucket_flatten.push((l1, i));
        }

        // Sort by decreasing order of bucket_sizes.
        bucket_flatten.sort_by(|&(ha, _), &(hb, _)| {
            // ha, hb are always within bounds of `bucket_sizes`
            (bucket_sizes[hb as usize], hb).cmp(&(bucket_sizes[ha as usize], ha))
        });

        // Generation count while iterating buckets.
        // Each trial of (seed, bucket chain) is a new generation.
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

        // As seed generation starts from 1, 0 can never be a valid seed hence using 0 as sentinal
        // for non-existing first level hashes.
        let mut displacements = vec![0; len];

        // Vec to store mapping to the original order of keys.
        // Normally this container should store (K, V) but we instead store the index of (K, V) in
        // the original container.
        let mut reverse_mapping = vec![0; len];

        let mut start = 0;
        while start < len {
            // Bucket span with the same first level hash
            // start is always within bounds of `bucket_flatten`
            let l1 = bucket_flatten[start].0;
            // l1 is always within bounds of `bucket_sizes`
            let end = start + bucket_sizes[l1];
            // start, end - 1 are always within bounds of `bucket_sizes`
            let buckets = &bucket_flatten[start..end];

            'seed: for seed in 0x1u32.. {
                current_displacements.clear();
                generation += 1;

                for (_, i) in buckets {
                    let displacement_idx = K::Container::zvl_get_as_t(
                        //  0 <= i < keys.len()
                        keys.zvl_get(*i).unwrap(),
                        |k| compute_hash(seed, &k, len),
                    );

                    // displacement_idx is always within bounds
                    if occupied[displacement_idx] || assignments[displacement_idx] == generation {
                        continue 'seed;
                    }
                    assignments[displacement_idx] = generation;

                    current_displacements.push(displacement_idx);
                }

                // Successfully found a seed, store it as index l1.
                // l1 < displacements.len() due to modulo operation
                displacements[l1] = seed as usize;

                for (i, displacement_idx) in current_displacements.iter().enumerate() {
                    // `current_displacements` has same size as `buckets`
                    let (_, idx) = &buckets[i];

                    // displacement_idx is always within bounds
                    occupied[*displacement_idx] = true;
                    reverse_mapping[*displacement_idx] = *idx;
                }
                break;
            }
            start = end;
        }

        keys.zvl_permute(&mut reverse_mapping.clone());
        values.zvl_permute(&mut reverse_mapping);

        Self {
            displacements: FlexZeroVec::Owned(FlexZeroVecOwned::from_iter(
                displacements.into_iter(),
            )),
        }
    }

    #[inline]
    pub fn index<K>(&self, k: &K) -> Option<usize>
    where
        K: Hash,
    {
        let l1 = compute_hash(0, k, self.displacements.len());

        #[allow(clippy::unwrap_used)] // l1 is in 0..self.displacements.len()
        let seed = self.displacements.get(l1).unwrap();
        if seed == 0 {
            None
        } else {
            Some(compute_hash(seed as u32, k, self.displacements.len()))
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZeroHashMapStatic<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[cfg_attr(feature = "serde", serde(borrow))]
    index: HashIndex<'a>,
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> ZeroHashMapStatic<'a, K, V>
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

impl<'a, K, V> ZeroHashMapStatic<'a, K, V>
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

    /// Build a [`ZeroHashMapStatic`] from an iterator returning (K, V) tuples.
    ///
    /// # Example
    /// ```
    /// use zerovec::ZeroHashMapStatic;
    ///
    /// let kv: Vec<(i32, &str)> = vec![(1,"a"), (2, "b"),(3, "c"),(4 , "d")];
    /// let hashmap: ZeroHashMapStatic<i32, str> = ZeroHashMapStatic::from_exact_iter(kv.into_iter());
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
        const N: usize = 1000000;
        let seed = u64::from_le_bytes(*b"testseed");
        let rng = Lcg64Xsh32::seed_from_u64(seed);
        let kv: Vec<(u64, u64)> = rng.sample_iter(&Standard).take(N).collect();
        let hashmap: ZeroHashMapStatic<u64, u64> =
            ZeroHashMapStatic::build_from_iter(kv.iter().map(|e| (&e.0, &e.1)));
        for (k, v) in kv {
            assert_eq!(
                hashmap.get(&k).copied().map(<u64 as AsULE>::from_unaligned),
                Some(v),
            );
        }
    }
}
