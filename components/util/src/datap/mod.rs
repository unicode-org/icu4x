// Data provider trait definitions

pub mod decimal;

use std::prelude::v1::*;
// use async_trait::async_trait;
use std::borrow::Cow;
use std::error::Error;
use std::any::Any;

pub type Str = Cow<'static, str>;

#[derive(PartialEq, Copy, Clone)]
pub enum Category {
    Undefined = 0,
    Decimal = 1,
    PrivateUse = 4096,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Key {
    Undefined,
    Decimal(decimal::Key),
    PrivateUse(u32)
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

pub trait Bovine: Any {
    fn clone_into_box(&self) -> Box<dyn Bovine>;
    fn as_any(&self) -> &dyn Any;
}

impl ToOwned for dyn Bovine {
    type Owned = Box<dyn Bovine>;
  
    fn to_owned(&self) -> Self::Owned {
        Bovine::clone_into_box(self)
    }
}

// TODO: Enable PartialEq on Response (is that necessary?)
#[derive(Clone)]
pub struct Response {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub payload2: Cow<'static, dyn Bovine>,
}

impl Response {
    pub fn unwrap_payload<T: 'static>(&self) -> &T {
        Any::downcast_ref::<T>(self.payload2.as_any()).unwrap()
    }
}

// TODO: Make this async
// #[async_trait]
pub trait DataProvider {
    fn load(&self, req: Request) -> Result<Response, Box<dyn Error>>;
}
