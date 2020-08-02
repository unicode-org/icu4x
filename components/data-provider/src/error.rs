use crate::data_key::Category;
use crate::data_key::DataKey;
use crate::data_provider::Request;
use core::ops::Deref;
use std::any::TypeId;
use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The data provider does not support the requested category.
    UnsupportedCategory(Category),

    /// The data provider supports the category, but not the key (sub-category or version).
    UnsupportedDataKey(DataKey),

    /// The data provider supports the data key, but does not have data for the specific entry
    /// (variant or language identifier).
    UnavailableEntry(Request),

    /// The TypeID of the payload does not match the expected TypeID.
    MismatchedType {
        /// The actual TypeID of the payload.
        actual: TypeId,

        /// The expected TypeID derived from the data key.
        data_key: Option<TypeId>,

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

impl From<&Category> for Error {
    fn from(category: &Category) -> Self {
        Error::UnsupportedCategory(*category)
    }
}

impl From<Request> for Error {
    fn from(req: Request) -> Self {
        Error::UnavailableEntry(req)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::ResourceError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnsupportedCategory(category) => write!(f, "Unsupported category: {}", category),
            Error::UnsupportedDataKey(data_key) => write!(f, "Unsupported data key: {}", data_key),
            Error::MismatchedType {
                actual,
                data_key,
                generic,
            } => {
                write!(f, "Mismatched type: payload is {:?}", actual)?;
                if let Some(type_id) = data_key {
                    write!(f, " (expected from data key: {:?})", type_id)?;
                }
                if let Some(type_id) = generic {
                    write!(f, " (expected from generic type parameter: {:?})", type_id)?;
                }
                Ok(())
            }
            Error::UnavailableEntry(request) => {
                write!(f, "Unavailable data entry: {}", request)
            }
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
