// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use litemap::LiteMap;
use writeable::Writeable;
use zerovec::map2d::ZeroMap2d;

/// A data exporter that writes data to a single-file blob.
/// See the module-level docs for an example.
pub struct BlobExporter<'w> {
    resources: LiteMap<(String, String), Vec<u8>>,
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
        key: ResourceKey,
        req: DataRequest,
        payload: DataPayload<SerializeMarker>,
    ) -> Result<(), DataError> {
        log::trace!("Adding: {}", req.resource_path);
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::AllocVec(Vec::new()),
        };
        payload.serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))?;
        self.resources.insert(
            (
                req.resource_path.key.writeable_to_string().into_owned(),
                req.resource_path.options.writeable_to_string().into_owned(),
            ),
            serializer.output.0,
        );
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        // Convert from LiteMap<(String, String), Vec<u8>> to ZeroMap2d<str, str, [u8]>
        let mut zm: ZeroMap2d<str, str, [u8]> = ZeroMap2d::with_capacity(self.resources.len());
        for ((key, option), bytes) in self.resources.iter() {
            zm.insert(key, option, bytes);
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
