// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::sync::Mutex;
use writeable::Writeable;
use zerovec::ZeroMap2d;

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    #[allow(clippy::type_complexity)]
    resources: Mutex<Vec<(ResourceKeyHash, Vec<u8>, Vec<u8>)>>,
    sink: Box<dyn std::io::Write + Sync + 'w>,
}

impl<'w> BlobExporter<'w> {
    /// Create a [`BlobExporter`] that writes to the given I/O stream.
    pub fn new_with_sink(sink: Box<dyn std::io::Write + Sync + 'w>) -> Self {
        Self {
            resources: Mutex::new(Vec::new()),
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
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        payload.serialize(&mut serializer)?;
        #[allow(clippy::expect_used)]
        self.resources.lock().expect("poison").push((
            key.get_hash(),
            options.write_to_string().into_owned().into_bytes(),
            serializer.output.0,
        ));
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        #[allow(clippy::expect_used)]
        let zm = self
            .resources
            .get_mut()
            .expect("poison")
            .drain(..)
            .collect::<ZeroMap2d<_, _, _>>();

        if !zm.is_empty() {
            let blob = BlobSchema::V001(BlobSchemaV1 {
                resources: zm.as_borrowed(),
            });
            log::info!("Serializing blob to output stream...");
            let mut serializer = postcard::Serializer {
                output: postcard::flavors::AllocVec(Vec::new()),
            };
            serde::Serialize::serialize(&blob, &mut serializer)?;
            self.sink.write_all(&serializer.output.0)?;
        }
        Ok(())
    }
}
