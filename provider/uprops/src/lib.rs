// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_uprops` contains implementations of the [`ICU4X`]
//! [data provider] interface backed by TOML files exported by the
//! ICU4C icuwriteuprops tool. Create a directory containing TOML files for
//! the necessary Unicode properties and then pass the path into the
//! [`PropertiesDataProvider`].
//!
//! **Important:** This data provider implementation is not optimized
//! for production use.  It is much more efficient if you use
//! [`FsDataProvider`] or [`StaticDataProvider`] instead.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html

mod bin_uniset;
mod enum_codepointtrie;
mod enum_uniset;
mod provider;
mod reader;
mod script;
mod uprops_helpers;
mod uprops_serde;

pub use provider::PropertiesDataProvider;

// Required by icu_provider_cldr::transform::list
pub use enum_uniset::EnumeratedPropertyUnicodeSetDataProvider;
