// Data provider trait definitions

pub mod decimal;

use std::prelude::v1::*;
// use async_trait::async_trait;
use core::ops::Deref;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Debug, Display};

pub type Str = Cow<'static, str>;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub category: Category,
    pub key: Key,

    // For now, the request payload is a string. If a more expressive type is needed in the
    // future, it should implement PartialEq and Clone. Consider using a concrete type, rather
    // than a detour through Any (like in Response), which complicates the code.
    pub payload: Option<Str>,
}

// Please do not to make this trait public, because it is easy to use incorrectly. It is fine as
// an internal auto-implemented trait.
trait CloneableAny: Debug + Downcast {
    fn clone_into_box(&self) -> Box<dyn CloneableAny>;
}

impl ToOwned for dyn CloneableAny {
    type Owned = Box<dyn CloneableAny>;

    fn to_owned(&self) -> Self::Owned {
        CloneableAny::clone_into_box(self)
    }
}

// Implement CloneableAny for all 'static types implementing Clone.
impl<S: 'static + Clone + Debug> CloneableAny for S {
    fn clone_into_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone())
    }
}

// Adds the Downcast methods to all 'static types implementing CloneableAny.
impl_downcast!(CloneableAny);

#[derive(Debug, Clone)]
pub struct Response {
    // TODO: Make this a Locale instead of a String
    pub data_locale: String,
    // TODO: Is it useful to have the Request saved in the Response?
    pub request: Request,
    payload: Cow<'static, dyn CloneableAny>,
}

// Used to build a response without a payload.
pub struct ResponseBuilder {
    pub data_locale: String,
    pub request: Request,
}

// TODO: Should this be an implemention of std::borrow::Borrow?
impl Response {
    pub fn borrow_payload<T: 'static>(&self) -> Option<&T> {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed.as_any().downcast_ref::<T>()
    }

    pub fn borrow_payload_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let boxed: &mut Box<dyn CloneableAny> = self.payload.to_mut();
        let borrowed: &mut dyn CloneableAny = boxed.borrow_mut();
        borrowed.as_any_mut().downcast_mut::<T>()
    }

    pub fn take_payload<T: 'static + Clone>(self) -> Option<Cow<'static, T>> {
        match self.payload {
            Cow::Borrowed(borrowed) => match borrowed.as_any().downcast_ref::<T>() {
                Some(v) => Some(Cow::Borrowed(v)),
                None => None,
            },
            Cow::Owned(boxed) => match boxed.into_any().downcast::<T>() {
                Ok(boxed_t) => Some(Cow::Owned(*boxed_t)),
                Err(_) => None,
            },
        }
    }
}

impl ResponseBuilder {
    // Move from self: the Response replaces the ResponseBuilder
    pub fn with_owned_payload<T: 'static + Clone + Debug>(self, t: T) -> Response {
        Response {
            data_locale: self.data_locale,
            request: self.request,
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    // Move from self: the Response replaces the ResponseBuilder
    pub fn with_borrowed_payload<T: 'static + Clone + Debug>(self, t: &'static T) -> Response {
        Response {
            data_locale: self.data_locale,
            request: self.request,
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

// TODO: Make this async
// #[async_trait]
pub trait DataProvider {
    fn load(&self, req: Request) -> Result<Response, ResponseError>;
}
