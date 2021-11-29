// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: module documentation
#![warn(missing_docs)]

mod error;

/// TODO: This module is only public to enable testing internal functions in the provider crate.
/// After we have finalized the public API, this can be made private.
pub mod internals;

pub mod provider;

pub use error::Error as CaseMappingError;
pub use internals::CaseMapping;
