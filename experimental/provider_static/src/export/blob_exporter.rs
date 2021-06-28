// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerdeSeDataStructMarker;
use crate::path_util;

pub struct BlobExporter {
    resources: LiteMap<String, Vec<u8>>,
}

impl<'d, 's: 'd> DataExporter<'d, 's, SerdeSeDataStructMarker> for BlobExporter {
    fn put_payload(
        &mut self,
        req: DataRequest,
        obj: DataPayload<'d, 's, SerdeSeDataStructMarker>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        log::trace!("Adding to blob: {}", req);
        let serialized = bincode::serialize(obj.get())?;
        self.resources.put(path, serialized);
        Ok(())
    }

    fn include_resource_options(&self, resc_options: &ResourceOptions) -> bool {
        // TODO(#830): Support locale filtering in BlobExporter
        // Depends on #831: Make Locale types compatible with [Var]ZeroVec
        true
    }
}
