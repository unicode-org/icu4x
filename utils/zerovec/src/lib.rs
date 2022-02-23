// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-copy vector abstractions over byte arrays.
//!
//! `zerovec` enable vectors of multibyte types to be backed by a byte array, abstracting away
//! issues including memory alignment and endianness.
//!
//! This crate has four main types:
//!
//! - [`ZeroVec<T>`](ZeroVec) for fixed-width types like `u32`
//! - [`VarZeroVec<T>`](VarZeroVec) for variable-width types like `str`
//! - [`ZeroMap<K, V>`](ZeroMap) to map from `K` to `V`
//! - [`ZeroMap2d<K0, K1, V>`](ZeroMap2d) to map from the pair `(K0, K1)` to `V`
//!
//! The first two are intended as drop-in replacements for `Vec<T>` in Serde structs serialized
//! with a format supporting a borrowed byte buffer, like Bincode. The third and fourth are
//! intended as a replacement for `HashMap` or `LiteMap`.
//!
//! Clients upgrading to `zerovec` benefit from zero heap allocations when deserializing
//! read-only data.
//!
//! This crate has three optional features: `serde`, `yoke`, and `derive`. `serde` allows serializing and deserializing
//! `zerovec`'s abstractions via [`serde`](https://docs.rs/serde), and `yoke` enables implementations of `Yokeable`
//! from the [`yoke`](https://docs.rs/yoke/) crate.
//!
//! `derive` makes it easier to use custom types in these collections by providing the [`#[make_ule]`](crate::make_ule) and
//! [`#[make_varule]`](crate::make_varule) proc macros, which generate appropriate [`ULE`](crate::ule::ULE) and
//! [`VarULE`](crate::ule::VarULE)-conformant types for a given "normal" type.
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
//! | Deserialize vec of 100 strings | 7.3740 μs\* | 1.4495 μs |
//! | Count chars in vec of 100 strings (read every element) | 747.50 ns | 955.28 ns |
//! | Binary search vec of 500 strings 10 times | 466.09 ns | 790.33 ns |
//!
//! \* *This result is reported for `Vec<String>`. However, Serde also supports deserializing to `Vec<&str>`; this gives 1.8420 μs, much faster than `Vec<String>` but a bit slower than `zerovec`.*
//!
//! | Operation | `HashMap<K,V>`  | `LiteMap<K,V>` | `ZeroMap<K,V>` |
//! |---|---|---|---|
//! | Deserialize a small map | 2.72 μs | 1.28 μs | 480 ns |
//! | Deserialize a large map | 50.5 ms | 18.3 ms | 3.74 ms |
//! | Look up from a small deserialized map | 49 ns | 42 ns | 54 ns |
//! | Look up from a large deserialized map | 51 ns | 155 ns | 213 ns |
//!
//! Small = 16 elements, large = 131,072 elements. Maps contain `<String, String>`.
//!
//! The benches used to generate the above table can be found in the `benches` directory in the project repository.
//! `zeromap` benches are named by convention, e.g. `zeromap/deserialize/small`, `zeromap/lookup/large`. The type
//! is appended for baseline comparisons, e.g. `zeromap/lookup/small/hashmap`.
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
//! pub struct DataStruct<'data> {
//!     #[serde(borrow)]
//!     nums: ZeroVec<'data, u32>,
//!     #[serde(borrow)]
//!     strs: VarZeroVec<'data, str>,
//! }
//!
//! let data = DataStruct {
//!     nums: ZeroVec::from_slice(&[211, 281, 421, 461]),
//!     strs: VarZeroVec::from(&["hello", "world"]),
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

#![cfg_attr(not(any(test, feature = "std")), no_std)]
// this crate does a lot of nuanced lifetime manipulation, being explicit
// is better here.
#![allow(clippy::needless_lifetimes)]

extern crate alloc;

mod error;
mod map;
mod map2d;
#[cfg(test)]
pub mod samples;
pub mod ule;
mod varzerovec;
mod zerovec;

#[cfg(feature = "yoke")]
mod yoke_impls;
mod zerofrom_impls;

pub use crate::error::ZeroVecError;
pub use crate::map::map::ZeroMap;
pub use crate::map2d::map::ZeroMap2d;
pub use crate::varzerovec::{slice::VarZeroSlice, vec::VarZeroVec};
pub use crate::zerovec::{ZeroSlice, ZeroVec};

#[cfg(feature = "serde")]
pub use zerovec_derive::{make_ule, make_varule};

#[doc(hidden)]
pub mod __zerovec_internal_reexport {
    pub use zerofrom::ZeroFrom;

    pub use alloc::boxed;

    #[cfg(feature = "serde")]
    pub use serde;
}

pub mod maps {
    //! This module contains additional utility types and traits for working with
    //! [`ZeroMap`] and [`ZeroMap2d`]. See their docs for more details on the general purpose
    //! of these types.
    //!
    //! [`ZeroMapBorrowed`] and [`ZeroMap2dBorrowed`] are versions of [`ZeroMap`] and [`ZeroMap2d`]
    //! that can be used when you wish to guarantee that the map data is always borrowed, leading to
    //! relaxed lifetime constraints.
    //!
    //! The [`ZeroMapKV`] trait is required to be implemented on any type that needs to be used
    //! within a map type. [`ZeroVecLike`], [`BorrowedZeroVecLike`], and [`MutableZeroVecLike`] are
    //! all traits used in the internal workings of the map types, and should typically not be used
    //! or implemented by users of this crate.
    #[doc(no_inline)]
    pub use crate::map::ZeroMap;
    pub use crate::map::ZeroMapBorrowed;

    #[doc(no_inline)]
    pub use crate::map2d::ZeroMap2d;
    pub use crate::map2d::ZeroMap2dBorrowed;

    pub use crate::map::{BorrowedZeroVecLike, MutableZeroVecLike, ZeroMapKV, ZeroVecLike};
    pub use crate::map2d::KeyError;
}

pub mod vecs {
    //! This module contains additional utility types for working with
    //! [`ZeroVec`] and  [`VarZeroVec`]. See their docs for more details on the general purpose
    //! of these types.
    //!
    //! [`ZeroSlice`] and [`VarZeroSlice`] provide slice-like versions of the vector types
    //! for use behind references and in custom ULE types.
    //!
    //! [`VarZeroVecOwned`] is a special owned/mutable version of [`VarZeroVec`], allowing
    //! direct manipulation of the backing buffer.

    #[doc(no_inline)]
    pub use crate::zerovec::{ZeroSlice, ZeroVec};

    #[doc(no_inline)]
    pub use crate::varzerovec::{VarZeroSlice, VarZeroVec};

    pub use crate::varzerovec::VarZeroVecOwned;
}
