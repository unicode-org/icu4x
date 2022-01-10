// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This crate provides [`Yoke`], a data structure that allows one
//! to "yoke" Cow-like borrowed data types to their backing storage,
//! enabling one to use Cow (etc) in zero-copy deserialization
//! with dynamic lifetimes for the borrowed data, for example caching it.
//!
//! See the documentation of [`Yoke`] for more details.

#![cfg_attr(not(test), no_std)]
// The lifetimes here are important for safety and explicitly writing
// them out is good even when redundant
#![allow(clippy::needless_lifetimes)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod either;
#[cfg(feature = "alloc")]
pub mod erased;
mod is_covariant;
mod macro_impls;
pub mod trait_hack;
mod yoke;
mod yokeable;
mod zero_copy_from;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "derive")]
pub use yoke_derive::{Yokeable, ZeroCopyFrom};

pub use crate::is_covariant::IsCovariant;
pub use crate::yoke::{CloneableCart, Yoke};
pub use crate::yokeable::Yokeable;
pub use crate::zero_copy_from::ZeroCopyFrom;
