// #![feature(type_alias_impl_trait)]

mod cldr_langid;
mod cldr_paths;
mod error;
mod reader;
mod support;

pub mod transform;

pub use cldr_paths::CldrPaths;
pub use error::Error as CldrError;
pub use transform::CldrJsonDataProvider;
