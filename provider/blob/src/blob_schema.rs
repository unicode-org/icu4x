// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::boxed::Box;
use icu_provider::prelude::*;
use serde::Deserialize;
use zerovec::maps::{ZeroMap2dBorrowed, ZeroMapKV};
use zerovec::vecs::{Index32, VarZeroSlice, VarZeroVec};

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Deserialize)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[derive(Debug)]
pub(crate) enum BlobSchema<'data> {
    #[serde(borrow)]
    V001(BlobSchemaV1<'data>),
}

impl<'data> BlobSchema<'data> {
    pub fn deserialize_v1<D: serde::Deserializer<'data>>(
        de: D,
    ) -> Result<BlobSchemaV1<'data>, D::Error> {
        let BlobSchema::V001(blob) = Self::deserialize(de)?;
        #[cfg(debug_assertions)]
        blob.check_invariants();
        Ok(blob)
    }
}

/// Version 1 of the ICU4X data blob schema.
#[derive(Clone, Copy, Debug, serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
pub(crate) struct BlobSchemaV1<'data> {
    /// Map from key hash and locale to buffer index.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in 0..self.buffers.len()
    #[serde(borrow)]
    pub keys: ZeroMap2dBorrowed<'data, DataKeyHash, Index32U8, usize>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8], Index32>,
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
    pub fn load(&self, key: DataKey, req: DataRequest) -> Result<&'data [u8], DataError> {
        let idx = self
            .keys
            .get0(&key.hashed())
            .ok_or(DataErrorKind::MissingDataKey)
            .and_then(|cursor| {
                if key.metadata().singleton && !req.locale.is_empty() {
                    return Err(DataErrorKind::ExtraneousLocale);
                }
                cursor
                    .get1_copied_by(|bytes| req.locale.strict_cmp(&bytes.0).reverse())
                    .ok_or(DataErrorKind::MissingLocale)
            })
            .map_err(|kind| kind.with_req(key, req))?;
        self.buffers
            .get(idx)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(key, req))
    }

    /// Verifies the weak invariants using debug assertions
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        if self.keys.is_empty() && self.buffers.is_empty() {
            return;
        }
        // Note: We could check that every index occurs at least once, but that's a more expensive
        // operation, so we will just check for the min and max index.
        let mut seen_min = false;
        let mut seen_max = self.buffers.is_empty();
        for cursor in self.keys.iter0() {
            for (locale, idx) in cursor.iter1_copied() {
                debug_assert!(idx < self.buffers.len() || locale == Index32U8::SENTINEL);
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

/// This type lets us use a u32-index-format VarZeroVec with the ZeroMap2dBorrowed.
///
/// Eventually we will have a FormatSelector type that lets us do `ZeroMap<FormatSelector<K, Index32>, V>`
/// (https://github.com/unicode-org/icu4x/issues/2312)
///
/// IndexU32Borrowed isn't actually important; it's just more convenient to use make_varule to get the
/// full suite of traits instead of `#[derive(VarULE)]`. (With `#[derive(VarULE)]` we would have to manually
/// define a Serialize implementation, and that would be gnarly)
/// https://github.com/unicode-org/icu4x/issues/2310 tracks being able to do this with derive(ULE)
#[zerovec::make_varule(Index32U8)]
#[zerovec::derive(Debug)]
#[zerovec::skip_derive(ZeroMapKV)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, serde::Deserialize)]
#[zerovec::derive(Deserialize)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[cfg_attr(feature = "export", zerovec::derive(Serialize))]
pub(crate) struct Index32U8Borrowed<'a>(
    #[cfg_attr(feature = "export", serde(borrow))] pub &'a [u8],
);

impl<'a> ZeroMapKV<'a> for Index32U8 {
    type Container = VarZeroVec<'a, Index32U8, Index32>;
    type Slice = VarZeroSlice<Index32U8, Index32>;
    type GetType = Index32U8;
    type OwnedType = Box<Index32U8>;
}

impl Index32U8 {
    #[allow(dead_code)]
    pub(crate) const SENTINEL: &'static Self = unsafe { &*(&[] as *const [u8] as *const Self) };
}
