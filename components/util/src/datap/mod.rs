// Data provider trait definitions

pub mod decimal;

use std::prelude::v1::*;
// use async_trait::async_trait;
use std::borrow::Cow;
use std::error::Error;

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

// TODO: Make ResponsePayload be a dynamic type specified by each caller
#[derive(PartialEq, Clone)]
pub enum ResponsePayload {
    Undefined,
    Decimal(decimal::Payload),
    // TODO: Enable a dynamic type here
    // PrivateUse(Any),
}

#[derive(PartialEq, Clone)]
pub struct Response {
    // TODO: Make this a Locale instead of a String
    pub locale: String,
    pub payload: Cow<'static, ResponsePayload>,
}

// TODO: Make this async
// #[async_trait]
pub trait DataProvider {
    fn load(&self, req: Request) -> Result<Response, Box<dyn Error>>;
}
