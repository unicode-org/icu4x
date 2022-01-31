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
//! [`BufferProvider`]: crate::buf::BufferProvider

mod de;
#[cfg(feature = "serialize")]
mod ser;
pub mod borrow_de_utils;

pub use de::check_format_supported;
pub use de::AsDeserializingBufferProvider;
pub use de::DeserializingBufferProvider;

#[cfg(feature = "serialize")]
pub use ser::SerializeBox;
#[cfg(feature = "serialize")]
pub use ser::SerializeMarker;

use crate::buf::BufferFormat;

/// Error type for deserialization.
#[derive(displaydoc::Display, Debug)]
pub enum Error {
    /// An error originating in [`serde_json`].
    #[cfg(feature = "deserialize_json")]
    #[displaydoc("{0}")]
    Json(serde_json::error::Error),

    /// An error originating in [`bincode`].
    #[cfg(feature = "deserialize_bincode_1")]
    #[displaydoc("{0}")]
    Bincode1(bincode::Error),

    /// An error originating in [`postcard`].
    #[cfg(feature = "deserialize_postcard_07")]
    #[displaydoc("{0}")]
    Postcard07(postcard::Error),

    /// An error indicating that the desired buffer format is not available. This usually
    /// means that a required feature was not enabled
    #[allow(dead_code)]
    #[displaydoc("Unavailable buffer format: {0:?} (does icu4x need to be compiled with an additional feature?)")]
    UnavailableFormat(BufferFormat),

    /// An error originating in [`erased_serde`].
    #[cfg(feature = "erased-serde")]
    #[displaydoc("{0}")]
    Serde(erased_serde::Error),

    /// An error indicating that the buffer format could not be deduced. This is usually
    /// unexpected and could indicate a problem with the data pipeline setup.
    #[displaydoc("Buffer format not specified")]
    FormatNotSpecified,
}

#[cfg(feature = "deserialize_json")]
impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Json(e)
    }
}

#[cfg(feature = "deserialize_bincode_1")]
impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode1(e)
    }
}

#[cfg(feature = "deserialize_postcard_07")]
impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error::Postcard07(e)
    }
}

#[cfg(feature = "erased-serde")]
impl From<erased_serde::Error> for Error {
    fn from(e: erased_serde::Error) -> Self {
        Error::Serde(e)
    }
}
