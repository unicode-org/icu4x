// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-copy vector abstractions for arbitrary types.
//!
//! `zerovec` enables a far wider range of types — beyond just `&[u8]` and `&str` — to participate in
//! efficient strategies like zero-copy deserialization. It is `serde` compatible and comes equipped with
//! proc macros
//!
//! Clients upgrading to `zerovec` benefit from zero heap allocations when deserializing
//! read-only data.
//!
//! This crate has four main types:
//!
//! - [`ZeroVec<'a, T>`] (and [`ZeroSlice<T>`](ZeroSlice)) for fixed-width types like `u32`
//! - [`VarZeroVec<'a, T>`] (and [`VarZeroSlice<T>`](ZeroSlice)) for variable-width types like `str`
//! - [`ZeroMap<'a, K, V>`] to map from `K` to `V`
//! - [`ZeroMap2d<'a, K0, K1, V>`] to map from the pair `(K0, K1)` to `V`
//!
//! The first two are intended as close-to-drop-in replacements for `Vec<T>` in Serde structs. The third and fourth are
//! intended as a replacement for `HashMap` or [`LiteMap`](docs.rs/litemap). When used with Serde derives, **be sure to apply
//! `#[serde(borrow)]` to these types**, same as one would for [`Cow<'a, T>`].
//!
//! [`ZeroVec<'a, T>`], [`VarZeroVec<'a, T>`], [`ZeroMap<'a, K, V>`], and [`ZeroMap2d<'a, K0, K1, V>`] all behave like
//! [`Cow<'a, T>`] in that they abstract over either borrowed or owned data. When performing deserialization
//! from human-readable formats (like `json` and `xml`), typically these types will allocate and fully own their data, whereas if deserializing
//! from binary formats like `bincode` and `postcard`, these types will borrow data directly from the buffer being deserialized from,
//! avoiding allocations and only performing validity checks. As such, this crate can be pretty fast (see [below](#Performance) for more information)
//! on deserialization.
//!
//! See [the design doc](https://github.com/unicode-org/icu4x/blob/main/utils/zerovec/design_doc.md) for details on how this crate
//! works under the hood.
//!
//! # Cargo features
//!
//! This crate has four optional features:
//!  -  `serde`: Allows serializing and deserializing `zerovec`'s abstractions via [`serde`](https://docs.rs/serde)
//!  -   `yoke`: Enables implementations of `Yokeable` from the [`yoke`](https://docs.rs/yoke/) crate, which is also useful
//!              in situations involving a lot of zero-copy deserialization.
//!  - `derive`: Makes it easier to use custom types in these collections by providing the [`#[make_ule]`](crate::make_ule) and
//!     [`#[make_varule]`](crate::make_varule) proc macros, which generate appropriate [`ULE`](crate::ule::ULE) and
//!     [`VarULE`](crate::ule::VarULE)-conformant types for a given "normal" type.
//!  - `std`: Enabled `std::Error` implementations for error types. This crate is by default `no_std` with a dependency on `alloc`.
//!
//! [`ZeroVec<'a, T>`]: ZeroVec
//! [`VarZeroVec<'a, T>`]: VarZeroVec
//! [`ZeroMap<'a, K, V>`]: ZeroMap
//! [`ZeroMap2d<'a, K0, K1, V>`]: ZeroMap2d
//! [`Cow<'a, T>`]: alloc::borrow::Cow
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
//!     chars: ZeroVec<'data, char>,
//!     #[serde(borrow)]
//!     strs: VarZeroVec<'data, str>,
//! }
//!
//! let data = DataStruct {
//!     nums: ZeroVec::from_slice(&[211, 281, 421, 461]),
//!     chars: ZeroVec::from_slice(&['ö', '冇', 'म']),
//!     strs: VarZeroVec::from(&["hello", "world"]),
//! };
//! let bincode_bytes = bincode::serialize(&data)
//!     .expect("Serialization should be successful");
//! assert_eq!(bincode_bytes.len(), 74);
//!
//! let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
//!     .expect("Deserialization should be successful");
//! assert_eq!(deserialized.nums.first(), Some(211));
//! assert_eq!(deserialized.chars.get(1), Some('冇'));
//! assert_eq!(deserialized.strs.get(1), Some("world"));
//! // The deserialization will not have allocated anything
//! assert!(matches!(deserialized.nums, ZeroVec::Borrowed(_)));
//! # } // feature = "serde"
//! ```
//!
//! Use custom types inside of ZeroVec:
//!
//! ```rust
//! # #[cfg(all(feature = "serde", feature = "derive"))] {
//! use zerovec::{ZeroVec, VarZeroVec, ZeroMap};
//! use std::borrow::Cow;
//! use zerovec::ule::encode_varule_to_box;
//!
//! // custom fixed-size ULE type for ZeroVec
//! #[zerovec::make_ule(DateULE)]
//! #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
//! struct Date {
//!     y: u64,
//!     m: u8,
//!     d: u8
//! }
//!
//! // custom variable sized VarULE type for VarZeroVec
//! #[zerovec::make_varule(PersonULE)]
//! #[zerovec::serde]
//! #[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
//! struct Person<'a> {
//!     birthday: Date,
//!     favorite_character: char,
//!     #[serde(borrow)]
//!     name: Cow<'a, str>,
//! }
//!
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct Data<'a> {
//!     #[serde(borrow)]
//!     important_dates: ZeroVec<'a, Date>,
//!     // note: VarZeroVec always must reference the ULE type directly
//!     #[serde(borrow)]
//!     important_people: VarZeroVec<'a, PersonULE>,
//!     #[serde(borrow)]
//!     birthdays_to_people: ZeroMap<'a, Date, PersonULE>
//! }
//!
//!
//! let person1 = Person {
//!     birthday: Date { y: 1990, m: 9, d: 7},
//!     favorite_character: 'π',
//!     name: Cow::from("Kate")
//! };
//! let person2 = Person {
//!     birthday: Date { y: 1960, m: 5, d: 25},
//!     favorite_character: '冇',
//!     name: Cow::from("Jesse")
//! };
//!
//! let important_dates = ZeroVec::alloc_from_slice(&[Date { y: 1943, m: 3, d: 20}, Date { y: 1976, m: 8, d: 2}, Date { y: 1998, m: 2, d: 15}]);
//! let important_people = VarZeroVec::from(&[person1.clone(), person2.clone()]);
//! let mut birthdays_to_people: ZeroMap<Date, PersonULE> = ZeroMap::new();
//! // `.insert_var_v()` is slightly more convenient over `.insert()` for custom ULE types
//! birthdays_to_people.insert_var_v(&person1.birthday, &person1);
//! birthdays_to_people.insert_var_v(&person2.birthday, &person2);
//!
//! let data = Data { important_dates, important_people, birthdays_to_people };
//!
//! let bincode_bytes = bincode::serialize(&data)
//!     .expect("Serialization should be successful");
//! assert_eq!(bincode_bytes.len(), 180);
//!
//! let deserialized: Data = bincode::deserialize(&bincode_bytes)
//!     .expect("Deserialization should be successful");
//!
//! assert_eq!(deserialized.important_dates.get(0).unwrap().y, 1943);
//! assert_eq!(&deserialized.important_people.get(1).unwrap().name, "Jesse");
//! assert_eq!(&deserialized.important_people.get(0).unwrap().name, "Kate");
//! assert_eq!(&deserialized.birthdays_to_people.get(&person1.birthday).unwrap().name, "Kate");
//!
//! } // feature = serde and derive
//! ```
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
//! \* *This result is reported for `Vec<String>`. However, Serde also supports deserializing to the partially-zero-copy `Vec<&str>`; this gives 1.8420 μs, much faster than `Vec<String>` but a bit slower than `zerovec`.*
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
mod varzerovec;
mod zerovec;

// This must be after `mod zerovec` for some impls on `ZeroSlice<RawBytesULE>`
// to show up in the right spot in the docs
pub mod ule;

#[cfg(feature = "yoke")]
mod yoke_impls;
mod zerofrom_impls;

pub use crate::error::ZeroVecError;
pub use crate::map::map::ZeroMap;
pub use crate::map2d::map::ZeroMap2d;
pub use crate::varzerovec::{slice::VarZeroSlice, vec::VarZeroVec};
pub use crate::zerovec::{ZeroSlice, ZeroVec};

#[cfg(feature = "derive")]
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
