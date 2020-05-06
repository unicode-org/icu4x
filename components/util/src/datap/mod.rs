// Data provider trait definitions

pub mod decimal;

use std::prelude::v1::*;
// use async_trait::async_trait;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::error::Error;
use std::any::Any;
use std::fmt::{Debug, Display, self};
use core::ops::Deref;

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
    PrivateUse(u32)
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(PartialEq, Clone)]
pub struct Request {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub category: Category,
    pub key: Key,
    // TODO: Make payload a dynamic type instead of a string
    pub payload: Option<Str>,
}

// Please try not to make this trait public, because it is easy to use incorrectly.  It is fine as an internal auto-implemented trait.
trait CloneableAny: Debug + Any {
    fn clone_into_box(&self) -> Box<dyn CloneableAny>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// TODO: Enable PartialEq on Response (is that necessary?)
#[derive(Debug, Clone)]
pub struct Response {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    payload: Cow<'static, dyn CloneableAny>,
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

    pub fn with_owned_payload<T: 'static + Clone + Debug>(t: T) -> Self {
        Response {
            locale: "und".to_string(),
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    pub fn with_borrowed_payload<T: 'static + Clone + Debug>(t: &'static T) -> Self {
        Response {
            locale: "und".to_string(),
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
            _ => None
        }
    }
}

// TODO: Make this async
// #[async_trait]
pub trait DataProvider {
    fn load(&self, req: Request) -> Result<Response, ResponseError>;
}
