#[macro_use]
mod uniset;
mod conversions;
mod utils;

pub use conversions::*;
pub use uniset::UnicodeSet;
pub use utils::*;

/// Custom Errors for UnicodeSet.
#[derive(Debug, PartialEq)]
pub enum UnicodeSetError {
    InvalidSet(Vec<u32>),
    InvalidRange(u32, u32),
}
