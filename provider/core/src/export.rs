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

    /// Whether to load and dump data for the given entry. This function enables the
    /// [`DataExporter`] to filter out certain data entries.
    fn include_resource_options(&self, resc_options: &ResourceOptions) -> bool;

    /// Auto-implemented function that loads data from an [`IterableDataProvider`] and dumps it
    /// into this [`DataExporter`].
    fn put_key_from_provider(
        &mut self,
        resc_key: &ResourceKey,
        provider: &impl IterableDataProvider<'d, 's, M>,
    ) -> Result<(), Error> {
        for resc_options in provider.supported_options_for_key(resc_key)? {
            if !self.include_resource_options(&resc_options) {
                continue;
            }
            let req = DataRequest {
                resource_path: ResourcePath {
                    key: *resc_key,
                    options: resc_options,
                },
            };
            let payload = provider.load_payload(&req)?.take_payload()?;
            self.put_payload(req, payload)?;
        }
        Ok(())
    }
}
