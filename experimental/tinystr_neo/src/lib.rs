// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(test), no_std)]

mod macros;

mod ascii;
mod error;
mod int_ops;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "zerovec")]
mod ule;

#[cfg(any(feature = "serde", feature = "alloc"))]
extern crate alloc;

pub use ascii::TinyAsciiStr;
pub use error::TinyStrError;

// /// Allows unit tests to use the macro
// #[cfg(test)]
// mod tinystr {
//     pub use super::{TinyAsciiStr, TinyStrError};
// }
