// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This is "export" feature, and there are many internal invariants
#![allow(clippy::expect_used)]

use crate::blob_schema::*;
use icu_provider::export::*;
use icu_provider::{marker::DataMarkerIdHash, prelude::*};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Mutex;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::vecs::Index32;
use zerovec::vecs::VarZeroVecOwned;
use zerovec::VarZeroVec;
use zerovec::ZeroVec;

use postcard::ser_flavors::{AllocVec, Flavor};

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    /// Map of marker path hash -> locale byte string -> blob ID
    #[allow(clippy::type_complexity)]
    resources: Mutex<BTreeMap<DataMarkerIdHash, BTreeMap<Vec<u8>, usize>>>,
    // All seen markers
    all_markers: Mutex<BTreeSet<DataMarkerIdHash>>,
    /// Map from blob to blob ID
    unique_resources: Mutex<HashMap<Vec<u8>, usize>>,
    sink: Box<dyn std::io::Write + Sync + 'w>,
}

impl core::fmt::Debug for BlobExporter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlobExporter")
            .field("resources", &self.resources)
            .field("unique_resources", &self.unique_resources)
            .field("all_markers", &self.all_markers)
            .field("sink", &"<sink>")
            .finish()
    }
}

impl<'w> BlobExporter<'w> {
    /// Creates a version 1 [`BlobExporter`] that writes to the given I/O stream.
    ///
    /// Version 1 is needed if the blob may be consumed by ICU4X versions 1.0 through 1.3. If
    /// targeting only ICU4X 1.4 and above, see [BlobExporter::new_with_sink()].
    pub fn new_with_sink(sink: Box<dyn std::io::Write + Sync + 'w>) -> Self {
        Self {
            resources: Default::default(),
            unique_resources: Default::default(),
            all_markers: Default::default(),
            sink,
        }
    }
}

impl DataExporter for BlobExporter<'_> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut serializer = postcard::Serializer {
            output: AllocVec::new(),
        };
        payload.serialize(&mut serializer)?;
        let output = serializer
            .output
            .finalize()
            .expect("Failed to finalize serializer output");
        let idx = {
            let mut unique_resources = self.unique_resources.lock().expect("poison");
            let len = unique_resources.len();
            *unique_resources.entry(output).or_insert(len)
        };
        #[allow(clippy::expect_used)]
        self.resources
            .lock()
            .expect("poison")
            .entry(marker.id.hashed())
            .or_default()
            .entry({
                let mut key = id.locale.to_string();
                if !id.marker_attributes.is_empty() {
                    key.push(crate::blob_schema::REQUEST_SEPARATOR);
                    key.push_str(id.marker_attributes);
                }
                key.into_bytes()
            })
            .or_insert(idx);
        Ok(())
    }

    fn flush(&self, marker: DataMarkerInfo, _metadata: FlushMetadata) -> Result<(), DataError> {
        self.all_markers
            .lock()
            .expect("poison")
            .insert(marker.id.hashed());
        Ok(())
    }

    fn close(&mut self) -> Result<ExporterCloseMetadata, DataError> {
        self.close_internal()
    }
}

struct FinalizedBuffers {
    /// Sorted list of blob to old ID; the index in the vec is the new ID
    vzv: VarZeroVec<'static, [u8], Index32>,
    /// Map from old ID to new ID
    remap: HashMap<usize, usize>,
}

impl BlobExporter<'_> {
    fn finalize_buffers(&mut self) -> FinalizedBuffers {
        // The blob IDs are unstable due to the parallel nature of datagen.
        // In order to make a canonical form, we sort them lexicographically now.

        // This is a sorted list of blob to old ID; the index in the vec is the new ID
        let sorted: Vec<(Vec<u8>, usize)> = {
            let mut unique_resources = self.unique_resources.lock().expect("poison");
            let mut sorted: Vec<(Vec<u8>, usize)> = unique_resources.drain().collect();
            sorted.sort();
            sorted
        };

        // This is a map from old ID to new ID
        let remap: HashMap<usize, usize> = sorted
            .iter()
            .enumerate()
            .map(|(new_id, (_, old_id))| (*old_id, new_id))
            .collect();

        // Convert the sorted list to a VarZeroVec
        let vzv: VarZeroVec<[u8], Index32> = {
            let buffers: Vec<Vec<u8>> = sorted.into_iter().map(|(blob, _)| blob).collect();
            buffers.as_slice().into()
        };

        FinalizedBuffers { vzv, remap }
    }

    fn close_internal(&mut self) -> Result<ExporterCloseMetadata, DataError> {
        let FinalizedBuffers { vzv, remap } = self.finalize_buffers();

        let all_markers = self.all_markers.lock().expect("poison");
        let resources = self.resources.lock().expect("poison");

        let markers: ZeroVec<DataMarkerIdHash> = all_markers.iter().copied().collect();

        let locales_vec: Vec<Vec<u8>> = all_markers
            .iter()
            .map(|marker_path_hash| resources.get(marker_path_hash))
            .map(|option_sub_map| {
                if let Some(sub_map) = option_sub_map {
                    let mut sub_map = sub_map.clone();
                    sub_map
                        .iter_mut()
                        .for_each(|(_, id)| *id = *remap.get(id).expect("in-bound index"));
                    let zerotrie = ZeroTrieSimpleAscii::try_from(&sub_map).expect("in-bounds");
                    zerotrie.into_store()
                } else {
                    // Key with no locales: insert an empty ZeroTrie
                    ZeroTrieSimpleAscii::default().into_store()
                }
            })
            .collect();

        if !markers.is_empty() {
            if let Ok(locales_vzv) =
                VarZeroVecOwned::<[u8]>::try_from_elements(locales_vec.as_slice())
            {
                let blob = BlobSchema::V003(BlobSchemaV3 {
                    markers: &markers,
                    locales: &locales_vzv,
                    buffers: &vzv,
                });
                log::info!("Serializing blob to output stream...");

                let output = postcard::to_allocvec(&blob)?;
                self.sink.write_all(&output)?;
            } else {
                log::info!("Upgrading to BlobSchemaV3 (bigger)...");
                let locales_vzv =
                    VarZeroVecOwned::<[u8], Index32>::try_from_elements(locales_vec.as_slice())
                        .expect("Locales vector does not fit in Index32 buffer!");
                let blob = BlobSchema::V003Bigger(BlobSchemaV3 {
                    markers: &markers,
                    locales: &locales_vzv,
                    buffers: &vzv,
                });
                log::info!("Serializing blob to output stream...");

                let output = postcard::to_allocvec(&blob)?;
                self.sink.write_all(&output)?;
            }
        }

        Ok(Default::default())
    }
}
