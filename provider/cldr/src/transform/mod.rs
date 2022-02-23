// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains the core transformer code from CLDR JSON to ICU4X Data Provider.
//!
//! Every ICU4X component should have its own private submodule and then export the types from here.

pub(crate) mod calendar;
pub(crate) mod datetime;
pub(crate) mod decimal;
#[cfg(feature = "icu_list")]
<<<<<<< HEAD
mod list;
mod locale_canonicalizer;
mod plurals;
mod time_zones;

use crate::support::KeyedDataProvider;
use crate::support::LazyCldrProvider;
use crate::CldrPaths;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;

macro_rules! cldr_json_data_provider {
    ($($ident: ident: $type: ty,)+) => {
        #[derive(Debug)]
        pub struct CldrJsonDataProvider<'a> {
            pub cldr_paths: &'a dyn CldrPaths,
            $(
                $ident: LazyCldrProvider<$type>,
            )+
        }

        impl<'a> CldrJsonDataProvider<'a> {
            pub fn new(cldr_paths: &'a dyn CldrPaths) -> Self {
                CldrJsonDataProvider {
                    cldr_paths,
                    $(
                        $ident: Default::default(),
                    )+
                }
            }
        }

        impl<'a> DynProvider<SerializeMarker> for CldrJsonDataProvider<'a> {
            fn load_payload(
                &self,
                key: ResourceKey,
                req: &DataRequest,
            ) -> Result<DataResponse<SerializeMarker>, DataError> {
                $(
                    if let Some(result) = self.$ident.try_load_serde(key, req, self.cldr_paths)? {
                        return Ok(result);
                    }
                )+
                Err(DataErrorKind::MissingResourceKey.with_req(key, req))
            }
        }

        impl<'a> IterableProvider for CldrJsonDataProvider<'a> {
            fn supported_options_for_key(
                &self,
                resc_key: &ResourceKey,
            ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
                $(
                    if let Some(resp) = self
                        .$ident
                        .try_supported_options(resc_key, self.cldr_paths)?
                    {
                        return Ok(Box::new(resp.into_iter()));
                    }
                )+
                Err(DataErrorKind::MissingResourceKey.with_key(*resc_key))
            }
        }

        impl<'a> KeyedDataProvider for CldrJsonDataProvider<'a> {
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
    week_data: datetime::week_data::WeekDataProvider,
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
    week_data: datetime::week_data::WeekDataProvider,
);
=======
pub(crate) mod list;
pub(crate) mod locale_canonicalizer;
pub(crate) mod plurals;
pub(crate) mod time_zones;
>>>>>>> upstream/main
