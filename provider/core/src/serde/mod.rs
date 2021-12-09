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
//! Providers that have full type information should implement [`DataProvider`]`<`[`SerializeMarker`]`>`.
//! Note that a provider like `FsDataProvider` cannot implement that trait, because type information
//! on the data structs is required in order to deserialize and then serialize them.
//!
//! [`DataProvider`]`<`[`SerializeMarker`]`>` is used by data exporters such as `FilesystemExporter`.
//!
//! [`DataProvider`]: crate::data_provider::DataProvider
//! [`BufferProvider`]: crate::buffer_provider::BufferProvider

mod de;
#[cfg(feature = "serialize")]
mod ser;

pub use de::Error as DeserializeError;
pub use de::check_format_supported;
pub use de::AsDeserializingBufferProvider;
pub use de::DeserializingBufferProvider;

#[cfg(feature = "serialize")]
pub use ser::Error as SerializeError;
#[cfg(feature = "serialize")]
pub use ser::SerializeBox;
#[cfg(feature = "serialize")]
pub use ser::SerializeMarker;

/// An enum expressing all Serde formats known to ICU4X.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum BufferFormat {
    /// Serialize using JavaScript Object Notation (JSON).
    Json,
    /// Serialize using Bincode version 1.
    Bincode1,
    /// Serialize using Postcard version 0.7.
    Postcard07,
}
