// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for data providers that produce opaque buffers.

use crate::prelude::*;

/// [`DataMarker`] for raw buffers. Returned by [`BufferProvider`].
///
/// The data is expected to be deserialized before it can be used; see
/// [`DataPayload::into_deserialized`].
#[allow(clippy::exhaustive_structs)] // marker type
pub struct BufferMarker;

impl DataMarker for BufferMarker {
    type Yokeable = &'static [u8];
}

/// A data provider that returns opaque bytes.
///
/// Generally, these bytes are expected to be deserializable with Serde. To get an object
/// implementing [`DataProvider`] via Serde, use [`as_deserializing()`], which requires
/// enabling at least one of the deserialization Cargo features:
///
/// - `deserialize_json`
/// - `deserialize_postcard_1`
/// - `deserialize_bincode_1`
///
/// Along with [`DataProvider`], this is one of the two foundational traits in this crate.
///
/// [`BufferProvider`] can be made into a trait object. It is used over FFI.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "deserialize_json")] {
/// use icu_locid::locale;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
///
/// let buffer_provider = HelloWorldProvider.into_json_provider();
///
/// let data_provider = buffer_provider.as_deserializing();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = data_provider
///     .load(DataRequest {
///         locale: &locale!("de").into(),
///         metadata: Default::default(),
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// # }
/// ```
///
/// [`as_deserializing()`]: AsDeserializingBufferProvider::as_deserializing
pub trait BufferProvider {
    /// Loads a [`DataPayload`]`<`[`BufferMarker`]`>` according to the key and request.
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError>;
}

impl<T: BufferProvider + ?Sized> BufferProvider for alloc::boxed::Box<T> {
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        (**self).load_buffer(key, req)
    }
}

/// An enum expressing all Serde formats known to ICU4X.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum BufferFormat {
    /// Serialize using JavaScript Object Notation (JSON).
    Json,
    /// Serialize using Bincode version 1.
    Bincode1,
    /// Serialize using Postcard version 0.7.
    Postcard1,
}

impl BufferFormat {
    /// Returns an error if the buffer format is not enabled.
    pub fn check_available(&self) -> Result<(), DataError> {
        match self {
            #[cfg(feature = "deserialize_json")]
            BufferFormat::Json => Ok(()),

            #[cfg(feature = "deserialize_bincode_1")]
            BufferFormat::Bincode1 => Ok(()),

            #[cfg(feature = "deserialize_postcard_1")]
            BufferFormat::Postcard1 => Ok(()),

            // Allowed for cases in which all features are enabled
            #[allow(unreachable_patterns)]
            _ => Err(DataErrorKind::UnavailableBufferFormat(*self).into_error()),
        }
    }
}
