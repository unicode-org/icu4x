//! This is a test doc doc 
//! 
//! 

#[macro_use]
mod uniset;
mod builder;
mod conversions;
mod utils;

pub use builder::UnicodeSetBuilder;
pub use conversions::*;
pub use uniset::UnicodeSet;
pub use utils::*;


/// Custom Errors for UnicodeSet.
#[derive(Debug, PartialEq)]
pub enum UnicodeSetError {
    InvalidSet(Vec<u32>),
    InvalidRange(u32, u32),
}
