// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen::segmenter` contains implementations of the [`ICU4X`] [data provider] interface
//! based on Unicode properties and TOML files implementing [Unicode Standard Annex #14][UAX14] and
//! [Unicode Standard Annex #29][UAX29] breaking rules.
//!
//! This module exports feature-specific providers. Use [`crate::create_datagen_provider`]
//! for an all-inclusive provider.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! # Examples
//!
//! ```
//! # use icu_datagen::SourceData;
//! use icu_datagen::segmenter::SegmenterRuleProvider;
//! let provider = SegmenterRuleProvider::from(&SourceData::for_test());
//! ```
//!
//! # Exporting data
//!
//! To generate the data required by the segmenters, run `cargo make testdata` from the top level.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/

mod transform;

pub use transform::SegmenterRuleProvider;