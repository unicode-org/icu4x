// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains implementations of the [`ICU4X`]
//! [data provider] interface backed by TOML files exported by the
//! ICU4C icuexportdata tool.
//!
//! Create a directory containing TOML files for
//! the necessary Unicode properties and then pass the path to the desired
//! provider.
//!
//! This module exports feature-specific providers. Use [`crate::create_datagen_provider`]
//! for an all-inclusive provider.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider

mod bin_uniset;
#[cfg(feature = "experimental")]
mod canonical_decompositions;
#[cfg(feature = "experimental")]
mod casemapping;
#[cfg(feature = "experimental")]
mod decompositions_serde;
mod enum_codepointtrie;
mod script;
mod uprops_serde;

pub use bin_uniset::BinaryPropertyUnicodeSetDataProvider;
#[cfg(feature = "experimental")]
pub use canonical_decompositions::CanonicalDecompositionDataProvider;
#[cfg(feature = "experimental")]
pub use canonical_decompositions::CanonicalDecompositionTablesProvider;
#[cfg(feature = "experimental")]
pub use canonical_decompositions::CompatibilityDecompositionSupplementProvider;
#[cfg(feature = "experimental")]
pub use canonical_decompositions::CompatibilityDecompositionTablesProvider;
#[cfg(feature = "experimental")]
pub use canonical_decompositions::Uts46DecompositionSupplementProvider;
#[cfg(feature = "experimental")]
pub use casemapping::CaseMappingDataProvider;
pub use enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider;
pub use script::ScriptWithExtensionsPropertyProvider;
