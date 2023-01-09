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

// Const seed to be used with [`T1haHasher::with_seed`].
const SEED: u64 = 0xaabbccdd;

/// Split the 64bit hash into (g, f0, f1).
/// g denotes the highest 16bits of the hash modulo m, and is referred to as first level hash.
/// (f0, f1) denotes the middle, and lower 24bits of the hash respectively.
/// (f0, f1) are used to distribute the keys with same g, into distinct slots.
/// m is the length of the container.
#[inline]
const fn split_hash64(hash: u64, m: u32) -> (usize, u32, u32) {
    (
        ((hash >> 48) as u32 % m) as usize,
        (hash >> 24) as u32 & 0xffffff,
        ((hash & 0xffffff) as u32),
    )
}

/// Calculate the displacement using (f0, f1), (d0, d1) in modulo m.
/// The displacement function is (f0 + f1 * d0 + d1) mod m.
#[inline]
fn compute_displacement(f: (u32, u32), d: (u32, u32), m: u32) -> usize {
    (f.1.wrapping_mul(d.0).wrapping_add(f.0).wrapping_add(d.1) % m) as usize
}

/// Compute hash using [`T1haHasher`] with modulo [`m`] and split using [`split_hash64`].
#[inline]
fn compute_hash<K: Hash>(k: &K, m: usize) -> (usize, u32, u32) {
    let mut hasher = T1haHasher::with_seed(SEED);
    k.hash(&mut hasher);
    split_hash64(hasher.finish(), m as u32)
}

/// Denotes an index that maps each of the keys into a distinct slot.
/// A two-level hashing scheme is used to construct this index.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct HashIndex<'a> {
    /// Array of (d0, d1) which splits the keys with same first level hash into distinct
    /// slots.
    /// The ith index of the array splits the keys with first level hash i.
    /// If no key with first level hash is found in the original keys, (0, 0) is used as an empty
    /// placeholder.
    #[cfg_attr(feature = "serde", serde(borrow))]
    displacements: ZeroVec<'a, (u32, u32)>,
}

impl<'a> HashIndex<'a> {
    /// Build a [`HashIndex`] from keys and values.
    /// A mutable keys, values container is required, as the containers are permuted according
    /// to the generated perfect hash.
    /// The steps are to build are.
    /// 1. Compute hash and split into (g, f0, f1).
    /// 2. Bucket and sort the split hash on g in descending order.
    /// 3. In decreasing order of bucket size, try until a (d0, d1) is found that splits the keys
    ///    in the bucket into distinct slots.
    /// 4. Mark the slots for current bucket as occupied and store the reverse mapping.
    /// 5. Repeat untill all the keys have been assigned distinct slots.
    /// 6. Permute the keys, values according to the reverse mapping.
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
                    if (d0, d1) == (0, 0) {
                        continue;
                    }
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

    /// Given a [`key`] return the probable index of the key or [`None`] if the key is guaranteed to be absent.
    /// Another check to determine if the key matches the one at the probable index is required.
    #[inline]
    pub fn index<K>(&self, key: &K) -> Option<usize>
    where
        K: Hash,
    {
        let (g, f0, f1) = compute_hash(key, self.displacements.len());
        #[allow(clippy::unwrap_used)] // g is in-range
        let (d0, d1) = self.displacements.get(g).unwrap();
        if (d0, d1) == (0, 0) {
            return None;
        }
        Some(compute_displacement(
            (f0, f1),
            (d0, d1),
            self.displacements.len() as u32,
        ))
    }
}

/// A perfect zerohashmap built using [`HashIndex`] optimized for lookups over immutable keys.
///
/// # Examples
/// ```
/// use zerovec::ZeroHashMap;
///
/// let kv = vec![(0, "a"), (1, "b"), (2, "c")];
/// let hashmap: ZeroHashMap<i32, str> = ZeroHashMap::build_from_iter(kv.into_iter());
/// assert_eq!(hashmap.get(&0), Some("a"));
/// assert_eq!(hashmap.get(&2), Some("c"));
/// assert_eq!(hashmap.get(&4), None);
/// ```
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
    /// The number of elements in the [`ZeroHashMap`].
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroHashMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, K, V> ZeroHashMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized + Hash + Eq,
    V: ZeroMapKV<'a> + ?Sized,
{
    /// Get the value corresponding to `key`.
    /// If absent `None` is returned.
    ///
    /// # Example
    /// ```
    /// use zerovec::ZeroHashMap;
    ///
    /// let hashmap: ZeroHashMap<str, str> =
    ///     ZeroHashMap::build_from_iter(vec![("a", "A"), ("z", "Z")].into_iter());
    ///
    /// assert_eq!(hashmap.get("a"), Some("A"));
    /// assert_eq!(hashmap.get("z"), Some("Z"));
    /// assert_eq!(hashmap.get("0"), None);
    /// ```
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
