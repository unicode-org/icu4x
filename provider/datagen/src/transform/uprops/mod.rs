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
mod casemapping;
mod enum_codepointtrie;
mod normalizer;
mod normalizer_serde;
mod script;
mod uprops_serde;

pub use bin_uniset::BinaryPropertyUnicodeSetDataProvider;
#[cfg(feature = "experimental")]
pub use casemapping::CaseMappingDataProvider;
pub use enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider;
pub use normalizer::CanonicalCompositionPassthroughProvider;
pub use normalizer::CanonicalCompositionsProvider;
pub use normalizer::CanonicalDecompositionDataProvider;
pub use normalizer::CanonicalDecompositionTablesProvider;
pub use normalizer::CompatibilityCompositionPassthroughProvider;
pub use normalizer::CompatibilityDecompositionSupplementProvider;
pub use normalizer::CompatibilityDecompositionTablesProvider;
#[cfg(feature = "experimental")]
pub use normalizer::Uts46CompositionPassthroughProvider;
#[cfg(feature = "experimental")]
pub use normalizer::Uts46DecompositionSupplementProvider;
pub use script::ScriptWithExtensionsPropertyProvider;
