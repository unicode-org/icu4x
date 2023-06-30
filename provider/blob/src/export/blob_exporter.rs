// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This is "export" feature, and there are many internal invariants
#![allow(clippy::expect_used)]

use crate::blob_schema::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use writeable::Writeable;
use zerovec::ule::VarULE;
use zerovec::vecs::Index32;
use zerovec::VarZeroVec;
use zerovec::ZeroMap2d;

use postcard::ser_flavors::{AllocVec, Flavor};

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    /// List of (key hash, locale byte string, blob ID)
    #[allow(clippy::type_complexity)]
    resources: Mutex<Vec<(DataKeyHash, Vec<u8>, usize)>>,
    // All seen keys
    all_keys: Mutex<Vec<DataKeyHash>>,
    /// Map from blob to blob ID
    unique_resources: Mutex<HashMap<Vec<u8>, usize>>,
    sink: Box<dyn std::io::Write + Sync + 'w>,
}

impl core::fmt::Debug for BlobExporter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlobExporter")
            .field("resources", &self.resources)
            .field("unique_resources", &self.unique_resources)
            .field("all_keys", &self.all_keys)
            .field("sink", &"<sink>")
            .finish()
    }
}

impl<'w> BlobExporter<'w> {
    /// Create a [`BlobExporter`] that writes to the given I/O stream.
    pub fn new_with_sink(sink: Box<dyn std::io::Write + Sync + 'w>) -> Self {
        Self {
            resources: Mutex::new(Vec::new()),
            unique_resources: Mutex::new(HashMap::new()),
            all_keys: Mutex::new(Vec::new()),
            sink,
        }
    }
}

impl DataExporter for BlobExporter<'_> {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
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
        self.resources.lock().expect("poison").push((
            key.hashed(),
            locale.write_to_string().into_owned().into_bytes(),
            idx,
        ));
        Ok(())
    }

    fn flush(&self, key: DataKey) -> Result<(), DataError> {
        self.all_keys.lock().expect("poison").push(key.hashed());
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
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

        // Now build up the ZeroMap2d, changing old ID to new ID
        let mut zm = self
            .resources
            .get_mut()
            .expect("poison")
            .iter()
            .map(|(hash, locale, old_id)| {
                (
                    hash,
                    Index32U8::parse_byte_slice(locale)
                        .expect("[u8] to IndexU32U8 should never fail"),
                    remap.get(old_id).expect("in-bound index"),
                )
            })
            .collect::<ZeroMap2d<DataKeyHash, Index32U8, usize>>();

        for key in self.all_keys.lock().expect("poison").iter() {
            if zm.get0(key).is_none() {
                zm.insert(key, Index32U8::SENTINEL, &sorted.len());
            }
        }

        // Convert the sorted list to a VarZeroVec
        let vzv: VarZeroVec<[u8], Index32> = {
            let buffers: Vec<Vec<u8>> = sorted.into_iter().map(|(blob, _)| blob).collect();
            buffers.as_slice().into()
        };

        if !zm.is_empty() {
            let blob = BlobSchema::V001(BlobSchemaV1 {
                keys: zm.as_borrowed(),
                buffers: &vzv,
            });
            log::info!("Serializing blob to output stream...");

            let output = postcard::to_allocvec(&blob)?;
            self.sink.write_all(&output)?;
        }
        Ok(())
    }
}
