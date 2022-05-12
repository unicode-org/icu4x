// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support serializing or deserializing data.
//!
//! ## Deserializing
//!
//! Providers that produce opaque buffers that need to be deserialized into concrete data structs,
//! such as `FsDataProvider`, should implement [`BufferProvider`].
//!
//! [`BufferProvider`] can be made into a trait object. It is used over FFI.
//!
//! ## Serializing
//!
//! Providers that have full type information should implement [`DynProvider`]`<`[`SerializeMarker`]`>`.
//! Note that a provider like `FsDataProvider` cannot implement that trait, because type information
//! on the data structs is required in order to deserialize and then serialize them.
//!
//! [`DynProvider`]`<`[`SerializeMarker`]`>` is used by data exporters such as `FilesystemExporter`.
//!
//! [`DynProvider`]: crate::data_provider::DynProvider
//! [`BufferProvider`]: crate::buf::BufferProvider

pub mod borrow_de_utils;
mod de;

pub use de::AsDeserializingBufferProvider;
pub use de::DeserializingBufferProvider;

#[cfg(feature = "serde_json")]
impl From<serde_json::error::Error> for crate::DataError {
    fn from(e: serde_json::error::Error) -> Self {
        crate::DataError::custom("JSON deserialize").with_display_context(&e)
    }
}

#[cfg(feature = "bincode")]
impl From<bincode::Error> for crate::DataError {
    fn from(e: bincode::Error) -> Self {
        crate::DataError::custom("Bincode deserialize").with_display_context(&e)
    }
}

#[cfg(feature = "postcard")]
impl From<postcard::Error> for crate::DataError {
    fn from(e: postcard::Error) -> Self {
        crate::DataError::custom("Postcard deserialize").with_display_context(&e)
    }
}
