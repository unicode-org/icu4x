// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`ZeroMap2d`](crate::ZeroMap2d) for details.


mod borrowed;
pub(crate) mod map;
#[cfg(feature = "serde")]
mod serde;

#[doc(no_inline)]
pub use crate::ZeroMap2d;
pub use borrowed::ZeroMap2dBorrowed;
pub use map::KeyError;
