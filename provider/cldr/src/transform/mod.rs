// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains the core transformer code from CLDR JSON to ICU4X Data Provider.
//!
//! Every ICU4X component should have its own private submodule and then export the types from here.

mod calendar;
mod datetime;
mod decimal;
#[cfg(feature = "icu_list")]
mod list;
mod locale_canonicalizer;
mod plurals;
mod time_zones;

use crate::error::Error;
use crate::CldrPaths;
use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::iter::IterableDynProvider;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::convert::TryFrom;
use std::path::PathBuf;

pub struct CldrJsonDataProvider;

impl CldrJsonDataProvider {
    pub fn try_new(
        cldr_paths: &dyn CldrPaths,
        uprops_root: PathBuf,
    ) -> Result<MultiForkByKeyProvider<Box<dyn IterableDynProvider<SerializeMarker>>>, Error> {
        Ok(MultiForkByKeyProvider {
            providers: vec![
                Box::new(locale_canonicalizer::aliases::AliasesProvider::try_from(
                    cldr_paths,
                )?),
                Box::new(datetime::symbols::DateSymbolsProvider::try_from(
                    cldr_paths,
                )?),
                Box::new(datetime::skeletons::DateSkeletonPatternsProvider::try_from(
                    cldr_paths,
                )?),
                Box::new(datetime::patterns::DatePatternsProvider::try_from(
                    cldr_paths,
                )?),
                Box::new(calendar::japanese::JapaneseErasProvider::try_from(
                    cldr_paths,
                )?),
                Box::new(
                    locale_canonicalizer::likely_subtags::LikelySubtagsProvider::try_from(
                        cldr_paths,
                    )?,
                ),
                Box::new(decimal::NumbersProvider::try_from(cldr_paths)?),
                Box::new(plurals::PluralsProvider::try_from(cldr_paths)?),
                Box::new(time_zones::TimeZonesProvider::try_from(cldr_paths)?),
                #[cfg(feature = "icu_list")]
                Box::new(list::ListProvider::try_from(cldr_paths, uprops_root)?),
            ],
        })
    }

    pub const ALL_KEYS: [ResourceKey; if cfg!(feature = "icu_list") { 18 } else { 15 }] = [
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
}
