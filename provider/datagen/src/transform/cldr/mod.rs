// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by the JSON files shipped by CLDR.

mod calendar;
mod cldr_serde;
mod datetime;
mod decimal;
mod fallback;
mod list;
mod locale_canonicalizer;
mod plurals;
pub(crate) mod source;
mod time_zones;

pub use calendar::japanese::JapaneseErasProvider;
pub use datetime::week_data::WeekDataProvider;
pub use datetime::CommonDateProvider;
pub use decimal::NumbersProvider;
pub use fallback::FallbackRulesProvider;
pub use list::ListProvider;
pub use locale_canonicalizer::aliases::AliasesProvider;
pub use locale_canonicalizer::likely_subtags::LikelySubtagsProvider;
pub use plurals::PluralsProvider;
pub use time_zones::TimeZonesProvider;
