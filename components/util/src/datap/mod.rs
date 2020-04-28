// Data provider trait definitions

pub mod decimal;

use crate::std::Box;
use crate::std::String;
use crate::std::Cow;
use crate::Str;
use async_trait::async_trait;

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
    pub category: Category,
    pub key: u32,
    pub payload: Cow<'static, ResponsePayload>,
}

#[async_trait]
pub trait DataProvider {
    async fn load(&self, req: Request) -> Response;
}
