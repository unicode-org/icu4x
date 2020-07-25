use crate::data_entry::DataEntry;
use crate::data_key::Category;
use crate::data_key::DataKey;
use core::ops::Deref;
use std::any::TypeId;
use std::error::Error;
use std::fmt::{self, Debug, Display};

#[non_exhaustive]
#[derive(PartialEq, Debug)]
pub enum PayloadError {
    /// The type argument does not match the payload. The actual TypeId is provided.
    TypeMismatchError(TypeId),
}

impl Display for PayloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: should the Error Display be different from Debug?
        Debug::fmt(self, f)
    }
}

impl Error for PayloadError {}

impl From<TypeId> for PayloadError {
    fn from(type_id: TypeId) -> Self {
        PayloadError::TypeMismatchError(type_id)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ResponseError {
    UnsupportedCategoryError(Category),
    UnsupportedDataKeyError(DataKey),
    InvalidTypeError {
        actual: TypeId,
        expected: Option<TypeId>,
    },
    UnavailableEntryError(DataEntry),
    ResourceError(Box<dyn Error>),
}

impl From<&DataKey> for ResponseError {
    fn from(data_key: &DataKey) -> Self {
        ResponseError::UnsupportedDataKeyError(*data_key)
    }
}

impl From<&Category> for ResponseError {
    fn from(category: &Category) -> Self {
        ResponseError::UnsupportedCategoryError(*category)
    }
}

impl From<DataEntry> for ResponseError {
    fn from(data_entry: DataEntry) -> Self {
        ResponseError::UnavailableEntryError(data_entry)
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: should the Error Display be different from Debug?
        Debug::fmt(self, f)
    }
}

impl Error for ResponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ResponseError::ResourceError(error) => Some(error.deref()),
            _ => None,
        }
    }
}
