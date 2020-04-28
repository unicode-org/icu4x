#![no_std]

pub mod datap;
pub mod std;

pub type Str = std::Cow<'static, str>;
