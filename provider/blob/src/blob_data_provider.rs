// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use alloc::boxed::Box;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use icu_provider::Cart;
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
/// use icu_locid::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_blob::BlobDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Read an ICU4X data blob dynamically:
/// let blob = std::fs::read("tests/data/v2.postcard")
/// .expect("Reading pre-computed postcard buffer");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     &locale!("la").into(),
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
/// use icu_locid::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_blob::BlobDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Read an ICU4X data blob statically:
/// const HELLO_WORLD_BLOB: &[u8] = include_bytes!("../tests/data/v2.postcard");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::try_new_from_static_blob(HELLO_WORLD_BLOB)
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     &locale!("la").into(),
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
    pub fn try_new_from_blob(blob: Box<[u8]>) -> Result<Self, DataError> {
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

    /// For testing purposes only: checks if it is using the V2Bigger format
    #[doc(hidden)]
    pub fn internal_is_using_v2_bigger_format(&self) -> bool {
        matches!(self.data.get(), BlobSchema::V002Bigger(..))
    }
}

impl BufferProvider for BlobDataProvider {
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(BufferFormat::Postcard1);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_yoked_buffer(
                self.data
                    .try_map_project_cloned(|blob, _| blob.load(key, req))?,
            )),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::export::*;
    use icu_provider::datagen::*;
    use icu_provider::hello_world::*;

    #[icu_provider::data_struct(marker(HelloSingletonV1Marker, "hello/singleton@1", singleton))]
    #[derive(Clone, Copy)]
    pub struct HelloSingletonV1;

    #[test]
    fn test_empty() {
        for version in [1, 2] {
            let mut blob: Vec<u8> = Vec::new();

            {
                let mut exporter = if version == 1 {
                    BlobExporter::new_with_sink(Box::new(&mut blob))
                } else {
                    BlobExporter::new_v2_with_sink(Box::new(&mut blob))
                };

                exporter.flush(HelloWorldV1Marker::KEY).unwrap();

                exporter.close().unwrap();
            }

            let provider = BlobDataProvider::try_new_from_blob(blob.into()).unwrap();

            assert!(
                matches!(
                    provider.load_buffer(HelloWorldV1Marker::KEY, Default::default()),
                    Err(DataError {
                        kind: DataErrorKind::MissingLocale,
                        ..
                    })
                ),
                "(version: {version})"
            );
        }
    }

    #[test]
    fn test_singleton() {
        for version in [1, 2] {
            let mut blob: Vec<u8> = Vec::new();

            {
                let mut exporter = if version == 1 {
                    BlobExporter::new_with_sink(Box::new(&mut blob))
                } else {
                    BlobExporter::new_v2_with_sink(Box::new(&mut blob))
                };

                exporter.flush(HelloSingletonV1Marker::KEY).unwrap();

                exporter.close().unwrap();
            }

            let provider = BlobDataProvider::try_new_from_blob(blob.into()).unwrap();

            assert!(
                matches!(
                    provider.load_buffer(
                        HelloSingletonV1Marker::KEY,
                        DataRequest {
                            locale: &icu_locid::langid!("de").into(),
                            metadata: Default::default()
                        }
                    ),
                    Err(DataError {
                        kind: DataErrorKind::ExtraneousLocale,
                        ..
                    })
                ),
                "(version: {version})"
            );

            assert!(
                matches!(
                    provider.load_buffer(HelloSingletonV1Marker::KEY, Default::default()),
                    Err(DataError {
                        kind: DataErrorKind::MissingLocale,
                        ..
                    })
                ),
                "(version: {version})"
            );
        }
    }
}
