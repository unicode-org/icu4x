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

use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use crate::error::Error;
use icu_provider::iter::IterableDynProvider;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;

macro_rules! cldr_json_data_provider {
    ($($ident: ident: $type: ty,)+) => {
        #[derive(Debug)]
        pub struct CldrJsonDataProvider {
            $(
                $ident: $type,
            )+
        }

        impl CldrJsonDataProvider {
            pub fn new(cldr_paths: &dyn CldrPaths) -> Result<Self, Error> {
                use std::convert::TryFrom;
                Ok(CldrJsonDataProvider {
                    $(
                        $ident: TryFrom::try_from(cldr_paths)?,
                    )+
                })
            }
        }

        impl DynProvider<SerializeMarker> for CldrJsonDataProvider {
            fn load_payload(
                &self,
                key: ResourceKey,
                req: &DataRequest,
            ) -> Result<DataResponse<SerializeMarker>, DataError> {
                $(
                    if <$type>::supported_keys().contains(&key) {
                        return DynProvider::load_payload(&self.$ident, key, req);
                    }
                )+
                Err(DataErrorKind::MissingResourceKey.with_req(key, req))
            }
        }

        impl IterableDynProvider<SerializeMarker> for CldrJsonDataProvider {
            fn supported_options_for_key(
                &self,
                key: &ResourceKey,
            ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
                $(
                    if <$type>::supported_keys().contains(key) {
                        return IterableDynProvider::supported_options_for_key(&self.$ident, key);
                    }
                )+
                Err(DataErrorKind::MissingResourceKey.with_key(*key))
            }
        }

        impl KeyedDataProvider for CldrJsonDataProvider {
            fn supported_keys() -> Vec<ResourceKey> {
                let mut result: Vec<ResourceKey> = vec![];
                $(
                    result.extend(<$type>::supported_keys());
                )+
                result
            }
        }
    };
}

#[cfg(feature = "icu_list")]
cldr_json_data_provider!(
    aliases: locale_canonicalizer::aliases::AliasesProvider,
    date_symbols: datetime::symbols::DateSymbolsProvider,
    date_skeletons: datetime::skeletons::DateSkeletonPatternsProvider,
    date_patterns: datetime::patterns::DatePatternsProvider,
    japanese: calendar::japanese::JapaneseErasProvider,
    likelysubtags: locale_canonicalizer::likely_subtags::LikelySubtagsProvider,
    numbers: decimal::NumbersProvider,
    plurals: plurals::PluralsProvider,
    time_zones: time_zones::TimeZonesProvider,
    list: list::ListProvider,
);

#[cfg(not(feature = "icu_list"))]
cldr_json_data_provider!(
    aliases: locale_canonicalizer::aliases::AliasesProvider,
    date_symbols: datetime::symbols::DateSymbolsProvider,
    date_skeletons: datetime::skeletons::DateSkeletonPatternsProvider,
    date_patterns: datetime::patterns::DatePatternsProvider,
    japanese: calendar::japanese::JapaneseErasProvider,
    likelysubtags: locale_canonicalizer::likely_subtags::LikelySubtagsProvider,
    numbers: decimal::NumbersProvider,
    plurals: plurals::PluralsProvider,
    time_zones: time_zones::TimeZonesProvider,
);
