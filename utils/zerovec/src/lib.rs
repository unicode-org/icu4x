// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-copy vector abstractions over byte arrays.
//!
//! ZeroVec and VarZeroVec enable vectors of multibyte types to be backed by a byte array,
//! abstracting away issues including memory alignment and endianness.
//!
//! This crate has two main types:
//!
//! - `ZeroVec<T>` for fixed-width types like `u32`
//! - `VarZeroVec<T>` for variable-width types like `str`
//!
//! Both are intended as drop-in replacements for `Vec<T>` in Serde structs serialized with a
//! format supporting a borrowed byte buffer, like Bincode. Clients upgrading from Vec to ZeroVec
//! or VarZeroVec benefit from zero heap allocations when deserializing read-only data.
//!
//! # Performance
//!
//! TODO
//!
//! # Features
//!
//! - `serde`: enables Serde Serialize/Deserialize impls for ZeroVec and VarZeroVec.
//!
//! # Examples
//!
//! Serialize and deserialize a struct with ZeroVec and VarZeroVec with Bincode:
//!
//! ```
//! # #[cfg(feature = "serde")] {
//! use zerovec::{ZeroVec, VarZeroVec};
//!
//! // This example requires the "serde" feature
//! #[derive(serde::Serialize, serde::Deserialize)]
//! pub struct DataStruct<'s> {
//!     #[serde(borrow)]
//!     nums: ZeroVec<'s, u32>,
//!     #[serde(borrow)]
//!     strs: VarZeroVec<'s, String>,
//! }
//!
//! let data = DataStruct {
//!     nums: ZeroVec::from_aligned(&[211, 281, 421, 461]),
//!     strs: VarZeroVec::from(vec!["hello".to_string(), "world".to_string()]),
//! };
//! let bincode_bytes = bincode::serialize(&data)
//!     .expect("Serialization should be successful");
//! assert_eq!(54, bincode_bytes.len());
//!
//! let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
//!     .expect("Deserialization should be successful");
//! assert_eq!(Some(211), deserialized.nums.first());
//! assert_eq!(Some("world"), deserialized.strs.get(1));
//! assert!(matches!(deserialized.nums, ZeroVec::Borrowed(_)));
//! # } // feature = "serde"
//! ```

pub mod samples;
pub mod ule;
mod varzerovec;
mod zerovec;

pub use crate::varzerovec::{VarZeroVec, VarZeroVecError};
pub use crate::zerovec::ZeroVec;
