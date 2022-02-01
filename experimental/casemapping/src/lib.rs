// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: module documentation
#![warn(missing_docs)]

mod casemapping;
pub mod provider;

mod error;
mod exceptions;
#[cfg(feature = "provider_transform_internals")]
mod exceptions_builder;
mod internals;

pub use error::Error as CaseMappingError;
pub use casemapping::CaseMapping;
#[cfg(feature = "provider_transform_internals")]
pub use internals::CaseMappingInternals;
