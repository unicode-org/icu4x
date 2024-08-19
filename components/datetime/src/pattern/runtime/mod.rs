// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Runtime `Pattern` implementation which is optimized for zero-allocation
//! deserialization and high-performance runtime use in `TypedDateTimeFormatter`.
//!
//! This module is meant to remain private and can evolve to utilize
//! all runtime performance optimizations `ICU4X` needs.
//!
//! For all spec compliant behaviors see `reference::Pattern` equivalent.
mod generic;
pub(crate) mod helpers;
mod pattern;
mod plural;

pub use generic::GenericPattern;
#[cfg(feature = "experimental")]
pub(crate) use generic::ZERO_ONE_TWO_SLICE;
pub use pattern::{Pattern, PatternBorrowed, PatternMetadata, PatternULE};
pub use plural::{PatternPlurals, PluralPattern};
