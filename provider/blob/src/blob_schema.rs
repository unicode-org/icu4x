// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Write;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use icu_provider::unstable::DataAttributesRequest;
use icu_provider::{marker::DataMarkerIdHash, prelude::yoke::Yokeable};
use serde::Deserialize;
use writeable::Writeable;
use zerotrie::{ZeroTrieSimpleAscii, cursor::ZeroTrieSimpleAsciiCursor};
use zerovec::vecs::{Index16, Index32, VarZeroSlice, VarZeroVecFormat, ZeroSlice};

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub(crate) enum BlobSchema<'data> {
    V001(NeverSchema),
    V002(NeverSchema),
    V002Bigger(NeverSchema),
    #[serde(borrow)]
    V003(BlobSchemaV1<'data, Index16>),
    #[serde(borrow)]
    V003Bigger(BlobSchemaV1<'data, Index32>),
    #[serde(borrow)]
    V004(BlobSchemaV4<'data, Index16>),
    #[serde(borrow)]
    V004Bigger(BlobSchemaV4<'data, Index32>),
}

#[derive(Clone, Yokeable)]
pub(crate) enum ValidatedBlobSchema<'data> {
    V004(BlobSchemaV4<'data, Index16>),
    V004Bigger(BlobSchemaV4<'data, Index32>),
}

#[derive(Clone, Copy, Debug, Yokeable)]
pub(crate) enum RawBlobKind {
    Postcard1,
    VarULE011,
}

impl RawBlobKind {
    pub(crate) fn into_buffer_format(self) -> BufferFormat {
        match self {
            Self::Postcard1 => BufferFormat::Postcard1,
            Self::VarULE011 => BufferFormat::VarULE011,
        }
    }
}

#[derive(Yokeable)]
pub(crate) struct RawBlob<'data> {
    pub(crate) bytes: &'data [u8],
    pub(crate) checksum: Option<u64>,
    pub(crate) kind: RawBlobKind,
}

// This is a valid separator as `DataLocale` will never produce it.
pub(crate) const REQUEST_SEPARATOR: char = '\x1E';
pub(crate) const CHECKSUM_KEY: &[u8] = b"\0c";

impl<'data> ValidatedBlobSchema<'data> {
    pub fn deserialize_and_check<D: serde::Deserializer<'data>>(
        de: D,
    ) -> Result<ValidatedBlobSchema<'data>, D::Error> {
        let blob = BlobSchema::deserialize(de)?;
        let blob = match blob {
            BlobSchema::V001(..) | BlobSchema::V002(..) | BlobSchema::V002Bigger(..) => {
                unreachable!("Unreachable blob schema")
            }
            BlobSchema::V003(schema) => ValidatedBlobSchema::V004(BlobSchemaV4 {
                markers_postcard1: schema.markers,
                markers_varule011: ZeroSlice::new_empty(),
                locales_postcard1: schema.locales,
                locales_varule011: VarZeroSlice::new_empty(),
                buffers: schema.buffers,
            }),
            BlobSchema::V003Bigger(schema) => ValidatedBlobSchema::V004Bigger(BlobSchemaV4 {
                markers_postcard1: schema.markers,
                markers_varule011: ZeroSlice::new_empty(),
                locales_postcard1: schema.locales,
                locales_varule011: VarZeroSlice::new_empty(),
                buffers: schema.buffers,
            }),
            BlobSchema::V004(schema) => ValidatedBlobSchema::V004(schema),
            BlobSchema::V004Bigger(schema) => ValidatedBlobSchema::V004Bigger(schema),
        };
        #[cfg(debug_assertions)]
        blob.check_invariants();
        Ok(blob)
    }

    pub fn load(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<RawBlob<'data>, DataError> {
        match self {
            Self::V004(s) => s.load(marker, req),
            Self::V004Bigger(s) => s.load(marker, req),
        }
    }

    pub(crate) fn bind_locale(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<(BlobBoundLocaleSchema<'data>, Option<u64>), DataError> {
        match self {
            Self::V004(s) => s.bind_locale(marker, req),
            Self::V004Bigger(s) => s.bind_locale(marker, req),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn iter_ids(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<alloc::collections::BTreeSet<DataIdentifierCow<'_>>, DataError> {
        match self {
            Self::V004(s) => s.iter_ids(marker),
            Self::V004Bigger(s) => s.iter_ids(marker),
        }
    }

    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        match self {
            Self::V004(s) => s.check_invariants(),
            Self::V004Bigger(s) => s.check_invariants(),
        }
    }
}

#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[derive(Debug, Clone, yoke::Yokeable)]
pub enum NeverSchema {}

impl<'de> Deserialize<'de> for NeverSchema {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        Err(D::Error::custom(
            "Attempted to read 1.0 blob format from ICU4X 2.0: please run ICU4X 2.0 datagen to generate a new file.",
        ))
    }
}

/// Version 3 of the ICU4X data blob schema.
///
/// This itself has two modes, using [`Index16`] or [`Index32`] buffers for the locales array.
///
/// The exporter will autoupgrade to the larger buffer as needed.
#[derive(Clone, Copy, Debug, serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[serde(bound = "")] // Override the autogenerated `LocaleVecFormat: Serialize/Deserialize` bound
pub(crate) struct BlobSchemaV1<'data, LocaleVecFormat: VarZeroVecFormat> {
    /// Map from marker hash to locale trie.
    /// Weak invariant: should be sorted.
    #[serde(borrow)]
    pub markers: &'data ZeroSlice<DataMarkerIdHash>,
    /// Map from locale to buffer index.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in `0..self.buffers.len()`
    /// Weak invariant: markers and locales are the same length
    // TODO: Make ZeroTrieSimpleAscii<[u8]> work when in this position.
    #[serde(borrow)]
    pub locales: &'data VarZeroSlice<[u8], LocaleVecFormat>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8], Index32>,
}

/// Version 4 of the ICU4X data blob schema.
///
/// This itself has two modes, using [`Index16`] or [`Index32`] buffers for the locales array.
///
/// The exporter will autoupgrade to the larger buffer as needed.
#[derive(Clone, Copy, Debug, serde::Deserialize, yoke::Yokeable)]
#[yoke(prove_covariance_manually)]
#[cfg_attr(feature = "export", derive(serde::Serialize))]
#[serde(bound = "")] // Override the autogenerated `LocaleVecFormat: Serialize/Deserialize` bound
pub(crate) struct BlobSchemaV4<'data, LocaleVecFormat: VarZeroVecFormat> {
    /// Map from marker hash to locale trie.
    /// Contains markers serialized as postcard.
    /// Weak invariant: should be sorted.
    #[serde(borrow)]
    pub markers_postcard1: &'data ZeroSlice<DataMarkerIdHash>,
    /// Map from marker hash to locale trie.
    /// Contains markers serialized as VarULE.
    /// Same invariants as `markers_postcard1`
    #[serde(borrow)]
    pub markers_varule011: &'data ZeroSlice<DataMarkerIdHash>,
    /// Map from locale to buffer index.
    /// Contains markers serialized as postcard.
    /// Weak invariant: the `usize` values are valid indices into `self.buffers`
    /// Weak invariant: there is at least one value for every integer in `0..self.buffers.len()`
    /// Weak invariant: markers and locales are the same length
    // TODO: Make ZeroTrieSimpleAscii<[u8]> work when in this position.
    #[serde(borrow)]
    pub locales_postcard1: &'data VarZeroSlice<[u8], LocaleVecFormat>,
    /// Map from locale to buffer index.
    /// Contains markers serialized as VarULE.
    /// Same invariants as `locales_postcard1`
    #[serde(borrow)]
    pub locales_varule011: &'data VarZeroSlice<[u8], LocaleVecFormat>,
    /// Vector of buffers
    #[serde(borrow)]
    pub buffers: &'data VarZeroSlice<[u8], Index32>,
}

impl<LocaleVecFormat: VarZeroVecFormat> Default for BlobSchemaV4<'_, LocaleVecFormat> {
    fn default() -> Self {
        Self {
            markers_postcard1: ZeroSlice::new_empty(),
            markers_varule011: ZeroSlice::new_empty(),
            locales_postcard1: VarZeroSlice::new_empty(),
            locales_varule011: VarZeroSlice::new_empty(),
            buffers: VarZeroSlice::new_empty(),
        }
    }
}

fn load_attributes(
    mut cursor: ZeroTrieSimpleAsciiCursor,
    marker_attributes: &DataMarkerAttributes,
    metadata: DataRequestMetadata,
) -> Option<usize> {
    let _infallible_ascii = marker_attributes.write_to(&mut cursor);
    loop {
        let index = cursor.take_value();
        if index.is_some() || !metadata.attributes_prefix_match {
            break index;
        }
        // Match the shortest attribute sharing a prefix.
        cursor.probe(0);
    }
}

impl<'data, LocaleVecFormat: VarZeroVecFormat> BlobSchemaV4<'data, LocaleVecFormat> {
    pub(crate) fn get_trie(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<(ZeroTrieSimpleAscii<&'data [u8]>, RawBlobKind), DataError> {
        // Try to find in the varule table first
        if marker.has_varule {
            if let Ok(marker_index) = self.markers_varule011.binary_search(&marker.id.hashed()) {
                let zerotrie = self
                    .locales_varule011
                    .get(marker_index)
                    .ok_or_else(|| DataError::custom("Invalid blob bytes"))?;
                return Ok((ZeroTrieSimpleAscii::from_store(zerotrie), RawBlobKind::VarULE011));
            }
        }
        let marker_index = self
            .markers_postcard1
            .binary_search(&marker.id.hashed())
            .ok()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())?;
        let zerotrie = self
            .locales_postcard1
            .get(marker_index)
            .ok_or_else(|| DataError::custom("Invalid blob bytes"))?;
        Ok((ZeroTrieSimpleAscii::from_store(zerotrie), RawBlobKind::Postcard1))
    }

    pub(crate) fn load(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<RawBlob<'data>, DataError> {
        if marker.is_singleton && !req.id.locale.is_unknown() {
            return Err(DataErrorKind::InvalidRequest.with_req(marker, req));
        }
        let (zerotrie, kind) = self.get_trie(marker).map_err(|err| err.with_req(marker, req))?;
        let mut cursor = zerotrie.into_cursor();
        let _infallible_ascii = req.id.locale.write_to(&mut cursor);
        let blob_index = if !req.id.marker_attributes.is_empty() {
            let _infallible_ascii = cursor.write_char(REQUEST_SEPARATOR);
            load_attributes(cursor, req.id.marker_attributes, req.metadata)
        } else {
            cursor.take_value()
        }
        .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(marker, req))?;
        let buffer = self
            .buffers
            .get(blob_index)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(marker, req))?;
        Ok(RawBlob {
            bytes: buffer,
            checksum: marker
                .has_checksum
                .then(|| self.get_checksum(zerotrie))
                .flatten(),
            kind,
        })
    }

    pub(crate) fn bind_locale(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<(BlobBoundLocaleSchema<'data>, Option<u64>), DataError> {
        // Note: singleton markers do not make sense with this function
        if marker.is_singleton || req.id.locale.is_unknown() {
            return Err(DataErrorKind::InvalidRequest.with_req(marker, req));
        }
        let (zerotrie, kind) = self.get_trie(marker).map_err(|err| err.with_req(marker, req))?;
        let mut cursor = zerotrie.into_cursor();
        let _infallible_ascii = req.id.locale.write_to(&mut cursor);
        let _infallible_ascii = cursor.write_char(REQUEST_SEPARATOR);
        if cursor.is_empty() {
            return Err(DataErrorKind::IdentifierNotFound.with_req(marker, req));
        }
        Ok((
            BlobBoundLocaleSchema {
                attributes_trie: cursor.into_suffix_trie(),
                buffers: self.buffers,
                kind,
            },
            marker
                .has_checksum
                .then(|| self.get_checksum(zerotrie))
                .flatten(),
        ))
    }

    fn get_checksum(&self, zerotrie: ZeroTrieSimpleAscii<&'data [u8]>) -> Option<u64> {
        zerotrie
            .get(CHECKSUM_KEY)
            .and_then(|cs| Some(u64::from_le_bytes(self.buffers.get(cs)?.try_into().ok()?)))
    }

    #[cfg(feature = "alloc")]
    pub fn iter_ids(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<alloc::collections::BTreeSet<DataIdentifierCow<'_>>, DataError> {
        let (zerotrie, _kind) = self.get_trie(marker).map_err(|err| err.with_marker(marker))?;
        Ok(zerotrie
            .iter()
            .filter_map(|(s, _)| {
                #[allow(unused_imports)]
                use alloc::borrow::ToOwned;
                if let Some((locale, attrs)) = s.split_once(REQUEST_SEPARATOR) {
                    Some(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_str(attrs).ok()?.to_owned(),
                        locale.parse().ok()?,
                    ))
                } else if s.as_bytes() == CHECKSUM_KEY {
                    None
                } else {
                    Some(DataIdentifierCow::from_locale(s.parse().ok()?))
                }
            })
            .collect())
    }

    /// Verifies the weak invariants using debug assertions
    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        if self.markers_postcard1.is_empty() && self.markers_varule011.is_empty() && self.locales_postcard1.is_empty() && self.locales_varule011.is_empty() && self.buffers.is_empty() {
            return;
        }
        debug_assert_eq!(self.markers_postcard1.len(), self.locales_postcard1.len());
        debug_assert_eq!(self.markers_varule011.len(), self.locales_varule011.len());
        // Note: We could check that every index occurs at least once, but that's a more expensive
        // operation, so we will just check for the min and max index.
        let mut seen_min = self.buffers.is_empty();
        let mut seen_max = self.buffers.is_empty();
        for zerotrie in self.locales_postcard1.iter().chain(self.locales_varule011.iter()) {
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

#[derive(Clone, Copy, Debug, yoke::Yokeable)]
pub(crate) struct BlobBoundLocaleSchema<'data> {
    pub(crate) attributes_trie: ZeroTrieSimpleAscii<&'data [u8]>,
    pub(crate) buffers: &'data VarZeroSlice<[u8], Index32>,
    pub(crate) kind: RawBlobKind,
}

impl<'data> BlobBoundLocaleSchema<'data> {
    pub(crate) fn load(&self, req: DataAttributesRequest) -> Result<&'data [u8], DataError> {
        let blob_index = load_attributes(
            self.attributes_trie.cursor(),
            req.marker_attributes,
            req.metadata,
        )
        .ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(req.marker_attributes)
        })?;
        let buffer = self.buffers.get(blob_index).ok_or_else(|| {
            DataError::custom("Invalid blob bytes").with_debug_context(req.marker_attributes)
        })?;
        Ok(buffer)
    }
}
