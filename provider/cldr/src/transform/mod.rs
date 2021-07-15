// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod aliases;
mod dates;
mod likelysubtags;
mod numbers;
mod plurals;
mod time_zones;

pub use aliases::AliasesProvider;
pub use dates::{patterns::DatePatternsProvider, symbols::DateSymbolsProvider};
pub use likelysubtags::LikelySubtagsProvider;
pub use numbers::NumbersProvider;
pub use plurals::PluralsProvider;

use crate::support::LazyCldrProvider;
use crate::CldrPaths;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use icu_provider::serde::SerdeSeDataStructMarker;

use self::time_zones::TimeZonesProvider;

/// Returns a list of all [`ResourceKeys`](ResourceKey) that this provider can produce.
pub fn get_all_cldr_keys() -> Vec<ResourceKey> {
    let mut result: Vec<ResourceKey> = vec![];
    result.extend(&aliases::ALL_KEYS);
    result.extend(&dates::symbols::ALL_KEYS);
    result.extend(&dates::patterns::ALL_KEYS);
    result.extend(&likelysubtags::ALL_KEYS);
    result.extend(&numbers::ALL_KEYS);
    result.extend(&plurals::ALL_KEYS);
    result.extend(&time_zones::ALL_KEYS);
    result
}

#[derive(Debug)]
pub struct CldrJsonDataProvider<'a, 'd> {
    pub cldr_paths: &'a dyn CldrPaths,
    aliases: LazyCldrProvider<AliasesProvider<'d>>,
    date_symbols: LazyCldrProvider<DateSymbolsProvider<'d>>,
    date_patterns: LazyCldrProvider<DatePatternsProvider<'d>>,
    likelysubtags: LazyCldrProvider<LikelySubtagsProvider<'d>>,
    numbers: LazyCldrProvider<NumbersProvider>,
    plurals: LazyCldrProvider<PluralsProvider<'d>>,
    time_zones: LazyCldrProvider<TimeZonesProvider<'d>>,
}

impl<'a, 'd> CldrJsonDataProvider<'a, 'd> {
    pub fn new(cldr_paths: &'a dyn CldrPaths) -> Self {
        CldrJsonDataProvider {
            cldr_paths,
            aliases: Default::default(),
            date_symbols: Default::default(),
            date_patterns: Default::default(),
            likelysubtags: Default::default(),
            numbers: Default::default(),
            plurals: Default::default(),
            time_zones: Default::default(),
        }
    }
}

impl<'a, 'd, 's: 'd> DataProvider<'d, 's, SerdeSeDataStructMarker>
    for CldrJsonDataProvider<'a, 'd>
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, SerdeSeDataStructMarker>, DataError> {
        if let Some(result) = self.aliases.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.date_symbols.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.date_patterns.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.likelysubtags.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.numbers.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.plurals.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.time_zones.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        Err(DataError::UnsupportedResourceKey(req.resource_path.key))
    }
}

impl<'a, 'd> IterableDataProviderCore for CldrJsonDataProvider<'a, 'd> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        if let Some(resp) = self
            .aliases
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .date_symbols
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .date_patterns
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .likelysubtags
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .numbers
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .plurals
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        if let Some(resp) = self
            .time_zones
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        Err(DataError::UnsupportedResourceKey(*resc_key))
    }
}

impl<'a, 'd> KeyedDataProvider for CldrJsonDataProvider<'a, 'd> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        PluralsProvider::supports_key(resc_key)
            .or_else(|err| DateSymbolsProvider::or_else_supports_key(err, resc_key))
            .or_else(|err| DatePatternsProvider::or_else_supports_key(err, resc_key))
    }
}
