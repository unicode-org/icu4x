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
use zerovec::VarZeroVec;
use zerovec::ZeroMap2d;

use postcard::ser_flavors::{AllocVec, Flavor};

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    #[allow(clippy::type_complexity)]
    resources: Mutex<Vec<(ResourceKeyHash, Vec<u8>, usize)>>,
    unique_resources: Mutex<HashMap<Vec<u8>, usize>>,
    sink: Box<dyn std::io::Write + Sync + 'w>,
}

impl<'w> BlobExporter<'w> {
    /// Create a [`BlobExporter`] that writes to the given I/O stream.
    pub fn new_with_sink(sink: Box<dyn std::io::Write + Sync + 'w>) -> Self {
        Self {
            resources: Mutex::new(Vec::new()),
            unique_resources: Mutex::new(HashMap::new()),
            sink,
        }
    }
}

impl DataExporter for BlobExporter<'_> {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: &ResourceOptions,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        log::trace!("Adding: {}/{}", key, options);
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
            key.get_hash(),
            options.write_to_string().into_owned().into_bytes(),
            idx,
        ));
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        let zm = self
            .resources
            .get_mut()
            .expect("poison")
            .drain(..)
            .collect::<ZeroMap2d<_, _, _>>();

        let vzv: VarZeroVec<[u8]> = {
            let mut unique_resources = self.unique_resources.lock().expect("poison");
            let mut buffer_options: Vec<Option<Vec<u8>>> = vec![None; unique_resources.len()];
            for (buffer, idx) in unique_resources.drain() {
                buffer_options
                    .get_mut(idx)
                    .expect("only in-bounds indices in the map")
                    .replace(buffer);
            }
            let buffers: Vec<Vec<u8>> = buffer_options
                .into_iter()
                .map(|opt| opt.expect("all buffers should be present"))
                .collect();
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
