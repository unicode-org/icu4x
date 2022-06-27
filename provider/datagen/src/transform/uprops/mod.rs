// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

mod bin_uniset;
#[cfg(feature = "experimental")]
mod casemapping;
mod enum_codepointtrie;
#[cfg(feature = "experimental")]
mod normalizer;
#[cfg(feature = "experimental")]
mod normalizer_serde;
mod script;
mod uprops_serde;

pub use bin_uniset::BinaryPropertyUnicodeSetDataProvider;
#[cfg(feature = "experimental")]
pub use casemapping::CaseMappingDataProvider;
pub use enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CanonicalCompositionPassthroughProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CanonicalCompositionsProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CanonicalDecompositionDataProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CanonicalDecompositionTablesProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CompatibilityCompositionPassthroughProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CompatibilityDecompositionSupplementProvider;
#[cfg(feature = "experimental")]
pub use normalizer::CompatibilityDecompositionTablesProvider;
#[cfg(feature = "experimental")]
pub use normalizer::Uts46CompositionPassthroughProvider;
#[cfg(feature = "experimental")]
pub use normalizer::Uts46DecompositionSupplementProvider;
pub use script::ScriptWithExtensionsPropertyProvider;
