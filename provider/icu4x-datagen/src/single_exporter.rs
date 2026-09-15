// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_export::fs_exporter::serializers::AbstractSerializer;
use std::io;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) struct SingleExporter {
    sink: Box<dyn io::Write + Sync>,
    serializer: Box<dyn AbstractSerializer + Sync>,
    payload: Mutex<Option<DataPayload<ExportMarker>>>,
    count: AtomicUsize,
}

impl SingleExporter {
    pub(crate) fn new(
        sink: Box<dyn io::Write + Sync>,
        serializer: Box<dyn AbstractSerializer + Sync>,
    ) -> Self {
        Self {
            sink,
            serializer,
            payload: Mutex::new(None),
            count: AtomicUsize::new(0),
        }
    }
}

impl DataExporter for SingleExporter {
    fn put_payload(
        &self,
        _marker: DataMarkerInfo,
        _id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let old = self.count.fetch_add(1, Ordering::SeqCst);
        if old == 0 {
            *self.payload.lock().unwrap() = Some(payload.clone());
        }
        Ok(())
    }

    fn close(&mut self) -> Result<ExporterCloseMetadata, DataError> {
        let count = self.count.load(Ordering::SeqCst);
        if count != 1 {
            return Err(DataErrorKind::Custom.with_str_context(if count == 0 {
                "Expected 1 payload for --format single, but found 0"
            } else {
                "Expected 1 payload for --format single, but found multiple"
            }));
        }
        if let Some(payload) = self.payload.get_mut().unwrap().take() {
            self.serializer.serialize(&payload, &mut self.sink)?;
        }
        Ok(ExporterCloseMetadata::default())
    }
}
