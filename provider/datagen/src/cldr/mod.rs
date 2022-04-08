// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen::cldr` contains implementations of the [`ICU4X`] [data provider] interface
//! based on the JSON files shipped by CLDR. Create a [`CldrPaths`] and then pass it into
//! [`create_exportable_provider`].
//!
//! This crate contains two implementations of [`CldrPaths`]:
//!
//! - [`CldrPathsLocal`] which points directly to each local CLDR file, and
//! - [`CldrPathsAllInOne`] which points to a local CLDR tree.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider

mod cldr_paths;
mod cldr_serde;
mod error;
mod reader;
mod transform;

pub use cldr_paths::CldrPaths;
pub use cldr_paths::CldrPathsAllInOne;
pub use cldr_paths::CldrPathsLocal;
pub use error::Error as CldrError;

use icu_provider::prelude::*;




pub use transform::calendar::japanese::JapaneseErasProvider;
pub use transform::datetime::week_data::WeekDataProvider;
pub use transform::datetime::CommonDateProvider;
pub use transform::decimal::NumbersProvider;
pub use transform::list::ListProvider;
pub use transform::locale_canonicalizer::aliases::AliasesProvider;
pub use transform::locale_canonicalizer::likely_subtags::LikelySubtagsProvider;
pub use transform::plurals::PluralsProvider;
pub use transform::time_zones::TimeZonesProvider;

pub const ALL_KEYS: [ResourceKey; 19] = [
    icu_calendar::provider::JapaneseErasV1Marker::KEY,
    icu_datetime::provider::calendar::DatePatternsV1Marker::KEY,
    icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
    icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY,
    icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY,
    icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker::KEY,
    icu_datetime::provider::week_data::WeekDataV1Marker::KEY,
    icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
    icu_list::provider::AndListV1Marker::KEY,
    icu_list::provider::OrListV1Marker::KEY,
    icu_list::provider::UnitListV1Marker::KEY,
    icu_locale_canonicalizer::provider::AliasesV1Marker::KEY,
    icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY,
    icu_plurals::provider::CardinalV1Marker::KEY,
    icu_plurals::provider::OrdinalV1Marker::KEY,
];
