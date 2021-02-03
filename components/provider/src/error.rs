// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::prelude::*;
use core::ops::Deref;
use std::any::TypeId;
use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The data provider does not support the requested category.
    UnsupportedCategory(ResourceCategory),

    /// The data provider supports the category, but not the key (sub-category or version).
    UnsupportedResourceKey(ResourceKey),

    /// The data provider supports the key, but does not have data for the specific entry.
    UnavailableResourceOptions(DataRequest),

    /// The data provider supports the key, but it requires a language identifier, which was
    /// missing from the request.
    NeedsLanguageIdentifier(DataRequest),

    /// The operation cannot be completed without more type information. For example, data
    /// cannot be deserialized without the concrete type.
    NeedsTypeInfo,

    /// The payload is missing. This error is usually unexpected.
    MissingPayload,

    /// The TypeID of the payload does not match the expected TypeID.
    MismatchedType {
        /// The actual TypeID of the payload, if available.
        actual: Option<TypeId>,

        /// The expected TypeID derived from the generic type parameter at the call site.
        generic: Option<TypeId>,
    },

    /// An error occured during serialization or deserialization.
    #[cfg(feature = "erased-serde")]
    Serde(erased_serde::Error),

    /// The data provider encountered some other error when loading the resource, such as I/O.
    Resource(Box<dyn std::error::Error>),
}

impl From<&ResourceKey> for Error {
    fn from(resc_key: &ResourceKey) -> Self {
        Self::UnsupportedResourceKey(*resc_key)
    }
}

impl From<&ResourceCategory> for Error {
    fn from(category: &ResourceCategory) -> Self {
        Self::UnsupportedCategory(*category)
    }
}

impl From<DataRequest> for Error {
    fn from(req: DataRequest) -> Self {
        Self::UnavailableResourceOptions(req)
    }
}

#[cfg(feature = "erased-serde")]
impl From<erased_serde::Error> for Error {
    fn from(err: erased_serde::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::Resource(err)
    }
}

impl Error {
    pub fn new_resc_error<T>(err: T) -> Self
    where
        T: 'static + std::error::Error,
    {
        Self::Resource(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnsupportedCategory(category) => write!(f, "Unsupported category: {}", category),
            Self::UnsupportedResourceKey(resc_key) => {
                write!(f, "Unsupported resource key: {}", resc_key)
            }
            Self::UnavailableResourceOptions(request) => {
                write!(f, "Unavailable resource options: {}", request)
            }
            Self::NeedsLanguageIdentifier(request) => write!(
                f,
                "Requested key needs language identifier in request: {}",
                request
            ),
            Self::NeedsTypeInfo => write!(f, "Complete type information is required"),
            Self::MissingPayload => write!(f, "Payload is missing"),
            Self::MismatchedType { actual, generic } => {
                write!(f, "Mismatched type: payload is {:?}", actual)?;
                if let Some(type_id) = generic {
                    write!(f, " (expected from generic type parameter: {:?})", type_id)?;
                }
                Ok(())
            }
            #[cfg(feature = "erased-serde")]
            Self::Serde(err) => write!(f, "Serde error: {}", err),
            Self::Resource(err) => write!(f, "Failed to load resource: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resource(error) => Some(error.deref()),
            _ => None,
        }
    }
}
