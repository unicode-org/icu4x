#![no_std]

extern crate no_std_compat as std;

pub mod decimal;
pub mod plurals;

mod helpers;

use core::ops::Deref;
use helpers::CloneableAny;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::prelude::v1::*;

use tinystr::TinyStr16;

use icu_locale::LanguageIdentifier;

/// A top-level collection of related data keys.
#[non_exhaustive]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Category {
    Decimal,
    Plurals,
    PrivateUse(TinyStr16),
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Decimal => f.write_str("decimal")?,
            Category::Plurals => f.write_str("plurals")?,
            Category::PrivateUse(id) => {
                f.write_str("x-")?;
                f.write_str(id)?;
            }
        }
        return Ok(());
    }
}

/// A category, subcategory, and version, used for requesting data from a DataProvider.
///
/// All of the fields in a DataKey should be resolved at build time.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct DataKey {
    pub category: Category,
    pub sub_category: TinyStr16,
    pub version: u32,
}

impl Display for DataKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Evaluate the code size of this implementation
        write!(
            f,
            "{}/{}/v{}",
            self.category, self.sub_category, self.version
        )
    }
}

impl DataKey {
    pub fn get_type_id(&self) -> Option<TypeId> {
        match self.category {
            Category::Decimal => decimal::get_type_id(self),
            Category::Plurals => plurals::get_type_id(self),
            Category::PrivateUse(_) => None,
        }
    }
}

/// A variant and language identifier, used for requesting data from a DataProvider.
/// 
/// All of the fields in a DataEntry should be resolved at runtime.
#[derive(PartialEq, Clone, Debug)]
pub struct DataEntry {
    // TODO: Consider making this a list of variants
    pub variant: Option<Cow<'static, str>>,
    pub langid: LanguageIdentifier,
}

/// A struct to request a certain hunk of data from a data provider.
#[derive(PartialEq, Clone, Debug)]
pub struct Request {
    pub data_key: DataKey,
    pub data_entry: DataEntry,
}

/// A response object containing a data hunk ("payload").
#[derive(Debug, Clone)]
pub struct Response<'d> {
    pub data_langid: LanguageIdentifier,
    payload: Cow<'d, dyn CloneableAny>,
    // source: Cow<'static, str>,
}

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

// TODO: Should this be an implemention of std::borrow::Borrow?
// TODO: Should the error types be &dyn Any, like for Box<dyn Any>::downcast?
impl<'d> Response<'d> {
    /// Get an immutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload<T: 'static>(&self) -> Result<&T, PayloadError> {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed
            .as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| PayloadError::from(borrowed.as_any().type_id()))
    }

    /// Get a mutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload_mut<T: 'static>(&mut self) -> Result<&mut T, PayloadError> {
        let borrowed_mut: &mut dyn CloneableAny = self.payload.to_mut().borrow_mut();
        // TODO: If I move this into the lambda, I get E0502. Why?
        let type_id = borrowed_mut.as_any().type_id();
        borrowed_mut
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or_else(|| PayloadError::from(type_id))
    }

    /// Take ownership of the payload from a Response object. Consumes the Response object.
    pub fn take_payload<T: 'static + Clone>(self) -> Result<Cow<'d, T>, PayloadError> {
        match self.payload {
            Cow::Borrowed(borrowed) => match borrowed.as_any().downcast_ref::<T>() {
                Some(v) => Ok(Cow::Borrowed(v)),
                None => Err(PayloadError::from(borrowed.as_any().type_id())),
            },
            Cow::Owned(boxed) => match boxed.into_any().downcast::<T>() {
                Ok(boxed_t) => Ok(Cow::Owned(*boxed_t)),
                Err(boxed_any) => Err(PayloadError::from(boxed_any.type_id())),
            },
        }
    }

    /// Get the TypeId of the payload.
    pub fn get_payload_type_id(&self) -> TypeId {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed.as_any().type_id()
    }
}

/// Builder class used to construct a Response.
pub struct ResponseBuilder {
    pub data_langid: LanguageIdentifier,
}

impl ResponseBuilder {
    /// Construct a Response from the builder, with owned data.
    /// Consumes both the builder and the data.
    /// Returns the 'static lifetime since there is no borrowed data.
    pub fn with_owned_payload<T: 'static + Clone + Debug>(self, t: T) -> Response<'static> {
        Response {
            data_langid: self.data_langid,
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    /// Construct a Response from the builder, with borrowed data.
    /// Consumes the builder, but not the data.
    #[allow(clippy::needless_lifetimes)]
    pub fn with_borrowed_payload<'d, T: 'static + Clone + Debug>(self, t: &'d T) -> Response<'d> {
        Response {
            data_langid: self.data_langid,
            payload: Cow::Borrowed(t),
        }
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
    UnavailableEntryError,
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

/// An abstract data provider that takes a request object and returns a response with a payload.
/// Lifetimes:
/// - 'a = lifetime of the DataProvider object
/// - 'd = lifetime of the borrowed payload
/// Note: 'd and 'a can be the same, but they do not need to be. For example, 'd = 'static if:
/// 1. The provider always returns data that lives in static memory
/// 2. The provider always returns owned data, not borrowed data
// TODO: Make this async
// #[async_trait]
pub trait DataProvider<'a, 'd> {
    fn load(&'a self, req: &Request) -> Result<Response<'d>, ResponseError>;
}

/// A data provider that can iterate over available DataEntry instances.
pub trait IterableDataProvider<'a> {
    type Iter: Iterator<Item=DataEntry>;
    fn iter_for_key(&'a self, data_key: &DataKey) -> Result<Self::Iter, ResponseError>;
}

/// A data provider that validates the type IDs returned by another data provider.
pub struct DataProviderValidator<'a, 'b, 'd> {
    pub data_provider: &'b dyn DataProvider<'a, 'd>,
}

impl<'a, 'b, 'd> DataProvider<'a, 'd> for DataProviderValidator<'a, 'b, 'd> {
    fn load(&'a self, req: &Request) -> Result<Response<'d>, ResponseError> {
        match self.data_provider.load(req) {
            Ok(res) => {
                let actual_type_id = res.get_payload_type_id();
                match req.data_key.get_type_id() {
                    Some(expected_type_id) => {
                        if expected_type_id == actual_type_id {
                            Ok(res)
                        } else {
                            Err(ResponseError::InvalidTypeError {
                                actual: actual_type_id,
                                expected: Some(expected_type_id),
                            })
                        }
                    }
                    None => Err(ResponseError::InvalidTypeError {
                        actual: actual_type_id,
                        expected: None,
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }
}
