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
    UnsupportedCategory(DataCategory),

    /// The data provider supports the category, but not the key (sub-category or version).
    UnsupportedDataKey(DataKey),

    /// The data provider supports the data key, but does not have data for the specific entry
    /// (variant or language identifier).
    UnavailableEntry(DataRequest),

    /// The TypeID of the payload does not match the expected TypeID.
    MismatchedType {
        /// The actual TypeID of the payload.
        actual: TypeId,

        /// The expected TypeID derived from the generic type parameter at the call site.
        generic: Option<TypeId>,
    },

    /// The data provider encountered some other error when loading the resource, such as I/O.
    ResourceError(Box<dyn std::error::Error>),
}

impl From<&DataKey> for Error {
    fn from(data_key: &DataKey) -> Self {
        Error::UnsupportedDataKey(*data_key)
    }
}

impl From<&DataCategory> for Error {
    fn from(category: &DataCategory) -> Self {
        Error::UnsupportedCategory(*category)
    }
}

impl From<DataRequest> for Error {
    fn from(req: DataRequest) -> Self {
        Error::UnavailableEntry(req)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::ResourceError(err)
    }
}

impl Error {
    pub fn new_resc_error<T>(err: T) -> Self
    where
        T: 'static + std::error::Error,
    {
        Error::ResourceError(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnsupportedCategory(category) => write!(f, "Unsupported category: {}", category),
            Error::UnsupportedDataKey(data_key) => write!(f, "Unsupported data key: {}", data_key),
            Error::MismatchedType { actual, generic } => {
                write!(f, "Mismatched type: payload is {:?}", actual)?;
                if let Some(type_id) = generic {
                    write!(f, " (expected from generic type parameter: {:?})", type_id)?;
                }
                Ok(())
            }
            Error::UnavailableEntry(request) => write!(f, "Unavailable data entry: {}", request),
            Error::ResourceError(err) => write!(f, "Failed to load resource: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ResourceError(error) => Some(error.deref()),
            _ => None,
        }
    }
}
