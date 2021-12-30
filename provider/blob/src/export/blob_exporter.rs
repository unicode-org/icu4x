// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use crate::path_util;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use litemap::LiteMap;
use zerovec::ZeroMap;

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    resources: LiteMap<String, Vec<u8>>,
    sink: Box<dyn std::io::Write + 'w>,
}

impl<'w> BlobExporter<'w> {
    /// Create a [`BlobExporter`] that writes to the given I/O stream.
    pub fn new_with_sink(sink: Box<dyn std::io::Write + 'w>) -> Self {
        Self {
            resources: LiteMap::new(),
            sink,
        }
    }
}

impl Drop for BlobExporter<'_> {
    fn drop(&mut self) {
        if !self.resources.is_empty() {
            panic!("Please call close before dropping FilesystemExporter");
        }
    }
}

impl DataExporter<SerializeMarker> for BlobExporter<'_> {
    fn put_payload(
        &mut self,
        req: DataRequest,
        payload: DataPayload<SerializeMarker>,
    ) -> Result<(), DataError> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        log::trace!("Adding: {}", path);
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        payload.serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))?;
        self.resources.insert(path, serializer.output.0);
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        // Convert from LiteMap<String, Vec<u8>> to ZeroVecBorrowed<&str, &[u8]>
        let mut zm: ZeroMap<str, [u8]> = ZeroMap::with_capacity(self.resources.len());
        for (k, v) in self.resources.iter() {
            zm.try_append(k, v).ok_or(()).expect_err("Same order");
        }
        let blob = BlobSchema::V001(BlobSchemaV1 {
            resources: zm.as_borrowed(),
        });
        log::info!("Serializing blob to output stream...");
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        serde::Serialize::serialize(&blob, &mut serializer)?;
        self.sink.write_all(&serializer.output.0)?;
        self.resources.clear();
        Ok(())
    }
}
