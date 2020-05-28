// Data provider trait definitions

pub mod decimal;

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

/// A top-level collection of related data keys.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Category {
    Undefined = 0,
    Decimal = 1,
    PrivateUse = 4096,
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// A specific key within a category.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Key {
    Undefined,
    Decimal(decimal::Key),
    PrivateUse(u32),
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// A struct to request a certain hunk of data from a data provider.
#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub category: Category,
    pub key: Key,

    // For now, the request payload is a string. If a more expressive type is needed in the
    // future, it should implement PartialEq and Clone. Consider using a concrete type, rather
    // than a detour through Any (like in Response), which complicates the code.
    pub payload: Option<Cow<'static, str>>,
}

/// A response object containing a data hunk ("payload").
#[derive(Debug, Clone)]
pub struct Response<'d> {
    // TODO: Make this a Locale instead of a String
    pub data_locale: String,
    payload: Cow<'d, dyn CloneableAny>,
}

#[derive(Debug)]
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
        let boxed: &mut Box<dyn CloneableAny> = self.payload.to_mut();
        let borrowed_mut: &mut dyn CloneableAny = boxed.borrow_mut();
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
}

/// Builder class used to construct a Response.
pub struct ResponseBuilder {
    pub data_locale: String,
}

impl ResponseBuilder {
    /// Construct a Response from the builder, with owned data.
    /// Consumes both the builder and the data.
    /// Returns the 'static lifetime since there is no borrowed data.
    pub fn with_owned_payload<T: 'static + Clone + Debug>(self, t: T) -> Response<'static> {
        Response {
            data_locale: self.data_locale,
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    /// Construct a Response from the builder, with borrowed data.
    /// Consumes the builder, but not the data.
    pub fn with_borrowed_payload<'d, T: 'static + Clone + Debug>(self, t: &'d T) -> Response<'d> {
        Response {
            data_locale: self.data_locale,
            payload: Cow::Borrowed(t),
        }
    }
}

#[derive(Debug)]
pub enum ResponseError {
    UnsupportedCategoryError(Category),
    UnsupportedKeyError(Category, Key),
    ResourceError(Box<dyn Error>),
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

/// An abstract data providewr that takes a request object and returns a response with a payload.
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
