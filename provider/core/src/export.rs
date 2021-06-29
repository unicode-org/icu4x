// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types having to do with the exporting of data.

use crate::error::Error;
use crate::iter::IterableDataProvider;
use crate::prelude::*;

/// An object capable of serializing data payloads to be read by a [`DataProvider`].
///
/// A [`DataProvider`] by itself is "read-only"; this trait enables it to be "read-write".
pub trait DataExporter<'d, 's: 'd, M>
where
    M: DataMarker<'s>,
{
    /// Save a `payload` corresponding to the given data request (resource path).
    fn put_payload(
        &mut self,
        req: DataRequest,
        payload: DataPayload<'d, 's, M>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Function called after a key has been fully dumped into the exporter.
    fn flush(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    /// Function called after all keys have been fully dumped.
    fn close(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    /// Auto-implemented function that loads data from an [`IterableDataProvider`] and dumps it
    /// into this [`DataExporter`].
    fn put_key_from_provider(
        &mut self,
        resc_key: &ResourceKey,
        provider: &(impl IterableDataProvider<'d, 's, M> + ?Sized),
    ) -> Result<(), Error> {
        for options in provider.supported_options_for_key(resc_key)? {
            let req = DataRequest {
                resource_path: ResourcePath {
                    key: *resc_key,
                    options,
                },
            };
            let payload = provider.load_payload(&req)?.take_payload()?;
            self.put_payload(req, payload)?;
        }
        Ok(())
    }
}
