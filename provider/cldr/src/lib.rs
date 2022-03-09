// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_cldr` contains implementations of the [`ICU4X`] [data provider] interface
//! based on the JSON files shipped by CLDR. Create a [`CldrPaths`] and then pass it into
//! [`create_exportable_provider`].
//!
//! This crate contains two implementations of [`CldrPaths`]:
//!
//! - [`CldrPathsLocal`] points to local copies of the CLDR JSON repositories.
//! - [`CldrAllInOneDownloader`](download::CldrAllInOneDownloader) downloads and caches the
//!   CLDR JSON repositories. Requires the "download" feature.
//!
//! **Important:** This data provider implementation is not optimized for production use.
//! It is much more efficient if you use [`FsDataProvider`] instead.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html

mod cldr_paths;
mod cldr_serde;
mod error;
mod reader;
mod transform;

#[cfg(feature = "download")]
pub mod download;

pub use cldr_paths::CldrPaths;
pub use cldr_paths::CldrPathsAllInOne;
pub use cldr_paths::CldrPathsLocal;
pub use error::Error as CldrError;

use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::iter::IterableDynProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;
use transform::calendar::japanese::JapaneseErasProvider;
use transform::datetime::week_data::WeekDataProvider;
use transform::datetime::CommonDateProvider;
use transform::decimal::NumbersProvider;
#[cfg(feature = "icu_list")]
use transform::list::ListProvider;
use transform::locale_canonicalizer::aliases::AliasesProvider;
use transform::locale_canonicalizer::likely_subtags::LikelySubtagsProvider;
use transform::plurals::PluralsProvider;
use transform::time_zones::TimeZonesProvider;

#[cfg(not(feature = "icu_list"))]
type ListProvider = PluralsProvider; // we can't cfg-exclude part of the bound, but we can do this...

pub fn create_exportable_provider<T: DataMarker>(
    cldr_paths: &dyn CldrPaths,
    _uprops_root: PathBuf,
) -> Result<MultiForkByKeyProvider<Box<dyn IterableDynProvider<T> + Sync>>, CldrError>
where
    AliasesProvider: IterableDynProvider<T>,
    CommonDateProvider: IterableDynProvider<T>,
    JapaneseErasProvider: IterableDynProvider<T>,
    LikelySubtagsProvider: IterableDynProvider<T>,
    NumbersProvider: IterableDynProvider<T>,
    PluralsProvider: IterableDynProvider<T>,
    TimeZonesProvider: IterableDynProvider<T>,
    ListProvider: IterableDynProvider<T>,
    WeekDataProvider: IterableDynProvider<T>,
{
    #[allow(unused_variables)] // uprops_root is only used if icu_list
    Ok(MultiForkByKeyProvider {
        providers: vec![
            Box::new(AliasesProvider::try_from(cldr_paths)?),
            Box::new(CommonDateProvider::try_from(cldr_paths)?),
            Box::new(JapaneseErasProvider::try_from(cldr_paths)?),
            Box::new(LikelySubtagsProvider::try_from(cldr_paths)?),
            Box::new(NumbersProvider::try_from(cldr_paths)?),
            Box::new(PluralsProvider::try_from(cldr_paths)?),
            Box::new(TimeZonesProvider::try_from(cldr_paths)?),
            Box::new(WeekDataProvider::try_from(cldr_paths)?),
            #[cfg(feature = "icu_list")]
            Box::new(ListProvider::try_from(cldr_paths, _uprops_root)?),
        ],
    })
}

pub const ALL_KEYS: [ResourceKey; if cfg!(feature = "icu_list") { 19 } else { 16 }] = [
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
    #[cfg(feature = "icu_list")]
    icu_list::provider::AndListV1Marker::KEY,
    #[cfg(feature = "icu_list")]
    icu_list::provider::OrListV1Marker::KEY,
    #[cfg(feature = "icu_list")]
    icu_list::provider::UnitListV1Marker::KEY,
    icu_locale_canonicalizer::provider::AliasesV1Marker::KEY,
    icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY,
    icu_plurals::provider::CardinalV1Marker::KEY,
    icu_plurals::provider::OrdinalV1Marker::KEY,
];
