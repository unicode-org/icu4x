// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Write;

use alloc::boxed::Box;
use icu_provider::{marker::DataMarkerPathHash, prelude::*};
use serde::Deserialize;
use writeable::Writeable;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::maps::{ZeroMap2dBorrowed, ZeroMapKV};
use zerovec::vecs::{Index16, Index32, VarZeroSlice, VarZeroVec, VarZeroVecFormat, ZeroSlice};

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub(crate) enum BlobSchema<'data> {
    #[serde(borrow)]
    V001(BlobSchemaV1<'data>),
    #[serde(borrow)]
    V002(BlobSchemaV2<'data, Index16>),
    #[serde(borrow)]
    V002Bigger(BlobSchemaV2<'data, Index32>),
}

// This is a valid separator as `DataLocale` will never produce it.
pub(crate) const REQUEST_SEPARATOR: char = '\x1E';

impl<'data> BlobSchema<'data> {
    pub fn deserialize_and_check<D: serde::Deserializer<'data>>(
        de: D,
    ) -> Result<BlobSchema<'data>, D::Error> {
        let blob = Self::deserialize(de)?;
        #[cfg(debug_assertions)]
        blob.check_invariants();
        Ok(blob)
    }

    pub fn load(&self, marker: DataMarkerInfo, req: DataRequest) -> Result<&'data [u8], DataError> {
        match self {
            BlobSchema::V001(s) => s.load(marker, req),
            BlobSchema::V002(s) => s.load(marker, req),
            BlobSchema::V002Bigger(s) => s.load(marker, req),
        }
    }

    #[cfg(feature = "export")]
    pub fn list_requests(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<std::collections::HashSet<DataIdentifierCow>, DataError> {
        match self {
            BlobSchema::V001(s) => s.list_requests(marker),
            BlobSchema::V002(s) => s.list_requests(marker),
            BlobSchema::V002Bigger(s) => s.list_requests(marker),
        }
    }

    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        match self {
            BlobSchema::V001(s) => s.check_invariants(),
            BlobSchema::V002(s) => s.check_invariants(),
            BlobSchema::V002Bigger(s) => s.check_invariants(),
        }
    }
}

/// Version 1 of the ICU4X data blob schema.
#[derive(Clone, Copy, Debug, serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
pub(crate) struct BlobSchemaV1<'data> {
    /// Map from marker hash and locale to buffer index.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in 0..self.buffers.len()
    #[serde(borrow)]
    pub markers: ZeroMap2dBorrowed<'data, DataMarkerPathHash, Index32U8, usize>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8], Index32>,
}

impl Default for BlobSchemaV1<'_> {
    fn default() -> Self {
        Self {
            markers: ZeroMap2dBorrowed::new(),
            buffers: VarZeroSlice::new_empty(),
        }
    }
}

impl<'data> BlobSchemaV1<'data> {
    pub fn load(&self, marker: DataMarkerInfo, req: DataRequest) -> Result<&'data [u8], DataError> {
        let idx = self
            .markers
            .get0(&marker.path.hashed())
            .ok_or(DataErrorKind::MarkerNotFound)
            .and_then(|cursor| {
                if marker.is_singleton && !req.id.locale.is_empty() {
                    return Err(DataErrorKind::InvalidRequest);
                }
                cursor
                    .get1_copied_by(|k| {
                        struct Comparator<'a>(&'a DataLocale, &'a DataMarkerAttributes);
                        impl writeable::Writeable for Comparator<'_> {
                            fn write_to<W: core::fmt::Write + ?Sized>(
                                &self,
                                sink: &mut W,
                            ) -> core::fmt::Result {
                                self.0.write_to(sink)?;
                                if !self.1.is_empty() {
                                    sink.write_char(REQUEST_SEPARATOR)?;
                                    sink.write_str(self.1)?;
                                }
                                Ok(())
                            }
                        }
                        Comparator(req.id.locale, req.id.marker_attributes)
                            .writeable_cmp_bytes(&k.0)
                            .reverse()
                    })
                    .ok_or(DataErrorKind::IdentifierNotFound)
            })
            .map_err(|kind| kind.with_req(marker, req))?;
        self.buffers
            .get(idx)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(marker, req))
    }

    #[cfg(feature = "export")]
    pub fn list_requests(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<std::collections::HashSet<DataIdentifierCow>, DataError> {
        Ok(self
            .markers
            .get0(&marker.path.hashed())
            .ok_or_else(|| DataErrorKind::MarkerNotFound.with_marker(marker))?
            .iter1_copied()
            .filter_map(|(s, _)| std::str::from_utf8(&s.0).ok())
            .filter_map(|s| {
                if let Some((locale, attrs)) = s.split_once(REQUEST_SEPARATOR) {
                    Some(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_str(attrs).ok()?.to_owned(),
                        locale.parse().ok()?,
                    ))
                } else {
                    Some(DataIdentifierCow::from_locale(s.parse().ok()?))
                }
            })
            .collect())
    }

    /// Verifies the weak invariants using debug assertions
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        if self.markers.is_empty() && self.buffers.is_empty() {
            return;
        }
        // Note: We could check that every index occurs at least once, but that's a more expensive
        // operation, so we will just check for the min and max index.
        let mut seen_min = false;
        let mut seen_max = self.buffers.is_empty();
        for cursor in self.markers.iter0() {
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

/// Version 2 of the ICU4X data blob schema.
///
/// This itself has two modes, using [`Index16`] or [`Index32`] buffers for the locales array.
///
/// The exporter will autoupgrade to the larger buffer as needed.
#[derive(Clone, Copy, Debug, serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[serde(bound = "")] // Override the autogenerated `LocaleVecFormat: Serialize/Deserialize` bound
pub(crate) struct BlobSchemaV2<'data, LocaleVecFormat: VarZeroVecFormat> {
    /// Map from marker hash to locale trie.
    /// Weak invariant: should be sorted.
    #[serde(borrow)]
    pub markers: &'data ZeroSlice<DataMarkerPathHash>,
    /// Map from locale to buffer index.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in 0..self.buffers.len()
    /// Weak invariant: markers and locales are the same length
    // TODO: Make ZeroTrieSimpleAscii<[u8]> work when in this position.
    #[serde(borrow)]
    pub locales: &'data VarZeroSlice<[u8], LocaleVecFormat>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8], Index32>,
}

impl<LocaleVecFormat: VarZeroVecFormat> Default for BlobSchemaV2<'_, LocaleVecFormat> {
    fn default() -> Self {
        Self {
            markers: ZeroSlice::new_empty(),
            locales: VarZeroSlice::new_empty(),
            buffers: VarZeroSlice::new_empty(),
        }
    }
}

impl<'data, LocaleVecFormat: VarZeroVecFormat> BlobSchemaV2<'data, LocaleVecFormat> {
    pub fn load(&self, marker: DataMarkerInfo, req: DataRequest) -> Result<&'data [u8], DataError> {
        let marker_index = self
            .markers
            .binary_search(&marker.path.hashed())
            .ok()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.with_req(marker, req))?;
        if marker.is_singleton && !req.id.locale.is_empty() {
            return Err(DataErrorKind::InvalidRequest.with_req(marker, req));
        }
        let zerotrie = self
            .locales
            .get(marker_index)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(marker, req))?;
        let mut cursor = ZeroTrieSimpleAscii::from_store(zerotrie).into_cursor();
        let _infallible_ascii = req.id.locale.write_to(&mut cursor);
        if !req.id.marker_attributes.is_empty() {
            let _infallible_ascii = cursor.write_char(REQUEST_SEPARATOR);
            req.id
                .marker_attributes
                .write_to(&mut cursor)
                .map_err(|_| DataErrorKind::IdentifierNotFound.with_req(marker, req))?;
        }
        let blob_index = cursor
            .take_value()
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(marker, req))?;
        let buffer = self
            .buffers
            .get(blob_index)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(marker, req))?;
        Ok(buffer)
    }

    #[cfg(feature = "export")]
    pub fn list_requests(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<std::collections::HashSet<DataIdentifierCow>, DataError> {
        let marker_index = self
            .markers
            .binary_search(&marker.path.hashed())
            .ok()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.with_marker(marker))?;
        let zerotrie = self
            .locales
            .get(marker_index)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_marker(marker))?;
        Ok(ZeroTrieSimpleAscii::from_store(zerotrie)
            .iter()
            .filter_map(|(s, _)| {
                if let Some((locale, attrs)) = s.split_once(REQUEST_SEPARATOR) {
                    Some(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_str(attrs).ok()?.to_owned(),
                        locale.parse().ok()?,
                    ))
                } else {
                    Some(DataIdentifierCow::from_locale(s.parse().ok()?))
                }
            })
            .collect())
    }

    /// Verifies the weak invariants using debug assertions
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        if self.markers.is_empty() && self.locales.is_empty() && self.buffers.is_empty() {
            return;
        }
        debug_assert_eq!(self.markers.len(), self.locales.len());
        // Note: We could check that every index occurs at least once, but that's a more expensive
        // operation, so we will just check for the min and max index.
        let mut seen_min = self.buffers.is_empty();
        let mut seen_max = self.buffers.is_empty();
        for zerotrie in self.locales.iter() {
            for (_locale, idx) in ZeroTrieSimpleAscii::from_store(zerotrie).iter() {
                debug_assert!(idx < self.buffers.len());
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
    // Safety: Index32U8 is a transparent DST wrapper around `[u8]`, so a transmute is fine
    #[allow(dead_code)]
    pub(crate) const SENTINEL: &'static Self = unsafe { &*(&[] as *const [u8] as *const Self) };
}
