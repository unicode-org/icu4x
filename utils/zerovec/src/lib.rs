// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-copy vector abstractions over byte arrays.
//!
//! `zerovec` enable vectors of multibyte types to be backed by a byte array, abstracting away
//! issues including memory alignment and endianness.
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
//! This crate has two optional features: `serde` and `yoke`. `serde` allows serializing and deserializing
//! `zerovec`'s abstractions via [`serde`](https://docs.rs/serde), and `yoke` enables implementations of `Yokeable`
//! from the [`yoke`](https://docs.rs/yoke/) crate.
//!
//! # Performance
//!
//! `zerovec` is designed for fast deserialization from byte buffers with zero memory allocations
//! while minimizing performance regressions for common vector operations.
//!
//! Benchmark results on x86_64:
//!
//! | Operation | `Vec<T>` | `zerovec` |
//! |---|---|---|
//! | Deserialize vec of 100 `u32` | 233.18 ns | 14.120 ns |
//! | Compute sum of vec of 100 `u32` (read every element) | 8.7472 ns | 10.775 ns |
//! | Binary search vec of 1000 `u32` 50 times | 442.80 ns | 472.51 ns |
//! | Deserialize vec of 100 strings | 7.3740 us\* | 1.4495 us |
//! | Count chars in vec of 100 strings (read every element) | 747.50 ns | 955.28 ns |
//! | Binary search vec of 500 strings 10 times | 466.09 ns | 790.33 ns |
//!
//! \* *This result is reported for `Vec<String>`. However, Serde also supports deserializing to `Vec<&str>`; this gives 1.8420 us, much faster than `Vec<String>` but a bit slower than `zerovec`.*
//!
//! The benches used to generate the above table can be found in the `benches` directory in the project repository.
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

pub mod map;
pub mod samples;
pub mod ule;
mod varzerovec;
mod zerovec;

#[cfg(feature = "yoke")]
mod yoke_impls;

pub use crate::map::ZeroMap;
pub use crate::varzerovec::{VarZeroVec, VarZeroVecError};
pub use crate::zerovec::ZeroVec;
