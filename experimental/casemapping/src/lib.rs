// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`icu_casemapping`](crate) is one of the ['ICU4X`] components.
//!
//! This API provides functionality for handling case mapping for Unicode
//! characters and strings.
//!
//! TODO: expand documentation
//!
//! [`ICU4X`]: ../icu/index.html

#![warn(missing_docs)]

mod casemapping;
pub mod provider;

mod error;
mod exceptions;
#[cfg(feature = "datagen")]
mod exceptions_builder;
mod internals;

pub use casemapping::CaseMapping;
pub use error::Error as CaseMappingError;
#[cfg(feature = "datagen")]
pub use internals::CaseMappingInternals;
