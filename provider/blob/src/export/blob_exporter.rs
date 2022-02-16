// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::sync::Mutex;
use writeable::Writeable;
use zerovec::map2d::ZeroMap2d;

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter {
    resources: Mutex<Vec<(ResourceKeyHash, String, Vec<u8>)>>,
    sink: Box<dyn std::io::Write>,
}

// This isn't auto-sync because sink is not Sync. But that's fine, because
// it's only ever used in a &mut method, so there will be no aliases on any
// thread.
unsafe impl Sync for BlobExporter {}

impl BlobExporter {
    /// Create a [`BlobExporter`] that writes to the given I/O stream.
    pub fn new_with_sink(sink: Box<dyn std::io::Write>) -> Self {
        Self {
            resources: Mutex::new(Vec::new()),
            sink,
        }
    }
}

impl DataExporter<SerializeMarker> for BlobExporter {
    fn put_payload(
        &self,
        key: ResourceKey,
        options: ResourceOptions,
        payload: DataPayload<SerializeMarker>,
    ) -> Result<(), DataError> {
        log::trace!("Adding: {}/{}", key, options);
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        payload.serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))?;
        self.resources.lock().unwrap().push((
            key.get_hash(),
            options.writeable_to_string().into_owned(),
            serializer.output.0,
        ));
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        let zm = self
            .resources
            .lock()
            .unwrap()
            .drain(..)
            .collect::<ZeroMap2d<_, _, _>>();
        let blob = BlobSchema::V001(BlobSchemaV1 {
            resources: zm.as_borrowed(),
        });
        log::info!("Serializing blob to output stream...");
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        serde::Serialize::serialize(&blob, &mut serializer)?;
        self.sink.write_all(&serializer.output.0)?;
        Ok(())
    }
}
