// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use icu_provider::Cart;
use icu_provider::DynamicDryDataProvider;
use yoke::*;

/// A data provider that reads from serialized blobs of data.
///
/// This enables data blobs to be read from arbitrary sources at runtime, allowing code and data
/// to be separated. Alternatively, blobs can also be statically included at compile time.
///
/// [`BlobDataProvider`] implements [`BufferProvider`], so it can be used in
/// `*_with_buffer_provider` constructors across ICU4X.
///
/// # `Sync + Send`
///
/// This provider uses reference counting internally. When the `sync` Cargo feature on the [`icu_provider`]
/// crate is enabled, it uses [`Arc`](alloc::sync::Arc) instead of [`Rc`](alloc::rc::Rc), making
/// it `Sync + Send`.
///
/// # Examples
///
/// ## Dynamic loading
///
/// Load "hello world" data from a postcard blob loaded at runtime:
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_blob::BlobDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Read an ICU4X data blob dynamically:
/// let blob = std::fs::read("tests/data/v3.postcard")
///     .expect("Reading pre-computed postcard buffer");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     locale!("la").into(),
/// )
/// .expect("locale exists");
///
/// assert_writeable_eq!(formatter.format(), "Ave, munde");
/// ```
///
/// ## Static loading
///
/// Load "hello world" data from a postcard blob statically linked at compile time:
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_blob::BlobDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Read an ICU4X data blob statically:
/// const HELLO_WORLD_BLOB: &[u8] = include_bytes!("../tests/data/v3.postcard");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::try_new_from_static_blob(HELLO_WORLD_BLOB)
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     locale!("la").into(),
/// )
/// .expect("locale exists");
///
/// assert_writeable_eq!(formatter.format(), "Ave, munde");
/// ```
#[derive(Clone)]
pub struct BlobDataProvider {
    pub(crate) data: Yoke<BlobSchema<'static>, Option<Cart>>,
}

impl core::fmt::Debug for BlobDataProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlobDataProvider")
            .field("data", &"[...]")
            .finish()
    }
}

impl BlobDataProvider {
    /// Create a [`BlobDataProvider`] from a blob of ICU4X data.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    #[cfg(feature = "alloc")]
    pub fn try_new_from_blob(blob: alloc::boxed::Box<[u8]>) -> Result<Self, DataError> {
        Ok(Self {
            data: Cart::try_make_yoke(blob, |bytes| {
                BlobSchema::deserialize_and_check(&mut postcard::Deserializer::from_bytes(bytes))
            })?,
        })
    }

    /// Create a [`BlobDataProvider`] from a static blob. This is a special case of
    /// [`try_new_from_blob`](BlobDataProvider::try_new_from_blob) and is allocation-free.
    pub fn try_new_from_static_blob(blob: &'static [u8]) -> Result<Self, DataError> {
        Ok(Self {
            data: Yoke::new_owned(BlobSchema::deserialize_and_check(
                &mut postcard::Deserializer::from_bytes(blob),
            )?),
        })
    }

    #[doc(hidden)] // for testing purposes only: checks if it is using the Bigger format
    pub fn internal_is_using_bigger_format(&self) -> bool {
        matches!(self.data.get(), BlobSchema::V003Bigger(..))
    }
}

impl DynamicDataProvider<BufferMarker> for BlobDataProvider {
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let payload: Yoke<(&[u8], Option<u64>), Option<Cart>> = self
            .data
            .try_map_project_cloned(|blob, _| blob.load(marker, req))?;
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(BufferFormat::Postcard1);
        metadata.checksum = payload.get().1;
        Ok(DataResponse {
            metadata,
            payload: DataPayload::from_yoked_buffer(payload.map_project(|(bytes, _), _| bytes)),
        })
    }
}

impl DynamicDryDataProvider<BufferMarker> for BlobDataProvider {
    fn dry_load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponseMetadata, DataError> {
        self.data.get().load(marker, req)?;
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(BufferFormat::Postcard1);
        Ok(metadata)
    }
}

/// ✨ *Enabled with the `alloc` Cargo feature.*
#[cfg(feature = "alloc")]
impl IterableDynamicDataProvider<BufferMarker> for BlobDataProvider {
    fn iter_ids_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<alloc::collections::BTreeSet<DataIdentifierCow<'_>>, DataError> {
        self.data.get().iter_ids(marker)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::export::*;
    use icu_provider::export::*;
    use icu_provider::hello_world::*;

    icu_provider::data_marker!(HelloSingletonV1, HelloSingleton, is_singleton = true);
    #[derive(Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
    pub struct HelloSingleton;

    #[test]
    fn test_empty() {
        let mut blob: Vec<u8> = Vec::new();

        {
            let mut exporter = BlobExporter::new_with_sink(Box::new(&mut blob));

            exporter
                .flush(HelloWorldV1::INFO, Default::default())
                .unwrap();

            exporter.close().unwrap();
        }

        let provider = BlobDataProvider::try_new_from_blob(blob.into()).unwrap();

        assert!(
            matches!(
                provider.load_data(HelloWorldV1::INFO, Default::default()),
                Err(DataError {
                    kind: DataErrorKind::IdentifierNotFound,
                    ..
                })
            ),
            "Empty blob test"
        );
    }

    #[test]
    fn test_singleton() {
        let mut blob: Vec<u8> = Vec::new();

        {
            let mut exporter = BlobExporter::new_with_sink(Box::new(&mut blob));

            exporter
                .flush(HelloSingletonV1::INFO, Default::default())
                .unwrap();

            exporter.close().unwrap();
        }

        let provider = BlobDataProvider::try_new_from_blob(blob.into()).unwrap();

        assert!(
            matches!(
                provider.load_data(
                    HelloSingletonV1::INFO,
                    DataRequest {
                        id: DataIdentifierBorrowed::for_locale(
                            &icu_locale_core::langid!("de").into()
                        ),
                        ..Default::default()
                    }
                ),
                Err(DataError {
                    kind: DataErrorKind::InvalidRequest,
                    ..
                })
            ),
            "Singleton blob test"
        );

        assert!(
            matches!(
                provider.load_data(HelloSingletonV1::INFO, Default::default()),
                Err(DataError {
                    kind: DataErrorKind::IdentifierNotFound,
                    ..
                })
            ),
            "Singleton blob test"
        );
    }
}
