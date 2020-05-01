// Data provider trait definitions

pub mod decimal;

use std::prelude::v1::*;
// use async_trait::async_trait;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
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
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ToOwned for dyn Bovine {
    type Owned = Box<dyn Bovine>;
  
    fn to_owned(&self) -> Self::Owned {
        Bovine::clone_into_box(self)
    }
}

// Implement Bovine for all 'static types implementing Clone.
impl<S: 'static + Clone> Bovine for S {
    fn clone_into_box(&self) -> Box<dyn Bovine> {
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
#[derive(Clone)]
pub struct Response {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub payload: Cow<'static, dyn Bovine>,
}

// TODO: Should this be an implemention of std::borrow::Borrow?
impl Response {
    pub fn borrow_payload<T: 'static>(&self) -> Option<&T> {
        let borrowed: &dyn Bovine = self.payload.borrow();
        borrowed.as_any().downcast_ref::<T>()
    }

    pub fn borrow_payload_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let boxed: &mut Box<dyn Bovine> = self.payload.to_mut();
        let borrowed: &mut dyn Bovine = boxed.borrow_mut();
        borrowed.as_any_mut().downcast_mut::<T>()
    }
}

// TODO: Make this async
// #[async_trait]
pub trait DataProvider {
    fn load(&self, req: Request) -> Result<Response, Box<dyn Error>>;
}
