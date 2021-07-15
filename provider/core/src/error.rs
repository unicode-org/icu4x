// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use std::any::TypeId;
use thiserror::Error;

/// Error enumeration for DataProvider.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// The data provider does not support the resource key.
    #[error("Unsupported resource key: {0}")]
    UnsupportedResourceKey(ResourceKey),

    /// The data provider supports the key, but does not have data for the specific entry.
    #[error("Unavailable resource options: {0}")]
    UnavailableResourceOptions(DataRequest),

    /// The resource was not returned due to a filter. The resource may or may not be available.
    #[error("Resource was filtered: {1}: {0}")]
    FilteredResource(DataRequest, String),

    /// The data provider supports the key, but it requires a language identifier, which was
    /// missing from the request.
    #[error("Requested key needs language identifier in request: {0}")]
    NeedsLanguageIdentifier(DataRequest),

    /// The operation cannot be completed without more type information. For example, data
    /// cannot be deserialized without the concrete type.
    #[error("Complete type information is required")]
    NeedsTypeInfo,

    /// The payload is missing. This error is usually unexpected.
    #[error("Payload is missing")]
    MissingPayload,

    /// The TypeID of the payload does not match the expected TypeID.
    #[error("Mismatched type: payload is {actual:?} (expected from generic type paramenter: {generic:?})")]
    MismatchedType {
        /// The actual TypeID of the payload, if available.
        actual: Option<TypeId>,

        /// The expected TypeID derived from the generic type parameter at the call site.
        generic: Option<TypeId>,
    },

    /// The requested operation failed to unwrap an Rc backing the data payload.
    #[error("Could not unwrap Rc due to multiple references")]
    MultipleReferences,

    /// An error occured during serialization or deserialization.
    #[cfg(feature = "erased-serde")]
    #[error("Serde error: {0}")]
    Serde(#[from] erased_serde::Error),

    /// The data provider encountered some other error when loading the resource, such as I/O.
    #[error("Failed to load resource: {0}")]
    Resource(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn new_resc_error<T>(err: T) -> Self
    where
        T: 'static + std::error::Error + Send + Sync,
    {
        Self::Resource(Box::new(err))
    }
}

impl From<&ResourceKey> for Error {
    fn from(resc_key: &ResourceKey) -> Self {
        Self::UnsupportedResourceKey(*resc_key)
    }
}

impl From<DataRequest> for Error {
    fn from(req: DataRequest) -> Self {
        Self::UnavailableResourceOptions(req)
    }
}
