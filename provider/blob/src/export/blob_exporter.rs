// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use crate::path_util;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerdeSeDataStructMarker;
use litemap::LiteMap;

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

/// TODO(#837): De-duplicate this code from icu_provider_fs.
fn serialize(
    obj: &dyn erased_serde::Serialize,
    mut sink: &mut (impl std::io::Write + ?Sized),
) -> Result<(), DataError> {
    use bincode::Options;
    obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
        &mut bincode::Serializer::new(
            &mut sink,
            bincode::config::DefaultOptions::new().with_fixint_encoding(),
        ),
    ))?;
    Ok(())
}

impl<'d, 's: 'd> DataExporter<'d, 's, SerdeSeDataStructMarker> for BlobExporter<'_> {
    fn put_payload(
        &mut self,
        req: DataRequest,
        obj: DataPayload<'d, 's, SerdeSeDataStructMarker>,
    ) -> Result<(), DataError> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        log::trace!("Adding: {}", path);
        let mut buffer: Vec<u8> = Vec::new();
        serialize(obj.get().as_serialize(), &mut buffer)?;
        self.resources.insert(path, buffer);
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        // Convert from LiteMap<String, Vec> to LiteMap<&str, &[]>
        let mut schema = BlobSchemaV1 {
            resources: LiteMap::with_capacity(self.resources.len()),
        };
        for (k, v) in self.resources.iter() {
            schema
                .resources
                .try_append(k, v)
                .ok_or(())
                .expect_err("Same order");
        }
        let blob = BlobSchema::V001(schema);
        log::info!("Serializing blob to output stream...");
        serialize(&blob, self.sink.as_mut())?;
        self.resources.clear();
        Ok(())
    }
}
