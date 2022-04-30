// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains implementations of the [`ICU4X`] [data provider] interface
//! based on the JSON files shipped by CLDR.
//!
//! This module exports feature-specific providers. Use [`crate::create_datagen_provider`]
//! for an all-inclusive provider.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//!

mod calendar;
mod cldr_serde;
mod datetime;
mod decimal;
mod list;
mod locale_canonicalizer;
mod plurals;
mod reader;
mod time_zones;

pub use calendar::japanese::JapaneseErasProvider;
pub use datetime::week_data::WeekDataProvider;
pub use datetime::CommonDateProvider;
pub use decimal::NumbersProvider;
pub use list::ListProvider;
pub use locale_canonicalizer::aliases::AliasesProvider;
pub use locale_canonicalizer::likely_subtags::LikelySubtagsProvider;
pub use plurals::PluralsProvider;
pub use time_zones::TimeZonesProvider;
