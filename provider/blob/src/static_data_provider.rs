// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use serde::de::Deserialize;

/// A data provider loading data statically baked in to the binary.
///
/// Although static data is convenient and highly portable, it also increases binary size. To
/// load the data files dynamically at runtime, see [`BlobDataProvider`].
///
/// To bake blob data into your binary, use [`include_bytes!`](std::include_bytes), as shown in
/// the example below.
///
/// [`StaticDataProvider`] implements [`BufferProvider`], so it can be used in
/// `*_with_buffer_provider` constructors across ICU4X.
///
/// # Examples
///
/// Load "hello world" data from a postcard blob statically linked at compile time:
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::hello_world::HelloWorldFormatter;
/// use icu_provider_blob::StaticDataProvider;
/// use writeable::assert_writeable_eq;
///
/// // Read an ICU4X data blob statically:
/// const HELLO_WORLD_BLOB: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/hello_world.postcard"
/// ));
///
/// // Create a DataProvider from it:
/// let provider = StaticDataProvider::try_new_from_static_blob(&HELLO_WORLD_BLOB)
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
///     &provider,
///     &locale!("la").into()
/// )
/// .expect("locale exists");
///
/// assert_writeable_eq!(formatter.format(), "Ave, munde");
/// ```
///
/// [`BlobDataProvider`]: crate::BlobDataProvider
#[derive(Clone, Copy)]
pub struct StaticDataProvider {
    data: BlobSchemaV1<'static>,
}

impl StaticDataProvider {
    /// Create a [`StaticDataProvider`] from a `'static` blob of ICU4X data.
    pub fn try_new_from_static_blob(blob: &'static [u8]) -> Result<Self, DataError> {
        Ok(StaticDataProvider {
            data: BlobSchema::deserialize(&mut postcard::Deserializer::from_bytes(blob)).map(
                |blob| {
                    let BlobSchema::V001(blob) = blob;
                    #[cfg(debug_assertions)]
                    blob.check_invariants();
                    blob
                },
            )?,
        })
    }

    /// Creates an empty [`StaticDataProvider`] that contains no data.
    ///
    /// Can be used as a stub for when a real data provider is not available.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_blob::StaticDataProvider;
    ///
    /// let stub_provider = StaticDataProvider::new_empty();
    ///
    /// DataProvider::<HelloWorldV1Marker>::load(
    ///     &stub_provider.as_deserializing(),
    ///     DataRequest {
    ///         locale: &locale!("la").into(),
    ///         metadata: Default::default(),
    ///     },
    /// )
    /// .expect_err("Stub provider returns no data");
    /// ```
    pub fn new_empty() -> Self {
        StaticDataProvider {
            data: Default::default(),
        }
    }
}

impl BufferProvider for StaticDataProvider {
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let mut metadata = DataResponseMetadata::default();
        metadata.buffer_format = Some(BufferFormat::Postcard1);
        let idx = self
            .data
            .keys
            .get0(&key.hashed())
            .ok_or(DataErrorKind::MissingDataKey)
            .and_then(|cursor| {
                cursor
                    .get1_copied_by(|bytes| req.locale.strict_cmp(&bytes.0).reverse())
                    .ok_or(DataErrorKind::MissingLocale)
            })
            .map_err(|kind| kind.with_req(key, req))?;
        let bytes = self
            .data
            .buffers
            .get(idx)
            .ok_or_else(|| DataError::custom("Invalid blob bytes").with_req(key, req))?;
        Ok(DataResponse {
            metadata,
            payload: { Some(DataPayload::from_static_buffer(bytes)) },
        })
    }
}
