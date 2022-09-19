// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::flexzerovec::{FlexZeroVec, FlexZeroVecOwned};
use crate::ule::AsULE;
use ahash::AHasher;
use alloc::borrow::Borrow;
use alloc::vec::{from_elem, Vec};
use core::hash::{Hash, Hasher};

#[inline]
fn create_hasher_with_seed(seed: u128) -> AHasher {
    AHasher::new_with_keys(seed, 0xaabbccdd)
}

#[inline]
fn compute_hash<K: Hash>(seed: u32, k: K, m: usize) -> usize {
    let mut hasher = create_hasher_with_seed(seed.into());
    k.hash(&mut hasher);
    (hasher.finish() as usize % m) as usize
}

#[derive(Debug)]
pub struct HashIndex<'a> {
    displacements: FlexZeroVec<'a>,
    reverse_mapping: FlexZeroVec<'a>,
}

impl<'a> HashIndex<'a> {
    #[inline]
    pub fn build_from_exact_iter<K, I, A>(keys: I) -> Self
    where
        A: Borrow<K>,
        K: 'a + ?Sized + Hash,
        I: ExactSizeIterator<Item = A>,
    {
        HashIndex::build_from_exact_iter_with_hash_fn(keys, |seed, k, len| {
            compute_hash(seed, k, len)
        })
    }

    #[inline]
    pub fn build_from_exact_iter_with_hash_fn<K, I, A, H>(keys: I, h: H) -> Self
    where
        A: Borrow<K>,
        K: 'a + ?Sized,
        I: ExactSizeIterator<Item = A>,
        H: Fn(u32, &K, usize) -> usize,
    {
        let iter_len = keys.len();

        // A vector to track the size of buckets for sorting.
        let mut bucket_sizes = from_elem(0, iter_len);

        // A flattened representation of items in the buckets after applying first level hash function
        let mut bucket_flatten = Vec::with_capacity(iter_len);

        // Compute initial displacement and bucket sizes
        for (i, k) in keys.enumerate() {
            // Compute first level hash of the key bytes.
            // First level uses a seed value of 0.
            let l1 = h(0x00, k.borrow(), iter_len);
            if let Some(v) = bucket_sizes.get_mut(l1 as usize) {
                *v += 1;
            }
            bucket_flatten.push((l1, (k, i)));
        }

        // Sort by decreasing order of bucket_sizes.
        bucket_flatten.sort_by(|&(ha, _), &(hb, _)| {
            #[allow(clippy::indexing_slicing)] // ha, hb are always within bounds of `bucket_sizes`
            (bucket_sizes[hb as usize], hb).cmp(&(bucket_sizes[ha as usize], ha))
        });

        // Generation count while iterating buckets.
        // Each trial of (seed, bucket chain) is a new generation.
        // We use this to track which all slots are assigned for the current bucket chain.
        let mut generation = 0;

        // Whether a slot has been occupied by previous buckets with a different first level hash (different
        // bucket chain).
        let mut occupied = from_elem(false, iter_len);

        // Track generation count for the slots.
        // A slot is empty if either it is unoccupied by the previous bucket chains and the
        // assignment is not equal to generation.
        let mut assignments = from_elem(0, iter_len);

        // Vec to store the displacements (saves us a recomputation of hash while assigning slots).
        let mut current_displacements = Vec::with_capacity(16);

        // As seed generation starts from 1, 0 can never be a valid seed hence using 0 as sentinal
        // for non-existing first level hashes.
        let mut displacements = from_elem(0, iter_len);

        // Vec to store mapping to the original order of keys.
        // Normally this container should store (K, V) but we instead store the index of (K, V) in
        // the original container.
        let mut reverse_mapping = from_elem(0, iter_len);

        let mut start = 0;
        while start < iter_len {
            // Bucket span with the same first level hash
            #[allow(clippy::indexing_slicing)] // start is always within bounds of `bucket_flatten`
            let l1 = bucket_flatten[start].0 as usize;
            #[allow(clippy::indexing_slicing)] // l1 is always within bounds of `bucket_sizes`
            let end = start + bucket_sizes[l1];
            #[allow(clippy::indexing_slicing)] // start, end are always within bounds of
            // `bucket_sizes`
            let buckets = &bucket_flatten[start..end];

            'seed: for seed in 0x1u32.. {
                current_displacements.clear();
                generation += 1;

                for (_, (k, _)) in buckets {
                    let displacement_idx = h(seed, k.borrow(), iter_len) as usize;
                    #[allow(clippy::indexing_slicing)] // displacement_idx is always within bounds
                    if occupied[displacement_idx] || assignments[displacement_idx] == generation {
                        continue 'seed;
                    }
                    current_displacements.push(displacement_idx);
                    if let Some(v) = assignments.get_mut(displacement_idx) {
                        *v = generation;
                    }
                }

                // Successfully found a seed, store it as index l1.
                if let Some(v) = displacements.get_mut(l1) {
                    *v = seed as usize;
                }

                for (i, displacement_idx) in current_displacements.iter().enumerate() {
                    #[allow(clippy::indexing_slicing)] // `current_displacements` has same size as
                    // `buckets`
                    let (_, (_, original_idx)) = &buckets[i];

                    // Make clippy happy
                    if let Some(v) = occupied.get_mut(*displacement_idx) {
                        *v = true;
                    }
                    if let Some(v) = reverse_mapping.get_mut(*displacement_idx) {
                        *v = *original_idx;
                    }
                }
                break;
            }
            start = end;
        }

        Self {
            displacements: FlexZeroVec::Owned(FlexZeroVecOwned::from_iter(
                displacements.into_iter(),
            )),
            reverse_mapping: FlexZeroVec::Owned(FlexZeroVecOwned::from_iter(
                reverse_mapping.into_iter(),
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
            let hash = compute_hash(seed as u32, k, self.displacements.len());
            self.reverse_mapping.get(hash).map(|i| i as usize)
        }
    }
}

pub struct ZeroHashMapStatic<'a, V>
where
    V: ZeroMapKV<'a> + ?Sized,
{
    index: HashIndex<'a>,
    values: V::Container,
}

impl<'a, V> ZeroHashMapStatic<'a, V>
where
    V: ZeroMapKV<'a> + ?Sized,
{
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, V> ZeroHashMapStatic<'a, V>
where
    V: ZeroMapKV<'a> + ?Sized,
{
    #[inline]
    pub fn get<'b, K>(&'b self, key: &'b K) -> Option<&'b V::GetType>
    where
        K: Hash + AsULE + ?Sized,
    {
        self.index.index(key).map(|i| {
            #[allow(clippy::unwrap_used)] // i is in 0..values.len() and there is a value at i
            self.values.zvl_get(i as usize).unwrap()
        })
    }

    /// Build a [`ZeroHashMapStatic`] from an iterator returning (K, V) tuples.
    ///
    /// # Example
    /// ```
    /// use zerovec::ZeroHashMapStatic;
    ///
    /// let kv: Vec<(i32, &str)> = vec![(1,"a"), (2, "b"),(3, "c"),(4 , "d")];
    /// let hashmap: ZeroHashMapStatic<str> = ZeroHashMapStatic::from_exact_iter(kv.into_iter());
    /// assert_eq!(hashmap.get(&1), Some("a"));
    /// assert_eq!(hashmap.get(&2), Some("b"));
    /// assert_eq!(hashmap.get(&3), Some("c"));
    /// assert_eq!(hashmap.get(&4), Some("d"));
    /// ```
    pub fn from_exact_iter<A, B, I, K>(iter: I) -> Self
    where
        A: Borrow<K>,
        B: Borrow<V>,
        K: Hash + AsULE + ?Sized + 'a,
        I: ExactSizeIterator<Item = (A, B)>,
    {
        let mut keys = Vec::with_capacity(iter.len());
        let mut values = V::Container::zvl_with_capacity(iter.len());
        for (k, v) in iter {
            keys.push(k);
            values.zvl_push(v.borrow());
        }
        let index = HashIndex::build_from_exact_iter::<K, _, _>(keys.into_iter());
        Self { index, values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Standard, Rng, SeedableRng};
    use rand_pcg::Lcg64Xsh32;

    #[test]
    fn test_zhms_u64k_u64v() {
        const N: usize = 1000000;
        let seed = u64::from_le_bytes(*b"testseed");
        let rng = Lcg64Xsh32::seed_from_u64(seed);
        let kv: Vec<(u64, u64)> = rng.sample_iter(&Standard).take(N).collect();
        let kv_copy = kv.clone();
        let hashmap: ZeroHashMapStatic<u64> =
            ZeroHashMapStatic::from_exact_iter(kv_copy.into_iter());
        for (k, v) in kv {
            assert_eq!(
                hashmap.get(&k).copied().map(<u64 as AsULE>::from_unaligned),
                Some(v),
            );
        }
    }
}
