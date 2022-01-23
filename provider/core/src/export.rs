// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types having to do with the exporting of data.

use crate::iter::IterableProvider;
use crate::prelude::*;

/// An object capable of serializing data payloads to be read by a [`DataProvider`].
///
/// A [`DataProvider`] by itself is "read-only"; this trait enables it to be "read-write".
pub trait DataExporter<M>
where
    M: DataMarker,
{
    /// Save a `payload` corresponding to the given data request (resource path).
    fn put_payload(&mut self, req: DataRequest, payload: DataPayload<M>) -> Result<(), DataError>;

    /// Function called after a key has been fully dumped into the exporter.
    fn flush(&mut self) -> Result<(), DataError> {
        Ok(())
    }

    /// Function called after all keys have been fully dumped.
    fn close(&mut self) -> Result<(), DataError> {
        Ok(())
    }
}

/// Convenience function to drive a [`DataExporter`] from an [`IterableProvider`].
///
/// # Example
///
/// [`HelloWorldProvider`] implements both [`DataExporter`] and [`IterableProvider`]. The
/// following example copies the data from one instance to another instance.
///
/// ```
/// use icu_provider::hello_world::*;
///
/// let source_provider = HelloWorldProvider::new_with_placeholder_data();
/// let mut dest_provider = HelloWorldProvider::default();
///
/// icu_provider::export::export_from_iterable(
///     &key::HELLO_WORLD_V1,
///     &source_provider,
///     &mut dest_provider,
/// )
/// .expect("Export should be successful");
///
/// assert_eq!(source_provider, dest_provider);
/// ```
///
/// [`HelloWorldProvider`]: crate::hello_world::HelloWorldProvider
pub fn export_from_iterable<P, E, M>(
    resc_key: &ResourceKey,
    provider: &P,
    exporter: &mut E,
) -> Result<(), DataError>
where
    M: DataMarker,
    P: DataProvider<M> + IterableProvider + ?Sized,
    E: DataExporter<M> + ?Sized,
{
    let it = provider.supported_options_for_key(resc_key)?;
    let try_export = || -> Result<(), DataError> {
        for options in it {
            let req = DataRequest {
                resource_path: ResourcePath {
                    key: *resc_key,
                    options,
                },
            };
            let payload = provider.load_payload(&req)?.take_payload()?;
            exporter.put_payload(req, payload)?;
        }
        Ok(())
    };

    // Ensure exporter.flush() is called, even if an error occurred
    let result = try_export();
    exporter.flush()?;
    result
}
