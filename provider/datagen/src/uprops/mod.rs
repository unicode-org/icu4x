// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen::uprops` contains implementations of the [`ICU4X`]
//! [data provider] interface backed by TOML files exported by the
//! ICU4C icuwriteuprops tool. Create a directory containing TOML files for
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
mod enum_codepointtrie;
mod enum_uniset;
mod reader;
mod script;
mod uprops_helpers;
mod uprops_serde;

#[cfg(feature = "casemapping")]
pub mod casemapping;

pub use bin_uniset::BinaryPropertyUnicodeSetDataProvider;
pub use enum_codepointtrie::EnumeratedPropertyCodePointTrieProvider;
pub use enum_uniset::EnumeratedPropertyUnicodeSetDataProvider;
pub use script::ScriptWithExtensionsPropertyProvider;
