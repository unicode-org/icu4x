// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains implementations of the [`ICU4X`] [data provider] interface
//! based on CLDR and ICU export data.
//!
//! This module exports feature-specific providers. Use [`crate::create_datagen_provider`]
//! for a provider that covers all ICU4X keys.
//!
//! **Important:** Therse data provider implementations are not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider

pub mod cldr;
#[cfg(feature = "experimental")]
pub mod collator;
#[cfg(feature = "experimental")]
pub mod segmenter;
pub mod uprops;
