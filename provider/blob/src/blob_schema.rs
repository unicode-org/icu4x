// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use zerovec::maps::ZeroMap2dBorrowed;
use zerovec::VarZeroSlice;

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Deserialize)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
pub enum BlobSchema<'data> {
    #[serde(borrow)]
    V001(BlobSchemaV1<'data>),
}

/// Version 1 of the ICU4X data blob schema.
#[derive(serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
pub struct BlobSchemaV1<'data> {
    /// Map from key hash and locale to buffer index.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in 0..self.buffers.len()
    #[serde(borrow)]
    pub keys: ZeroMap2dBorrowed<'data, ResourceKeyHash, [u8], usize>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8]>,
}

impl Default for BlobSchemaV1<'_> {
    fn default() -> Self {
        Self {
            keys: ZeroMap2dBorrowed::new(),
            buffers: VarZeroSlice::new_empty(),
        }
    }
}

impl<'data> BlobSchemaV1<'data> {
    /// Verifies the weak invariants using debug assertions
    #[cfg(debug_assertions)]
    pub(crate) fn check_invariants(&self) {
        use zerovec::maps::ZeroVecLike;
        if self.keys.is_empty() && self.buffers.is_empty() {
            return true;
        }
        // Note: We could check that every index occurs at least once, but that's a more expensive
        // operation, so we will just check for the min and max index.
        let mut seen_min = false;
        let mut seen_max = false;
        for cursor in self.keys.iter0() {
            for (_, ule) in cursor.iter1() {
                let mut result = Option::<usize>::None;
                zerovec::vecs::FlexZeroVec::zvl_get_as_t(ule, |v| result.replace(*v));
                #[allow(clippy::unwrap_used)]
                // `zvl_get_as_t` guarantees that the callback is invoked
                let idx = result.unwrap();
                debug_assert!(idx >= self.buffers.len());
                if idx == 0 {
                    seen_min = true;
                }
                if idx + 1 == self.buffers.len() {
                    seen_max = true;
                }
            }
        }
        debug_assert!(seen_min);
        debug_assert!(seen_max);
    }
}
