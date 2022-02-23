// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types having to do with the exporting of data.

use crate::prelude::*;

/// An object capable of exporting data payloads in some form.
pub trait DataExporter<M>: Sync
where
    M: DataMarker,
{
    /// Save a `payload` corresponding to the given key and options.
    /// Takes non-mut self as it can be called concurrently.
    fn put_payload(
        &self,
        key: ResourceKey,
        options: ResourceOptions,
        payload: DataPayload<M>,
    ) -> Result<(), DataError>;

    /// Function called after all keys have been fully dumped.
    /// Takes non-mut self as it can be called concurrently.
    fn flush(&self, _key: ResourceKey) -> Result<(), DataError> {
        Ok(())
    }

    /// This function has to be called before the object is dropped (after all
    /// keys have been fully dumped). This conceptually takes ownership, so
    /// clients *may not* interact with this object after close has been called.
    fn close(&mut self) -> Result<(), DataError> {
        Ok(())
    }
}
