// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This is "export" feature, and there are many internal invariants
#![allow(clippy::expect_used)]

use crate::blob_schema::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;
use writeable::Writeable;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ule::VarULE;
use zerovec::vecs::Index32;
use zerovec::VarZeroVec;
use zerovec::ZeroMap2d;
use zerovec::ZeroVec;

use postcard::ser_flavors::{AllocVec, Flavor};

enum VersionConfig {
    V001,
    V002,
}

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    /// Map of (key hash, locale byte string, blob ID)
    #[allow(clippy::type_complexity)]
    resources: Mutex<BTreeMap<DataKeyHash, BTreeMap<Vec<u8>, usize>>>,
    // All seen keys
    all_keys: Mutex<Vec<DataKeyHash>>,
    /// Map from blob to blob ID
    unique_resources: Mutex<HashMap<Vec<u8>, usize>>,
    sink: Box<dyn std::io::Write + Sync + 'w>,
    version: VersionConfig,
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
            resources: Mutex::new(BTreeMap::new()),
            unique_resources: Mutex::new(HashMap::new()),
            all_keys: Mutex::new(Vec::new()),
            sink,
            version: VersionConfig::V001,
        }
    }

    /// Set the exporter to produce v1 blobs.
    pub fn set_v1(&mut self) {
        self.version = VersionConfig::V001;
    }

    /// Set the exporter to produce v2 blobs.
    pub fn set_v2(&mut self) {
        self.version = VersionConfig::V002;
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
        self.resources
            .lock()
            .expect("poison")
            .entry(key.hashed())
            .or_insert_with(Default::default)
            .entry(locale.write_to_string().into_owned().into_bytes())
            .or_insert(idx);
        Ok(())
    }

    fn flush(&self, key: DataKey) -> Result<(), DataError> {
        self.all_keys.lock().expect("poison").push(key.hashed());
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        match self.version {
            VersionConfig::V001 => self.close_v1(),
            VersionConfig::V002 => self.close_v2(),
        }
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

    fn close_v1(&mut self) -> Result<(), DataError> {
        let FinalizedBuffers { vzv, remap } = self.finalize_buffers();

        // Now build up the ZeroMap2d, changing old ID to new ID
        let mut zm = self
            .resources
            .get_mut()
            .expect("poison")
            .iter()
            .flat_map(|(hash, sub_map)| {
                sub_map
                    .iter()
                    .map(|(locale, old_id)| (*hash, locale, old_id))
            })
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
                zm.insert(key, Index32U8::SENTINEL, &vzv.len());
            }
        }

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

    fn close_v2(&mut self) -> Result<(), DataError> {
        let FinalizedBuffers { vzv, remap } = self.finalize_buffers();

        let keys: ZeroVec<DataKeyHash> = self
            .resources
            .lock()
            .expect("poison")
            .keys()
            .copied()
            .collect();

        let locales_vec: Vec<Vec<u8>> = self
            .resources
            .lock()
            .expect("poison")
            .values()
            .map(|sub_map| {
                let mut sub_map = sub_map.clone();
                sub_map
                    .iter_mut()
                    .for_each(|(_, id)| *id = *remap.get(id).expect("in-bound index"));
                let zerotrie = ZeroTrieSimpleAscii::try_from(&sub_map).expect("in-bounds");
                zerotrie.take_store()
            })
            .collect();

        let locales_vzv: VarZeroVec<[u8]> = locales_vec.as_slice().into();

        if !keys.is_empty() {
            let blob = BlobSchema::V002(BlobSchemaV2 {
                keys: &*keys,
                locales: &*locales_vzv,
                buffers: &vzv,
            });
            log::info!("Serializing blob to output stream...");

            let output = postcard::to_allocvec(&blob)?;
            self.sink.write_all(&output)?;
        }
        Ok(())
    }
}
