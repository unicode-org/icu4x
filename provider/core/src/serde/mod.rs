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
//! Providers that have full type information should implement [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>`.
//! Note that a provider like `FsDataProvider` cannot implement that trait, because type information
//! on the data structs is required in order to deserialize and then serialize them.
//!
//! [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>` is used by data exporters such as `FilesystemExporter`.
//!
//! [`BufferProvider`]: crate::buffer_provider::BufferProvider

mod ser;
mod de;

pub use ser::SerdeSeDataStruct;
pub use ser::SerdeSeDataStructBox;
pub use ser::SerdeSeDataStructMarker;

pub use de::SerdeBufferProvider;
pub use de::AsSerdeBufferProvider;

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

/// An Error type specifically for the [`Deserializer`](serde::Deserializer)
#[derive(displaydoc::Display, Debug)]
pub enum Error {
    /// An error originating in [`serde_json`].
    #[cfg(feature = "provider_json")]
    #[displaydoc("{0}")]
    Json(serde_json::error::Error),

    /// An error originating in [`bincode`].
    #[cfg(feature = "provider_bincode1")]
    #[displaydoc("{0}")]
    Bincode1(bincode::Error),

    /// An error originating in [`postcard`].
    #[cfg(feature = "provider_postcard07")]
    #[displaydoc("{0}")]
    Postcard07(postcard::Error),

    /// An error indicating that the desired buffer format is not available. This usually
    /// means that a required feature was not enabled
    #[allow(dead_code)]
    #[displaydoc("Unavailable buffer format: {0:?} (do you need to enable a feature?)")]
    UnavailableFormat(BufferFormat),

    /// An error indicating that the buffer format could not be deduced. This is usually
    /// unexpected and could indicate a problem with the data pipeline setup.
    #[displaydoc("Buffer format not specified")]
    FormatNotSpecified,
}

#[cfg(feature = "provider_json")]
impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Json(e)
    }
}

#[cfg(feature = "provider_bincode1")]
impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode1(e)
    }
}

#[cfg(feature = "provider_postcard07")]
impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error::Postcard07(e)
    }
}

/// Returns an error if the buffer format is not enabled.
pub fn check_format_supported(buffer_format: BufferFormat) -> Result<(), Error> {
    match buffer_format {
        #[cfg(feature = "provider_json")]
        BufferFormat::Json => Ok(()),

        #[cfg(feature = "provider_bincode1")]
        BufferFormat::Bincode1 => Ok(()),

        #[cfg(feature = "provider_postcard07")]
        BufferFormat::Postcard07 => Ok(()),

        // Allowed for cases in which all features are enabled
        #[allow(unreachable_patterns)]
        _ => Err(Error::UnavailableFormat(buffer_format)),
    }
}
