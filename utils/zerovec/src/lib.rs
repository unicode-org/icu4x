// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-copy vector abstractions over byte arrays.
//!
//! This crate has two main types:
//!
//! - `ZeroVec<T>` for fixed-width types like `u32`
//! - `VarZeroVec<T>` for variable-width types like `str`
//!
//! Both are intended as drop-in replacements for `Vec<T>` in Serde structs serialized with a
//! format supporting a borrowed byte buffer, like Bincode.
//!
//! # Features
//!
//! - `serde`: enables Serde Serialize/Deserialize impls for ZeroVec and VarZeroVec.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "serde")] {
//! use zerovec::ZeroVec;
//!
//! // This example requires the "serde" feature
//! #[derive(serde::Serialize, serde::Deserialize)]
//! pub struct DataStruct<'s> {
//!     #[serde(borrow)]
//!     nums: ZeroVec<'s, u32>,
//! }
//!
//! let data = DataStruct {
//!     nums: ZeroVec::from_aligned(&[211, 281, 421, 461]),
//! };
//! let bincode_bytes = bincode::serialize(&data)
//!     .expect("Serialization should be successful");
//! assert_eq!(24, bincode_bytes.len());
//!
//! let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
//!     .expect("Deserialization should be successful");
//! assert_eq!(Some(211), deserialized.nums.first());
//! assert!(matches!(deserialized.nums, ZeroVec::Borrowed(_)));
//! # } // feature = "serde"
//! ```

pub mod samples;
pub mod ule;
mod varzerovec;
mod zerovec;

pub use crate::varzerovec::{VarZeroVec, VarZeroVecError};
pub use crate::zerovec::ZeroVec;
