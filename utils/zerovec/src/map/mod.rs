// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`ZeroMap`](crate::ZeroMap) for details.

mod borrowed;
mod kv;
#[allow(clippy::module_inception)] // module is purely internal
pub(crate) mod map;
#[cfg(feature = "serde")]
mod serde;
mod vecs;

pub use crate::ZeroMap;
pub use borrowed::ZeroMapBorrowed;
pub use kv::ZeroMapKV;
pub use vecs::{BorrowedZeroVecLike, MutableZeroVecLike, ZeroVecLike};
