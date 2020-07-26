use crate::data_entry::DataEntry;
use crate::data_key::Category;
use crate::data_key::DataKey;
use core::ops::Deref;
use std::any::TypeId;
use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    UnsupportedCategoryError(Category),
    UnsupportedDataKeyError(DataKey),
    MismatchedType {
        actual: TypeId,
        data_key: Option<TypeId>,
        generic: Option<TypeId>,
    },
    UnavailableEntryError(DataEntry),
    ResourceError(Box<dyn std::error::Error>),
}

impl From<&DataKey> for Error {
    fn from(data_key: &DataKey) -> Self {
        Error::UnsupportedDataKeyError(*data_key)
    }
}

impl From<&Category> for Error {
    fn from(category: &Category) -> Self {
        Error::UnsupportedCategoryError(*category)
    }
}

impl From<DataEntry> for Error {
    fn from(data_entry: DataEntry) -> Self {
        Error::UnavailableEntryError(data_entry)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnsupportedCategoryError(category) => {
                write!(f, "Unsupported category: {}", category)
            }
            Error::UnsupportedDataKeyError(data_key) => {
                write!(f, "Unsupported data key: {}", data_key)
            }
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
            Error::UnavailableEntryError(data_entry) => {
                write!(f, "Unsupported data entry: {}", data_entry)
            }
            Error::ResourceError(err) => write!(f, "Error while loading resource: {}", err),
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
